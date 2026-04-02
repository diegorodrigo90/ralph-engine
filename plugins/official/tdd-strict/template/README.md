# Ralph Engine TDD Strict Starter

This starter is for repositories that want runtime defaults and guardrails aligned with strict TDD.

## Included files

- `config.yaml` — conservative runtime settings for one failing-test-first loop at a time
- `hooks.yaml` — required test and build checks after each agent step
- `prompt.md` — strict TDD operating rules for the session

## Next steps

1. Point the hook commands at the repository's real test and build commands.
2. Add the project-specific constraints that must hold during refactors.
3. Keep the guardrail wording aligned with the policy plugin if the policy contract evolves.
