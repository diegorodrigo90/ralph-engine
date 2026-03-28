// Package hooks loads and executes hooks.yaml lifecycle steps.
// Hooks run at defined points in the engine loop: preflight, pre-story,
// quality-gates, post-story, post-session. Each step can be required
// (blocks on failure) or optional (warns and continues).
package hooks

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"

	"gopkg.in/yaml.v3"
)

// DefaultTimeout is the maximum time a single hook step can run.
const DefaultTimeout = 5 * time.Minute

// HooksConfig represents the full hooks.yaml file.
type HooksConfig struct {
	Preflight    HookPhase `yaml:"preflight"`
	PreStory     HookPhase `yaml:"pre_story"`
	QualityGates HookPhase `yaml:"quality_gates"`
	PostStory    HookPhase `yaml:"post_story"`
	PostSession  HookPhase `yaml:"post_session"`
}

// HookPhase groups steps for one lifecycle point.
type HookPhase struct {
	Steps []HookStep `yaml:"steps"`
}

// HookStep is a single executable step within a phase.
type HookStep struct {
	Name     string   `yaml:"name"`
	Run      string   `yaml:"run"`
	Required bool     `yaml:"required"`
	Paths    []string `yaml:"paths"`
	Timeout  string   `yaml:"timeout"` // e.g., "5m", "30s" (optional, default 5m)
}

// StepResult holds the outcome of executing a single step.
type StepResult struct {
	Name     string
	OK       bool
	Required bool
	Output   string
	Duration time.Duration
	Skipped  bool // True when path filter excluded this step.
	Error    error
}

// PhaseResult holds the outcome of executing all steps in a phase.
type PhaseResult struct {
	Steps   []StepResult
	Blocked bool   // True if a required step failed.
	Reason  string // Which required step caused the block.
}

// Load reads hooks.yaml from the project's .ralph-engine/ directory.
// Returns nil config (not error) if hooks.yaml doesn't exist.
func Load(projectDir string) (*HooksConfig, error) {
	path := filepath.Join(projectDir, ".ralph-engine", "hooks.yaml")
	data, err := os.ReadFile(path) // #nosec G304 -- path is projectDir + known config path
	if err != nil {
		if os.IsNotExist(err) {
			return nil, nil // No hooks file — not an error.
		}
		return nil, fmt.Errorf("reading hooks.yaml: %w", err)
	}

	var cfg HooksConfig
	if err := yaml.Unmarshal(data, &cfg); err != nil {
		return nil, fmt.Errorf("parsing hooks.yaml: %w", err)
	}
	return &cfg, nil
}

// RunPhase executes all steps in a phase, respecting path filters and context cancellation.
// changedFiles is the list of files changed since last commit (from git diff).
// If changedFiles is nil, path filtering is disabled (all steps run).
func RunPhase(ctx context.Context, phase HookPhase, projectDir string, changedFiles []string, onStep func(StepResult)) PhaseResult {
	var result PhaseResult

	for _, step := range phase.Steps {
		if ctx.Err() != nil {
			break
		}

		// Skip empty run commands.
		if strings.TrimSpace(step.Run) == "" {
			continue
		}

		// Path filtering: skip if no changed files match the step's paths.
		if len(step.Paths) > 0 && changedFiles != nil && !MatchesAnyPath(changedFiles, step.Paths) {
			sr := StepResult{
				Name:     step.Name,
				OK:       true,
				Required: step.Required,
				Skipped:  true,
			}
			result.Steps = append(result.Steps, sr)
			if onStep != nil {
				onStep(sr)
			}
			continue
		}

		sr := executeStep(ctx, step, projectDir)
		result.Steps = append(result.Steps, sr)

		if onStep != nil {
			onStep(sr)
		}

		// Block on required step failure.
		if !sr.OK && sr.Required {
			result.Blocked = true
			result.Reason = fmt.Sprintf("required step '%s' failed", step.Name)
			break
		}
	}

	return result
}

