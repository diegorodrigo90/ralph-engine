// Package config — validate.go provides pre-run validation of the merged config.
// It catches misconfigurations before the engine loop starts, avoiding
// cryptic runtime errors mid-session.
package config

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
)

// ValidationError represents a single config validation failure.
type ValidationError struct {
	Field      string // Config key (e.g., "agent.type", "tracker.status_file")
	Message    string // Human-readable error
	Suggestion string // Suggested fix (optional)
}

// Error implements the error interface.
func (e ValidationError) Error() string {
	s := fmt.Sprintf("%s: %s", e.Field, e.Message)
	if e.Suggestion != "" {
		s += fmt.Sprintf(" (%s)", e.Suggestion)
	}
	return s
}

// ValidationResult holds all validation errors and warnings.
type ValidationResult struct {
	Errors   []ValidationError
	Warnings []ValidationError
}

// OK returns true if there are no errors (warnings are acceptable).
func (r *ValidationResult) OK() bool {
	return len(r.Errors) == 0
}

// Summary returns a human-readable summary of all issues.
func (r *ValidationResult) Summary() string {
	if r.OK() && len(r.Warnings) == 0 {
		return "Config OK"
	}
	var b strings.Builder
	for _, e := range r.Errors {
		b.WriteString(fmt.Sprintf("  ✗ %s\n", e.Error()))
	}
	for _, w := range r.Warnings {
		b.WriteString(fmt.Sprintf("  ⚠ %s\n", w.Error()))
	}
	return b.String()
}

// Validate checks the merged config for issues before running.
// projectDir is needed to resolve relative paths.
func Validate(cfg *Config, projectDir string) *ValidationResult {
	r := &ValidationResult{}

	validateAgent(cfg, r)
	validateTracker(cfg, projectDir, r)
	validateWorkflow(cfg, r)
	validateQuality(cfg, r)
	validateResources(cfg, r)
	validateCircuitBreaker(cfg, r)
	validateSSH(cfg, projectDir, r)
	validatePaths(cfg, projectDir, r)
	validateResearch(cfg, r)

	return r
}

// validateAgent checks agent config.
func validateAgent(cfg *Config, r *ValidationResult) {
	if cfg.Agent.Type == "" {
		r.Errors = append(r.Errors, ValidationError{
			Field:      "agent.type",
			Message:    "agent type is required",
			Suggestion: "set agent.type to 'claude', 'claudebox', or your agent binary name",
		})
		return
	}

	// Check if binary exists in PATH.
	if _, err := exec.LookPath(cfg.Agent.Type); err != nil {
		r.Warnings = append(r.Warnings, ValidationError{
			Field:      "agent.type",
			Message:    fmt.Sprintf("'%s' not found in PATH", cfg.Agent.Type),
			Suggestion: fmt.Sprintf("install %s or set agent.type to the correct binary name", cfg.Agent.Type),
		})
	}

	if cfg.Agent.MaxStoriesPerSession < 1 {
		r.Warnings = append(r.Warnings, ValidationError{
			Field:      "agent.max_stories_per_session",
			Message:    fmt.Sprintf("value %d is < 1, will use default (5)", cfg.Agent.MaxStoriesPerSession),
			Suggestion: "set to 1-10 for optimal context management",
		})
	}

	if cfg.Agent.CooldownSeconds < 0 {
		r.Errors = append(r.Errors, ValidationError{
			Field:   "agent.cooldown_seconds",
			Message: "cannot be negative",
		})
	}
}

// validateTracker checks tracker config.
func validateTracker(cfg *Config, projectDir string, r *ValidationResult) {
	validTypes := map[string]bool{"file": true, "flat": true, "command": true}
	if !validTypes[cfg.Tracker.Type] {
		r.Errors = append(r.Errors, ValidationError{
			Field:      "tracker.type",
			Message:    fmt.Sprintf("unknown tracker type '%s'", cfg.Tracker.Type),
			Suggestion: "use 'file', 'flat', or 'command'",
		})
	}

	if cfg.Tracker.Type == "file" || cfg.Tracker.Type == "flat" || cfg.Tracker.Type == "" {
		statusPath := cfg.Tracker.StatusFile
		if statusPath == "" {
			r.Errors = append(r.Errors, ValidationError{
				Field:      "tracker.status_file",
				Message:    "status file path is required for file tracker",
				Suggestion: "set tracker.status_file to your sprint status file (e.g., 'sprint-status.yaml')",
			})
			return
		}

		// Resolve and check existence.
		fullPath := statusPath
		if !filepath.IsAbs(statusPath) {
			fullPath = filepath.Join(projectDir, statusPath)
		}
		if _, err := os.Stat(fullPath); os.IsNotExist(err) {
			r.Errors = append(r.Errors, ValidationError{
				Field:      "tracker.status_file",
				Message:    fmt.Sprintf("file not found: %s", statusPath),
				Suggestion: "create the file or update tracker.status_file path",
			})
		}
	}
}

// validateWorkflow checks workflow config.
func validateWorkflow(cfg *Config, r *ValidationResult) {
	validTypes := map[string]bool{"basic": true, "bmad-v6": true, "tdd-strict": true, "custom": true}
	if cfg.Workflow.Type != "" && !validTypes[cfg.Workflow.Type] {
		r.Warnings = append(r.Warnings, ValidationError{
			Field:      "workflow.type",
			Message:    fmt.Sprintf("unknown workflow type '%s', will use as custom", cfg.Workflow.Type),
			Suggestion: "known types: basic, bmad-v6, tdd-strict, custom",
		})
	}
}

