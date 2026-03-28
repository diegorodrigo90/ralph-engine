# Roadmap

## Phase 2 — Complete (Beta)

Everything marked with [x] is implemented and tested.

- [x] Core engine loop (prepare → pick story → call agent → check gates → repeat)
- [x] 4-level config cascade (CLI > env > project > user > defaults)
- [x] Pluggable trackers (file, flat YAML, command)
- [x] Circuit breaker (stagnation detection)
- [x] Resource monitoring (RAM, CPU, disk — cross-platform)
- [x] AI agent subprocess client (stream-json parser, usage limit detection)
- [x] Session persistence (state.json with atomic writes)
- [x] Security notice (first-run acceptance)
- [x] Runtime dependency checker
- [x] Bubbletea TUI dashboard
- [x] Structured logging (human + JSON formats)
- [x] SSH health + self-healing
- [x] Dynamic prompt builder
- [x] Presets (basic, bmad-v6, tdd-strict)
- [x] Testing modes (--dry-run, --max-iterations, --single-story)
- [x] Debug mode (--debug, JSON output for AI agents)
- [x] Config auto-read + --save flag
- [x] Cross-platform builds (Linux/macOS/Windows × amd64/arm64)
- [x] CI/CD (lint, test on 3 OS, security scanning, smoke tests)
- [x] Distribution (npm, Homebrew, curl, go install, binary)
- [x] Documentation (install, quickstart, config, CLI, hooks, building, troubleshooting, architecture)
- [x] AI tool configs (AGENTS.md, CLAUDE.md, GEMINI.md, .cursorrules, .windsurfrules, copilot-instructions)
- [x] OSS infrastructure (LICENSE, CONTRIBUTING, SECURITY, issue templates, PR template)

## Phase 2.5 — Complete (v0.1.0-alpha)

### P0 — Must have for usable product (ALL COMPLETE)

#### Smart Init Wizard ✅

- [x] Greenfield/brownfield detection
- [x] Auto-detect: BMAD, Claude, Cursor, Windsurf, Gemini, Node.js, Go, Rust, Python, Java, Ruby, Elixir, PHP
- [x] Detect test runners (Vitest, Jest, Playwright, pytest), linters (ESLint, Prettier, golangci-lint, Ruff)
- [x] Detect monorepo (pnpm-workspace, turbo, lerna, nx), CI/CD (GitHub Actions, GitLab, CircleCI, Jenkins)
- [x] Detect trackers (sprint-status.yaml, TODO.md, Linear, GitHub Issues)
- [x] Suggest preset based on detected tools (basic/bmad-v6/tdd-strict)
- [ ] Generate hooks.yaml from detected stack (future: detect language → generate matching gates)
- [ ] Interactive mode with user confirmation (future: bubbletea interactive prompts)

#### Hook Execution Engine ✅

- [x] Prepare hooks — execute before loop starts, block on failure
- [x] Pre-story hooks — execute before each agent session
- [x] Quality gate hooks — execute after session, block commit if required step fails
- [x] Post-story hooks — execute after story marked complete
- [x] Post-session hooks — execute on engine stop (best-effort, 30s timeout)
- [x] Path-based filtering — git diff → skip steps when no matching files changed
- [x] Timeout enforcement — kill process group after timeout, mark as failed
- [ ] Remote execution abstraction — Generic `executor` concept (future P1)

#### Config Validation ✅

- [x] Required fields (agent type, status file)
- [x] Path validation (status_file exists, binary in PATH, scripts exist)
- [x] Type validation (numeric ranges, boolean)
- [x] Cross-field validation (tracker type + commands)
- [x] Helpful error messages with suggestions
- [x] Research config consistency (tools, priority, strategy)

#### Prompt Injection ✅

- [x] Read .ralph-engine/prompt.md at session start
- [x] Variable substitution: `{{story_id}}`, `{{story_title}}`, `{{epic_id}}`, etc.
- [x] Merge with built-in prompt (user extends, doesn't replace)
- [x] Composable sections from config (file + inline content)
- [x] Story file injection from tracker FilePath or paths.stories search
- [x] Research tools instructions injected from config
- [x] DRY: rules-digest.md as single source of truth

#### Usage Limit + State Fixes (Session 7) ✅

- [x] Usage limit false positive fix (strict pattern matching, ignores agent text)
- [x] State reset between runs (per-run counters separate from lifetime totals)
- [x] All quality gates skipped detection (metadata-only changes not marked complete)
- [x] Empty session detection (no file changes = not complete)

#### Debug + Logging Enrichment (Session 7) ✅

- [x] Tool names in debug log (tool_use name + key input params)
- [x] MCP tool details (server.tool(params) format)
- [x] Bash command extraction (command → exit code)
- [x] Cross-platform log file location (XDG Linux, ~/Library macOS, %APPDATA% Windows)
- [x] Log rotation (per-run files, max 10, max 50MB, configurable)

#### Handoff + First-Turn (Session 7) ✅

- [x] Handoff save WITHOUT AI (engine saves from memory on usage limit)
- [x] First-turn prompt fix (agent must use tools immediately)

### P1 — Important for quality

#### CLI Tests

Cobra command integration tests.

- [ ] **Test each command** — run, prepare, status, config set/list, init, version
- [ ] **Test flag parsing** — Verify flags override config correctly
- [ ] **Test --save** — Verify config file is written with correct values
- [ ] **Test error cases** — Missing binary, bad config, no stories

#### Remote Execution Abstraction

Replace SSH-specific code with generic executor interface.

- [ ] **Executor interface** — `Execute(ctx, command) (output, error)`
- [ ] **LocalExecutor** — Run commands locally (default)
- [ ] **SSHExecutor** — Run via SSH (current ssh package)
- [ ] **DockerExecutor** — Run via `docker exec`
- [ ] **KubeExecutor** — Run via `kubectl exec`
- [ ] **Config** — `remote.type: "local" | "ssh" | "docker" | "kubectl"`

#### Additional Tracker Formats

Support more task file formats.

- [ ] **TOML tracker** — Parse TOML task files
- [ ] **Markdown tracker** — Parse TODO.md / TASKS.md with checkbox format
- [ ] **JSON tracker** — Parse JSON task files
- [ ] **Auto-detect by extension** — .yaml/.yml → YAML, .toml → TOML, .md → Markdown, .json → JSON

### P2 — Nice to have

#### GitHub Pages Documentation Site

Full docs site with search, navigation, theming.

- [ ] **MkDocs or Hugo** — Static site generator from current markdown docs
- [ ] **Auto-deploy on release** — GitHub Actions workflow
- [ ] **Search** — Full-text search across docs
- [ ] **API reference** — Auto-generated from godoc comments

#### Native GitHub Issues Tracker

Built-in tracker that reads/writes GitHub Issues.

- [ ] **List issues by label** — `ralph-engine` label or configurable
- [ ] **Create/update issues** — Mark in-progress, add comments, close on complete
- [ ] **Project board integration** — Move cards between columns

#### Native Linear Tracker

Built-in tracker for Linear.

- [ ] **API integration** — Read/write issues via Linear API
- [ ] **State mapping** — Linear states → ralph-engine statuses

## Phase 3 — Open Source Launch

- [ ] First stable release v1.0.0
- [ ] npm package published
- [ ] Homebrew tap configured
- [ ] Documentation site live
- [ ] Blog post / announcement
- [ ] Community templates (presets for popular stacks)