// executeStep runs a single hook step with timeout.
// Uses process groups + explicit kill to handle stuck child processes.
func executeStep(ctx context.Context, step HookStep, projectDir string) StepResult {
	timeout := DefaultTimeout
	if step.Timeout != "" {
		if parsed, err := time.ParseDuration(step.Timeout); err == nil {
			timeout = parsed
		}
	}

	stepCtx, cancel := context.WithTimeout(ctx, timeout)
	defer cancel()

	start := time.Now()

	cmd := exec.Command("sh", "-c", step.Run) // #nosec G204 -- user controls hooks.yaml commands, by design
	cmd.Dir = projectDir
	setSysProcAttr(cmd)

	// Use Start + Wait with a done channel so we can kill the process group on timeout.
	var outputBuf strings.Builder
	cmd.Stdout = &outputBuf
	cmd.Stderr = &outputBuf

	if err := cmd.Start(); err != nil {
		return StepResult{
			Name: step.Name, Required: step.Required,
			Duration: time.Since(start), Error: err,
		}
	}

	// Wait for completion or timeout in a goroutine.
	doneCh := make(chan error, 1)
	go func() { doneCh <- cmd.Wait() }()

	var cmdErr error
	timedOut := false

	select {
	case cmdErr = <-doneCh:
		// Command finished normally.
	case <-stepCtx.Done():
		// Timeout or cancellation — kill process group.
		timedOut = true
		killProcessGroup(cmd)
		// Give a brief moment for cleanup, then continue.
		select {
		case <-doneCh:
		case <-time.After(2 * time.Second):
			// Process didn't die — force kill individual process.
			if cmd.Process != nil {
				_ = cmd.Process.Kill() // Best-effort cleanup.
			}
			<-doneCh
		}
	}

	duration := time.Since(start)
	sr := StepResult{
		Name:     step.Name,
		Required: step.Required,
		Output:   truncateOutput(outputBuf.String()),
		Duration: duration,
	}

	if timedOut {
		sr.OK = false
		sr.Error = fmt.Errorf("timeout after %v", timeout)
	} else if cmdErr != nil {
		sr.OK = false
		sr.Error = cmdErr
	} else {
		sr.OK = true
	}

	return sr
}

// MatchesAnyPath checks if any changed file matches any of the step's path patterns.
// Supports: exact match, single glob (*.ts), directory prefix (src/**), and suffix (**/*.graphql).
func MatchesAnyPath(changedFiles, patterns []string) bool {
	for _, file := range changedFiles {
		for _, pattern := range patterns {
			// Try standard filepath.Match first (handles single-level globs).
			if matched, _ := filepath.Match(pattern, file); matched {
				return true
			}

			if !strings.Contains(pattern, "**") {
				continue
			}

			// Handle ** patterns manually.
			parts := strings.SplitN(pattern, "**", 2)
			prefix := parts[0] // e.g., "apps/" from "apps/**"
			suffix := ""
			if len(parts) > 1 {
				suffix = strings.TrimPrefix(parts[1], "/") // e.g., "*.graphql" from "**/*.graphql"
			}

			// Prefix match: "apps/**" → file must start with "apps/"
			prefixMatch := prefix == "" || strings.HasPrefix(file, prefix)

			// Suffix match: "**/*.graphql" → file must end with matching pattern
			suffixMatch := true
			if suffix != "" {
				_, fileName := filepath.Split(file)
				matched, _ := filepath.Match(suffix, fileName)
				suffixMatch = matched
			}

			if prefixMatch && suffixMatch {
				return true
			}
		}
	}
	return false
}

// GetChangedFiles returns the list of files changed by the agent.
// It checks multiple git states to handle both pre-commit and post-commit scenarios:
//  1. Uncommitted changes (working tree + staged vs HEAD)
//  2. Last commit changes (HEAD vs HEAD~1) — for when agent already committed
//  3. Both combined (deduped) — covers the full picture
//
// This is critical because the agent may or may not have committed before quality gates run.
func GetChangedFiles(projectDir string) []string {
	seen := make(map[string]bool)
	var files []string

	addFiles := func(output string) {
		for _, line := range strings.Split(strings.TrimSpace(output), "\n") {
			if line != "" && !seen[line] {
				seen[line] = true
				files = append(files, line)
			}
		}
	}

	// 1. Modified files (tracked, uncommitted changes vs HEAD).
	cmd := exec.Command("git", "diff", "--name-only", "HEAD")
	cmd.Dir = projectDir
	if output, err := cmd.Output(); err == nil {
		addFiles(string(output))
	}

	// 2. Staged changes not yet committed.
	cmd2 := exec.Command("git", "diff", "--name-only", "--cached")
	cmd2.Dir = projectDir
	if output, err := cmd2.Output(); err == nil {
		addFiles(string(output))
	}

	// 3. Untracked new files (not yet added to git).
	cmd3 := exec.Command("git", "ls-files", "--others", "--exclude-standard")
	cmd3.Dir = projectDir
	if output, err := cmd3.Output(); err == nil {
		addFiles(string(output))
	}

	// 4. Last commit changes (HEAD vs HEAD~1) — for when agent already committed.
	cmd4 := exec.Command("git", "diff", "--name-only", "HEAD~1", "HEAD")
	cmd4.Dir = projectDir
	if output, err := cmd4.Output(); err == nil {
		addFiles(string(output))
	}

	return files
}

// truncateOutput limits output to 2000 chars to avoid memory bloat in results.
func truncateOutput(s string) string {
	const maxLen = 2000
	if len(s) > maxLen {
		return s[:maxLen] + "\n[output truncated]"
	}
	return s
}
