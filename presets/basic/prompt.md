# Ralph Engine — Project Prompt

<!-- This prompt is injected into every AI agent session. -->
<!-- Customize it with YOUR project's context, rules, and conventions. -->
<!-- ralph-engine NEVER overwrites this file after init. -->

## Project Context

<!-- Replace with your project description -->

This project uses ralph-engine for autonomous development.

## Development Rules

- Write tests before implementation (TDD).
- Run all tests before committing.
- Use conventional commit messages: `type(scope): description`
- Keep functions small and focused.

## Story Workflow

1. Read the story description and acceptance criteria.
2. Implement changes following project conventions.
3. Write tests for each acceptance criterion.
4. Run test suite and fix failures.
5. Commit with descriptive message.

## Quality Gates

- Tests must pass.
- Build must succeed.

## Progress

After completing a story:

1. Commit all changes.
2. Report the story as complete.
3. If approaching usage limits, save progress immediately.
