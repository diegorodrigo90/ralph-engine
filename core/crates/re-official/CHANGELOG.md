# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0-alpha.1](https://github.com/diegorodrigo90/ralph-engine/releases/tag/re-official-v0.2.0-alpha.1) - 2026-04-10

### Added

- unified autocomplete, preset kind, multi-project config (RE-8/9/10)
- agent routing plugin, routing types, run modes (RE-7)
- context management, agent routing types, Model B rule (RE-6)
- guided plugin, TUI input bar, autocomplete, Model B compliance (RE-4 + RE-5)
- community plugin discovery, audit skill, crates.io prep
- *(plugin)* add tui_contributions() trait and tui_widgets capability
- *(findings)* add official findings plugin for feedback loop
- *(bmad,claude)* implement workflow resolution and agent launch
- *(plugin)* add WORKFLOW capability, run types, and trait methods
- *(config)* load project config from .ralph-engine/config.yaml
- add official.hello-world example plugin, make test counts dynamic
- complete runtime coverage and functional CLI
- *(official)* add plugin runtime registry with auto-discovery
- *(official)* auto-generate plugin registration from manifest.yaml
- *(plugin)* add descriptor validation with isolation-ready error reporting
- *(cli)* execute and materialize runtime surfaces
- *(runtime)* expose provider registration plans

### Other

- *(deps)* bump toml from 0.8.23 to 1.1.2+spec-1.1.0
- *(lint)* enforce pedantic clippy rules for readability
- *(plugin)* add #[non_exhaustive] to all public enums in re-plugin
- *(official)* replace fixed arrays with Vec for plugin catalog
- *(runtime)* preserve agent identifiers in registrations
- *(core)* move official runtime catalog into crate
