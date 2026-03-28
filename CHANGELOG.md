# Changelog

## [0.1.0-alpha] — 2026-03-28

### Added

- Autonomous AI development loop with quality gate retries
- Stream-json real-time progress (tool calls, agent responses visible)
- Enriched debug logging (tool names, MCP details, bash commands)
- Cross-platform log rotation (XDG Linux, ~/Library macOS, %APPDATA% Windows)
- Handoff save on usage limit (no AI needed — engine saves from memory)
- First-turn prompt fix (agent must use tools immediately)
- Safety guardrails (destructive action prevention, prompt injection defense)
- `prepare` command with customizable validation hooks
- `doctor` command for project health check
- `init` command with framework presets (BMAD v6, basic)
- Agnostic workflow config (commands + instructions from config.yaml)
- Research-first prompt injection (Archon RAG, Context7, WebSearch)
- Pluggable task tracker (file-based YAML, extensible interface)
- Circuit breaker with configurable failure threshold
- Resource monitoring (RAM, CPU, disk) with auto-pause/stop
- Self-update from GitHub Releases
- npm, Homebrew, curl, go install distribution
- Cross-platform builds (Linux/macOS/Windows, amd64/arm64)
- 350+ tests including 2 fuzz tests, gosec 0 findings

### Fixed

- Usage limit false positive (strict pattern matching, ignores agent text)
- State reset between runs (per-run counters separate from lifetime totals)
- All quality gates skipped detection (metadata-only changes not marked complete)
- Empty session detection (no file changes = not complete)

### Changed

- Replaced `preflight` command with `prepare` (customizable hooks)
- Default output format: stream-json (real-time events)
