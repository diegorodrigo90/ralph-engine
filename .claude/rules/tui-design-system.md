# TUI Design System Rules

Rules follow EARS syntax (SHALL keyword).

## Theme (ratatui-themekit)

- ALL colors in re-tui SHALL come from Theme trait slots — zero hardcoded `Color::Rgb(...)` or `Color::Red`
- Plugins SHALL NEVER reference colors — they use `TuiBlock` (data), core renders with theme
- Theme switching SHALL work at runtime — zero code changes needed
- `ThemeExt` builders SHALL be preferred over manual `Span::styled(...)` in new code
- `NO_COLOR` env SHALL always be respected

## TuiBlock Contract (re-plugin)

- `TuiBlock` is a STRUCT with `RenderHint` + `Severity` — NOT a semantic enum
- Plugins compose blocks via builder methods: `TuiBlock::indicator()`, `TuiBlock::metric()`, etc.
- Core renders blocks by `hint` + `severity` — NEVER interprets content semantically
- New hints can be added without changing plugin code (backward compatible)

## Plugin Contributions (Model B)

- Sidebar panels: `tui_contributions()` returns `Vec<TuiPanel>` with `zone_hint`
- Zone hints: `"sidebar"` (default), `"main"` (feed area), `"bottom"` (status area)
- Keybindings: `tui_keybindings()` — core dispatches, plugin handles
- Text input: `handle_tui_text_input()` — plugins process user input
- Indicators: plugins create `StatusIndicator` and update via `phase_marker` on blocks

## Phase Markers

- `FeedBlock.phase_marker` carries orchestration phase updates from workflow plugin
- Format: `"start:id"`, `"pass:id"`, `"fail:id"` — core parses generically
- Core SHALL NEVER hardcode phase names — only the workflow plugin knows them

## What Core Knows vs What Plugins Know

| Concept | Core knows | Plugin knows |
|---------|-----------|-------------|
| Colors | Theme slots (accent, success, error) | Nothing — uses TuiBlock severity |
| Phases | Generic indicators (pending/running/passed) | Phase names (resolve, test, build) |
| Layout | Zone rendering (sidebar, main, bottom) | Zone hint ("sidebar", "main") |
| Content | How to render RenderHint+Severity | What data to show |
| Timing | Drip cadence per block kind | Nothing — core handles |
| Cost | How to display cost_label string | Pricing calculation |
| Thinking | How to show thinking message | Message pool + rotation |

## CR SHALL Flag

- `Color::Rgb(...)` or `Color::Red` in re-tui or plugins → VIOLATION
- Plugin code referencing theme methods → VIOLATION (use TuiBlock)
- Core code inspecting TuiBlock content semantically → VIOLATION
- Hardcoded phase names in core → VIOLATION
- `Span::styled(...)` with manual Style in new code → prefer ThemeExt builders
