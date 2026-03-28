package claude

import (
	"testing"
)

func TestNewClientDefaults(t *testing.T) {
	c := NewClient(ClientConfig{})

	if c.config.Binary != "claude" {
		t.Errorf("Binary = %q, want %q", c.config.Binary, "claude")
	}
	if c.config.OutputFormat != "json" {
		t.Errorf("OutputFormat = %q, want %q", c.config.OutputFormat, "json")
	}
	if c.config.MaxTurns != 0 {
		t.Errorf("MaxTurns = %d, want 0 (unlimited)", c.config.MaxTurns)
	}
}

func TestNewClientCustomBinary(t *testing.T) {
	c := NewClient(ClientConfig{
		Binary: "claudebox",
	})

	if c.config.Binary != "claudebox" {
		t.Errorf("Binary = %q, want %q", c.config.Binary, "claudebox")
	}
}

func TestBuildArgsNonInteractive(t *testing.T) {
	c := NewClient(ClientConfig{})
	args := c.buildArgs(SessionRequest{
		Prompt:     "implement story 1.1",
		ProjectDir: "/workspace/my-project",
	})

	assertContains(t, args, "-p")
	assertContains(t, args, "--output-format")
	assertContains(t, args, "json")
	// Prompt MUST be the last argument (positional arg for claude CLI).
	if args[len(args)-1] != "implement story 1.1" {
		t.Errorf("prompt should be last arg, got %q at position %d", args[len(args)-1], len(args)-1)
	}
}

func TestBuildArgsStreamJSONAddsVerbose(t *testing.T) {
	c := NewClient(ClientConfig{OutputFormat: "stream-json"})
	args := c.buildArgs(SessionRequest{Prompt: "test"})

	assertContains(t, args, "--verbose")
	assertContains(t, args, "stream-json")
}

func TestBuildArgsWithSessionResume(t *testing.T) {
	c := NewClient(ClientConfig{})
	args := c.buildArgs(SessionRequest{
		Prompt:    "continue",
		SessionID: "abc-123",
	})

	assertContains(t, args, "--resume")
	assertContains(t, args, "abc-123")
}

func TestBuildArgsWithAppendSystemPrompt(t *testing.T) {
	c := NewClient(ClientConfig{})
	args := c.buildArgs(SessionRequest{
		Prompt:       "implement story",
		SystemPrompt: "You are in a sprint loop. Save progress to state.json.",
	})

	assertContains(t, args, "--append-system-prompt")
	assertContains(t, args, "You are in a sprint loop. Save progress to state.json.")
}

func TestBuildArgsWithAllowedTools(t *testing.T) {
	c := NewClient(ClientConfig{
		AllowedTools: []string{"Bash", "Read", "Write", "Edit", "mcp__*"},
	})
	args := c.buildArgs(SessionRequest{
		Prompt: "test",
	})

	assertContains(t, args, "--allowedTools")
	assertContains(t, args, "Bash,Read,Write,Edit,mcp__*")
}

func TestBuildArgsWithMaxTurns(t *testing.T) {
	c := NewClient(ClientConfig{
		MaxTurns: 50,
	})
	args := c.buildArgs(SessionRequest{
		Prompt: "test",
	})

	assertContains(t, args, "--max-turns")
	assertContains(t, args, "50")
}

func TestBuildArgsSkipPermissions(t *testing.T) {
	c := NewClient(ClientConfig{
		SkipPermissions: true,
	})
	args := c.buildArgs(SessionRequest{Prompt: "test"})

	assertContains(t, args, "--dangerously-skip-permissions")
}

func TestBuildArgsWithModel(t *testing.T) {
	c := NewClient(ClientConfig{
		Model: "opus",
	})
	args := c.buildArgs(SessionRequest{Prompt: "test"})

	assertContains(t, args, "--model")
	assertContains(t, args, "opus")
}

func TestParseStreamEventResult(t *testing.T) {
	line := `{"type":"result","result":{"session_id":"ses_abc123","cost_usd":0.15,"duration_ms":45000,"num_turns":12}}`

	event, err := parseStreamLine(line)
	if err != nil {
		t.Fatalf("parseStreamLine() error: %v", err)
	}
	if event.Type != "result" {
		t.Errorf("Type = %q, want %q", event.Type, "result")
	}
	if event.Result == nil {
		t.Fatal("Result should not be nil for result event")
	}
	if event.Result.SessionID != "ses_abc123" {
		t.Errorf("SessionID = %q, want %q", event.Result.SessionID, "ses_abc123")
	}
	if event.Result.CostUSD != 0.15 {
		t.Errorf("CostUSD = %f, want 0.15", event.Result.CostUSD)
	}
}

func TestParseStreamEventAssistantMessage(t *testing.T) {
	line := `{"type":"assistant","message":{"content":"Working on story 1.1..."}}`

	event, err := parseStreamLine(line)
	if err != nil {
		t.Fatalf("parseStreamLine() error: %v", err)
	}
	if event.Type != "assistant" {
		t.Errorf("Type = %q, want %q", event.Type, "assistant")
	}
}

func TestParseStreamEventInvalidJSON(t *testing.T) {
	_, err := parseStreamLine("not json at all")
	if err == nil {
		t.Error("parseStreamLine() should error on invalid JSON")
	}
}

func TestParseStreamEventEmptyLine(t *testing.T) {
	_, err := parseStreamLine("")
	if err == nil {
		t.Error("parseStreamLine() should error on empty line")
	}
}

func TestDetectUsageLimit(t *testing.T) {
	tests := []struct {
		name     string
		text     string
		expected bool
	}{
		{"usage limit message", "You've reached your usage limit", true},
		{"rate limit message", "rate limit exceeded", true},
		{"billing message", "billing limit reached", true},
		{"normal message", "Working on story 1.1", false},
		{"empty", "", false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := detectUsageLimit(tt.text); got != tt.expected {
				t.Errorf("detectUsageLimit(%q) = %v, want %v", tt.text, got, tt.expected)
			}
		})
	}
}

// assertContains checks that needle exists in the string slice.
func assertContains(t *testing.T, haystack []string, needle string) {
	t.Helper()
	for _, s := range haystack {
		if s == needle {
			return
		}
	}
	t.Errorf("args %v does not contain %q", haystack, needle)
}
