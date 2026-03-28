package engine

import (
	"os"
	"strings"
	"testing"

	"github.com/diegorodrigo90/ralph-engine/internal/config"
)

func TestBuildPromptContainsSessionContext(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		StoryID:       "65.3",
		StoryTitle:    "Permission UI Components",
		EpicID:        "65",
		EpicTitle:     "Permission System",
		SessionNumber: 5,
		StoriesDone:   12,
		StoriesTotal:  40,
		WorkflowType:  "bmad-v6",
		QualityGate:   "full",
	})

	mustContain := []string{
		"65.3",
		"Permission UI Components",
		"epic: 65",
		"session: 5",
		"12/40",
		"bmad-v6",
	}

	for _, s := range mustContain {
		if !strings.Contains(prompt, s) {
			t.Errorf("prompt should contain %q", s)
		}
	}
}

func TestBuildPromptBMADWorkflow(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		WorkflowType: "bmad-v6",
		QualityGate:  "full",
		WorkflowCommands: map[string]string{
			"implement":   "dev",
			"code_review": "bmad-bmm-code-review",
		},
		WorkflowInstructions: "Use Skill tool to invoke BMAD agents. TDD per AC.",
	})

	if !strings.Contains(prompt, "bmad-v6") {
		t.Error("prompt should contain bmad-v6 workflow type")
	}
	if !strings.Contains(prompt, "TDD per AC") {
		t.Error("prompt should contain workflow instructions")
	}
	if !strings.Contains(prompt, "implement") {
		t.Error("prompt should contain implement command")
	}
}

func TestBuildPromptWithInstructions(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		WorkflowType:         "tdd-strict",
		QualityGate:          "standard",
		WorkflowInstructions: "RED (failing test) → GREEN (minimal impl) → REFACTOR per AC",
	})

	if !strings.Contains(prompt, "tdd-strict") {
		t.Error("prompt should contain workflow type")
	}
	if !strings.Contains(prompt, "RED") {
		t.Error("prompt should include user workflow instructions")
	}
}

func TestBuildPromptBasicWorkflow(t *testing.T) {
	// No commands, no instructions — should show default steps.
	prompt := BuildPrompt(PromptContext{
		WorkflowType: "basic",
		QualityGate:  "minimal",
	})

	if !strings.Contains(prompt, "basic") {
		t.Error("prompt should contain basic workflow type")
	}
	if !strings.Contains(prompt, "Default Steps") {
		t.Error("prompt should show default steps when nothing configured")
	}
}

func TestBuildPromptCommandsTable(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		WorkflowType: "custom",
		WorkflowCommands: map[string]string{
			"build":  "make build",
			"test":   "pytest",
			"deploy": "kubectl apply",
		},
	})

	if !strings.Contains(prompt, "Agent Commands") {
		t.Error("prompt should contain Agent Commands table")
	}
	if !strings.Contains(prompt, "pytest") {
		t.Error("prompt should contain configured commands")
	}
}

func TestBuildPromptFullQualityGate(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		QualityGate: "full",
	})

	mustContain := []string{
		"Storybook",
		"E2E",
		"Browser validation",
	}
	for _, s := range mustContain {
		if !strings.Contains(prompt, s) {
			t.Errorf("full quality gate prompt should contain %q", s)
		}
	}
}

func TestBuildPromptMinimalQualityGate(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		QualityGate: "minimal",
	})

	// Minimal should NOT require Storybook/E2E
	if strings.Contains(prompt, "Storybook") {
		t.Error("minimal quality gate should not mention Storybook")
	}
}

func TestBuildPromptSSHStatus(t *testing.T) {
	withSSH := BuildPrompt(PromptContext{SSHAvailable: true})
	withoutSSH := BuildPrompt(PromptContext{SSHAvailable: false})

	if !strings.Contains(withSSH, "remote: connected") {
		t.Error("SSH available prompt should say remote: connected")
	}
	// When SSH is not available, the line is simply omitted (no noise).
	if strings.Contains(withoutSSH, "remote:") {
		t.Error("SSH unavailable prompt should NOT contain remote line")
	}
}

func TestBuildPromptAutonomyRules(t *testing.T) {
	prompt := BuildPrompt(PromptContext{})

	mustContain := []string{
		"Autonomous Operation",
		"usage limit",
		"Findings Pipeline",
		"RALPH_STATUS",
		"EXIT_REASON",
		"NEXT_STEP",
		"FIRST action in the session MUST be a tool call",
	}
	for _, s := range mustContain {
		if !strings.Contains(prompt, s) {
			t.Errorf("prompt should contain %q for autonomy rules", s)
		}
	}
}

