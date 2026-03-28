# Ralph Engine — Improvements Backlog

Collected from session 6 and 7 host testing. Prioritized by impact.

## P0 — COMPLETED (v0.1.0-alpha)

All P0 items from session 6 testing have been resolved:

- [x] **Handoff save WITHOUT AI** — Engine saves progress from memory on usage limit (no AI call needed). Writes to `.ralph-engine/handoff-{story_id}.json`.
- [x] **First-turn problem** — Stronger prompt forces agent to START with tool calls on turn 1. No more "I'll implement..." text-only exits.
- [x] **state.json / sprint-status.yaml sync** — Both written atomically, state reads from YAML as source of truth.
- [x] **SSH reconnect cooldown** — Time-based retry (not count). Config: `ssh.reconnect_cooldown_minutes: 5`.
- [x] **Usage limit false positive** — Strict pattern matching, ignores agent text that mentions limits.
- [x] **State reset between runs** — Per-run counters separate from lifetime totals.
- [x] **All quality gates skipped detection** — Metadata-only changes not marked complete.
- [x] **Empty session detection** — No file changes = not complete.

Debug log enrichment (previously P1) also completed:

- [x] **Tool names in debug log** — `STDOUT-EVENT: type=assistant tool=Read file=geo.service.ts`
- [x] **MCP tool details** — `MCP archon.rag_search(query="nestjs guards", source_id="6a8b...")`
- [x] **Bash command extraction** — `Bash $ pnpm test --filter @cp/api → exit 0`
- [x] **Log file location** — Cross-platform: XDG Linux, ~/Library macOS, %APPDATA% Windows. Path shown at start AND end of run.
- [x] **Log rotation** — New file per run, max 10 files, max 50MB per file. Configurable via `debug.max_log_files` and `debug.max_log_size_mb`.

---

## P1 — TUI Dashboard (UX spec: ux-design-ralph-engine-tui-dashboard.md)

### Phase 1 — Wire to stream events

- Connect DashboardState to EngineEvent stream
- Activity feed with tool icons
- Basic status bar (gates, progress)
- Ctrl+C safe interrupt (2-step)

### Phase 2 — Diff panel

- Parse Edit/Write events for file content
- Line-level diff with red/green colors
- File cycling with `f` key

### Phase 3 — Rich details

- MCP server.tool(params) display
- Bash $ command → result
- Skill invocation display

### Phase 4 — Polish

- Word-level diff highlighting
- Responsive layouts (narrow, minimal)
- Keyboard shortcuts overlay
- Activity entry expansion

### TUI Design Details

- Colors: green (+additions), red (-deletions), cyan (tools), yellow (warnings)
- Diff panel: 60% viewport, syntax-aware, context lines
- Activity feed: last N entries, auto-scroll, icons per tool type
- Safe interrupt: S (save), W (wait), F (force), C (cancel), auto-save 10s
- Ctrl+C twice = force stop (escape hatch)
- Show MCP as: `🌐 MCP server.tool("query")`
- Show Bash as: `🔧 $ command → result`

---

## P1 — CLI Tests

Cobra command integration tests.

- [ ] **Test each command** — run, prepare, status, config set/list, init, version
- [ ] **Test flag parsing** — Verify flags override config correctly
- [ ] **Test --save** — Verify config file is written with correct values
- [ ] **Test error cases** — Missing binary, bad config, no stories

---

## P1 — Remote Execution Abstraction

Replace SSH-specific code with generic executor interface.

- [ ] **Executor interface** — `Execute(ctx, command) (output, error)`
- [ ] **LocalExecutor** — Run commands locally (default)
- [ ] **SSHExecutor** — Run via SSH (current ssh package)
- [ ] **DockerExecutor** — Run via `docker exec`
- [ ] **KubeExecutor** — Run via `kubectl exec`
- [ ] **Config** — `remote.type: "local" | "ssh" | "docker" | "kubectl"`

---

## P2 — Boilerplate Presets

### Framework presets for `ralph-engine init`

- `bmad-v6-full` — all commands + instructions + research (like CP)
- `bmad-v6-lite` — dev + code_review only, no research
- `tdd-strict` — instructions only, no framework dependency
- `basic` — minimal, no commands
- `cursor` — Cursor IDE integration
- `aider` — Aider CLI integration
- `codex` — OpenAI Codex integration

### Each preset generates:

- config.yaml with appropriate workflow.commands + instructions
- hooks.yaml with language-detected quality gates
- prompt.md with project-specific context template

---

## P2 — Documentation

### Update these docs after changes stabilize:

- README.md — add run-in-dev.sh, debug log, safety guardrails
- cli-reference.md — --debug flag, log location
- config-reference.md — workflow.commands, workflow.instructions, devcontainer section
- architecture.md — stream-json flow, event parsing
- CLAUDE.md — backward compat note for new fields
