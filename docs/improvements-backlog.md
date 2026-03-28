# Ralph Engine — Improvements Backlog

Collected from session 6 host testing. Prioritized by impact.

## P0 — Must fix before next host test

### 1. Handoff save WITHOUT AI

When usage limit hits, save progress from engine memory (no AI call):

- Files changed (git diff --stat)
- Tools used count
- Last agent message (from stream event buffer)
- Quality gates status
- Elapsed time, turns count
- Write to `.ralph-engine/handoff-{story_id}.json`

### 2. First-turn problem

Agent responds with text "I'll implement..." and exits (1 turn, no tools).
Fix: stronger prompt instruction to START tools immediately on turn 1.
Maybe: "Your FIRST action must be a tool call (Read, Glob, or Skill). Do NOT start with a text response."

### 3. state.json ↔ sprint-status.yaml sync

MarkComplete updates YAML but state.json shows different stories as "completed".
Fix: ensure both are written atomically, or state.json reads from YAML.

### 4. SSH reconnect cooldown

Time-based retry (not count). If last failure was >5 min ago, try again.
Config: `ssh.reconnect_cooldown_minutes: 5`

---

## P1 — Debug Log Enrichment

### 5. Tool names in debug log

Currently: `STDOUT-EVENT: type=assistant tool= subtype=`
Should be: `STDOUT-EVENT: type=assistant tool=Read file=geo.service.ts`
Parse `message.content[]` blocks for tool_use name + key input params.

### 6. MCP tool details

Format: `MCP archon.rag_search(query="nestjs guards", source_id="6a8b...")`
Parse from tool_use name prefix `mcp__server__tool`.

### 7. Bash command extraction

Format: `Bash $ pnpm test --filter @cp/api → exit 0`
Parse from tool_use input.command.

### 8. Log file location

- Binary on host: `~/.ralph-engine/logs/` (XDG compliant)
- Project: `.ralph-engine/logs/` (current, gitignored)
- Show path at start AND end of run
- Cross-platform: Windows `%APPDATA%`, macOS `~/Library/Logs/`, Linux `~/.local/state/`

### 9. Log rotation

- New file per run (timestamped) — DONE
- Max 10 log files, delete oldest
- Max 50MB per file, truncate
- Config: `debug.max_log_files: 10`, `debug.max_log_size_mb: 50`

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
