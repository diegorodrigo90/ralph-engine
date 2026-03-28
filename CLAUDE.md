# CLAUDE.md

Instructions for Claude Code. Full coding standards in [AGENTS.md](AGENTS.md).

## Project Overview

Autonomous AI development loop engine written in Go. Orchestrates AI agent sessions (Claude, Cursor, Codex) with quality gates, resource monitoring, and progress persistence.

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
```

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
