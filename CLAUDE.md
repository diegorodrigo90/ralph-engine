# CLAUDE.md

Instructions for Claude Code. Full coding standards in [AGENTS.md](AGENTS.md).

## Project Overview

Autonomous AI development loop engine written in Go. Orchestrates CLI-based AI agent sessions (Claude Code, Codex, Aider, custom) with quality gates, resource monitoring, and progress persistence.

## Commands

```bash
# Build
go build -o bin/ralph-engine ./cmd/ralph-engine/

# Test (all packages)
go test ./... -count=1

# Test verbose
go test ./... -count=1 -v

# Test single package
go test ./internal/tracker/ -count=1 -v

# Lint (requires golangci-lint)
golangci-lint run ./...

# Format
go fmt ./...

# Vet
go vet ./...

# Cross-compile check
GOOS=darwin GOARCH=arm64 go build -o /dev/null ./cmd/ralph-engine/
GOOS=windows GOARCH=amd64 go build -o /dev/null ./cmd/ralph-engine/
```

## CLI Commands

```bash
ralph-engine init --preset basic    # Initialize project config
ralph-engine prepare                # Run validation hooks (built-in + custom)
ralph-engine doctor                 # Detailed project health diagnostics
ralph-engine run                    # Start the autonomous loop
ralph-engine run --dry-run          # Preview without executing
ralph-engine status                 # Show current engine state
ralph-engine config set <key> <val> # Set user config
ralph-engine config list            # Show merged config
ralph-engine update                 # Self-update to latest
ralph-engine version                # Show version
```

## Architecture

```
cmd/ralph-engine/main.go     → Entry point (calls cli.Execute)
internal/cli/                → Cobra command tree
internal/config/             → 4-level config cascade (Viper)
internal/engine/             → Core loop + prompt builder
internal/claude/             → AI agent subprocess client
internal/tracker/            → Pluggable task tracking (interface + file impl)
internal/runner/             → Circuit breaker (stagnation detection)
internal/state/              → Persistent state.json
internal/system/             → Resource monitoring (cross-platform)
internal/dashboard/          → Bubbletea TUI
internal/ssh/                → SSH health + self-healing
internal/security/           → First-run security notice
internal/deps/               → Runtime dependency checker
internal/updater/            → Self-update from GitHub Releases
```

## Backward Compatibility (post-v1.0.0 — GOLDEN RULE)

After v1.0.0, ALL changes SHALL be backward-compatible with previous versions:

- **Config fields**: New fields get defaults. NEVER remove or rename existing fields.
- **CLI flags**: New flags OK. NEVER remove or rename existing flags.
- **YAML formats**: New keys OK. Old configs MUST continue to work unchanged.
- **Prompt template**: Additions OK. NEVER remove sections users may depend on.
- **Exit codes**: NEVER change meaning of existing exit codes.
- **Deprecation path**: Old → warn "deprecated, use X instead" for 2 minor versions → remove in next major.
- **User files**: `ralph-engine update` NEVER touches config.yaml, prompt.md, hooks.yaml.
- **Breaking changes**: ONLY in major version bumps (v2.0.0). Document migration guide.

Pre-v1.0.0 (current): API may change freely. Use this window to get the design right.

## Code Standards

- **Language**: ALL code, comments, docs, tests in English
- **TDD**: Write failing test first, then implement
- **Godoc**: All exported types, functions, methods must have doc comments
- **Formatting**: `go fmt` (enforced by CI)
- **Linting**: `golangci-lint` with `.golangci.yml` config
- **Functions**: ≤ 20 lines, ≤ 3 params, meaningful names
- **Errors**: Return errors with context (`fmt.Errorf("doing X: %w", err)`)
- **No panics**: Never panic in library code. Return errors.
- **Testing**: Table-driven tests, test behavior not implementation

## Key Interfaces

```go
// TaskTracker — implement to add new task sources (GitHub, Linear, etc.)
type TaskTracker interface {
    NextStory() (*Story, error)
    MarkComplete(storyID string) error
    MarkInProgress(storyID string) error
    ListPending() ([]Story, error)
    ListAll() ([]Story, error)
}
```

## Commit Messages

Conventional commits: `type(scope): description`
Types: feat, fix, docs, test, refactor, perf, ci, chore

## Before Committing

1. `go fmt ./...`
2. `go vet ./...`
3. `go test ./... -count=1`
4. `go build ./cmd/ralph-engine/`
5. Cross-platform: `GOOS=windows go build -o /dev/null ./cmd/ralph-engine/`