func TestBuildPromptFirstActionRule(t *testing.T) {
	prompt := BuildPrompt(PromptContext{})

	mustContain := []string{
		"CRITICAL: First Action Rule",
		"FIRST response MUST be a tool call",
		"Do NOT start with a text response",
		"reading the story file or exploring the codebase immediately",
		"Text-only first responses waste a turn",
	}
	for _, s := range mustContain {
		if !strings.Contains(prompt, s) {
			t.Errorf("prompt should contain first-action rule %q", s)
		}
	}

	// First Action Rule should appear before Session State (early in prompt).
	firstActionIdx := strings.Index(prompt, "CRITICAL: First Action Rule")
	sessionStateIdx := strings.Index(prompt, "## Session State")
	if firstActionIdx > sessionStateIdx {
		t.Error("First Action Rule should appear before Session State section")
	}
}

func TestBuildPromptResearchToolsEnabled(t *testing.T) {
	research := &config.ResearchConfig{
		Enabled:  true,
		Strategy: "always",
		Tools: []config.ResearchTool{
			{
				Name:        "Project RAG",
				Type:        "rag",
				Priority:    1,
				Enabled:     true,
				Description: "Knowledge base with indexed docs",
				WhenToUse:   "First choice for known libraries",
				HowToUse:    "rag_search_knowledge_base(query, source_id)",
				Sources: []config.ResearchSource{
					{Name: "NestJS", ID: "abc123", Description: "Backend framework"},
					{Name: "Prisma", ID: "def456", Description: "ORM"},
				},
			},
			{
				Name:        "WebSearch",
				Type:        "search",
				Priority:    3,
				Enabled:     true,
				Description: "Broad web search",
				WhenToUse:   "When RAG has no answer",
				HowToUse:    "WebSearch with keywords",
			},
			{
				Name:    "DisabledTool",
				Type:    "mcp",
				Enabled: false,
			},
		},
	}

	prompt := BuildPrompt(PromptContext{
		Research: research,
	})

	mustContain := []string{
		"Research-First",
		"Project RAG",
		"WebSearch",
		"Knowledge base",
		"NestJS",
		"abc123",
		"Prisma",
		"rag_search_knowledge_base",
		"EVERY story",
		"Anti-patterns",
	}
	for _, s := range mustContain {
		if !strings.Contains(prompt, s) {
			t.Errorf("research prompt should contain %q", s)
		}
	}

	// Disabled tool should NOT appear.
	if strings.Contains(prompt, "DisabledTool") {
		t.Error("disabled tool should not appear in prompt")
	}
}

func TestBuildPromptResearchToolsPriorityOrder(t *testing.T) {
	research := &config.ResearchConfig{
		Enabled:  true,
		Strategy: "always",
		Tools: []config.ResearchTool{
			{Name: "Third", Type: "search", Priority: 3, Enabled: true},
			{Name: "First", Type: "rag", Priority: 1, Enabled: true},
			{Name: "Second", Type: "mcp", Priority: 2, Enabled: true},
		},
	}

	prompt := BuildPrompt(PromptContext{Research: research})

	firstIdx := strings.Index(prompt, "First")
	secondIdx := strings.Index(prompt, "Second")
	thirdIdx := strings.Index(prompt, "Third")

	if firstIdx > secondIdx || secondIdx > thirdIdx {
		t.Error("tools should appear in priority order (1, 2, 3)")
	}
}

func TestBuildPromptResearchDisabled(t *testing.T) {
	research := &config.ResearchConfig{
		Enabled: false,
		Tools: []config.ResearchTool{
			{Name: "ShouldNotAppear", Enabled: true},
		},
	}

	prompt := BuildPrompt(PromptContext{Research: research})

	if strings.Contains(prompt, "Research-First") {
		t.Error("research section should not appear when disabled")
	}
	if strings.Contains(prompt, "ShouldNotAppear") {
		t.Error("tools should not appear when research is disabled")
	}
}

func TestBuildPromptResearchNil(t *testing.T) {
	prompt := BuildPrompt(PromptContext{Research: nil})

	if strings.Contains(prompt, "Research-First") {
		t.Error("research section should not appear when nil")
	}
}

func TestBuildPromptStoryContent(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		StoryContent: "### Story 65.3: Permission UI\n\nGiven a user with MANAGE_ROLES...",
	})

	if !strings.Contains(prompt, "Specification") {
		t.Error("prompt should include specification section")
	}
	if !strings.Contains(prompt, "Permission UI") {
		t.Error("prompt should include story content")
	}
}

func TestBuildPromptPromptMD(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		PromptMD: "## My Project\n\nTech stack: Go + React",
	})

	if !strings.Contains(prompt, "Project Instructions") {
		t.Error("prompt should include project instructions section")
	}
	if !strings.Contains(prompt, "Tech stack: Go + React") {
		t.Error("prompt should include prompt.md content")
	}
}

