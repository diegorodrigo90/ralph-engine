// prompt.go contains the system prompt template that is injected into
// each Claude session via --append-system-prompt. It provides the AI agent
// with context about the current sprint state, quality requirements,
// and the engine's expectations for the session.
package engine

import (
	"fmt"
	"strings"
)

// PromptContext holds the dynamic values injected into the session prompt.
type PromptContext struct {
	StoryID       string
	StoryTitle    string
	EpicID        string
	EpicTitle     string
	SessionNumber int
	StoriesDone   int
	StoriesTotal  int
	WorkflowType  string // "bmad-v6", "basic", "tdd-strict"
	QualityGate   string // "full", "standard", "minimal"
	SSHAvailable  bool
	Findings      int
}

// BuildPrompt generates the system prompt for a Claude session.
// It combines static quality rules with dynamic session context.
func BuildPrompt(ctx PromptContext) string {
	var b strings.Builder

	b.WriteString("# Ralph Engine — Autonomous Sprint Session\n\n")

	// Session context
	b.WriteString(fmt.Sprintf("## Session Context\n"))
	b.WriteString(fmt.Sprintf("- Session: #%d\n", ctx.SessionNumber))
	b.WriteString(fmt.Sprintf("- Progress: %d/%d stories completed\n", ctx.StoriesDone, ctx.StoriesTotal))
	b.WriteString(fmt.Sprintf("- Workflow: %s\n", ctx.WorkflowType))
	b.WriteString(fmt.Sprintf("- Quality gate: %s\n", ctx.QualityGate))
	if ctx.SSHAvailable {
		b.WriteString("- SSH: connected (DevContainer available)\n")
	} else {
		b.WriteString("- SSH: not available (local mode)\n")
	}
	if ctx.Findings > 0 {
		b.WriteString(fmt.Sprintf("- Accumulated findings: %d\n", ctx.Findings))
	}
	b.WriteString("\n")

	// Current story
	b.WriteString(fmt.Sprintf("## Current Story\n"))
	b.WriteString(fmt.Sprintf("- Epic: %s — %s\n", ctx.EpicID, ctx.EpicTitle))
	b.WriteString(fmt.Sprintf("- Story: %s — %s\n", ctx.StoryID, ctx.StoryTitle))
	b.WriteString("\n")

	// Quality rules
	b.WriteString(qualityRules(ctx.QualityGate))
	b.WriteString("\n")

	// Session instructions
	b.WriteString(sessionInstructions(ctx.WorkflowType))
	b.WriteString("\n")

	// Progress persistence
	b.WriteString(persistenceRules())

	return b.String()
}

// qualityRules returns quality gate instructions based on the configured level.
func qualityRules(gate string) string {
	base := `## Quality Rules (MANDATORY)
- ALL tests must pass before commit
- Code review findings must be fixed
- Zero errors in type-check
- Zero errors in build
`

	switch gate {
	case "full":
		return base + `- Storybook stories required for UI components
- E2E tests for user flows
- Browser validation (console errors, viewports, dark mode)
- Dev logs must show zero errors
`
	case "standard":
		return base + `- Build must pass (full monorepo)
`
	case "minimal":
		return `## Quality Rules
- Tests must pass for changed packages
`
	default:
		return base
	}
}

// sessionInstructions returns workflow-specific instructions.
func sessionInstructions(workflow string) string {
	switch workflow {
	case "bmad-v6":
		return `## Workflow: BMAD v6
1. Read the story file and understand all ACs
2. Architect validates DoR (if not already validated)
3. TDD per AC: write failing test → implement → pass → refactor
4. Run code review (CR) — fix ALL findings
5. Run quality gates: tests → build → type-check
6. Commit with descriptive message
7. Update sprint-status.yaml
8. Note any findings for the findings pipeline
9. Pick next story or save progress if session limit reached

IMPORTANT: Use BMAD skills (/dev, /bmad-bmm-code-review) when available.
`
	case "tdd-strict":
		return `## Workflow: TDD Strict
1. Read story/spec
2. For each AC: RED (failing test) → GREEN (minimal implementation) → REFACTOR
3. Never write implementation before the test
4. Commit test + implementation together
5. Run full test suite before moving to next AC
`
	default:
		return `## Workflow: Basic
1. Read the task description
2. Implement the changes
3. Write tests
4. Run tests and fix failures
5. Commit
`
	}
}

// persistenceRules returns instructions for saving progress.
func persistenceRules() string {
	return `## Progress Persistence (CRITICAL)
- After EVERY commit, report stories completed in this session
- If you sense a usage limit approaching, IMMEDIATELY:
  1. Commit any pending work
  2. Update sprint-status.yaml
  3. Save a handoff note describing next steps
- The engine will detect your exit and save state automatically
- Do NOT try to manage billing or usage limits — just save progress

## Findings Pipeline
- Note any bugs, patterns, or improvements discovered during implementation
- Report findings at end of session — the engine will create stories for them
- Do NOT fix unrelated bugs inline — note them as findings instead
`
}
