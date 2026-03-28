# AGENTS.md — AI Coding Assistant Instructions

Universal instructions for ALL AI coding assistants working on ralph-engine.
Rules follow EARS syntax (SHALL keyword) for clarity and testability.
This is the SINGLE source of truth — other files (.cursorrules, CLAUDE.md, GEMINI.md, .windsurfrules, copilot-instructions.md) reference this document.

## Project

ralph-engine is an autonomous AI development loop engine written in Go.
It orchestrates AI agent sessions with quality gates, resource monitoring, and progress persistence.

## Golden Rules

1. **English only** — ALL code, comments, docs, tests, commit messages SHALL be in English.
2. **TDD** — Tests SHALL be written BEFORE implementation. RED → GREEN → REFACTOR.
3. **Verify before commit** — `go vet` → `go test` → `go build` SHALL pass before any commit.
4. **No panics** — Library code SHALL return errors with context. Never `panic()`.
5. **Cross-platform** — Code SHALL compile on Linux, macOS, and Windows. OS-specific code SHALL use build tags.
6. **Security** — No hardcoded secrets. No command injection. External input SHALL be validated.

## Commands

```bash
go build -o bin/ralph-engine ./cmd/ralph-engine/   # Build
go test ./... -count=1                              # Test all
go test ./... -count=1 -v                           # Test verbose
go test ./internal/tracker/ -count=1 -v             # Test one package
go vet ./...                                        # Static analysis
go fmt ./...                                        # Format
golangci-lint run ./...                             # Lint (21 linters)
GOOS=windows GOARCH=amd64 go build -o /dev/null ./cmd/ralph-engine/  # Cross-compile check
```

## Architecture

```
cmd/ralph-engine/main.go     → Entry point
internal/
  cli/       → Cobra commands (run, prepare, doctor, status, config, init, version)
  config/    → 4-level config cascade (Viper: CLI > env > project > user > defaults)
  engine/    → Core loop orchestration + prompt builder
  claude/    → AI agent subprocess client (os/exec, stream-json)
  tracker/   → Pluggable task tracking interface + file tracker (YAML)
  runner/    → Circuit breaker (stagnation detection)
  state/     → Persistent state.json (atomic writes)
  system/    → Resource monitoring: RAM/CPU/disk (cross-platform via build tags)
  dashboard/ → Bubbletea TUI (real-time progress)
  ssh/       → SSH health checking + self-healing
  security/  → First-run security notice + acceptance
  deps/      → Runtime dependency checker
  logger/    → Structured logging (human + JSON for AI agents)
```

## Code Standards (EARS)

- **Godoc** — All exported types, functions, methods, interfaces SHALL have doc comments ending with a period.
- **Functions** — Functions SHALL have ≤ 20 lines, ≤ 3 parameters, and single responsibility.
- **Error handling** — Errors SHALL be wrapped with context: `fmt.Errorf("doing X: %w", err)`. Errors SHALL NOT be silently ignored.
- **Testing** — Tests SHALL be table-driven. Tests SHALL test behavior, not implementation details. Test helpers SHALL use `t.Helper()`.
- **Naming** — Go naming conventions SHALL be followed: `MixedCaps` for exported, `mixedCaps` for unexported. No underscores in Go identifiers.
- **Packages** — Each package SHALL have a single responsibility. Circular dependencies SHALL NOT exist.
- **Interfaces** — Interfaces SHALL be defined in the consumer package, not the provider. Functions SHALL accept interfaces and return concrete structs.
- **Zero values** — Struct constructors SHALL handle zero-value fields with sensible defaults.
- **Context** — Long-running operations SHALL accept `context.Context` as the first parameter.

## Key Extension Points

### Adding a new tracker

1. Implement `tracker.TaskTracker` interface in `internal/tracker/`
2. Register factory in `tracker.Registry`
3. Add table-driven tests
4. Update README.md trackers section

```go
type TaskTracker interface {
    NextStory() (*Story, error)
    MarkComplete(storyID string) error
    MarkInProgress(storyID string) error
    ListPending() ([]Story, error)
    ListAll() ([]Story, error)
}
```

### Adding a new agent

1. Extend `claude.ClientConfig` with agent-specific flags
2. Add case to `buildArgs()` for the agent's CLI format
3. Add tests for argument building

### Adding a new workflow

1. Add case to `sessionInstructions()` in `internal/engine/prompt.go`
2. Add preset in `internal/config/config.go`
3. Add tests for prompt generation

## Commit Convention

Commit messages SHALL follow conventional format:

```
type(scope): description

Types: feat, fix, docs, test, refactor, perf, ci, chore, build
Scope: package name (e.g., tracker, engine, cli) — optional

Examples:
  feat(tracker): add GitHub Issues tracker
  fix(engine): prevent nil panic when tracker returns empty
  test(ssh): add reconnection timeout test
  docs: update README with new workflow presets
```

## Quality Checklist

WHEN committing code, the following checks SHALL pass:

- [ ] `go fmt ./...` — code formatted
- [ ] `go vet ./...` — no static analysis issues
- [ ] `go test ./... -count=1` — all tests pass
- [ ] `go build ./cmd/ralph-engine/` — binary compiles
- [ ] `GOOS=windows go build -o /dev/null ./cmd/ralph-engine/` — cross-platform
- [ ] Godoc on all new exported symbols
- [ ] No `TODO` or `FIXME` without linked issue number
- [ ] Conventional commit message format

## Debug Mode

WHEN debugging issues, use `--debug` flag for AI-friendly JSON output:

```bash
ralph-engine --debug run        # JSON structured logs with component, suggestion, docs fields
ralph-engine --log-format json  # Force JSON without full debug verbosity
```

Error JSON includes `component`, `suggestion`, and `docs` fields for autonomous diagnosis.