func TestBuildPromptCustomSections(t *testing.T) {
	dir := t.TempDir()
	// Create a file to be loaded as a section.
	os.MkdirAll(dir, 0755)
	os.WriteFile(dir+"/rules.md", []byte("- Never skip tests\n- Always review"), 0644)

	boolTrue := true
	boolFalse := false

	prompt := BuildPrompt(PromptContext{
		ProjectDir: dir,
		Sections: []config.PromptSection{
			{
				Name:    "Golden Rules",
				File:    "rules.md",
				Enabled: &boolTrue,
			},
			{
				Name:    "Domain Rules",
				Content: "Platform is for sporting events.",
				Enabled: &boolTrue,
			},
			{
				Name:    "Disabled Section",
				Content: "Should not appear",
				Enabled: &boolFalse,
			},
			{
				Name: "Default Enabled",
				Content: "Enabled by default when field is nil",
			},
		},
	})

	mustContain := []string{
		"Golden Rules",
		"Never skip tests",
		"Domain Rules",
		"sporting events",
		"Default Enabled",
		"Enabled by default",
	}
	for _, s := range mustContain {
		if !strings.Contains(prompt, s) {
			t.Errorf("prompt should contain %q", s)
		}
	}

	// Disabled section should NOT appear.
	if strings.Contains(prompt, "Should not appear") {
		t.Error("disabled section should not appear in prompt")
	}
}

func TestBuildPromptSectionFileFallsBackToContent(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		ProjectDir: t.TempDir(),
		Sections: []config.PromptSection{
			{
				Name:    "Fallback",
				File:    "nonexistent.md",
				Content: "Inline fallback content",
			},
		},
	})

	if !strings.Contains(prompt, "Inline fallback content") {
		t.Error("should fall back to inline content when file missing")
	}
}

func TestVariableSubstitution(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		StoryID:       "65.3",
		StoryTitle:    "Permission UI",
		EpicID:        "65",
		EpicTitle:     "Permissions",
		SessionNumber: 7,
		StoriesDone:   42,
		StoriesTotal:  100,
		WorkflowType:  "bmad-v6",
		QualityGate:   "full",
		PromptMD:      "Working on {{story_id}}: {{story_title}} (Epic {{epic_id}}). Session {{session}}, progress {{progress}}.",
	})

	mustContain := []string{
		"Working on 65.3: Permission UI",
		"(Epic 65)",
		"Session 7",
		"progress 42/100",
	}
	for _, s := range mustContain {
		if !strings.Contains(prompt, s) {
			t.Errorf("prompt should contain %q after variable substitution", s)
		}
	}

	// Should NOT contain raw template vars.
	mustNotContain := []string{"{{story_id}}", "{{epic_id}}", "{{session}}", "{{progress}}"}
	for _, s := range mustNotContain {
		if strings.Contains(prompt, s) {
			t.Errorf("prompt should NOT contain raw template var %q", s)
		}
	}
}

func TestVariableSubstitutionInSections(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(dir+"/template.md", []byte("Story {{story_id}} — {{story_title}}"), 0644)

	prompt := BuildPrompt(PromptContext{
		StoryID:    "1.2",
		StoryTitle: "Login",
		ProjectDir: dir,
		Sections: []config.PromptSection{
			{Name: "Template", File: "template.md"},
		},
	})

	if !strings.Contains(prompt, "Story 1.2 — Login") {
		t.Error("variables in file sections should be substituted")
	}
}

func TestVariableSubstitutionNoVarsIsNoop(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		PromptMD: "No variables here.",
	})
	if !strings.Contains(prompt, "No variables here.") {
		t.Error("prompt without variables should pass through unchanged")
	}
}

func TestBuildPromptResearchStrategies(t *testing.T) {
	tests := []struct {
		strategy string
		expected string
	}{
		{"always", "EVERY story"},
		{"story-start", "START of each story"},
		{"on-demand", "encountering unfamiliar"},
	}

	for _, tt := range tests {
		t.Run(tt.strategy, func(t *testing.T) {
			research := &config.ResearchConfig{
				Enabled:  true,
				Strategy: tt.strategy,
				Tools:    []config.ResearchTool{{Name: "Test", Enabled: true}},
			}
			prompt := BuildPrompt(PromptContext{Research: research})
			if !strings.Contains(prompt, tt.expected) {
				t.Errorf("strategy %q should produce prompt containing %q", tt.strategy, tt.expected)
			}
		})
	}
}

// --- HIGH PRIORITY GAP TESTS ---

func TestSubstituteVarsEmptyContext(t *testing.T) {
	// substituteVars with zero-value PromptContext should replace vars with empty/zero strings.
	ctx := PromptContext{}
	input := "id={{story_id}} title={{story_title}} epic={{epic_id}} etitle={{epic_title}} s={{session}} p={{progress}} w={{workflow}} q={{quality}} done={{stories_done}} total={{stories_total}}"
	result := substituteVars(input, ctx)

	tests := []struct {
		name     string
		contains string
	}{
		{"story_id replaced with empty", "id="},
		{"session replaced with 0", "s=0"},
		{"progress replaced with 0/0", "p=0/0"},
		{"workflow replaced with empty", "w="},
		{"quality replaced with empty", "q="},
		{"stories_done replaced with 0", "done=0"},
		{"stories_total replaced with 0", "total=0"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if !strings.Contains(result, tt.contains) {
				t.Errorf("expected result to contain %q, got: %s", tt.contains, result)
			}
		})
	}

	// No raw {{var}} should remain.
	if strings.Contains(result, "{{") {
		t.Errorf("raw template variables remain after substitution: %s", result)
	}
}

