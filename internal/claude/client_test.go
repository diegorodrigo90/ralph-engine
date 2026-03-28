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

// assertNotContains checks that needle does NOT exist in the string slice.
func assertNotContains(t *testing.T, haystack []string, needle string) {
	t.Helper()
	for _, s := range haystack {
		if s == needle {
			t.Errorf("args %v should NOT contain %q", haystack, needle)
			return
		}
	}
}

// --- Additional tests for buildArgs edge cases ---

func TestBuildArgsAllOptionsSet(t *testing.T) {
	c := NewClient(ClientConfig{
		Binary:          "claudebox",
		OutputFormat:    "stream-json",
		MaxTurns:        25,
		AllowedTools:    []string{"Bash", "Read", "Write"},
		SkipPermissions: true,
		Model:           "opus",
	})

	args := c.buildArgs(SessionRequest{
		Prompt:       "implement story 2.1",
		ProjectDir:   "/workspace/project",
		SessionID:    "ses_resume_123",
		SystemPrompt: "You are a sprint bot.",
	})

	// -p must be first.
	if args[0] != "-p" {
		t.Errorf("first arg should be -p, got %q", args[0])
	}
	// Prompt must be last.
	if args[len(args)-1] != "implement story 2.1" {
		t.Errorf("last arg should be prompt, got %q", args[len(args)-1])
	}

	assertContains(t, args, "--output-format")
	assertContains(t, args, "stream-json")
	assertContains(t, args, "--verbose")
	assertContains(t, args, "--max-turns")
	assertContains(t, args, "25")
	assertContains(t, args, "--allowedTools")
	assertContains(t, args, "Bash,Read,Write")
	assertContains(t, args, "--dangerously-skip-permissions")
	assertContains(t, args, "--model")
	assertContains(t, args, "opus")
	assertContains(t, args, "--resume")
	assertContains(t, args, "ses_resume_123")
	assertContains(t, args, "--append-system-prompt")
	assertContains(t, args, "You are a sprint bot.")
}

func TestBuildArgsEmptyAllowedTools(t *testing.T) {
	tests := []struct {
		name  string
		tools []string
	}{
		{"nil tools", nil},
		{"empty slice", []string{}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			c := NewClient(ClientConfig{AllowedTools: tt.tools})
			args := c.buildArgs(SessionRequest{Prompt: "test"})

			assertNotContains(t, args, "--allowedTools")
		})
	}
}

func TestBuildArgsSkipPermissionsFalse(t *testing.T) {
	c := NewClient(ClientConfig{SkipPermissions: false})
	args := c.buildArgs(SessionRequest{Prompt: "test"})

	assertNotContains(t, args, "--dangerously-skip-permissions")
}

func TestBuildArgsEmptyPrompt(t *testing.T) {
	c := NewClient(ClientConfig{})
	args := c.buildArgs(SessionRequest{Prompt: ""})

	// Even with empty prompt, it should still be the last arg.
	if args[len(args)-1] != "" {
		t.Errorf("last arg should be empty prompt, got %q", args[len(args)-1])
	}
	// -p should still be first.
	if args[0] != "-p" {
		t.Errorf("first arg should be -p, got %q", args[0])
	}
}

func TestBuildArgsPrintFlagAlwaysFirst(t *testing.T) {
	tests := []struct {
		name string
		cfg  ClientConfig
		req  SessionRequest
	}{
		{
			name: "minimal config",
			cfg:  ClientConfig{},
			req:  SessionRequest{Prompt: "hello"},
		},
		{
			name: "full config",
			cfg:  ClientConfig{Model: "opus", MaxTurns: 10, SkipPermissions: true},
			req:  SessionRequest{Prompt: "test", SessionID: "abc"},
		},
		{
			name: "stream-json",
			cfg:  ClientConfig{OutputFormat: "stream-json"},
			req:  SessionRequest{Prompt: "test"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			c := NewClient(tt.cfg)
			args := c.buildArgs(tt.req)

			if len(args) == 0 {
				t.Fatal("args should not be empty")
			}
			if args[0] != "-p" {
				t.Errorf("first arg must be -p, got %q", args[0])
			}
		})
	}
}

