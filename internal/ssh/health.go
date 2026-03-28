// Package ssh provides SSH connectivity checking and self-healing
// for ClaudeBox ↔ DevContainer communication. It detects disconnections,
// attempts reconnection via configurable scripts, and restarts services.
package ssh

import (
	"context"
	"fmt"
	"os/exec"
	"strings"
	"time"
)

// Default SSH configuration values.
const (
	DefaultHost       = "devcontainer"
	DefaultPort       = 22
	DefaultTimeout    = 5 * time.Second
	DefaultMaxRetries = 3
)

// HealthStatus represents the SSH connection state.
type HealthStatus string

const (
	StatusHealthy      HealthStatus = "healthy"
	StatusUnhealthy    HealthStatus = "unhealthy"
	StatusReconnecting HealthStatus = "reconnecting"
	StatusUnavailable  HealthStatus = "unavailable"
)

// HealthConfig configures the SSH health checker.
type HealthConfig struct {
	Host            string        // SSH host (default: devcontainer)
	Port            int           // SSH port (default: 22)
	User            string        // SSH user (optional, uses ssh config default)
	Timeout         time.Duration // Connection timeout
	MaxRetries      int           // Max reconnection attempts
	ReconnectScript string        // Script to run for reconnection (e.g., claude-dev.sh)
	ExecScript      string        // Script for remote execution (e.g., dev-exec.sh)
}

// HealthResult holds the outcome of a health check.
type HealthResult struct {
	Status   HealthStatus
	Latency  time.Duration
	Error    string
	Attempts int
}

// IsOK returns true if SSH is connected and responsive.
func (hr HealthResult) IsOK() bool {
	return hr.Status == StatusHealthy
}

// HealthChecker monitors and heals SSH connectivity.
type HealthChecker struct {
	config       HealthConfig
	lastResult   HealthResult
	lastCheckAt  time.Time
	reconnectCnt int
}

// NewHealthChecker creates an SSH health checker with the given config.
// Zero-value fields are replaced with defaults.
func NewHealthChecker(config HealthConfig) *HealthChecker {
	if config.Host == "" {
		config.Host = DefaultHost
	}
	if config.Port == 0 {
		config.Port = DefaultPort
	}
	if config.Timeout == 0 {
		config.Timeout = DefaultTimeout
	}
	if config.MaxRetries == 0 {
		config.MaxRetries = DefaultMaxRetries
	}
	return &HealthChecker{config: config}
}

// Check tests SSH connectivity by running "echo ok" on the remote host.
func (hc *HealthChecker) Check(ctx context.Context) HealthResult {
	start := time.Now()
	hc.lastCheckAt = start

	args := hc.buildExecArgs("echo ok")
	cmd := exec.CommandContext(ctx, "ssh", args...) // #nosec G204 -- SSH args from config, by design
	cmd.Stdin = nil

	output, err := cmd.CombinedOutput()
	latency := time.Since(start)

	if err != nil {
		result := HealthResult{
			Status:  StatusUnhealthy,
			Latency: latency,
			Error:   fmt.Sprintf("ssh check failed: %v (output: %s)", err, strings.TrimSpace(string(output))),
		}
		hc.lastResult = result
		return result
	}

	if !strings.Contains(string(output), "ok") {
		result := HealthResult{
			Status:  StatusUnhealthy,
			Latency: latency,
			Error:   fmt.Sprintf("unexpected ssh output: %s", strings.TrimSpace(string(output))),
		}
		hc.lastResult = result
		return result
	}

	result := HealthResult{
		Status:  StatusHealthy,
		Latency: latency,
	}
	hc.lastResult = result
	hc.reconnectCnt = 0
	return result
}

// Reconnect attempts to restore SSH by running the reconnect script.
// Returns the health result after reconnection attempt.
func (hc *HealthChecker) Reconnect(ctx context.Context) HealthResult {
	if hc.config.ReconnectScript == "" {
		return HealthResult{
			Status: StatusUnavailable,
			Error:  "no reconnect script configured",
		}
	}

	hc.reconnectCnt++
	if hc.reconnectCnt > hc.config.MaxRetries {
		return HealthResult{
			Status:   StatusUnavailable,
			Error:    fmt.Sprintf("max reconnection attempts exceeded (%d)", hc.config.MaxRetries),
			Attempts: hc.reconnectCnt,
		}
	}

	cmd := exec.CommandContext(ctx, hc.config.ReconnectScript) // #nosec G204 -- reconnect script from config, by design
	cmd.Stdin = nil
	if err := cmd.Run(); err != nil {
		return HealthResult{
			Status:   StatusReconnecting,
			Error:    fmt.Sprintf("reconnect script failed: %v", err),
			Attempts: hc.reconnectCnt,
		}
	}

	// Wait briefly for connection to stabilize — respects context cancellation.
	select {
	case <-time.After(2 * time.Second):
	case <-ctx.Done():
		return HealthResult{Status: StatusUnavailable, Error: "cancelled during reconnect"}
	}

	// Verify connectivity after reconnection
	result := hc.Check(ctx)
	result.Attempts = hc.reconnectCnt
	return result
}

// CheckAndHeal checks SSH and attempts reconnection if unhealthy.
func (hc *HealthChecker) CheckAndHeal(ctx context.Context) HealthResult {
	result := hc.Check(ctx)
	if result.IsOK() {
		return result
	}

	// Attempt reconnection
	return hc.Reconnect(ctx)
}

// Exec runs a command on the remote host via SSH or exec script.
// Returns combined stdout+stderr output.
func (hc *HealthChecker) Exec(ctx context.Context, command string) (string, error) {
	args := hc.buildRemoteExecArgs(command)

	cmd := exec.CommandContext(ctx, args[0], args[1:]...) // #nosec G204 -- remote exec args from config, by design
	cmd.Stdin = nil

	output, err := cmd.CombinedOutput()
	if err != nil {
		return string(output), fmt.Errorf("remote exec failed: %w", err)
	}
	return strings.TrimSpace(string(output)), nil
}

// LastResult returns the most recent health check result.
func (hc *HealthChecker) LastResult() HealthResult {
	return hc.lastResult
}

// buildExecArgs constructs SSH command arguments for a remote command.
func (hc *HealthChecker) buildExecArgs(command string) []string {
	args := []string{
		"-o", "ConnectTimeout=5",
		"-o", "StrictHostKeyChecking=no",
		"-o", "BatchMode=yes",
	}

	if hc.config.Port != 22 {
		args = append(args, "-p", fmt.Sprintf("%d", hc.config.Port))
	}

	host := hc.config.Host
	if hc.config.User != "" {
		host = hc.config.User + "@" + hc.config.Host
	}
	args = append(args, host, command)

	return args
}

// buildRemoteExecArgs constructs the command for remote execution.
// Prefers ExecScript (dev-exec.sh) if configured, falls back to SSH.
func (hc *HealthChecker) buildRemoteExecArgs(command string) []string {
	if hc.config.ExecScript != "" {
		return []string{hc.config.ExecScript, command}
	}

	// Fall back to direct SSH
	args := append([]string{"ssh"}, hc.buildExecArgs(command)...)
	return args
}