func TestBuildPromptLargeVariableValues(t *testing.T) {
	longTitle := strings.Repeat("A", 1500)
	longEpic := strings.Repeat("B", 2000)
	prompt := BuildPrompt(PromptContext{
		StoryID:    "99.1",
		StoryTitle: longTitle,
		EpicID:     "99",
		EpicTitle:  longEpic,
		PromptMD:   "Story: {{story_title}}, Epic: {{epic_title}}",
	})

	if !strings.Contains(prompt, longTitle) {
		t.Error("prompt should contain the full long story title")
	}
	if !strings.Contains(prompt, longEpic) {
		t.Error("prompt should contain the full long epic title")
	}
	// Variable substitution should also work for large values in PromptMD.
	if strings.Contains(prompt, "{{story_title}}") {
		t.Error("{{story_title}} should be substituted even with large value")
	}
	if strings.Contains(prompt, "{{epic_title}}") {
		t.Error("{{epic_title}} should be substituted even with large value")
	}
}

func TestResolveSectionNonexistentProjectDir(t *testing.T) {
	section := config.PromptSection{
		Name:    "Test",
		File:    "some-file.md",
		Content: "fallback content",
	}
	// ProjectDir points to a directory that does not exist.
	result := resolveSection(section, "/nonexistent/dir/that/does/not/exist")

	// Should fall back to inline content since file cannot be read.
	if result != "fallback content" {
		t.Errorf("expected fallback content, got: %q", result)
	}
}

func TestResolveSectionEmptyProjectDir(t *testing.T) {
	section := config.PromptSection{
		File:    "rules.md",
		Content: "inline rules",
	}
	result := resolveSection(section, "")

	if result != "inline rules" {
		t.Errorf("expected inline content when projectDir is empty, got: %q", result)
	}
}

func TestResolveSectionNoFileNoContent(t *testing.T) {
	section := config.PromptSection{Name: "Empty"}
	result := resolveSection(section, t.TempDir())

	if result != "" {
		t.Errorf("expected empty string when no file and no content, got: %q", result)
	}
}

func TestBuildResearchInstructionsPriorityZeroAndDuplicates(t *testing.T) {
	research := &config.ResearchConfig{
		Enabled:  true,
		Strategy: "always",
		Tools: []config.ResearchTool{
			{Name: "ToolC", Type: "search", Priority: 0, Enabled: true},
			{Name: "ToolA", Type: "rag", Priority: 0, Enabled: true},
			{Name: "ToolB", Type: "mcp", Priority: 1, Enabled: true},
			{Name: "ToolD", Type: "docs", Priority: 1, Enabled: true},
		},
	}

	prompt := BuildPrompt(PromptContext{Research: research})

	// All four tools should appear.
	for _, name := range []string{"ToolA", "ToolB", "ToolC", "ToolD"} {
		if !strings.Contains(prompt, name) {
			t.Errorf("prompt should contain tool %q", name)
		}
	}

	// Priority 0 tools should appear before priority 1 tools.
	idxA := strings.Index(prompt, "ToolA")
	idxC := strings.Index(prompt, "ToolC")
	idxB := strings.Index(prompt, "ToolB")
	// Both ToolA and ToolC (priority 0) should appear before ToolB (priority 1).
	if idxA > idxB {
		t.Error("ToolA (priority 0) should appear before ToolB (priority 1)")
	}
	if idxC > idxB {
		t.Error("ToolC (priority 0) should appear before ToolB (priority 1)")
	}
}

func TestBuildPromptStoriesTotal0(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		StoriesDone:  0,
		StoriesTotal: 0,
		PromptMD:     "Progress: {{progress}}, done={{stories_done}}, total={{stories_total}}",
	})

	if !strings.Contains(prompt, "0/0 stories") {
		t.Error("prompt session state should show 0/0 stories")
	}
	if !strings.Contains(prompt, "Progress: 0/0") {
		t.Error("{{progress}} should resolve to 0/0")
	}
	if !strings.Contains(prompt, "done=0") {
		t.Error("{{stories_done}} should resolve to 0")
	}
	if !strings.Contains(prompt, "total=0") {
		t.Error("{{stories_total}} should resolve to 0")
	}
}