func TestBuildArgsPromptAlwaysLast(t *testing.T) {
	tests := []struct {
		name   string
		prompt string
	}{
		{"normal prompt", "implement story 1.1"},
		{"prompt with special chars", "fix bug in user's profile (urgent!)"},
		{"multiline prompt", "line one\nline two\nline three"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			c := NewClient(ClientConfig{
				Model:    "opus",
				MaxTurns: 5,
			})
			args := c.buildArgs(SessionRequest{Prompt: tt.prompt})

			if args[len(args)-1] != tt.prompt {
				t.Errorf("last arg should be prompt %q, got %q", tt.prompt, args[len(args)-1])
			}
		})
	}
}

func TestBuildArgsNoSessionIDOmitsResume(t *testing.T) {
	c := NewClient(ClientConfig{})
	args := c.buildArgs(SessionRequest{Prompt: "test", SessionID: ""})

	assertNotContains(t, args, "--resume")
}

func TestBuildArgsNoModelOmitsModelFlag(t *testing.T) {
	c := NewClient(ClientConfig{Model: ""})
	args := c.buildArgs(SessionRequest{Prompt: "test"})

	assertNotContains(t, args, "--model")
}

// --- Additional tests for detectUsageLimit edge cases ---

func TestDetectUsageLimitComprehensive(t *testing.T) {
	tests := []struct {
		name     string
		text     string
		expected bool
	}{
		// All known limit phrases.
		{"usage limit", "You've reached your usage limit", true},
		{"rate limit", "rate limit exceeded", true},
		{"billing limit", "billing limit reached", true},
		{"quota exceeded", "Your quota exceeded for this period", true},
		{"token limit", "token limit has been reached", true},

		// Case variations (detectUsageLimit lowercases).
		{"uppercase USAGE LIMIT", "USAGE LIMIT EXCEEDED", true},
		{"mixed case Rate Limit", "Rate Limit Exceeded", true},
		{"mixed case Billing Limit", "Billing Limit Reached", true},
		{"mixed case Quota Exceeded", "Quota Exceeded", true},
		{"mixed case Token Limit", "Token Limit Hit", true},

		// Empty and whitespace.
		{"empty string", "", false},
		{"whitespace only", "   ", false},

		// Partial matches that should NOT trigger.
		{"just rate without limit", "rate increased", false},
		{"just limit without qualifier", "the limit is high", false},
		{"just usage without limit", "usage is normal", false},
		{"just billing alone", "billing address updated", false},
		{"just quota alone", "quota information", false},
		{"just token alone", "token refreshed successfully", false},
		{"exceeded alone", "expectations exceeded", false},

		// Normal messages that should not trigger.
		{"normal progress", "Working on story 1.1", false},
		{"code output", "function() { return true; }", false},
		{"error but not limit", "connection refused", false},

		// Phrase embedded in longer text.
		{"limit phrase in sentence", "Error: Your usage limit has been reached. Please wait.", true},
		{"rate limit in json", `{"error":"rate limit exceeded","code":429}`, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := detectUsageLimit(tt.text); got != tt.expected {
				t.Errorf("detectUsageLimit(%q) = %v, want %v", tt.text, got, tt.expected)
			}
		})
	}
}

// --- Additional tests for parseStreamLine edge cases ---

