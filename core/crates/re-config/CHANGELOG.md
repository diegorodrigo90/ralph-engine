# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0-alpha.1](https://github.com/diegorodrigo90/ralph-engine/releases/tag/re-config-v0.2.0-alpha.1) - 2026-04-10

### Added

- unified autocomplete, preset kind, multi-project config (RE-8/9/10)
- agent routing plugin, routing types, run modes (RE-7)
- context management, agent routing types, Model B rule (RE-6)
- community plugin discovery, audit skill, crates.io prep
- *(plugin)* add WORKFLOW capability, run types, and trait methods
- *(config)* load project config from .ralph-engine/config.yaml
- *(i18n)* detect OS locale from LANG/LC_ALL/LC_MESSAGES
- *(runtime)* render patched config output
- *(runtime)* add typed runtime config patches
- *(i18n)* expose typed locale catalog
- *(cli)* expose typed locale config
- *(config)* add typed runtime budgets
- *(config)* expose typed configuration layers
- *(config)* add typed layered resolution
- *(config)* add typed plugin activation defaults
- *(core)* add typed config foundation

### Fixed

- basic template config incompatible with parser (config audit)
- *(plugin)* address CR findings — tests, bounds, probe bug

### Other

- *(config)* neutralize shared patch fixtures
- *(config)* neutralize shared patch fixtures
- *(contract)* decouple shared fixtures from official plugins
- *(i18n)* add typed shared locale contract
- *(i18n)* share locale resolution across crates
