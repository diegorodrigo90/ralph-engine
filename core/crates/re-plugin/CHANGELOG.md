# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0-alpha.1](https://github.com/diegorodrigo90/ralph-engine/releases/tag/re-plugin-v0.2.0-alpha.1) - 2026-04-10

### Added

- complete TUI redesign — idle/active modes, progressive disclosure
- zone_hint=main tests, BMAD feed contributions, 988 tests
- *(tui)* zone_hint=main — plugins inject panels and feed blocks into central area
- *(tui)* TuiWidget design system — typed blocks in re-plugin, PanelItem renderer in re-tui, all plugins migrated
- *(tui)* thinking messages from agent plugin, rotating status bar, Claude messages
- *(tui)* usage report trait, cost/extra-usage in header, claude plugin stub
- unified autocomplete, preset kind, multi-project config (RE-8/9/10)
- agent routing plugin, routing types, run modes (RE-7)
- context management, agent routing types, Model B rule (RE-6)
- guided plugin, TUI input bar, autocomplete, Model B compliance (RE-4 + RE-5)
- feedback input, plugin-owned pause/resume/inject, Golden Rule 63
- security audit skill, Golden Rules 59-62, non-TTY stdin guard, tui_contribution hook
- *(run)* integrate TUI dashboard with real agent process
- community plugin discovery, audit skill, crates.io prep
- *(cli)* route plugin-contributed subcommands via auto-discovery
- *(plugin)* add tui_contributions() trait and tui_widgets capability
- *(plugin)* add extensibility hooks, init command
- *(findings)* add official findings plugin for feedback loop
- *(run)* programmatic agent launch with auto-discovery and prompt optimization
- *(scaffolder)* add workflow kind and capability to plugin contracts
- *(plugin)* add WORKFLOW capability, run types, and trait methods
- *(plugin)* add PluginRuntime trait with first real implementation in bmad
- *(plugin)* add descriptor validation with isolation-ready error reporting
- *(plugin)* add plugin_api_version field with runtime compatibility check
- *(cli)* execute and materialize runtime surfaces
- *(plugin)* add typed check and provider contributions
- *(prompt)* expose typed prompt assets
- *(template)* expose typed template assets
- *(plugin)* add typed agent and policy contributions
- *(plugin)* add typed template and prompt contributions
- *(i18n)* naturalize pt-br runtime labels
- *(plugin)* add localized summaries
- *(cli)* add localized plugin surfaces
- *(plugin)* add typed plugin kinds
- *(plugin)* add typed runtime hook contracts
- *(plugin)* add typed load boundaries
- *(plugin)* add typed lifecycle contract
- *(core)* introduce shared plugin contract

### Fixed

- *(plugins)* extract shared agent helpers, strengthen tests, fix 4 bugs
- *(plugin)* address CR findings — tests, bounds, probe bug

### Other

- *(re)* enforce Model B compliance for TUI agent detection and idle hints
- *(tui)* structural design system — TuiBlock struct with RenderHint/Severity, builder API, all plugins migrated
- *(plugin)* extract probe_binary_on_path as shared utility
- *(plugin)* add community plugin end-to-end test
- *(i18n)* migrate re-plugin to TOML-based locale generation
- *(plugin)* consolidate 8 enums into define_plugin_enum! macro
- *(plugin)* add #[non_exhaustive] to all public enums in re-plugin
- *(contract)* decouple shared fixtures from official plugins
- *(plugin)* share reviewed capability parser
- *(plugin)* share runtime hook parser
- *(i18n)* use typed locales in public surfaces
- *(i18n)* add typed shared locale contract
- *(i18n)* centralize plugin and mcp catalogs
- *(plugin)* share runtime surface ownership
- *(i18n)* canonicalize plugin locale lookup
- *(i18n)* share locale resolution across crates
- *(i18n)* normalize locale resolvers
- *(plugin)* modularize locale catalogs
- *(contracts)* verify plugin surface alignment
- *(core)* modularize cli and typed capabilities
