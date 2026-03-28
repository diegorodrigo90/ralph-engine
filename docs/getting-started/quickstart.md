# Quick Start

Get ralph-engine running in 3 commands.

## Prerequisites

- An AI coding agent installed (Claude Code, Cursor, Codex, etc.)
- A project with tasks to work on (sprint-status.yaml, GitHub Issues, or custom tracker)

## 1. Initialize

```bash
cd your-project
ralph-engine init --preset basic
```

This creates `.ralph-engine/` in your project with:

- `config.yaml` — Engine configuration
- `prompt.md` — Prompt template injected into each AI session
- `hooks.yaml` — Quality gate steps (tests, build, lint)

Available presets:

| Preset       | Description                                         |
| ------------ | --------------------------------------------------- |
| `basic`      | Minimal setup — tests only, file tracker            |
| `bmad-v6`    | Full BMAD workflow — all quality gates, code review |
| `tdd-strict` | TDD-first — strict test enforcement                 |

## 2. Verify setup

```bash
ralph-engine prepare
```

Checks:

- AI agent binary is available
- System resources are adequate (RAM, CPU, disk)
- Project directory is writable
- Sprint status file exists

## 3. Run

```bash
# Dry run first — shows plan without executing
ralph-engine run --dry-run

# Real run — starts the autonomous loop
ralph-engine run
```

Press `Ctrl+C` to save progress and stop gracefully. Resume with `ralph-engine run`.

## What happens during a run

1. Engine reads stories from your tracker (sprint-status.yaml)
2. Picks the next `ready-for-dev` story
3. Calls your AI agent with context prompt
4. Monitors output, checks quality gates
5. Saves state after each story
6. Repeats until all stories are done (or circuit breaker triggers)

## Testing modes

```bash
# Process only 1 story, then stop
ralph-engine run --max-iterations 1

# Process a specific story
ralph-engine run --single-story "65.2"

# Debug mode — JSON structured output
ralph-engine --debug run
```

## Next steps

- [Configuration](../guides/configuration.md) — Customize agent, tracker, quality gates
- [CLI Reference](../reference/cli.md) — All commands and flags
- [Hooks](../guides/hooks.md) — Define custom quality gate steps
