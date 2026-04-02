# Ralph Engine BMAD Starter

This starter sets up a project for a BMAD-guided workflow where artifacts, prompts, and checks remain explicit.

## Included files

- `config.yaml` — runtime defaults for a BMAD-oriented implementation loop
- `hooks.yaml` — required checks that keep artifact and code alignment visible
- `prompt.md` — shared workflow prompt bundle for BMAD sessions

## Next steps

1. Replace the domain context in `prompt.md`.
2. Connect `hooks.yaml` to the real validation commands of the repository.
3. Tune the workflow instructions in `config.yaml` to the project's artifact contract.
