# Architecture

## Design Principles

1. **Engine is separate from project config** — Tool is global, configs are per-project
2. **Updates never overwrite user configs** — `init` uses `writeIfNotExists`
3. **Pluggable everything** — Trackers, quality gates, workflows
4. **Context-aware** — Each AI session gets fresh context, state persists in files
5. **Resource-safe** — Monitors RAM/CPU/disk between iterations
6. **Security-transparent** — Clear warnings, container isolation recommended
7. **Never manages billing** — Detects limits, saves progress, stops gracefully

## Package Map

```
cmd/ralph-engine/main.go
    │
    └── internal/cli/          ← Cobra command tree
        ├── root.go            ← --debug, --log-format flags
        ├── run.go             ← Main loop command
        ├── preflight.go       ← Pre-execution checks
        ├── status.go          ← Show engine state
        ├── config.go          ← Config get/set/list
        ├── init.go            ← Project initialization
        └── version.go         ← Version info

internal/engine/               ← Core loop
    ├── engine.go              ← Engine struct, Start/Stop/Status
    ├── run.go                 ← Autonomous loop (pick story → call agent → check gates)
    └── prompt.go              ← Dynamic prompt builder

internal/claude/               ← AI agent client
    └── client.go              ← Subprocess management, stream parsing, usage limit detection

internal/config/               ← Configuration
    └── config.go              ← 4-level cascade (Viper), Load/Save, presets

internal/state/                ← Persistence
    └── state.go               ← state.json read/write (atomic via tmp+rename)

internal/tracker/              ← Task tracking
    ├── tracker.go             ← TaskTracker interface + Story type
    ├── registry.go            ← Tracker factory registry
    ├── file_tracker.go        ← Structured YAML (epics[].stories[])
    ├── flat_tracker.go        ← Flat YAML (BMAD v6 format)
    ├── command_tracker.go     ← User-defined scripts (any task system)
    └── detect.go              ← Auto-detect tracker type from file content

internal/runner/               ← Circuit breaker
    └── circuit_breaker.go     ← Nygard pattern: CLOSED → HALF_OPEN → OPEN

internal/system/               ← Resource monitoring
    ├── resources.go           ← RAM, CPU, disk checks
    ├── disk_unix.go           ← Unix disk stats (build tag: !windows)
    └── disk_windows.go        ← Windows disk stats (build tag: windows)

internal/dashboard/            ← TUI
    └── model.go               ← Bubbletea model/view/update

internal/ssh/                  ← Remote execution
    └── health.go              ← SSH check, reconnect, exec, self-healing

internal/security/             ← Security
    └── notice.go              ← First-run acceptance, persistence

internal/deps/                 ← Dependencies
    └── checker.go             ← Runtime binary verification

internal/logger/               ← Logging
    └── logger.go              ← Human (colored) + JSON (AI-friendly) formats
```

## Data Flow

```
User: ralph-engine run
  │
  ├─ CLI (cobra) parses flags
  ├─ Config loads (4-level cascade)
  ├─ Logger initializes (human or JSON)
  │
  ├─ Engine.Run()
  │   ├─ Preflight checks (deps, resources, security)
  │   ├─ State.Load() (resume from checkpoint)
  │   ├─ Tracker.ListPending() (read sprint-status.yaml)
  │   │
  │   ├─ LOOP:
  │   │   ├─ Tracker.NextStory()
  │   │   ├─ Tracker.MarkInProgress(storyID)
  │   │   ├─ Prompt.Build(story, config, state)
  │   │   ├─ Claude.Run(prompt)
  │   │   │   ├─ os/exec.Command(binary, args...)
  │   │   │   ├─ Stream stdout → parse JSON lines
  │   │   │   └─ Check exit code + usage limits
  │   │   ├─ CircuitBreaker.Record(success/failure)
  │   │   ├─ Tracker.MarkComplete(storyID)
  │   │   ├─ State.Save() (checkpoint)
  │   │   ├─ System.Check() (resources OK?)
  │   │   ├─ SSH.Check() (connection alive?)
  │   │   └─ time.Sleep(cooldown)
  │   │
  │   └─ EXIT: all_done | circuit_breaker | usage_limit | ctrl+c | resource_critical
  │
  └─ State.Save() (final checkpoint)
```

## Key Interfaces

```go
// TaskTracker — the extension point for adding task sources.
type TaskTracker interface {
    NextStory() (*Story, error)
    MarkComplete(storyID string) error
    MarkInProgress(storyID string) error
    ListPending() ([]Story, error)
    ListAll() ([]Story, error)
}
```

## Concurrency Model

- Engine runs in a single goroutine (no concurrent story processing)
- `sync.RWMutex` protects status fields (read by Status(), written by Run())
- `context.Context` propagates cancellation (Ctrl+C → graceful shutdown)
- State writes are atomic (tmp file + rename)

## Cross-Platform

- OS-specific code uses Go build tags (`//go:build !windows`)
- Currently: disk monitoring (syscall.Statfs vs GetDiskFreeSpaceEx)
- All file paths use `filepath.Join` (handles `/` vs `\`)
- Shell commands in CommandTracker use `findShell()` for portability
