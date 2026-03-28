package engine

import (
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
	})

	if !strings.Contains(prompt, "BMAD v6") {
		t.Error("prompt should contain BMAD v6 workflow")
	}
	if !strings.Contains(prompt, "TDD per AC") {
		t.Error("prompt should mention TDD per AC")
	}
}

func TestBuildPromptTDDWorkflow(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		WorkflowType: "tdd-strict",
		QualityGate:  "standard",
	})

	if !strings.Contains(prompt, "TDD Strict") {
		t.Error("prompt should contain TDD Strict workflow")
	}
	if !strings.Contains(prompt, "RED") {
		t.Error("prompt should mention RED-GREEN-REFACTOR")
	}
}

func TestBuildPromptBasicWorkflow(t *testing.T) {
	prompt := BuildPrompt(PromptContext{
		WorkflowType: "basic",
		QualityGate:  "minimal",
	})

	if !strings.Contains(prompt, "Basic") {
		t.Error("prompt should contain Basic workflow")
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
	}
	for _, s := range mustContain {
		if !strings.Contains(prompt, s) {
			t.Errorf("prompt should contain %q for autonomy rules", s)
		}
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
