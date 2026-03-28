# CLI Reference

## Global Flags

| Flag           | Description                             | Default |
| -------------- | --------------------------------------- | ------- |
| `--debug`      | Enable debug mode (verbose JSON output) | `false` |
| `--log-format` | Log format: `human` or `json`           | `human` |

## Commands

### `ralph-engine run`

Start the autonomous development loop.

```bash
ralph-engine run [flags]
```

| Flag               | Description                          | Default              |
| ------------------ | ------------------------------------ | -------------------- |
| `--project`        | Project directory                    | Current directory    |
| `--binary`         | AI agent binary                      | `claude`             |
| `--status-file`    | Sprint status file path              | `sprint-status.yaml` |
| `--max-failures`   | Circuit breaker threshold            | `3`                  |
| `--dry-run`        | Show execution plan without running  | `false`              |
| `--max-iterations` | Stop after N stories                 | `0` (unlimited)      |
| `--single-story`   | Process only this story ID           | `""`                 |
| `--save`           | Save changed flags to project config | `false`              |

**Examples:**

```bash
# Normal run
ralph-engine run

# Dry run — see what would happen
ralph-engine run --dry-run

# Process 1 story only
ralph-engine run --max-iterations 1

# Process specific story
ralph-engine run --single-story "65.2"

# Use ClaudeBox and save preference
ralph-engine run --binary claudebox --save

# Debug mode with JSON output
ralph-engine --debug run
```

### `ralph-engine preflight`

Run pre-execution checks without starting the loop.

```bash
ralph-engine preflight [flags]
```

| Flag        | Description              | Default           |
| ----------- | ------------------------ | ----------------- |
| `--project` | Project directory        | Current directory |
| `--binary`  | AI agent binary to check | `claude`          |

Checks:

- Agent binary is available in PATH
- System resources (RAM, CPU, disk) are adequate
- Project directory exists and is writable
- State directory is writable

### `ralph-engine status`

Show current engine state.

```bash
ralph-engine status [flags]
```

| Flag        | Description       | Default           |
| ----------- | ----------------- | ----------------- |
| `--project` | Project directory | Current directory |

Shows:

- Current story (if in progress)
- Session count and duration
- Circuit breaker state
- Resource snapshot

### `ralph-engine config set`

Set a user-level config value.

```bash
ralph-engine config set <key> <value>
```

**Examples:**

```bash
ralph-engine config set agent.type claudebox
ralph-engine config set agent.model opus
ralph-engine config set circuit_breaker.max_failures 5
```

### `ralph-engine config list`

Show merged config from all sources.

```bash
ralph-engine config list
```

### `ralph-engine init`

Initialize project config.

```bash
ralph-engine init [flags]
```

| Flag       | Description                                      | Default |
| ---------- | ------------------------------------------------ | ------- |
| `--preset` | Config preset (`basic`, `bmad-v6`, `tdd-strict`) | `basic` |

Creates `.ralph-engine/` directory with config.yaml, prompt.md, and hooks.yaml.

**Never overwrites existing files.**

### `ralph-engine version`

Show version information.

```bash
ralph-engine version
```

## Exit codes

| Code  | Meaning                                      |
| ----- | -------------------------------------------- |
| `0`   | Success (all stories done, or clean exit)    |
| `1`   | Error (config issue, binary not found, etc.) |
| `2`   | Circuit breaker triggered                    |
| `3`   | Usage limit reached (progress saved)         |
| `130` | User interrupt (Ctrl+C — progress saved)     |
