package claude

import (
	"encoding/json"
	"strings"
	"testing"
)

func TestNewClientDefaults(t *testing.T) {
	c := NewClient(ClientConfig{})

	if c.config.Binary != "claude" {
		t.Errorf("Binary = %q, want %q", c.config.Binary, "claude")
	}
	if c.config.OutputFormat != "stream-json" {
		t.Errorf("OutputFormat = %q, want %q", c.config.OutputFormat, "stream-json")
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
	assertContains(t, args, "stream-json")
	assertContains(t, args, "--verbose") // stream-json requires --verbose
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
		// Strict stderr patterns that SHOULD trigger.
		{"stderr usage limit hit", "You've hit your usage limit", true},
		{"stderr usage limit have", "You have hit your usage limit", true},
		{"stderr usage limit reached", "usage limit reached", true},
		{"stderr rate limit exceeded", "rate limit exceeded", true},
		{"stderr billing limit", "billing limit reached", true},
		{"stderr quota exceeded", "quota exceeded", true},
		{"stderr token limit", "token limit reached", true},
		{"stderr too many requests", "too many requests", true},
		{"stderr 429", "429 too many requests", true},
		{"stderr api rate limit", "api rate limit", true},

		// Non-JSON normal text should NOT trigger (false positive fix).
		{"normal message", "Working on story 1.1", false},
		{"empty", "", false},
		{"agent text mentioning usage limit", "I checked the usage limit documentation and it says...", false},
		{"agent text mentioning rate limit", "The rate limit for this API is 100 requests per minute", false},
		{"vague limit text", "You've reached your usage limit", true}, // Strict pattern match

		// JSON assistant events should NEVER trigger (false positive fix).
		{"assistant event with usage limit in content", `{"type":"assistant","message":{"text":"I checked the usage limit documentation"}}`, false},
		{"assistant event with rate limit text", `{"type":"assistant","message":{"text":"The rate limit is configured correctly"}}`, false},

		// JSON system/error events SHOULD trigger.
		{"system event with usage limit", `{"type":"system","subtype":"usage_limit","message":{"text":"usage limit reached"}}`, true},
		{"error event with rate limit", `{"type":"error","subtype":"rate_limit","message":{"text":"rate limit exceeded"}}`, true},
		{"system event with limit in message only", `{"type":"system","subtype":"info","message":{"text":"billing limit reached"}}`, true},
		{"system event with limit in subtype only", `{"type":"system","subtype":"quota exceeded","message":{"text":""}}`, true},

		// JSON system/error events without limit phrases should NOT trigger.
		{"system event normal", `{"type":"system","subtype":"init","message":{"text":"session started"}}`, false},
		{"error event no limit", `{"type":"error","subtype":"timeout","message":{"text":"connection timeout"}}`, false},

		// Other JSON event types should NOT trigger.
		{"result event", `{"type":"result","subtype":"success","message":{"text":"usage limit info logged"}}`, false},
		{"tool_use event", `{"type":"tool_use","message":{"text":"checking usage limit"}}`, false},
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
		// Strict stderr patterns (non-JSON text).
		{"stderr you've hit usage limit", "You've hit your usage limit", true},
		{"stderr you have hit usage limit", "You have hit your usage limit", true},
		{"stderr usage limit reached", "usage limit reached", true},
		{"stderr rate limit exceeded", "rate limit exceeded", true},
		{"stderr billing limit reached", "billing limit reached", true},
		{"stderr quota exceeded", "quota exceeded for this period", true},
		{"stderr token limit reached", "token limit reached", true},
		{"stderr too many requests", "Error: too many requests", true},
		{"stderr 429 too many", "HTTP 429 too many requests", true},
		{"stderr api rate limit", "api rate limit hit", true},

		// Case variations in strict patterns.
		{"uppercase TOO MANY REQUESTS", "TOO MANY REQUESTS", true},
		{"mixed case Rate Limit Exceeded", "Rate Limit Exceeded", true},
		{"mixed case Billing Limit Reached", "Billing Limit Reached", true},
		{"mixed case Quota Exceeded", "Quota Exceeded", true},

		// Empty and whitespace.
		{"empty string", "", false},
		{"whitespace only", "   ", false},

		// Non-JSON text that does NOT match strict patterns (false positive prevention).
		{"vague usage limit mention", "I reviewed the usage limit configuration", false},
		{"vague rate limit mention", "The rate limit for the API is 100/min", false},
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

		// JSON assistant events — NEVER trigger (false positive fix).
		{"assistant mentions usage limit", `{"type":"assistant","message":{"text":"The usage limit is documented here"}}`, false},
		{"assistant mentions rate limit", `{"type":"assistant","message":{"text":"rate limit exceeded in the test"}}`, false},

		// JSON system/error events — SHOULD trigger when limit phrase present.
		{"system event usage limit", `{"type":"system","subtype":"limit","message":{"text":"usage limit reached"}}`, true},
		{"error event rate limit", `{"type":"error","subtype":"rate_limit","message":{"text":"rate limit"}}`, true},
		{"error event quota", `{"type":"error","subtype":"","message":{"text":"quota exceeded"}}`, true},

		// JSON result/tool events — should NOT trigger even with limit phrases.
		{"result event with limit text", `{"type":"result","message":{"text":"usage limit info"}}`, false},
		{"tool_use event", `{"type":"tool_use","message":{"text":"rate limit check"}}`, false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := detectUsageLimit(tt.text); got != tt.expected {
				t.Errorf("detectUsageLimit(%q) = %v, want %v", tt.text, got, tt.expected)
			}
		})
	}
}