func TestQualityRulesUnknownGateType(t *testing.T) {
	tests := []struct {
		gate string
	}{
		{"unknown"},
		{"FULL"},
		{""},
		{"super-strict"},
	}
	for _, tt := range tests {
		t.Run(tt.gate, func(t *testing.T) {
			result := qualityRules(tt.gate)
			// Unknown gate types should fall back to standard (base) rules.
			if !strings.Contains(result, "ALL tests must pass") {
				t.Error("unknown quality gate should include base rules")
			}
			// Should NOT include full-specific extras.
			if strings.Contains(result, "Storybook stories") {
				t.Error("unknown quality gate should not include full-gate extras")
			}
		})
	}
}

func TestSessionInstructionsEmptyCommandsAndInstructions(t *testing.T) {
	result := sessionInstructions("test-workflow", nil, "")

	// Should show default steps fallback.
	if !strings.Contains(result, "Default Steps") {
		t.Error("empty commands and instructions should trigger default steps fallback")
	}
	if !strings.Contains(result, "test-workflow") {
		t.Error("workflow type should still appear in header")
	}
	// Should always contain critical implementation rules.
	if !strings.Contains(result, "MUST write actual source code") {
		t.Error("critical implementation rules should always be present")
	}
}

func TestSessionInstructionsEmptyMapNotNil(t *testing.T) {
	// Empty map (not nil) + empty instructions should also trigger fallback.
	result := sessionInstructions("basic", map[string]string{}, "")

	if !strings.Contains(result, "Default Steps") {
		t.Error("empty (non-nil) commands map should trigger default steps fallback")
	}
}

func TestBuildPromptResearchNilTools(t *testing.T) {
	research := &config.ResearchConfig{
		Enabled:  true,
		Strategy: "always",
		Tools:    nil,
	}

	prompt := BuildPrompt(PromptContext{Research: research})

	// With nil tools array, research section should not appear.
	if strings.Contains(prompt, "Research-First") {
		t.Error("research section should not appear when Tools is nil")
	}
}

func TestBuildPromptResearchEmptyTools(t *testing.T) {
	research := &config.ResearchConfig{
		Enabled:  true,
		Strategy: "always",
		Tools:    []config.ResearchTool{},
	}

	prompt := BuildPrompt(PromptContext{Research: research})

	if strings.Contains(prompt, "Research-First") {
		t.Error("research section should not appear when Tools is empty")
	}
}

func TestBuildPromptResearchUnknownStrategy(t *testing.T) {
	research := &config.ResearchConfig{
		Enabled:  true,
		Strategy: "unknown-strategy",
		Tools:    []config.ResearchTool{{Name: "TestTool", Enabled: true}},
	}

	prompt := BuildPrompt(PromptContext{Research: research})

	// Section should still appear with tools.
	if !strings.Contains(prompt, "Research-First") {
		t.Error("research section should appear even with unknown strategy")
	}
	if !strings.Contains(prompt, "TestTool") {
		t.Error("tools should still appear with unknown strategy")
	}
	// None of the known strategy strings should appear.
	for _, s := range []string{"EVERY story", "START of each story", "encountering unfamiliar"} {
		if strings.Contains(prompt, s) {
			t.Errorf("unknown strategy should not produce known hint %q", s)
		}
	}
}

func TestBuildPromptSectionWithEmptyName(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		Sections: []config.PromptSection{
			{
				Name:    "",
				Content: "Content without heading",
			},
		},
	})

	// Content should appear but no "## " heading for it.
	if !strings.Contains(prompt, "Content without heading") {
		t.Error("section content should appear even with empty name")
	}
	// There should be no "## \n" (empty heading).
	if strings.Contains(prompt, "## \n") {
		t.Error("empty section name should not produce empty heading")
	}
}

func TestBuildPromptAllFieldsPopulated(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(dir+"/spec.md", []byte("Full spec content"), 0644)
	boolTrue := true

	prompt := BuildPrompt(PromptContext{
		StoryID:       "42.7",
		StoryTitle:    "Integration Test Story",
		EpicID:        "42",
		EpicTitle:     "Integration Epic",
		SessionNumber: 99,
		StoriesDone:   41,
		StoriesTotal:  50,
		WorkflowType:  "bmad-v6",
		WorkflowCommands: map[string]string{
			"implement":   "/dev",
			"code_review": "/cr",
			"test":        "/qa",
		},
		WorkflowInstructions: "Use BMAD agents for everything.",
		QualityGate:          "full",
		SSHAvailable:         true,
		Findings:             3,
		StoryContent:         "### Story 42.7\nGiven a fully populated context...",
		PromptMD:             "Project uses {{workflow}} workflow. Story {{story_id}}.",
		Sections: []config.PromptSection{
			{Name: "Custom Section", Content: "Custom inline content", Enabled: &boolTrue},
			{Name: "File Section", File: "spec.md", Enabled: &boolTrue},
		},
		ProjectDir: dir,
		Research: &config.ResearchConfig{
			Enabled:  true,
			Strategy: "always",
			Tools: []config.ResearchTool{
				{
					Name:        "RAG",
					Type:        "rag",
					Priority:    1,
					Enabled:     true,
					Description: "Knowledge base",
					Sources:     []config.ResearchSource{{Name: "Go", ID: "go123"}},
				},
			},
		},
	})

	mustContain := []string{
		"42.7",
		"Integration Test Story",
		"epic: 42",
		"Integration Epic",
		"session: 99",
		"41/50",
		"bmad-v6",
		"full",
		"remote: connected",
		"findings: 3 pending",
		"Specification",
		"Story 42.7",
		"Project Instructions",
		"Project uses bmad-v6 workflow. Story 42.7.",
		"Custom Section",
		"Custom inline content",
		"File Section",
		"Full spec content",
		"Research-First",
		"RAG",
		"Knowledge base",
		"Go",
		"go123",
		"Agent Commands",
		"/dev",
		"/cr",
		"/qa",
		"Use BMAD agents",
		"Storybook",   // full quality gate
		"E2E",         // full quality gate
		"RALPH_STATUS", // autonomy rules
		"NEXT_STEP",    // autonomy rules
	}
	for _, s := range mustContain {
		if !strings.Contains(prompt, s) {
			t.Errorf("fully populated prompt should contain %q", s)
		}
	}

	// Variable substitution should have worked in PromptMD.
	if strings.Contains(prompt, "{{workflow}}") {
		t.Error("{{workflow}} should be substituted")
	}
	if strings.Contains(prompt, "{{story_id}}") {
		t.Error("{{story_id}} should be substituted")
	}
}