func TestParseStreamLineEdgeCases(t *testing.T) {
	tests := []struct {
		name      string
		input     string
		wantErr   bool
		wantType  string
		checkFunc func(t *testing.T, event StreamEvent)
	}{
		{
			name:    "empty string returns error",
			input:   "",
			wantErr: true,
		},
		{
			name:    "whitespace-only is invalid JSON",
			input:   "   ",
			wantErr: true,
		},
		{
			name:    "plain text is invalid",
			input:   "hello world",
			wantErr: true,
		},
		{
			name:    "incomplete JSON",
			input:   `{"type": "result"`,
			wantErr: true,
		},
		{
			name:    "array instead of object errors",
			input:   `[1, 2, 3]`,
			wantErr: true, // Valid JSON but cannot unmarshal into StreamEvent struct
		},
		{
			name:     "valid result with session_id",
			input:    `{"type":"result","result":{"session_id":"ses_xyz","cost_usd":0.05,"duration_ms":10000,"num_turns":3}}`,
			wantErr:  false,
			wantType: "result",
			checkFunc: func(t *testing.T, event StreamEvent) {
				if event.Result == nil {
					t.Fatal("Result should not be nil")
				}
				if event.Result.SessionID != "ses_xyz" {
					t.Errorf("SessionID = %q, want %q", event.Result.SessionID, "ses_xyz")
				}
				if event.Result.NumTurns != 3 {
					t.Errorf("NumTurns = %d, want 3", event.Result.NumTurns)
				}
			},
		},
		{
			name:     "valid system type",
			input:    `{"type":"system","message":"initializing"}`,
			wantErr:  false,
			wantType: "system",
		},
		{
			name:     "valid event with no result field",
			input:    `{"type":"tool_use"}`,
			wantErr:  false,
			wantType: "tool_use",
			checkFunc: func(t *testing.T, event StreamEvent) {
				if event.Result != nil {
					t.Error("Result should be nil for non-result type")
				}
			},
		},
		{
			name:     "empty JSON object",
			input:    `{}`,
			wantErr:  false,
			wantType: "", // no type field
		},
		{
			name:     "result with zero cost",
			input:    `{"type":"result","result":{"session_id":"ses_zero","cost_usd":0,"duration_ms":0,"num_turns":0}}`,
			wantErr:  false,
			wantType: "result",
			checkFunc: func(t *testing.T, event StreamEvent) {
				if event.Result == nil {
					t.Fatal("Result should not be nil")
				}
				if event.Result.CostUSD != 0 {
					t.Errorf("CostUSD = %f, want 0", event.Result.CostUSD)
				}
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			event, err := parseStreamLine(tt.input)

			if tt.wantErr {
				if err == nil {
					t.Error("expected error, got nil")
				}
				return
			}

			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}

			if tt.wantType != "" && event.Type != tt.wantType {
				t.Errorf("Type = %q, want %q", event.Type, tt.wantType)
			}

			if tt.checkFunc != nil {
				tt.checkFunc(t, event)
			}
		})
	}
}

// --- Additional tests for NewClient defaults ---

func TestNewClientDefaultsComprehensive(t *testing.T) {
	c := NewClient(ClientConfig{})

	tests := []struct {
		name string
		got  interface{}
		want interface{}
	}{
		{"Binary defaults to claude", c.config.Binary, "claude"},
		{"OutputFormat defaults to json", c.config.OutputFormat, "json"},
		{"MaxTurns defaults to 0", c.config.MaxTurns, 0},
		{"SkipPermissions defaults to false", c.config.SkipPermissions, false},
		{"Model defaults to empty", c.config.Model, ""},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.got != tt.want {
				t.Errorf("got %v, want %v", tt.got, tt.want)
			}
		})
	}

	if c.config.AllowedTools != nil {
		t.Errorf("AllowedTools should be nil by default, got %v", c.config.AllowedTools)
	}
}

func TestNewClientPreservesNonDefaults(t *testing.T) {
	c := NewClient(ClientConfig{
		Binary:          "custom-binary",
		OutputFormat:    "stream-json",
		MaxTurns:        42,
		AllowedTools:    []string{"Bash"},
		SkipPermissions: true,
		Model:           "sonnet",
	})

	if c.config.Binary != "custom-binary" {
		t.Errorf("Binary = %q, want %q", c.config.Binary, "custom-binary")
	}
	if c.config.OutputFormat != "stream-json" {
		t.Errorf("OutputFormat = %q, want %q", c.config.OutputFormat, "stream-json")
	}
	if c.config.MaxTurns != 42 {
		t.Errorf("MaxTurns = %d, want 42", c.config.MaxTurns)
	}
	if len(c.config.AllowedTools) != 1 || c.config.AllowedTools[0] != "Bash" {
		t.Errorf("AllowedTools = %v, want [Bash]", c.config.AllowedTools)
	}
	if !c.config.SkipPermissions {
		t.Error("SkipPermissions should be true")
	}
	if c.config.Model != "sonnet" {
		t.Errorf("Model = %q, want %q", c.config.Model, "sonnet")
	}
}
