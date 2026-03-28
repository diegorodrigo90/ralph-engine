// prompt.go contains the system prompt template that is injected into
// each Claude session via --append-system-prompt. It provides the AI agent
// with context about the current sprint state, quality requirements,
// research tools, and the engine's expectations for the session.
package engine

import (
	"fmt"
	"sort"
	"strings"

	"github.com/diegorodrigo90/ralph-engine/internal/config"
	appcontext "github.com/diegorodrigo90/ralph-engine/internal/context"
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
	WorkflowType     string            // "bmad-v6", "basic", "tdd-strict"
	WorkflowCommands map[string]string // Phase → agent command mapping from config
	QualityGate      string            // "full", "standard", "minimal"
	SSHAvailable  bool
	Findings      int
	// StoryContent is the full story file content (read from paths.stories).
	StoryContent string
	// PromptMD is the content of .ralph-engine/prompt.md.
	PromptMD string
	// Sections holds custom prompt sections from config (files + inline content).
	Sections []config.PromptSection
	// ProjectDir is needed to resolve file paths in prompt sections.
	ProjectDir string
	// Research holds configured research tools for prompt injection.
	Research *config.ResearchConfig
}

// BuildPrompt generates the system prompt for an autonomous agent session.
// Structure: Identity → Context → Story → Research → Workflow → Quality → Persistence → Exit.
// Minimal boilerplate. Dynamic sections injected only when configured.
// The prompt is agent-agnostic — works with any CLI-based AI agent.
func BuildPrompt(ctx PromptContext) string {
	var b strings.Builder

	// Identity — who you are this session.
	b.WriteString("# Ralph Engine — Autonomous Session\n\n")
	b.WriteString("You are an autonomous development agent inside a Ralph Engine loop.\n")
	b.WriteString("You will implement ONE story per session. Work autonomously — do NOT ask questions.\n")
	b.WriteString("If ambiguous, make the safest reasonable assumption and continue.\n\n")

	// Session state — machine-readable context for the agent.
	b.WriteString("## Session State\n")
	b.WriteString(fmt.Sprintf("- session: %d\n", ctx.SessionNumber))
	b.WriteString(fmt.Sprintf("- progress: %d/%d stories\n", ctx.StoriesDone, ctx.StoriesTotal))
	b.WriteString(fmt.Sprintf("- workflow: %s\n", ctx.WorkflowType))
	b.WriteString(fmt.Sprintf("- quality: %s\n", ctx.QualityGate))
	if ctx.SSHAvailable {
		b.WriteString("- remote: connected\n")
	}
	if ctx.Findings > 0 {
		b.WriteString(fmt.Sprintf("- findings: %d pending\n", ctx.Findings))
	}
	b.WriteString("\n")

	// Current story — what to implement.
	b.WriteString("## Story\n")
	if ctx.EpicID != "" {
		b.WriteString(fmt.Sprintf("- epic: %s — %s\n", ctx.EpicID, ctx.EpicTitle))
	}
	b.WriteString(fmt.Sprintf("- story: %s — %s\n", ctx.StoryID, ctx.StoryTitle))
	b.WriteString("\n")

	// Story specification — full content if loaded from paths config.
	if ctx.StoryContent != "" {
		b.WriteString("### Specification\n\n")
		b.WriteString(ctx.StoryContent)
		b.WriteString("\n\n")
	}

	// User prompt.md — project-specific context BEFORE workflow instructions.
	// This lets users override/extend any default behavior.
	if ctx.PromptMD != "" {
		b.WriteString("## Project Instructions\n\n")
		b.WriteString(ctx.PromptMD)
		b.WriteString("\n\n")
	}

	// Custom sections from config — files and inline content, in order.
	if len(ctx.Sections) > 0 {
		for _, section := range ctx.Sections {
			if !section.IsEnabled() {
				continue
			}
			content := resolveSection(section, ctx.ProjectDir)
			if content == "" {
				continue
			}
			if section.Name != "" {
				b.WriteString(fmt.Sprintf("## %s\n\n", section.Name))
			}
			b.WriteString(content)
			b.WriteString("\n\n")
		}
	}

	// Research tools — injected only when configured and enabled.
	if ctx.Research != nil && ctx.Research.Enabled && len(ctx.Research.Tools) > 0 {
		b.WriteString(buildResearchInstructions(ctx.Research))
		b.WriteString("\n")
	}

	// Workflow — how to implement. Pass commands from config if available.
	b.WriteString(sessionInstructions(ctx.WorkflowType, ctx.WorkflowCommands))
	b.WriteString("\n")

	// Quality gates — what must pass.
	b.WriteString(qualityRules(ctx.QualityGate))
	b.WriteString("\n")

	// Autonomy rules — persistence + exit protocol.
	b.WriteString(autonomyRules())

	// Variable substitution — replace {{var}} placeholders with dynamic values.
	return substituteVars(b.String(), ctx)
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
		return base
	case "minimal":
		return `## Quality Rules
- Tests must pass for changed packages
`
	default:
		return base
	}
}