func TestBuildPromptZeroValueStruct(t *testing.T) {
	// Completely zero-value PromptContext should not panic and produce valid output.
	prompt := BuildPrompt(PromptContext{})

	if prompt == "" {
		t.Fatal("prompt from zero-value context should not be empty")
	}

	// Should still contain core sections.
	mustContain := []string{
		"Ralph Engine",
		"Autonomous Session",
		"CRITICAL: First Action Rule",
		"Session State",
		"session: 0",
		"0/0 stories",
		"Story",
		"Quality Rules",
		"Autonomous Operation",
		"RALPH_STATUS",
	}
	for _, s := range mustContain {
		if !strings.Contains(prompt, s) {
			t.Errorf("zero-value prompt should contain %q", s)
		}
	}

	// Should NOT contain optional sections.
	mustNotContain := []string{
		"Specification",
		"Project Instructions",
		"Research-First",
		"remote: connected",
		"findings:",
	}
	for _, s := range mustNotContain {
		if strings.Contains(prompt, s) {
			t.Errorf("zero-value prompt should NOT contain %q", s)
		}
	}
}

func TestBuildPromptNoRawVariablesAfterSubstitution(t *testing.T) {
	// Regression: ensure ALL known variables are substituted and no {{...}} remains.
	prompt := BuildPrompt(PromptContext{
		StoryID:       "1.1",
		StoryTitle:    "Test",
		EpicID:        "1",
		EpicTitle:     "Epic",
		SessionNumber: 1,
		StoriesDone:   0,
		StoriesTotal:  5,
		WorkflowType:  "basic",
		QualityGate:   "standard",
		PromptMD: strings.Join([]string{
			"{{story_id}}", "{{story_title}}", "{{epic_id}}", "{{epic_title}}",
			"{{session}}", "{{progress}}", "{{workflow}}", "{{quality}}",
			"{{stories_done}}", "{{stories_total}}",
		}, " "),
	})

	if strings.Contains(prompt, "{{") {
		// Find what remains for a useful error message.
		idx := strings.Index(prompt, "{{")
		end := strings.Index(prompt[idx:], "}}") + idx + 2
		remaining := prompt[idx:end]
		t.Errorf("raw template variable %q found after substitution", remaining)
	}
}

func TestSubstituteVarsUnknownVariablePassesThrough(t *testing.T) {
	// Unknown {{variables}} that are not in the replacement map should pass through unchanged.
	ctx := PromptContext{StoryID: "1.1"}
	result := substituteVars("Known: {{story_id}}, Unknown: {{custom_var}}", ctx)

	if !strings.Contains(result, "Known: 1.1") {
		t.Error("known variable should be substituted")
	}
	if !strings.Contains(result, "{{custom_var}}") {
		t.Error("unknown variables should pass through unchanged")
	}
}

func TestSubstituteVarsNoPlaceholders(t *testing.T) {
	// Fast path: no {{ at all should return input unchanged.
	ctx := PromptContext{StoryID: "1.1"}
	input := "No variables here at all."
	result := substituteVars(input, ctx)

	if result != input {
		t.Errorf("input without {{ should be returned unchanged, got: %q", result)
	}
}

func TestBuildPromptFindingsZero(t *testing.T) {
	prompt := BuildPrompt(PromptContext{Findings: 0})
	if strings.Contains(prompt, "findings:") {
		t.Error("findings line should not appear when Findings=0")
	}
}

