# Roadmap

## Phase 2 — Complete (Beta)

Everything marked with [x] is implemented and tested.

- [x] Core engine loop (preflight → pick story → call agent → check gates → repeat)
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

## Phase 2.5 — Near-term (before v1.0.0)

### P0 — Must have for usable product

#### Smart Init Wizard

Interactive project setup that detects existing tools and suggests config.

- [ ] **Greenfield detection** — Empty project or no task files → suggest creating sprint-status.yaml
- [ ] **Brownfield detection** — Scan for existing tools:
  - BMAD: `.bmad/`, `_bmad/`, `sprint-status.yaml` → suggest bmad-v6 preset
  - Claude: `.claude/`, `CLAUDE.md` → suggest claude agent
  - Cursor: `.cursorrules`, `.cursor/` → note: IDE agent, not CLI
  - GitHub: `.github/workflows/` → suggest GitHub Issues tracker
  - Linear: detect via `.linear/` or ask
  - Package managers: `package.json` (npm/pnpm/yarn), `go.mod`, `Cargo.toml`, `pyproject.toml`
  - Test runners: detect from package.json scripts, Makefile, etc.
  - Linters: ESLint, ruff, golangci-lint, etc.
- [ ] **Auto-detect and pre-select** — Show detected tools as pre-selected options
- [ ] **User confirms everything** — Never auto-save without explicit confirmation
- [ ] **Generate hooks.yaml from detected stack** — If TypeScript + Vitest detected → add test/build/type-check gates
- [ ] **Task system selection** — BMAD sprint-status, GitHub Issues, Linear, Jira, plain TODO.md, custom
- [ ] **Multi-language support** — Detect monorepo with multiple languages, add path-based gates per language

#### Hook Execution Engine

Wire hooks.yaml steps into the engine loop.

- [ ] **Preflight hooks** — Execute before loop starts
- [ ] **Pre-story hooks** — Execute before each story
- [ ] **Quality gate hooks** — Execute after implementation, block commit if required step fails
- [ ] **Post-story hooks** — Execute after commit
- [ ] **Post-session hooks** — Execute when engine stops
- [ ] **Path-based filtering** — Only run steps when matching files changed (git diff)
- [ ] **Timeout enforcement** — Kill step after timeout, mark as failed
- [ ] **Remote execution abstraction** — Generic `executor` concept (local, SSH, Docker exec, kubectl)

#### Config Validation

Validate config before run to prevent runtime errors.

- [ ] **YAML format validation** — Parse and report syntax errors with line numbers
- [ ] **Required fields** — Agent type must be set, tracker must have status_file or commands
- [ ] **Path validation** — status_file exists, binary is in PATH, scripts are executable
- [ ] **Type validation** — Numeric fields are numbers, booleans are booleans
- [ ] **Cross-field validation** — If tracker.type=command, commands must be defined
- [ ] **Helpful error messages** — "status_file 'tasks.yaml' not found. Did you mean 'sprint-status.yaml'?"

#### Prompt Injection from prompt.md

Read user's prompt.md and inject into AI sessions.

- [ ] **Read .ralph-engine/prompt.md** at session start
- [ ] **Variable substitution** — `{{story_id}}`, `{{story_title}}`, `{{story_file}}`, `{{status_file}}`
- [ ] **Merge with built-in prompt** — User prompt extends, doesn't replace engine instructions
- [ ] **Max token awareness** — Warn if prompt.md is too large for context window

### P1 — Important for quality

#### CLI Tests

Cobra command integration tests.

- [ ] **Test each command** — run, preflight, status, config set/list, init, version
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
