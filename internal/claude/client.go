// Package claude provides a client for invoking Claude Code CLI (or ClaudeBox)
// as a subprocess. It builds command-line arguments, streams JSON output,
// and detects usage limits for graceful shutdown.
package claude

import (
	"bufio"
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"os/exec"
	"strings"
)

// ClientConfig holds the configuration for the Claude CLI client.
type ClientConfig struct {
	Binary          string   // "claude" or "claudebox"
	OutputFormat    string   // "stream-json" (default) or "json"
	MaxTurns        int      // 0 = unlimited
	AllowedTools    []string // e.g. ["Bash","Read","Write","mcp__*"]
	DisallowedTools []string // e.g. ["Bash(rm -rf *)"] — takes precedence over AllowedTools
	SkipPermissions bool     // --dangerously-skip-permissions
	Model           string   // e.g. "opus", "sonnet"
}

// SessionRequest describes a single Claude invocation.
type SessionRequest struct {
	Prompt       string
	ProjectDir   string
	SessionID    string // For --resume
	SystemPrompt string // For --append-system-prompt
}

// SessionResult holds the outcome of a completed Claude session.
type SessionResult struct {
	SessionID  string  `json:"session_id"`
	CostUSD    float64 `json:"cost_usd"`
	DurationMs int64   `json:"duration_ms"`
	NumTurns   int     `json:"num_turns"`
	ExitCode   int     `json:"-"`
	UsageLimit bool    `json:"-"` // True if usage limit was detected
}

// StreamEvent represents a single line of stream-json output from Claude.
// Claude CLI stream-json emits events like:
//
//	{"type":"system","subtype":"init","message":{...}}
//	{"type":"assistant","message":{"content":"..."}}
//	{"type":"tool_use","tool":"Read","input":{...}}
//	{"type":"tool_result","content":"..."}
//	{"type":"result","subtype":"success","result":{...}}
type StreamEvent struct {
	Type    string          `json:"type"`
	Subtype string          `json:"subtype,omitempty"`
	Tool    string          `json:"tool,omitempty"`
	Message json.RawMessage `json:"message,omitempty"`
	Result  *SessionResult  `json:"result,omitempty"`
}

// StreamCallback is called for each stream event received from Claude.
type StreamCallback func(event StreamEvent)

// Client invokes Claude Code CLI as a subprocess.
type Client struct {
	config ClientConfig
}

// NewClient creates a Claude CLI client with the given config.
// Zero-value config fields are replaced with sensible defaults.
func NewClient(config ClientConfig) *Client {
	if config.Binary == "" {
		config.Binary = "claude"
	}
	if config.OutputFormat == "" {
		config.OutputFormat = "stream-json"
	}
	return &Client{config: config}
}

// Run executes a Claude session and streams events to the callback.
// It blocks until the session completes or the context is cancelled.
func (c *Client) Run(ctx context.Context, req SessionRequest, callback StreamCallback) (*SessionResult, error) {
	args := c.buildArgs(req)

	cmd := exec.CommandContext(ctx, c.config.Binary, args...) // #nosec G204 -- agent binary path from user config, by design
	if req.ProjectDir != "" {
		cmd.Dir = req.ProjectDir
	}

	// Prevent stdin hang in non-interactive mode.
	cmd.Stdin = nil

	stdout, err := cmd.StdoutPipe()
	if err != nil {
		return nil, fmt.Errorf("creating stdout pipe: %w", err)
	}

	// Capture stderr — also used for progress when --verbose is set.
	// Claude CLI outputs progress events to stderr with --verbose.
	stderrReader, err := cmd.StderrPipe()
	if err != nil {
		return nil, fmt.Errorf("creating stderr pipe: %w", err)
	}
	var stderrBuf bytes.Buffer

	if err := cmd.Start(); err != nil {
		return nil, fmt.Errorf("starting %s: %w", c.config.Binary, err)
	}

	var result *SessionResult
	var allOutput bytes.Buffer

	// Read stderr in background — captures both error output and verbose progress.
	// Claude CLI emits stream events to stderr when --verbose is used.
	stderrDone := make(chan struct{})
	go func() {
		defer close(stderrDone)
		stderrScanner := bufio.NewScanner(stderrReader)
		stderrScanner.Buffer(make([]byte, 0, 1024*1024), 1024*1024)
		for stderrScanner.Scan() {
			line := stderrScanner.Text()
			stderrBuf.WriteString(line)
			stderrBuf.WriteByte('\n')

			// Try parsing stderr lines as stream events too.
			event, parseErr := parseStreamLine(line)
			if parseErr == nil && callback != nil {
				callback(event)
				if event.Type == "result" && event.Result != nil {
					result = event.Result
				}
			}
		}
	}()

	scanner := bufio.NewScanner(stdout)
	scanner.Buffer(make([]byte, 0, 1024*1024), 1024*1024) // 1MB buffer for large outputs

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			continue
		}

		allOutput.WriteString(line)
		allOutput.WriteByte('\n')

		// Try parsing as stream-json event.
		event, parseErr := parseStreamLine(line)
		if parseErr == nil {
			if callback != nil {
				callback(event)
			}
			if event.Type == "result" && event.Result != nil {
				result = event.Result
			}
		}

		// Check for usage limit in any output.
		if detectUsageLimit(line) {
			if result == nil {
				result = &SessionResult{}
			}
			result.UsageLimit = true
		}
	}

	// Wait for stderr goroutine to finish reading.
	<-stderrDone

	// For "json" format: output is a single JSON blob (not wrapped in stream events).
	// Try parsing the full output as a SessionResult if we didn't get one from streaming.
	if result == nil && allOutput.Len() > 0 {
		var directResult SessionResult
		if err := json.Unmarshal(allOutput.Bytes(), &directResult); err == nil && directResult.SessionID != "" {
			result = &directResult
		}
	}

	exitErr := cmd.Wait()
	if result == nil {
		result = &SessionResult{}
	}

	if exitErr != nil {
		if exitError, ok := exitErr.(*exec.ExitError); ok {
			result.ExitCode = exitError.ExitCode()
			// Include last 500 chars of stderr for error diagnosis.
			stderr := strings.TrimSpace(stderrBuf.String())
			if len(stderr) > 500 {
				stderr = stderr[len(stderr)-500:]
			}
			if stderr != "" {
				return result, fmt.Errorf("%s exited %d: %s", c.config.Binary, result.ExitCode, stderr)
			}
		} else {
			return result, fmt.Errorf("waiting for %s: %w", c.config.Binary, exitErr)
		}
	}

	return result, nil
}

