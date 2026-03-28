// Package deps checks for runtime dependencies and suggests installation
// when tools are missing. Ensures the engine can operate before starting.
package deps

import (
	"os/exec"
)

// Dependency describes a required or optional external tool.
type Dependency struct {
	Name        string // Binary name (e.g., "claude", "git", "docker")
	Required    bool   // If true, missing = fatal error
	InstallHint string // Human-readable install instructions
}

// CheckResult holds the outcome of checking a single dependency.
type CheckResult struct {
	Name        string
	Found       bool
	Path        string // Full path to binary
	Required    bool
	InstallHint string
}

// CheckBinary checks if a binary exists in PATH.
func CheckBinary(name string) CheckResult {
	path, err := exec.LookPath(name)
	return CheckResult{
		Name:  name,
		Found: err == nil,
		Path:  path,
	}
}

// CheckAll verifies all dependencies and returns results.
func CheckAll(deps []Dependency) []CheckResult {
	results := make([]CheckResult, len(deps))
	for i, dep := range deps {
		result := CheckBinary(dep.Name)
		result.Required = dep.Required
		result.InstallHint = dep.InstallHint
		results[i] = result
	}
	return results
}

// MissingRequired returns only the required dependencies that are missing.
func MissingRequired(results []CheckResult) []CheckResult {
	var missing []CheckResult
	for _, r := range results {
		if r.Required && !r.Found {
			missing = append(missing, r)
		}
	}
	return missing
}

// DefaultDeps returns the standard dependency list for an agent binary.
func DefaultDeps(agentBinary string) []Dependency {
	return []Dependency{
		{Name: agentBinary, Required: true, InstallHint: "Install from https://claude.ai/code or use: npm install -g @anthropic-ai/claude-code"},
		{Name: "git", Required: true, InstallHint: "Install: apt install git / brew install git"},
		{Name: "docker", Required: false, InstallHint: "Install from https://docker.com (recommended for container isolation)"},
		{Name: "jq", Required: false, InstallHint: "Install: apt install jq / brew install jq (useful for JSON processing)"},
		{Name: "ssh", Required: false, InstallHint: "Install: apt install openssh-client (needed for DevContainer SSH)"},
	}
}