// sessionInstructions returns workflow-specific instructions.
// When workflow commands are configured, they are injected into the instructions
// so the agent knows which specific tools/skills to invoke for each phase.
func sessionInstructions(workflow string, commands map[string]string) string {
	// Helper to get a command or default.
	cmd := func(phase, fallback string) string {
		if commands != nil {
			if v, ok := commands[phase]; ok && v != "" {
				return v
			}
		}
		return fallback
	}

	switch workflow {
	case "bmad-v6":
		implCmd := cmd("implement", "/dev")
		crCmd := cmd("code_review", "/bmad-bmm-code-review")
		return fmt.Sprintf(`## Workflow: BMAD v6 (MANDATORY — follow exactly)

You MUST use the configured agents for implementation. Invoke the Skill tool:
- Use Skill with skill="%s" to start story implementation
- Use Skill with skill="%s" before EVERY commit

### Execution Steps (in order)
1. **Read story file** — understand ALL acceptance criteria, tasks, and test requirements
2. **Research first** — use research tools for libraries/patterns this story touches
3. **Invoke %s** — this activates the dev agent which follows DoR/DoD
4. **TDD per AC** — for each acceptance criterion: write failing test → implement → pass → refactor
5. **Write REAL code** — create/modify source files in the project (NOT just docs or metadata)
6. **Run %s** — fix ALL findings (HIGH, MEDIUM, LOW)
7. **Run quality gates** — tests pass, build passes, type-check passes, zero dev log errors
8. **Commit** — conventional message with story ID (e.g., "feat: description (65.24)")
9. **Update tracker** — mark story status in sprint-status.yaml

### CRITICAL Rules
- You MUST write actual source code — NOT just docs, metadata, or tracker updates
- You MUST run tests and ensure they pass before committing
- You MUST use the Skill tool to invoke the configured agents
- If the story requires UI changes, create/modify components AND stories
- If the story requires API changes, create/modify resolvers, services, tests
- NEVER mark a story complete without writing and testing real code
`, implCmd, crCmd, implCmd, crCmd)

	case "tdd-strict":
		return `## Workflow: TDD Strict
1. Read story/spec
2. For each AC: RED (failing test) → GREEN (minimal implementation) → REFACTOR
3. Never write implementation before the test
4. Commit test + implementation together
5. Run full test suite before moving to next AC
`
	default:
		var extra string
		if commands != nil && len(commands) > 0 {
			extra = "\n### Agent Commands\n"
			for phase, command := range commands {
				extra += fmt.Sprintf("- %s: invoke `%s`\n", phase, command)
			}
		}
		return `## Workflow: Basic
1. Read the task description
2. Implement the changes
3. Write tests
4. Run tests and fix failures
5. Commit
` + extra
	}
}

// substituteVars replaces {{var}} placeholders in the prompt with dynamic values.
// This allows prompt.md, sections, and inline content to use template variables.
// Supported variables: story_id, story_title, epic_id, epic_title,
// session, progress, workflow, quality, stories_done, stories_total.
func substituteVars(prompt string, ctx PromptContext) string {
	if !strings.Contains(prompt, "{{") {
		return prompt // Fast path — no variables to substitute.
	}

	replacements := map[string]string{
		"{{story_id}}":     ctx.StoryID,
		"{{story_title}}":  ctx.StoryTitle,
		"{{epic_id}}":      ctx.EpicID,
		"{{epic_title}}":   ctx.EpicTitle,
		"{{session}}":      fmt.Sprintf("%d", ctx.SessionNumber),
		"{{progress}}":     fmt.Sprintf("%d/%d", ctx.StoriesDone, ctx.StoriesTotal),
		"{{workflow}}":     ctx.WorkflowType,
		"{{quality}}":      ctx.QualityGate,
		"{{stories_done}}": fmt.Sprintf("%d", ctx.StoriesDone),
		"{{stories_total}}": fmt.Sprintf("%d", ctx.StoriesTotal),
	}

	result := prompt
	for key, val := range replacements {
		result = strings.ReplaceAll(result, key, val)
	}
	return result
}

