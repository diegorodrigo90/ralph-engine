# ralph-engine

[![CI](https://github.com/diegorodrigo90/ralph-engine/actions/workflows/ci.yml/badge.svg)](https://github.com/diegorodrigo90/ralph-engine/actions/workflows/ci.yml)
[![Go Report Card](https://goreportcard.com/badge/github.com/diegorodrigo90/ralph-engine)](https://goreportcard.com/report/github.com/diegorodrigo90/ralph-engine)
[![Go Reference](https://pkg.go.dev/badge/github.com/diegorodrigo90/ralph-engine.svg)](https://pkg.go.dev/github.com/diegorodrigo90/ralph-engine)
[![npm](https://img.shields.io/npm/v/ralph-engine)](https://www.npmjs.com/package/ralph-engine)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Autonomous AI development loop engine. Orchestrates CLI-based AI agent sessions (Claude Code, Codex, Aider, custom) in an infinite loop with quality gates, resource monitoring, and persistent progress tracking.

## Why ralph-engine?

AI coding agents are powerful but need orchestration for large projects. ralph-engine solves:

- **Context limits** — Each agent session gets fresh context. State persists in files between sessions.
- **Quality drift** — Enforces TDD, code review, tests, build, type-check between every story.
- **Stagnation** — Circuit breaker stops after N consecutive failures instead of burning tokens.
- **Resource safety** — Monitors RAM/CPU/disk to prevent freezing the host machine.
- **Progress loss** — Saves state after every commit. Resume exactly where you stopped.

## Features

- **Infinite loop** — Calls AI agent repeatedly, each invocation gets fresh context
- **Pluggable agents** — Claude Code, ClaudeBox, Codex, Aider, any CLI agent
- **Pluggable workflows** — BMAD v6, basic, TDD-strict, spec-driven, custom
- **Pluggable quality gates** — Full (CR + tests + build + type-check + E2E), standard, minimal
- **Pluggable trackers** — File (sprint-status.yaml), GitHub Issues, Linear, custom
- **Resource monitoring** — RAM, CPU, disk checks prevent host freezing
- **Circuit breaker** — Stops after N consecutive failures (stagnation detection)
- **Usage limit detection** — Detects API limits, saves progress, graceful stop
- **Session persistence** — Resume from exact checkpoint after interruption
- **Cross-platform** — Linux, macOS, Windows (WSL2)
- **Debug mode** — JSON structured output optimized for AI agent consumption
- **Professional TUI** — Real-time dashboard with bubbletea

## Quick Start

### Install

Pick your preferred method — all are automatically updated on every release:

| Method       | Command                                                                     | Requires    |
| ------------ | --------------------------------------------------------------------------- | ----------- |
| **npm**      | `npm install -g ralph-engine`                                               | Node.js 16+ |
| **npx**      | `npx ralph-engine run --dry-run`                                            | Node.js 16+ |
| **Homebrew** | `brew install diegorodrigo90/tap/ralph-engine`                              | macOS/Linux |
| **curl**     | see below                                                                   | curl        |
| **Go**       | `go install github.com/diegorodrigo90/ralph-engine/cmd/ralph-engine@latest` | Go 1.24+    |
| **Binary**   | [GitHub Releases](https://github.com/diegorodrigo90/ralph-engine/releases)  | —           |

**One-line install** (Linux, macOS, WSL):

```bash
curl -fsSL https://raw.githubusercontent.com/diegorodrigo90/ralph-engine/main/scripts/install.sh | bash
```

**Build from source:**

```bash
git clone https://github.com/diegorodrigo90/ralph-engine.git
cd ralph-engine
./scripts/build-local.sh
# Or: go build -o bin/ralph-engine ./cmd/ralph-engine/
```

### Usage

```bash
# 1. Initialize project config
ralph-engine init --preset basic

# 2. Verify your setup
ralph-engine preflight

# 3. Start the autonomous loop
ralph-engine run
```

The engine reads stories from `sprint-status.yaml`, calls your AI agent for each one, enforces quality gates, and saves progress between sessions.

Press `Ctrl+C` to save progress and stop gracefully. Resume with `ralph-engine run`.

## Configuration

ralph-engine uses a 4-level config cascade (highest priority first):

| Priority | Source         | Example                                 |
| -------- | -------------- | --------------------------------------- |
| Highest  | CLI flags      | `ralph-engine run --binary claudebox`   |
| High     | Environment    | `RALPH_AGENT_TYPE=claudebox`            |
| Medium   | Project config | `.ralph-engine/config.yaml` (committed) |
| Low      | User config    | `~/.config/ralph-engine/config.yaml`    |
| Baseline | Defaults       | Built-in sensible values                |

### Presets

```bash
ralph-engine init --preset basic      # Tests only, file tracker
ralph-engine init --preset bmad-v6    # Full BMAD workflow, all quality gates
ralph-engine init --preset tdd-strict # TDD-first, strict test enforcement
```

### Example config

```yaml
agent:
  type: "claude"
  model: "opus"
  max_stories_per_session: 5 # default: 5
  cooldown_seconds: 30 # default: 30

workflow:
  type: "bmad-v6"

quality:
  type: "full"
  gates:
    cr: true
    tests: true
    build: true
    type_check: true

tracker:
  type: "file"
  status_file: "sprint-status.yaml"

circuit_breaker:
  max_failures: 3
  cooldown_minutes: 5

resources:
  min_free_ram_mb: 2048
  max_cpu_load_percent: 80
  min_free_disk_gb: 5
```

### User preferences

```bash
ralph-engine config set agent.type claudebox
ralph-engine config set agent.model opus
ralph-engine config list
```

## Commands

| Command                                 | Description               |
| --------------------------------------- | ------------------------- |
| `ralph-engine run`                      | Start the autonomous loop |
| `ralph-engine preflight`                | Run pre-execution checks  |
| `ralph-engine status`                   | Show current engine state |
| `ralph-engine config set <key> <value>` | Set user config           |
| `ralph-engine config list`              | Show merged config        |
| `ralph-engine init [--preset name]`     | Initialize project        |
| `ralph-engine update`                   | Self-update to latest     |
| `ralph-engine version`                  | Show version              |

### Debug Mode

For AI-friendly structured output (great for debugging with AI agents):

```bash
ralph-engine --debug run           # JSON logs with component, suggestion, docs
ralph-engine --log-format json run # Force JSON without debug verbosity
```

## Tracker Integration

### File Tracker (default)

Reads `sprint-status.yaml`:

```yaml
epics:
  - id: "65"
    title: "Permission System"
    status: "in-progress"
    stories:
      - id: "65.1"
        title: "Custom Roles CRUD"
        status: "done"
      - id: "65.2"
        title: "User Permission Grant/Deny"
        status: "ready-for-dev"
```

### Coming Soon

- **GitHub Issues** — Track stories via GitHub Issues/Projects
- **Linear** — Track stories via Linear API
- **Custom** — Any script that outputs JSON

## Execution Flow

```
ralph-engine run
  │
  ├─ PREFLIGHT
  │   ├─ Project directory exists
  │   ├─ Agent binary available (claude, claudebox, etc.)
  │   ├─ System resources OK (RAM, CPU, disk)
  │   └─ State directory writable
  │
  ├─ LOOP (infinite)
  │   ├─ Pick next story from tracker
  │   ├─ Call AI agent session with context prompt
  │   ├─ Stream output → dashboard
  │   ├─ Check results (exit code, usage limit)
  │   ├─ Resource check between iterations
  │   ├─ Circuit breaker check
  │   ├─ Save state checkpoint
  │   └─ Cooldown → next story
  │
  └─ EXIT
      ├─ all_complete — all stories done
      ├─ circuit_breaker — too many consecutive failures
      ├─ usage_limit — API limit reached (progress saved)
      ├─ user_interrupt — Ctrl+C (progress saved)
      └─ resource_critical — host resources critically low
```

## Security

- **Container isolation recommended** — Run inside ClaudeBox or Docker
- **First-run security notice** — Explicit acceptance required for `--dangerously-skip-permissions`
- **Engine NEVER manages billing** — Only detects usage limits and saves progress
- **No secrets in engine** — API keys are managed by the agent externally
- **CI security scanning** — gosec (SAST), govulncheck (CVEs), Trivy (filesystem)

See [SECURITY.md](.github/SECURITY.md) for vulnerability reporting.

## Contributing

We welcome contributions! Whether you're fixing a typo or adding a new tracker, every contribution helps.

**New to open source?** Check out issues labeled [`good first issue`](https://github.com/diegorodrigo90/ralph-engine/labels/good%20first%20issue).

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full guide including:

- Development setup (clone → build in 3 commands)
- How to add trackers, agents, and workflows
- Code standards and commit conventions
- Pull request process

## Architecture

```
ralph-engine/
├── cmd/ralph-engine/main.go     # Entry point
├── internal/
│   ├── cli/                     # Cobra command tree
│   ├── claude/                  # AI agent subprocess client
│   ├── config/                  # Viper 4-level config cascade
│   ├── dashboard/               # Bubbletea TUI
│   ├── deps/                    # Runtime dependency checker
│   ├── engine/                  # Core loop + prompt builder
│   ├── logger/                  # Structured logging (human/JSON, debug mode)
│   ├── runner/                  # Circuit breaker
│   ├── security/                # First-run security notice
│   ├── ssh/                     # SSH health + self-healing
│   ├── state/                   # Persistent state.json
│   ├── system/                  # Resource monitoring (cross-platform)
│   ├── tracker/                 # Pluggable task tracking
│   └── updater/                 # Self-update from GitHub Releases
├── AGENTS.md                    # AI assistant instructions (universal)
├── CONTRIBUTING.md              # Contribution guide
├── .golangci.yml                # Linter config (21 rules)
├── .goreleaser.yaml             # Cross-platform release automation
└── .github/workflows/           # CI + Release pipelines
```

## Versioning

ralph-engine follows [Semantic Versioning](https://semver.org/):

- `v0.x.x` — Pre-release, API may change
- `v1.0.0` — First stable release
- Tag a release: `git tag v1.0.0 && git push origin v1.0.0`
- CI automatically builds binaries for all platforms via GoReleaser

## License

[MIT](LICENSE) — Free and open source.
