# Configuration

ralph-engine uses a 4-level config cascade. Higher priority wins.

## Priority Order

| Priority | Source         | Location                                       |
| -------- | -------------- | ---------------------------------------------- |
| Highest  | CLI flags      | `ralph-engine run --binary claude`             |
| High     | Environment    | `RALPH_AGENT_TYPE=claude`                      |
| Medium   | Project config | `.ralph-engine/config.yaml` (committed to git) |
| Low      | User config    | `~/.config/ralph-engine/config.yaml`           |
| Baseline | Defaults       | Built-in sensible values                       |

## Project config (.ralph-engine/config.yaml)

Created by `ralph-engine init`. Lives in your project root, committed to git.

```yaml
agent:
  type: "claude" # Agent CLI binary name or path
  model: "opus" # Model to use (agent-specific)
  max_stories_per_session: 5 # Stories per agent invocation
  cooldown_seconds: 30 # Pause between iterations
  allowed_tools: "mcp__*" # Comma-separated tool names

workflow:
  type: "basic" # Workflow preset (basic, bmad-v6, tdd-strict)

quality:
  type: "standard" # Quality gate preset
  gates:
    cr: true # Code review between stories
    tests: true # Run tests
    build: true # Run build
    type_check: true # Run type checker

tracker:
  type: "file" # Tracker type (file, command)
  status_file: "sprint-status.yaml" # Path to status file

circuit_breaker:
  max_failures: 3 # Consecutive failures before stop
  cooldown_minutes: 5 # Wait time in half-open state

resources:
  min_free_ram_mb: 2048 # Minimum free RAM (MB)
  max_cpu_load_percent: 80 # Maximum CPU load (%)
  min_free_disk_gb: 5 # Minimum free disk (GB)

ssh:
  enabled: false # SSH for remote command execution
  reconnect_script: "" # Script to reconnect if SSH drops
  dev_exec_script: "" # Script to run commands in remote env
```

## User config (~/.config/ralph-engine/config.yaml)

Personal preferences that apply to all projects:

```bash
ralph-engine config set agent.type claude
ralph-engine config set agent.model opus
ralph-engine config list
```

## Saving CLI flags to config

Use `--save` to persist any CLI flag to project config:

```bash
ralph-engine run --binary claudebox --save
# Saves agent.type=claudebox to .ralph-engine/config.yaml
```

Only explicitly-changed flags are saved — defaults are never persisted.

## Presets

Presets provide ready-made configs for common workflows:

```bash
ralph-engine init --preset basic      # Tests only
ralph-engine init --preset bmad-v6    # Full BMAD workflow
ralph-engine init --preset tdd-strict # TDD-first
```

Each preset creates 3 files:

| File          | Purpose                         |
| ------------- | ------------------------------- |
| `config.yaml` | Engine configuration            |
| `prompt.md`   | Prompt template for AI sessions |
| `hooks.yaml`  | Quality gate steps              |

**Presets never overwrite existing files.** If `.ralph-engine/config.yaml` exists, `init` skips it.

## Environment variables

All config keys can be set via environment with `RALPH_` prefix:

```bash
export RALPH_AGENT_TYPE=claudebox
export RALPH_TRACKER_STATUS_FILE=my-tasks.yaml
export RALPH_CIRCUIT_BREAKER_MAX_FAILURES=5
```

Nested keys use underscore: `agent.type` → `RALPH_AGENT_TYPE`.

## .ralph-engine/ directory structure

```
.ralph-engine/
├── config.yaml    # Project config (committed)
├── prompt.md      # Prompt template (committed)
├── hooks.yaml     # Quality gate steps (committed)
├── state.json     # Engine state (gitignored — auto-generated)
└── .gitignore     # Protects state.json
```

## Supported AI agents

ralph-engine works with any CLI-based AI coding agent:

| Agent       | Binary      | Notes                                       |
| ----------- | ----------- | ------------------------------------------- |
| Claude Code | `claude`    | Default. Uses `--output-format stream-json` |
| ClaudeBox   | `claudebox` | Containerized Claude Code                   |
| Codex CLI   | `codex`     | OpenAI's CLI agent                          |
| Aider       | `aider`     | Open-source coding assistant                |
| Custom      | any path    | Any CLI that accepts a prompt via `-p` flag |

**Note:** IDE-based agents (Cursor, Windsurf, Copilot in VS Code) are not CLI tools. ralph-engine orchestrates CLI agents that can be called as subprocesses. For IDE agents, use their built-in automation features.

## Next steps

- [Hooks](hooks.md) — Define custom quality gate steps
- [CLI Reference](cli-reference.md) — All commands and flags
- [Troubleshooting](troubleshooting.md) — Common issues