// buildArgs constructs the CLI arguments for a Claude invocation.
func (c *Client) buildArgs(req SessionRequest) []string {
	var args []string

	// Print mode (non-interactive) — -p must come before prompt.
	args = append(args, "-p")

	// Output format.
	args = append(args, "--output-format", c.config.OutputFormat)
	// stream-json requires --verbose in claude CLI print mode.
	// Note: some wrappers (e.g., claudebox) consume --verbose as their own flag.
	// Use "json" format for maximum compatibility across agent wrappers.
	if c.config.OutputFormat == "stream-json" {
		args = append(args, "--verbose")
	}

	// Session resume.
	if req.SessionID != "" {
		args = append(args, "--resume", req.SessionID)
	}

	// System prompt injection.
	if req.SystemPrompt != "" {
		args = append(args, "--append-system-prompt", req.SystemPrompt)
	}

	// Allowed tools.
	if len(c.config.AllowedTools) > 0 {
		args = append(args, "--allowedTools", strings.Join(c.config.AllowedTools, ","))
	}

	// Disallowed tools — takes precedence over allowed tools.
	// This is the primary defense against destructive commands.
	if len(c.config.DisallowedTools) > 0 {
		args = append(args, "--disallowedTools", strings.Join(c.config.DisallowedTools, ","))
	}

	// Max turns.
	if c.config.MaxTurns > 0 {
		args = append(args, "--max-turns", fmt.Sprintf("%d", c.config.MaxTurns))
	}

	// Skip permissions (dangerous — security notice required).
	if c.config.SkipPermissions {
		args = append(args, "--dangerously-skip-permissions")
	}

	// Model override.
	if c.config.Model != "" {
		args = append(args, "--model", c.config.Model)
	}

	// Prompt MUST be the last positional argument.
	args = append(args, req.Prompt)

	return args
}

// parseStreamLine parses a single line of stream-json output.
func parseStreamLine(line string) (StreamEvent, error) {
	if line == "" {
		return StreamEvent{}, fmt.Errorf("empty line")
	}

	var event StreamEvent
	if err := json.Unmarshal([]byte(line), &event); err != nil {
		return StreamEvent{}, fmt.Errorf("parsing stream event: %w", err)
	}
	return event, nil
}

// detectUsageLimit checks if a stream output line indicates a usage limit.
// The engine NEVER manages billing — it only detects limits and saves progress.
func detectUsageLimit(text string) bool {
	if text == "" {
		return false
	}
	lower := strings.ToLower(text)
	limitPhrases := []string{
		"usage limit",
		"rate limit",
		"billing limit",
		"quota exceeded",
		"token limit",
	}
	for _, phrase := range limitPhrases {
		if strings.Contains(lower, phrase) {
			return true
		}
	}
	return false
}
