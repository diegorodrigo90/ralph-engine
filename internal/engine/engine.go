// Package engine implements the core autonomous execution loop.
// It orchestrates Claude sessions, enforces quality gates between iterations,
// monitors resources, and persists progress for resume capability.
package engine

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"sync"
	"time"

	"github.com/diegorodrigo90/ralph-engine/internal/config"
	"github.com/diegorodrigo90/ralph-engine/internal/hooks"
	"github.com/diegorodrigo90/ralph-engine/internal/runner"
	"github.com/diegorodrigo90/ralph-engine/internal/system"
)

// DefaultCooldown is the pause between Claude sessions.
const DefaultCooldown = 10 * time.Second

// EngineStatus represents the current state of the engine.
type EngineStatus string

const (
	StatusIdle    EngineStatus = "idle"
	StatusRunning EngineStatus = "running"
	StatusPaused  EngineStatus = "paused"
	StatusStopped EngineStatus = "stopped"
	StatusBlocked EngineStatus = "blocked"
)

// ExitReason indicates why the engine stopped.
type ExitReason string

const (
	ExitAllComplete      ExitReason = "all_complete"
	ExitCircuitBreaker   ExitReason = "circuit_breaker"
	ExitUsageLimit       ExitReason = "usage_limit"
	ExitUserInterrupt    ExitReason = "user_interrupt"
	ExitResourceCritical ExitReason = "resource_critical"
	ExitBlocked          ExitReason = "blocked"
)

// PreflightResult holds the outcome of a single preflight check.
type PreflightResult struct {
	Name    string
	OK      bool
	Message string
}

// EngineOpts configures a new Engine instance.
type EngineOpts struct {
	ProjectDir        string
	StateDir          string
	Binary            string // "claude" or "claudebox"
	Model             string // Agent model (e.g., "opus", "sonnet")
	MaxTurns          int    // Max agent turns per session (0 = unlimited)
	AllowedTools      string // Comma-separated allowed tools
	DisallowedTools   string // Comma-separated denied tools (takes precedence)
	SkipPermissions   bool   // --dangerously-skip-permissions
	MaxFailures       int    // Circuit breaker threshold
	CooldownSeconds   int    // Seconds between sessions
	CooldownMinutes   int    // Circuit breaker cooldown
	StoriesPerSession int    // Target stories per Claude session
	DryRun            bool   // Show what would happen without calling agent
	MaxIterations     int    // Stop after N iterations (0 = infinite)
	SingleStory       string // Run only this story ID, then stop
	MaxGateRetries    int    // Max quality gate fix attempts (0 = unlimited)
	// Config sections passed through from config.yaml for prompt building.
	WorkflowType         string            // "bmad-v6", "basic", "tdd-strict"
	WorkflowCommands     map[string]string // Phase → agent command mapping
	WorkflowInstructions string            // Free-form workflow instructions for prompt
	QualityGate  string                  // "full", "standard", "minimal"
	Paths        *config.PathsConfig     // Project artifact paths
	Prompt       *config.PromptConfig   // Custom prompt sections
	Research     *config.ResearchConfig // Research tools config
	Hooks        *hooks.HooksConfig    // Lifecycle hooks (nil = no hooks)
}

// Engine is the core autonomous execution loop.
type Engine struct {
	opts           EngineOpts
	mu             sync.RWMutex // Protects status, sessionCount, exitReason.
	status         EngineStatus
	circuitBreaker *runner.CircuitBreaker
	resourceMon    *system.ResourceMonitor
	cooldown       time.Duration
	sessionCount   int
	exitReason     ExitReason
}

// New creates an Engine with validated configuration.
func New(opts EngineOpts) (*Engine, error) {
	if opts.ProjectDir == "" {
		return nil, fmt.Errorf("ProjectDir is required")
	}
	if opts.StateDir == "" {
		opts.StateDir = opts.ProjectDir
	}
	if opts.Binary == "" {
		opts.Binary = "claude"
	}
	if opts.MaxFailures == 0 {
		opts.MaxFailures = 3
	}
	if opts.CooldownMinutes == 0 {
		opts.CooldownMinutes = 5
	}
	if opts.StoriesPerSession == 0 {
		opts.StoriesPerSession = 4
	}

	cooldown := DefaultCooldown
	if opts.CooldownSeconds > 0 {
		cooldown = time.Duration(opts.CooldownSeconds) * time.Second
	}

	return &Engine{
		opts:           opts,
		status:         StatusIdle,
		circuitBreaker: runner.NewCircuitBreaker(opts.MaxFailures, opts.CooldownMinutes),
		resourceMon:    system.NewResourceMonitor(system.ResourceThresholds{}),
		cooldown:       cooldown,
	}, nil
}

// Status returns the current engine status (thread-safe).
func (e *Engine) Status() EngineStatus {
	e.mu.RLock()
	defer e.mu.RUnlock()
	return e.status
}

// ExitInfo returns the reason the engine stopped (thread-safe).
func (e *Engine) ExitInfo() ExitReason {
	e.mu.RLock()
	defer e.mu.RUnlock()
	return e.exitReason
}

// Preflight runs pre-execution checks and returns results.
// Each check is independent — all are run even if some fail.
func (e *Engine) Preflight(ctx context.Context) []PreflightResult {
	var results []PreflightResult

	// Check project directory exists
	if _, err := os.Stat(e.opts.ProjectDir); os.IsNotExist(err) {
		results = append(results, PreflightResult{
			Name:    "project directory",
			OK:      false,
			Message: fmt.Sprintf("project directory not found: %s", e.opts.ProjectDir),
		})
	} else {
		results = append(results, PreflightResult{
			Name:    "project directory",
			OK:      true,
			Message: fmt.Sprintf("project directory exists: %s", e.opts.ProjectDir),
		})
	}

	// Check agent binary is available
	if _, err := exec.LookPath(e.opts.Binary); err != nil {
		results = append(results, PreflightResult{
			Name:    "agent binary",
			OK:      false,
			Message: fmt.Sprintf("%s not found in PATH", e.opts.Binary),
		})
	} else {
		results = append(results, PreflightResult{
			Name:    "agent binary",
			OK:      true,
			Message: fmt.Sprintf("%s found", e.opts.Binary),
		})
	}

	// Check system resources
	snap, err := e.resourceMon.Check()
	if err != nil {
		results = append(results, PreflightResult{
			Name:    "system resources",
			OK:      false,
			Message: fmt.Sprintf("resource check failed: %v", err),
		})
	} else {
		status := e.resourceMon.Evaluate(snap)
		results = append(results, PreflightResult{
			Name:    "system resources",
			OK:      !status.ShouldStop,
			Message: status.Summary(),
		})
	}

	// Check state directory is writable
	stateTestFile := fmt.Sprintf("%s/.ralph-engine-preflight-test", e.opts.StateDir)
	if err := os.WriteFile(stateTestFile, []byte("test"), 0600); err != nil {
		results = append(results, PreflightResult{
			Name:    "state directory",
			OK:      false,
			Message: fmt.Sprintf("state directory not writable: %s", e.opts.StateDir),
		})
	} else {
		_ = os.Remove(stateTestFile) //nolint:errcheck // best-effort cleanup of temp test file
		results = append(results, PreflightResult{
			Name:    "state directory",
			OK:      true,
			Message: "state directory writable",
		})
	}

	return results
}
