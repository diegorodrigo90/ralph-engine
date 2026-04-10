# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0-alpha.1](https://github.com/diegorodrigo90/ralph-engine/releases/tag/re-tui-v0.2.0-alpha.1) - 2026-04-10

### Added

- *(re)* show project name in header and accent focus indicator on feed
- *(tui)* cursor movement, Delete key, paste at cursor
- *(tui)* collapse large paste into compact indicator
- *(tui)* input scrollbar, paste limit, Tab focus toggle
- redesign tui with proper naming, commands, and polished sidebar
- complete TUI redesign — idle/active modes, progressive disclosure
- condensed sidebar rendering per group type
- improve tab rendering — counts, icons, grouped config
- integrate ratatui-zonekit and grouped sidebar design
- tui redesign with tabs, focus, ansi output, cli colors, canvas
- zone_hint=main tests, BMAD feed contributions, 988 tests
- *(tui)* zone_hint=main — plugins inject panels and feed blocks into central area
- *(tui)* orchestration pipeline phases in demo — indicators advance as blocks drip
- *(tui)* premium panel rendering — gradient bars, mini charts, arrow pairs, numbered lists, dotted separators
- *(tui)* TuiWidget design system — typed blocks in re-plugin, PanelItem renderer in re-tui, all plugins migrated
- *(tui)* smart sidebar styling — key:value colors, status indicators, richer plugin panels
- *(tui)* thinking messages from agent plugin, rotating status bar, Claude messages
- *(tui)* usage report trait, cost/extra-usage in header, claude plugin stub
- *(tui)* realistic demo timing (4-5s holds), token/tool tracking in header
- *(tui)* realistic loading cadence — thinking/command hold with spinner before next block
- *(tui)* variable drip cadence, auto-progress bar, active block finalization
- *(tui)* smooth scroll, diff colors, thinking indicator, command output colors
- *(tui)* feed redesign with borders, demo command with i18n, bracketed paste
- *(tui)* OSC 52 clipboard fallback, mouse click focus, base64 dep (RE-20)
- *(tui)* session management, toast notifications, agent switching (RE-16)
- complete tui i18n — all strings via TuiLabels, state labels, metrics, logo tagline
- full i18n for tui via TuiLabels, pt-br modals and idle dashboard
- modal popups for quit and help, logo health indicator, ux polish
- auto-init for tui, improved help with keys and commands
- launch tui dashboard as default, enhanced idle screen, unicode keys
- *(tui)* agent switcher, sidebar toggle, idle dashboard (RE-14-11/14/15)
- *(tui)* orchestration header, spinners, diff hunk headers (RE-14-8/9/10)
- *(tui)* feed readability with separators, padding, truncation (RE-14-7)
- *(tui)* block copy to clipboard with y key and feedback (RE-14-6)
- *(tui)* block focus navigation with j/k, Enter toggle, Esc clear (RE-14-5)
- *(tui)* scrollable feed with tui-scrollview and follow mode (RE-14-4)
- *(tui)* semantic theme system with 6 built-in themes (RE-14-3)
- *(tui)* status indicators + Model B enforcement (RE-14-3)
- *(tui)* block-based activity feed for orchestration dashboard
- unified autocomplete, preset kind, multi-project config (RE-8/9/10)
- guided plugin, TUI input bar, autocomplete, Model B compliance (RE-4 + RE-5)
- feedback input, plugin-owned pause/resume/inject, Golden Rule 63
- security audit skill, Golden Rules 59-62, non-TTY stdin guard, tui_contribution hook
- *(tui)* render logo in activity viewport with scroll
- *(tui)* hand-crafted Unicode logo with brand colors
- *(tui)* logo rendering, quit confirmation, real agent pause
- *(tui)* add tui demo command with brand icon in header
- *(tui)* guided mode state machine, process control, plugin panels
- *(tui)* keybinding system with core protection and plugin extension
- *(tui)* render plugin panels in sidebar via auto-discovery
- *(tui)* normalized agent event stream and process_event
- *(tui)* responsive zone-based layout with 3 tiers
- *(tui)* add re-tui crate with ratatui shell and tracing logging

### Fixed

- *(re)* address CR findings — idle hints in run, test helpers, hex entities
- *(tui)* backspace/delete clears entire collapsed paste block
- *(tui)* i18n for paste chars suffix, complete pt-br coverage
- *(tui)* normalize line endings in paste, add paste toast
- *(tui)* move paste collapse to render layer for reliability
- *(tui)* enable mouse capture for click-to-focus and scroll
- *(tui)* input focus, undo, click-to-focus, duplicate /theme
- idle dashboard shows /list instead of /run --list
- esc with buffer clears text, esc without buffer exits focus
- esc exits input focus, enables q/shortcuts after unfocus
- input captures all keys, correct help bar text
- fluid layout, deduplicate panels, filter sidebar noise
- fluid layout — sidebar adapts to plugin content
- eliminate all remaining raw Span/Line in production TUI code
- *(tui)* CR fixes — VecDeque drip, OSC52 via crossterm, i18n extra_usage, scroll padding, Model B cost
- *(tui)* clean startup screen, fix follow mode scroll, remove doctor dump
- complete pt-br tui — control panel, help modal, metrics, no-agent message
- cr findings model b fixtures cursor separator

### Other

- *(re)* enforce Model B compliance for TUI agent detection and idle hints
- *(tui)* use InputStyles from themekit for input rendering
- add coverage for grouped sidebar, tab rendering, icons
- add agent integration model and trademark disclaimers
- upgrade ratatui-themekit to 0.5, 100% themekit in re-tui
- upgrade ratatui-themekit to 0.4 — widget builders, style bundles
- upgrade ratatui-themekit to 0.3 — clean architecture
- upgrade ratatui-themekit to 0.2 — 11 themes, ThemeData, style helpers
- switch themekit to crates.io dependency (v0.1.0)
- switch themekit from path to git dependency
- *(tui)* 100% themekit — zero Span::styled, zero Style::default().fg() in production code
- *(tui)* complete ThemeExt migration — only Block widget styles remain as Style::default()
- *(tui)* convert render modules to ThemeExt builders — 54 Span::styled removed
- *(tui)* split terminal.rs into 9 modules, fix parallel hook deadlock
- *(tui)* use ThemeExt builders in header render — proof of concept
- *(tui)* replace internal theme.rs with ratatui-themekit dependency (709→71 lines)
- *(tui)* structural design system — TuiBlock struct with RenderHint/Severity, builder API, all plugins migrated
- *(tui)* visual redesign — badge-style header, colored sidebar panels, premium control panel
- model b i18n — all tui strings via TuiLabels, zero inline locale checks
- polish pass — exit codes, PT-BR accents, NO_COLOR, rules, suggestions
- update lockfile, logo asset, and generated files
- *(i18n)* improve autonomous mode warning with AI safety notes
- *(tui)* add rendering snapshot tests with TestBackend