func TestBuildPromptFindingsPositive(t *testing.T) {
	prompt := BuildPrompt(PromptContext{Findings: 7})
	if !strings.Contains(prompt, "findings: 7 pending") {
		t.Error("prompt should show findings count when positive")
	}
}

func TestBuildPromptEpicOmittedWhenEmpty(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		StoryID:    "1.1",
		StoryTitle: "Solo Story",
		EpicID:     "",
		EpicTitle:  "",
	})
	if strings.Contains(prompt, "epic:") {
		t.Error("epic line should not appear when EpicID is empty")
	}
}

func TestBuildPromptSectionFileOnlyNoContent(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(dir+"/existing.md", []byte("File-only content"), 0644)

	section := config.PromptSection{
		Name: "FileOnly",
		File: "existing.md",
	}
	result := resolveSection(section, dir)

	if result != "File-only content" {
		t.Errorf("expected file content, got: %q", result)
	}
}

func TestBuildPromptResearchAllToolsDisabled(t *testing.T) {
	research := &config.ResearchConfig{
		Enabled:  true,
		Strategy: "always",
		Tools: []config.ResearchTool{
			{Name: "Disabled1", Enabled: false},
			{Name: "Disabled2", Enabled: false},
		},
	}

	prompt := BuildPrompt(PromptContext{Research: research})

	// Research section appears but no tools listed.
	if !strings.Contains(prompt, "Research-First") {
		t.Error("research section should appear when Enabled=true and Tools is non-empty")
	}
	if strings.Contains(prompt, "Disabled1") {
		t.Error("disabled tools should not appear in prompt")
	}
	if strings.Contains(prompt, "Disabled2") {
		t.Error("disabled tools should not appear in prompt")
	}
}

func TestBuildPromptResearchSourceWithoutID(t *testing.T) {
	research := &config.ResearchConfig{
		Enabled:  true,
		Strategy: "on-demand",
		Tools: []config.ResearchTool{
			{
				Name:     "Docs",
				Type:     "docs",
				Priority: 1,
				Enabled:  true,
				Sources: []config.ResearchSource{
					{Name: "React", Description: "UI library"},
					{Name: "Vue", ID: "vue-id", Description: "Progressive framework"},
				},
			},
		},
	}

	prompt := BuildPrompt(PromptContext{Research: research})

	if !strings.Contains(prompt, "React") {
		t.Error("source without ID should still appear")
	}
	if !strings.Contains(prompt, "vue-id") {
		t.Error("source with ID should show ID")
	}
}

func TestSessionInstructionsCommandsAreSorted(t *testing.T) {
	result := sessionInstructions("test", map[string]string{
		"z_deploy":  "deploy.sh",
		"a_build":   "make build",
		"m_test":    "go test",
	}, "")

	// Commands table should appear in sorted order.
	idxA := strings.Index(result, "a_build")
	idxM := strings.Index(result, "m_test")
	idxZ := strings.Index(result, "z_deploy")

	if idxA > idxM || idxM > idxZ {
		t.Error("command phases should be sorted alphabetically")
	}
}

func TestSessionInstructionsOnlyInstructionsNoCommands(t *testing.T) {
	result := sessionInstructions("custom", nil, "Follow TDD strictly.")

	if !strings.Contains(result, "Follow TDD strictly.") {
		t.Error("instructions should appear when provided")
	}
	if strings.Contains(result, "Default Steps") {
		t.Error("default steps should not appear when instructions are provided")
	}
	if strings.Contains(result, "Agent Commands") {
		t.Error("commands table should not appear when commands is nil")
	}
}

func TestSessionInstructionsOnlyCommandsNoInstructions(t *testing.T) {
	result := sessionInstructions("custom", map[string]string{"build": "make"}, "")

	if !strings.Contains(result, "Agent Commands") {
		t.Error("commands table should appear when commands provided")
	}
	if strings.Contains(result, "Default Steps") {
		t.Error("default steps should not appear when commands are provided")
	}
	if strings.Contains(result, "Workflow Instructions") {
		t.Error("workflow instructions header should not appear when instructions empty")
	}
}

// --- FUZZ TESTS ---

func FuzzBuildPrompt(f *testing.F) {
	// Seeds with various story IDs, titles, workflows, quality gates.
	f.Add("65.3", "Permission UI", "bmad-v6", "full")
	f.Add("", "", "", "")
	f.Add("1.1", "A", "basic", "minimal")
	f.Add("{{story_id}}", "{{story_title}}", "{{workflow}}", "{{quality}}")
	f.Add("99.99", strings.Repeat("X", 500), "tdd-strict", "standard")
	f.Add("a/b/c", "Title with\nnewlines\tand\ttabs", "unknown", "unknown")

	f.Fuzz(func(t *testing.T, storyID, storyTitle, workflow, quality string) {
		// Should never panic.
		result := BuildPrompt(PromptContext{
			StoryID:      storyID,
			StoryTitle:   storyTitle,
			WorkflowType: workflow,
			QualityGate:  quality,
		})
		// Result should always be non-empty (core sections always present).
		if result == "" {
			t.Error("BuildPrompt should never return empty string")
		}
	})
}

