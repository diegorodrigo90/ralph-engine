package engine

import (
	"strings"
	"testing"
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
		"Epic: 65",
		"Session: #5",
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

	if !strings.Contains(withSSH, "connected") {
		t.Error("SSH available prompt should say connected")
	}
	if !strings.Contains(withoutSSH, "not available") {
		t.Error("SSH unavailable prompt should say not available")
	}
}

func TestBuildPromptPersistenceRules(t *testing.T) {
	prompt := BuildPrompt(PromptContext{})

	mustContain := []string{
		"Progress Persistence",
		"usage limit",
		"sprint-status",
		"Findings Pipeline",
	}
	for _, s := range mustContain {
		if !strings.Contains(prompt, s) {
			t.Errorf("prompt should contain %q for persistence rules", s)
		}
	}
}
