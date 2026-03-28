// Package claude provides a client for invoking Claude Code CLI (or ClaudeBox)
// as a subprocess. It builds command-line arguments, streams JSON output,
// and detects usage limits for graceful shutdown.
package claude

import (
	"bufio"
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
type StreamEvent struct {
	Type    string          `json:"type"`
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

	cmd := exec.CommandContext(ctx, c.config.Binary, args...)
	if req.ProjectDir != "" {
		cmd.Dir = req.ProjectDir
	}

	// Prevent stdin hang in non-interactive mode
	cmd.Stdin = nil

	stdout, err := cmd.StdoutPipe()
	if err != nil {
		return nil, fmt.Errorf("creating stdout pipe: %w", err)
	}

	if err := cmd.Start(); err != nil {
		return nil, fmt.Errorf("starting %s: %w", c.config.Binary, err)
	}

	var result *SessionResult
	scanner := bufio.NewScanner(stdout)
	scanner.Buffer(make([]byte, 0, 1024*1024), 1024*1024) // 1MB buffer for large outputs

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			continue
		}

		event, err := parseStreamLine(line)
		if err != nil {
			continue // Skip malformed lines
		}

		if callback != nil {
			callback(event)
		}

		if event.Type == "result" && event.Result != nil {
			result = event.Result
		}

		// Check for usage limit in any message content
		if detectUsageLimit(line) {
			if result == nil {
				result = &SessionResult{}
			}
			result.UsageLimit = true
		}
	}

	exitErr := cmd.Wait()
	if result == nil {
		result = &SessionResult{}
	}

	if exitErr != nil {
		if exitError, ok := exitErr.(*exec.ExitError); ok {
			result.ExitCode = exitError.ExitCode()
		} else {
			return result, fmt.Errorf("waiting for %s: %w", c.config.Binary, exitErr)
		}
	}

	return result, nil
}

// buildArgs constructs the CLI arguments for a Claude invocation.
func (c *Client) buildArgs(req SessionRequest) []string {
	var args []string

	// Non-interactive prompt mode
	args = append(args, "-p", req.Prompt)

	// Output format
	args = append(args, "--output-format", c.config.OutputFormat)

	// Session resume
	if req.SessionID != "" {
		args = append(args, "--resume", req.SessionID)
	}

	// System prompt injection
	if req.SystemPrompt != "" {
		args = append(args, "--append-system-prompt", req.SystemPrompt)
	}

	// Allowed tools
	if len(c.config.AllowedTools) > 0 {
		args = append(args, "--allowedTools", strings.Join(c.config.AllowedTools, ","))
	}

	// Max turns
	if c.config.MaxTurns > 0 {
		args = append(args, "--max-turns", fmt.Sprintf("%d", c.config.MaxTurns))
	}

	// Skip permissions (dangerous — security notice required)
	if c.config.SkipPermissions {
		args = append(args, "--dangerously-skip-permissions")
	}

	// Model override
	if c.config.Model != "" {
		args = append(args, "--model", c.config.Model)
	}

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
