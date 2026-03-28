# Ralph Engine — BMAD v6 Prompt

<!-- Injected into every AI session via --append-system-prompt. -->
<!-- Customize with YOUR project's rules, conventions, and context. -->
<!-- ralph-engine NEVER overwrites this file after init. -->

## BMAD v6 Workflow

You are operating inside a ralph-engine autonomous loop using BMAD v6 methodology.

### Story Execution Order (MANDATORY)

1. **Read story file** — Understand all acceptance criteria (ACs).
2. **DoR validation** — Verify ACs are testable, tasks sequenced, deps resolved.
3. **TDD per AC** — For each AC: write failing test → implement → pass → refactor.
4. **Code review** — Run CR. Fix ALL findings (HIGH, MEDIUM, LOW).
5. **Quality gates** — tests → build → type-check → dev logs.
6. **Commit** — Conventional message with story ID.
7. **Update tracker** — Mark story status in sprint-status file.
8. **Note findings** — Report bugs/patterns discovered for findings pipeline.

### Quality Rules (MANDATORY — No Exceptions)

- ALL tests SHALL pass before commit.
- Build SHALL pass for the full project.
- Type-check SHALL show zero new errors.
- Code review findings SHALL be fixed, not deferred.
- Dev logs SHALL show ZERO errors.

### Testing Standards

- TDD per AC: RED → GREEN → REFACTOR.
- Each AC SHALL have ≥ 1 test.
- Test behavior, not implementation details.
- Error scenarios SHALL be tested (not-found, forbidden, invalid-input).

### Code Standards

- Functions ≤ 20 lines, ≤ 3 parameters.
- Meaningful names. No abbreviations.
- Errors wrapped with context.
- Exported symbols have doc comments.
- SOLID principles enforced.

### Findings Pipeline

During implementation, note any:

- Bugs found in existing code
- Patterns that should become rules
- Missing specs or inconsistencies
- Performance issues discovered

Report findings at end of session. Do NOT fix unrelated bugs inline.

### Progress Persistence (CRITICAL)

- After EVERY commit, report stories completed.
- If usage limit approaching, IMMEDIATELY:
  1. Commit pending work.
  2. Update sprint status.
  3. Save handoff note with next steps.
- The engine saves state automatically on exit.

### Session Context

<!-- ralph-engine injects dynamic context here at runtime: -->
<!-- - Current story ID, title, epic -->
<!-- - Progress (N/M stories done) -->
<!-- - Session number -->
<!-- - SSH availability -->
<!-- - Accumulated findings count -->
