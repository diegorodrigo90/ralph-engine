# Ralph Engine — TDD Strict Prompt

<!-- Injected into every AI session. Customize for your project. -->

## TDD Rules (MANDATORY)

For EVERY acceptance criterion:

1. **RED** — Write a failing test FIRST. Run it. Confirm it fails.
2. **GREEN** — Write MINIMAL code to make the test pass. Nothing more.
3. **REFACTOR** — Clean up code while keeping tests green.

NEVER write implementation before the test exists and fails.
NEVER write multiple tests before implementing each one.

## Commit Strategy

Each commit SHALL contain:

- The failing test
- The minimal implementation that passes it
- Any necessary refactoring

Commit after each RED→GREEN→REFACTOR cycle.

## Quality Gates

- `go test ./... -count=1` (or project equivalent) SHALL pass.
- `go vet ./...` (or equivalent) SHALL pass.
- Build SHALL succeed.
- Type-check SHALL pass.
- Code review findings SHALL be fixed before commit.

## Progress

After completing a story:

1. Verify all tests pass.
2. Commit with message: `feat(scope): description`
3. Report story complete.