// validateQuality checks quality config.
func validateQuality(cfg *Config, r *ValidationResult) {
	validTypes := map[string]bool{"minimal": true, "standard": true, "full": true, "custom": true}
	if cfg.Quality.Type != "" && !validTypes[cfg.Quality.Type] {
		r.Warnings = append(r.Warnings, ValidationError{
			Field:      "quality.type",
			Message:    fmt.Sprintf("unknown quality type '%s', will use as custom", cfg.Quality.Type),
			Suggestion: "known types: minimal, standard, full, custom",
		})
	}
}

// validateResources checks resource limits are sane.
func validateResources(cfg *Config, r *ValidationResult) {
	if cfg.Resources.MinFreeRAMMB < 0 {
		r.Errors = append(r.Errors, ValidationError{
			Field:   "resources.min_free_ram_mb",
			Message: "cannot be negative",
		})
	}
	if cfg.Resources.MaxCPULoadPercent < 0 || cfg.Resources.MaxCPULoadPercent > 100 {
		r.Errors = append(r.Errors, ValidationError{
			Field:      "resources.max_cpu_load_percent",
			Message:    fmt.Sprintf("value %d is out of range", cfg.Resources.MaxCPULoadPercent),
			Suggestion: "must be between 0 and 100",
		})
	}
	if cfg.Resources.MinFreeDiskGB < 0 {
		r.Errors = append(r.Errors, ValidationError{
			Field:   "resources.min_free_disk_gb",
			Message: "cannot be negative",
		})
	}
}

// validateCircuitBreaker checks circuit breaker config.
func validateCircuitBreaker(cfg *Config, r *ValidationResult) {
	if cfg.CircuitBreaker.MaxFailures < 1 {
		r.Warnings = append(r.Warnings, ValidationError{
			Field:      "circuit_breaker.max_failures",
			Message:    fmt.Sprintf("value %d is < 1, will use default (3)", cfg.CircuitBreaker.MaxFailures),
			Suggestion: "set to 1-10",
		})
	}
}

// validateSSH checks SSH config when enabled.
func validateSSH(cfg *Config, projectDir string, r *ValidationResult) {
	if !cfg.SSH.Enabled {
		return
	}

	if cfg.SSH.DevExecScript != "" {
		scriptPath := cfg.SSH.DevExecScript
		if !filepath.IsAbs(scriptPath) {
			scriptPath = filepath.Join(projectDir, scriptPath)
		}
		if _, err := os.Stat(scriptPath); os.IsNotExist(err) {
			r.Warnings = append(r.Warnings, ValidationError{
				Field:      "ssh.dev_exec_script",
				Message:    fmt.Sprintf("script not found: %s", cfg.SSH.DevExecScript),
				Suggestion: "create the script or disable ssh",
			})
		}
	}
}

// validatePaths checks configured artifact paths exist.
func validatePaths(cfg *Config, projectDir string, r *ValidationResult) {
	checkPath := func(field, path string) {
		if path == "" {
			return
		}
		fullPath := path
		if !filepath.IsAbs(path) {
			fullPath = filepath.Join(projectDir, path)
		}
		if _, err := os.Stat(fullPath); os.IsNotExist(err) {
			r.Warnings = append(r.Warnings, ValidationError{
				Field:      "paths." + field,
				Message:    fmt.Sprintf("path not found: %s", path),
				Suggestion: "create the path or remove from config",
			})
		}
	}

	checkPath("stories", cfg.Paths.Stories)
	checkPath("architecture", cfg.Paths.Architecture)
	checkPath("prd", cfg.Paths.PRD)
	checkPath("ux", cfg.Paths.UX)
	checkPath("decisions", cfg.Paths.Decisions)
	checkPath("status", cfg.Paths.Status)
	checkPath("rules", cfg.Paths.Rules)

	for key, path := range cfg.Paths.Custom {
		checkPath("custom."+key, path)
	}
}

// validateResearch checks research config consistency.
func validateResearch(cfg *Config, r *ValidationResult) {
	if !cfg.Research.Enabled {
		return
	}

	validStrategies := map[string]bool{"always": true, "story-start": true, "on-demand": true}
	if cfg.Research.Strategy != "" && !validStrategies[cfg.Research.Strategy] {
		r.Warnings = append(r.Warnings, ValidationError{
			Field:      "research.strategy",
			Message:    fmt.Sprintf("unknown strategy '%s'", cfg.Research.Strategy),
			Suggestion: "use 'always', 'story-start', or 'on-demand'",
		})
	}

	enabledCount := 0
	for i, tool := range cfg.Research.Tools {
		if !tool.Enabled {
			continue
		}
		enabledCount++
		prefix := "research.tools[" + strconv.Itoa(i) + "]"

		if tool.Name == "" {
			r.Warnings = append(r.Warnings, ValidationError{
				Field:   prefix + ".name",
				Message: "tool has no name",
			})
		}
		if tool.Priority < 1 {
			r.Warnings = append(r.Warnings, ValidationError{
				Field:      prefix + ".priority",
				Message:    "priority should be ≥ 1",
				Suggestion: "set priority to define search order (1 = first)",
			})
		}
	}

	if enabledCount == 0 {
		r.Warnings = append(r.Warnings, ValidationError{
			Field:      "research.tools",
			Message:    "research is enabled but no tools are configured",
			Suggestion: "add tools or set research.enabled: false",
		})
	}
}