func FuzzSubstituteVars(f *testing.F) {
	f.Add("Hello {{story_id}}", "65.3", "Permission UI", 5, 10, 20)
	f.Add("{{progress}} done", "", "", 0, 0, 0)
	f.Add("No vars", "1.1", "Title", 1, 5, 10)
	f.Add("{{unknown}}", "id", "title", 99, 50, 100)
	f.Add("{{story_id}}{{story_id}}", "double", "test", 1, 1, 1)

	f.Fuzz(func(t *testing.T, template, storyID, storyTitle string, session, done, total int) {
		ctx := PromptContext{
			StoryID:       storyID,
			StoryTitle:    storyTitle,
			SessionNumber: session,
			StoriesDone:   done,
			StoriesTotal:  total,
		}
		// Should never panic.
		_ = substituteVars(template, ctx)
	})
}

// --- Tests for prompt with workflow commands/instructions combinations ---

func TestBuildPromptWithWorkflowCommandsOnly(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		WorkflowType: "custom",
		WorkflowCommands: map[string]string{
			"build": "make build",
			"test":  "pytest",
		},
	})

	if !strings.Contains(prompt, "Agent Commands") {
		t.Error("prompt should show Agent Commands table when commands set")
	}
	if !strings.Contains(prompt, "make build") {
		t.Error("prompt should contain the build command")
	}
	if !strings.Contains(prompt, "pytest") {
		t.Error("prompt should contain the test command")
	}
	// Without instructions, there should be no user-provided instructions text.
	// But "Default Steps" should also NOT appear since commands are set.
	if strings.Contains(prompt, "Default Steps") {
		t.Error("prompt should NOT show Default Steps when commands are configured")
	}
}

func TestBuildPromptWithInstructionsOnly(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		WorkflowType:         "tdd-strict",
		WorkflowInstructions: "RED-GREEN-REFACTOR per AC. Never skip tests.",
	})

	if !strings.Contains(prompt, "RED-GREEN-REFACTOR") {
		t.Error("prompt should contain the workflow instructions text")
	}
	// No commands table should appear.
	if strings.Contains(prompt, "Agent Commands") {
		t.Error("prompt should NOT show Agent Commands table when no commands set")
	}
	// Default Steps should NOT appear since instructions are set.
	if strings.Contains(prompt, "Default Steps") {
		t.Error("prompt should NOT show Default Steps when instructions are configured")
	}
}

func TestBuildPromptWithBothCommandsAndInstructions(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		WorkflowType: "bmad-v6",
		WorkflowCommands: map[string]string{
			"implement":   "dev",
			"code_review": "bmad-bmm-code-review",
		},
		WorkflowInstructions: "Use Skill tool to invoke BMAD agents. TDD per AC.",
	})

	// Both sections should be present.
	if !strings.Contains(prompt, "Agent Commands") {
		t.Error("prompt should show Agent Commands table")
	}
	if !strings.Contains(prompt, "implement") {
		t.Error("prompt should contain implement command")
	}
	if !strings.Contains(prompt, "TDD per AC") {
		t.Error("prompt should contain instructions text")
	}
	if !strings.Contains(prompt, "Use Skill tool") {
		t.Error("prompt should contain full instructions")
	}
	if strings.Contains(prompt, "Default Steps") {
		t.Error("prompt should NOT show Default Steps when both are configured")
	}
}

func TestBuildPromptWithNoWorkflow(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		WorkflowType: "basic",
	})

	// Neither commands nor instructions — should show default steps.
	if !strings.Contains(prompt, "Default Steps") {
		t.Error("prompt should show Default Steps when nothing configured")
	}
	if strings.Contains(prompt, "Agent Commands") {
		t.Error("prompt should NOT show Agent Commands when no commands")
	}
}

func TestBuildPromptFirstActionRuleAlwaysPresent(t *testing.T) {
	// Test with various config combinations — first action rule must always be there.
	configs := []PromptContext{
		{}, // empty
		{WorkflowType: "basic", QualityGate: "minimal"},
		{WorkflowType: "bmad-v6", QualityGate: "full", SSHAvailable: true},
		{WorkflowCommands: map[string]string{"build": "make"}, WorkflowInstructions: "TDD"},
		{Research: &config.ResearchConfig{Enabled: true, Tools: []config.ResearchTool{{Name: "RAG", Enabled: true}}}},
	}

	for i, ctx := range configs {
		prompt := BuildPrompt(ctx)
		if !strings.Contains(prompt, "CRITICAL: First Action Rule") {
			t.Errorf("config %d: prompt missing First Action Rule", i)
		}
		if !strings.Contains(prompt, "FIRST response MUST be a tool call") {
			t.Errorf("config %d: prompt missing first-action requirement text", i)
		}
	}
}