// resolveSection loads content for a prompt section.
// File takes precedence over inline content. If file is missing, falls back to content.
func resolveSection(section config.PromptSection, projectDir string) string {
	// Try file first.
	if section.File != "" && projectDir != "" {
		content := appcontext.LoadStoryFile(projectDir, section.File)
		if content != "" {
			return content
		}
	}
	// Fallback to inline content.
	return section.Content
}

// buildResearchInstructions generates research-first workflow instructions
// from configured tools. The engine tells the agent WHAT to use and HOW —
// it does NOT call tools directly. This is agnostic to any specific provider.
func buildResearchInstructions(research *config.ResearchConfig) string {
	var b strings.Builder

	b.WriteString("## Research-First (MANDATORY — before implementing)\n\n")
	b.WriteString("Before writing ANY code, you SHALL research using the tools below.\n")
	b.WriteString("Implementation SHALL follow official docs, not stale knowledge.\n\n")

	// Sort tools by priority.
	tools := make([]config.ResearchTool, 0, len(research.Tools))
	for _, t := range research.Tools {
		if t.Enabled {
			tools = append(tools, t)
		}
	}
	sort.Slice(tools, func(i, j int) bool {
		return tools[i].Priority < tools[j].Priority
	})

	// Strategy hint.
	switch research.Strategy {
	case "always":
		b.WriteString("**Strategy:** Research EVERY story before implementation.\n\n")
	case "story-start":
		b.WriteString("**Strategy:** Research at the START of each story, not mid-implementation.\n\n")
	case "on-demand":
		b.WriteString("**Strategy:** Research when encountering unfamiliar APIs or unclear patterns.\n\n")
	}

	// Tool catalog with priority order.
	b.WriteString("### Available Research Tools (use in priority order)\n\n")

	for i, tool := range tools {
		b.WriteString(fmt.Sprintf("**%d. %s** (%s)\n", i+1, tool.Name, tool.Type))
		if tool.Description != "" {
			b.WriteString(fmt.Sprintf("   - What: %s\n", tool.Description))
		}
		if tool.WhenToUse != "" {
			b.WriteString(fmt.Sprintf("   - When: %s\n", tool.WhenToUse))
		}
		if tool.HowToUse != "" {
			b.WriteString(fmt.Sprintf("   - How: %s\n", tool.HowToUse))
		}

		// Sources (for RAG tools with pre-indexed knowledge).
		if len(tool.Sources) > 0 {
			b.WriteString("   - Sources:\n")
			for _, src := range tool.Sources {
				if src.ID != "" {
					b.WriteString(fmt.Sprintf("     - **%s** (id: `%s`)", src.Name, src.ID))
				} else {
					b.WriteString(fmt.Sprintf("     - **%s**", src.Name))
				}
				if src.Description != "" {
					b.WriteString(fmt.Sprintf(" — %s", src.Description))
				}
				b.WriteString("\n")
			}
		}
		b.WriteString("\n")
	}

	b.WriteString("### Anti-patterns (SHALL NOT)\n")
	b.WriteString("- Implementing from memory without checking docs\n")
	b.WriteString("- Guessing API signatures or configuration options\n")
	b.WriteString("- Applying workarounds when official patterns exist\n")
	b.WriteString("- Skipping research because \"I already know this\"\n")

	return b.String()
}

// autonomyRules returns instructions for autonomous operation, progress
// persistence, findings pipeline, and structured exit reporting.
func autonomyRules() string {
	return `## Autonomous Operation

### Decision Making
- Do NOT ask questions — make the safest reasonable assumption
- If a dependency is missing, note it as a finding and continue
- If a test fails, fix it — do not skip or mark as known failure
- If you discover a bug unrelated to this story, note as finding — do not fix inline

### Progress Persistence
- Commit after each completed acceptance criterion (atomic commits)
- Update the status/tracker file after story completion
- If you sense a usage limit approaching: commit → update tracker → stop cleanly

### Findings Pipeline
Note any bugs, patterns, or improvements discovered during implementation.
Do NOT fix unrelated issues. Report them so the engine creates follow-up stories.

### Session Exit
End EVERY session with this structured block (the engine parses it):

` + "```" + `
RALPH_STATUS: <IN_PROGRESS|COMPLETE|BLOCKED>
STORY_ID: <story id>
TASKS_DONE: <number of ACs completed>
TASKS_TOTAL: <total ACs in story>
FILES_MODIFIED: <count>
TESTS_PASSED: <true|false>
BUILD_PASSED: <true|false>
FINDINGS: <count of discovered issues>
EXIT_REASON: <completed|usage_limit|blocked|error>
NEXT_STEP: <brief description of what to do next>
` + "```" + `
`
}