func TestContainsLimitPhrase(t *testing.T) {
	tests := []struct {
		text string
		want bool
	}{
		{"", false},
		{"usage limit", true},
		{"rate limit exceeded", true},
		{"billing limit reached", true},
		{"quota exceeded", true},
		{"token limit hit", true},
		{"normal text", false},
		{"USAGE LIMIT", true}, // case insensitive
	}
	for _, tt := range tests {
		if got := containsLimitPhrase(tt.text); got != tt.want {
			t.Errorf("containsLimitPhrase(%q) = %v, want %v", tt.text, got, tt.want)
		}
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
		{"OutputFormat defaults to stream-json", c.config.OutputFormat, "stream-json"},
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

// --- Tests for extractToolDetails and formatToolDetail ---

func TestFormatToolDetailRead(t *testing.T) {
	input := json.RawMessage(`{"file_path":"/workspace/main.go"}`)
	got := formatToolDetail("Read", input)
	if got != "Read /workspace/main.go" {
		t.Errorf("formatToolDetail(Read) = %q, want %q", got, "Read /workspace/main.go")
	}
}

func TestFormatToolDetailWrite(t *testing.T) {
	input := json.RawMessage(`{"file_path":"/workspace/output.txt"}`)
	got := formatToolDetail("Write", input)
	if got != "Write /workspace/output.txt" {
		t.Errorf("formatToolDetail(Write) = %q, want %q", got, "Write /workspace/output.txt")
	}
}

func TestFormatToolDetailEdit(t *testing.T) {
	input := json.RawMessage(`{"file_path":"/workspace/config.yaml","old_string":"foo","new_string":"bar"}`)
	got := formatToolDetail("Edit", input)
	if got != "Edit /workspace/config.yaml" {
		t.Errorf("formatToolDetail(Edit) = %q, want %q", got, "Edit /workspace/config.yaml")
	}
}

func TestFormatToolDetailBash(t *testing.T) {
	input := json.RawMessage(`{"command":"pnpm test"}`)
	got := formatToolDetail("Bash", input)
	if got != "Bash $ pnpm test" {
		t.Errorf("formatToolDetail(Bash) = %q, want %q", got, "Bash $ pnpm test")
	}
}

func TestFormatToolDetailBashTruncatesLongCommand(t *testing.T) {
	longCmd := strings.Repeat("x", 100)
	input := json.RawMessage(`{"command":"` + longCmd + `"}`)
	got := formatToolDetail("Bash", input)
	// Should be truncated to 80 chars + "..."
	if !strings.HasPrefix(got, "Bash $ ") {
		t.Errorf("formatToolDetail(Bash) should start with 'Bash $ ', got %q", got)
	}
	if !strings.HasSuffix(got, "...") {
		t.Errorf("formatToolDetail(Bash) with long command should end with '...', got %q", got)
	}
}

func TestFormatToolDetailGlob(t *testing.T) {
	input := json.RawMessage(`{"pattern":"**/*.ts"}`)
	got := formatToolDetail("Glob", input)
	if got != "Glob **/*.ts" {
		t.Errorf("formatToolDetail(Glob) = %q, want %q", got, "Glob **/*.ts")
	}
}

func TestFormatToolDetailGrep(t *testing.T) {
	input := json.RawMessage(`{"pattern":"func\\s+Test"}`)
	got := formatToolDetail("Grep", input)
	if got != `Grep func\s+Test` {
		t.Errorf("formatToolDetail(Grep) = %q, want %q", got, `Grep func\s+Test`)
	}
}

func TestFormatToolDetailSkill(t *testing.T) {
	input := json.RawMessage(`{"skill":"dev"}`)
	got := formatToolDetail("Skill", input)
	if got != "Skill /dev" {
		t.Errorf("formatToolDetail(Skill) = %q, want %q", got, "Skill /dev")
	}
}

func TestFormatToolDetailAgent(t *testing.T) {
	input := json.RawMessage(`{"description":"Search for auth patterns"}`)
	got := formatToolDetail("Agent", input)
	if got != "Agent: Search for auth patterns" {
		t.Errorf("formatToolDetail(Agent) = %q, want %q", got, "Agent: Search for auth patterns")
	}
}

func TestFormatToolDetailMCPTool(t *testing.T) {
	input := json.RawMessage(`{"query":"nestjs guards"}`)
	got := formatToolDetail("mcp__archon__rag_search", input)
	if !strings.HasPrefix(got, "MCP archon.rag_search(") {
		t.Errorf("formatToolDetail(MCP) should start with 'MCP archon.rag_search(', got %q", got)
	}
	if !strings.Contains(got, "query=") {
		t.Errorf("formatToolDetail(MCP) should contain param, got %q", got)
	}
}

func TestFormatToolDetailMCPToolNoParams(t *testing.T) {
	input := json.RawMessage(`{}`)
	got := formatToolDetail("mcp__archon__health_check", input)
	if got != "MCP archon.health_check()" {
		t.Errorf("formatToolDetail(MCP no params) = %q, want %q", got, "MCP archon.health_check()")
	}
}

func TestFormatToolDetailNilInput(t *testing.T) {
	got := formatToolDetail("Read", nil)
	if got != "Read" {
		t.Errorf("formatToolDetail with nil input = %q, want %q", got, "Read")
	}
}

func TestFormatToolDetailMalformedJSON(t *testing.T) {
	input := json.RawMessage(`not json`)
	got := formatToolDetail("Read", input)
	if got != "Read" {
		t.Errorf("formatToolDetail with bad JSON = %q, want %q", got, "Read")
	}
}

func TestFormatToolDetailUnknownTool(t *testing.T) {
	input := json.RawMessage(`{"foo":"bar"}`)
	got := formatToolDetail("CustomTool", input)
	if got != "CustomTool" {
		t.Errorf("formatToolDetail(unknown) = %q, want %q", got, "CustomTool")
	}
}

func TestFormatToolDetailMissingExpectedParam(t *testing.T) {
	// Read without file_path param should fall back to just the name.
	input := json.RawMessage(`{"other":"value"}`)
	got := formatToolDetail("Read", input)
	if got != "Read" {
		t.Errorf("formatToolDetail(Read missing file_path) = %q, want %q", got, "Read")
	}
}

func TestExtractToolDetailsToolUseBlocks(t *testing.T) {
	msg := json.RawMessage(`{"content":[
		{"type":"tool_use","name":"Read","input":{"file_path":"/workspace/main.go"}},
		{"type":"tool_use","name":"Bash","input":{"command":"go test ./..."}}
	]}`)
	details := extractToolDetails(msg)
	if len(details) != 2 {
		t.Fatalf("expected 2 details, got %d", len(details))
	}
	if details[0] != "Read /workspace/main.go" {
		t.Errorf("details[0] = %q, want %q", details[0], "Read /workspace/main.go")
	}
	if details[1] != "Bash $ go test ./..." {
		t.Errorf("details[1] = %q, want %q", details[1], "Bash $ go test ./...")
	}
}

func TestExtractToolDetailsTextBlock(t *testing.T) {
	msg := json.RawMessage(`{"content":[
		{"type":"text","text":"Let me read the file and understand the structure of this project first."}
	]}`)
	details := extractToolDetails(msg)
	if len(details) != 1 {
		t.Fatalf("expected 1 detail, got %d", len(details))
	}
	if !strings.HasPrefix(details[0], "Text:") {
		t.Errorf("text block detail should start with 'Text:', got %q", details[0])
	}
}

func TestExtractToolDetailsShortTextIgnored(t *testing.T) {
	msg := json.RawMessage(`{"content":[
		{"type":"text","text":"OK"}
	]}`)
	details := extractToolDetails(msg)
	if len(details) != 0 {
		t.Errorf("short text (<=10 chars) should be ignored, got %d details", len(details))
	}
}

func TestExtractToolDetailsLongTextTruncated(t *testing.T) {
	longText := strings.Repeat("a", 200)
	msg := json.RawMessage(`{"content":[{"type":"text","text":"` + longText + `"}]}`)
	details := extractToolDetails(msg)
	if len(details) != 1 {
		t.Fatalf("expected 1 detail, got %d", len(details))
	}
	if !strings.HasSuffix(details[0], "...") {
		t.Errorf("long text should be truncated with '...', got %q", details[0])
	}
}

func TestExtractToolDetailsNilMessage(t *testing.T) {
	details := extractToolDetails(nil)
	if details != nil {
		t.Errorf("nil message should return nil, got %v", details)
	}
}

func TestExtractToolDetailsEmptyContent(t *testing.T) {
	msg := json.RawMessage(`{"content":[]}`)
	details := extractToolDetails(msg)
	if len(details) != 0 {
		t.Errorf("empty content should return empty details, got %d", len(details))
	}
}

func TestExtractToolDetailsMalformedMessage(t *testing.T) {
	msg := json.RawMessage(`not valid json`)
	details := extractToolDetails(msg)
	if details != nil {
		t.Errorf("malformed message should return nil, got %v", details)
	}
}

func TestExtractToolDetailsNoContentField(t *testing.T) {
	msg := json.RawMessage(`{"role":"assistant"}`)
	details := extractToolDetails(msg)
	if details != nil {
		t.Errorf("message without content should return nil, got %v", details)
	}
}

func TestExtractToolDetailsMixedBlocks(t *testing.T) {
	msg := json.RawMessage(`{"content":[
		{"type":"text","text":"I will now read the file to understand the code."},
		{"type":"tool_use","name":"Read","input":{"file_path":"/workspace/main.go"}},
		{"type":"tool_use","name":"Glob","input":{"pattern":"**/*.go"}},
		{"type":"text","text":"short"}
	]}`)
	details := extractToolDetails(msg)
	// text (>10 chars) + Read + Glob = 3 (short text ignored)
	if len(details) != 3 {
		t.Fatalf("expected 3 details from mixed blocks, got %d: %v", len(details), details)
	}
	if !strings.HasPrefix(details[0], "Text:") {
		t.Errorf("first detail should be text, got %q", details[0])
	}
	if details[1] != "Read /workspace/main.go" {
		t.Errorf("second detail = %q, want Read", details[1])
	}
	if details[2] != "Glob **/*.go" {
		t.Errorf("third detail = %q, want Glob", details[2])
	}
}

// --- Tests for detectUsageLimit edge cases ---

func TestDetectUsageLimitAgentTextVariations(t *testing.T) {
	// These are all wrapped in assistant JSON events — agent TALKING about limits.
	// NONE should trigger usage limit detection.
	tests := []struct {
		name string
		text string
	}{
		{
			"agent documenting usage limit handling",
			`{"type":"assistant","message":{"text":"I've documented the usage limit handling in the README"}}`,
		},
		{
			"agent discussing rate limit middleware",
			`{"type":"assistant","message":{"text":"The rate limit middleware is configured to allow 100 req/s"}}`,
		},
		{
			"agent referencing token limit config",
			`{"type":"assistant","message":{"text":"Token limit is set to 4096 in config.yaml"}}`,
		},
		{
			"agent mentioning quota exceeded handler",
			`{"type":"assistant","message":{"text":"I'll check the quota exceeded handler to fix the error"}}`,
		},
		{
			"agent implementing billing limit feature",
			`{"type":"assistant","message":{"text":"Adding billing limit reached notification for users"}}`,
		},
		{
			"agent writing test for too many requests",
			`{"type":"assistant","message":{"text":"Writing test for 429 too many requests error handling"}}`,
		},
		{
			"agent discussing api rate limit design",
			`{"type":"assistant","message":{"text":"The api rate limit should use a sliding window algorithm"}}`,
		},
		{
			"agent with usage limit in code snippet",
			`{"type":"assistant","message":{"text":"if err.Code == 'usage_limit_reached' { return retry() }"}}`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if detectUsageLimit(tt.text) {
				t.Errorf("detectUsageLimit should NOT trigger on assistant event: %s", tt.text)
			}
		})
	}
}

func TestDetectUsageLimitStderrVariations(t *testing.T) {
	// These are non-JSON stderr messages that SHOULD trigger limit detection.
	tests := []struct {
		name string
		text string
	}{
		{
			"billing period usage limit",
			"Error: You've hit your usage limit for this billing period",
		},
		{
			"HTTP 429 error",
			"HTTP 429: Too Many Requests",
		},
		{
			"API rate limit with retry",
			"API rate limit exceeded, retry after 60s",
		},
		{
			"quota exceeded with details",
			"Error: quota exceeded - upgrade your plan at https://example.com",
		},
		{
			"billing limit with account info",
			"billing limit reached for account acc_123456",
		},
		{
			"token limit with model info",
			"token limit reached for model claude-opus-4-20250514",
		},
		{
			"rate limit with numeric info",
			"rate limit exceeded: 50/50 requests used",
		},
		{
			"lowercase 429 too many",
			"error: 429 too many requests - please wait",
		},
		{
			"usage limit reached plain",
			"usage limit reached",
		},
		{
			"you have reached your usage limit",
			"You have reached your usage limit. Please upgrade.",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if !detectUsageLimit(tt.text) {
				t.Errorf("detectUsageLimit SHOULD trigger on stderr: %s", tt.text)
			}
		})
	}
}
