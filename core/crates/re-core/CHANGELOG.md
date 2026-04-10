# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0-alpha.1](https://github.com/diegorodrigo90/ralph-engine/releases/tag/re-core-v0.2.0-alpha.1) - 2026-04-10

### Added

- localize product tagline in CLI banner
- community plugin discovery, audit skill, crates.io prep
- *(plugin)* add plugin_api_version field with runtime compatibility check
- *(runtime)* add typed MCP server status assessment
- *(cli)* execute and materialize runtime surfaces
- *(runtime)* expose policy enforcement plans
- *(runtime)* add typed check execution plans
- *(runtime)* expose provider registration plans
- *(runtime)* add typed agent bootstrap plans
- *(runtime)* expose executable mcp launch plans
- *(runtime)* materialize patched config in core
- *(runtime)* add typed runtime config patches
- *(mcp)* localize official server catalogs
- *(runtime)* add typed runtime snapshots
- *(i18n)* naturalize pt-br runtime labels
- *(plugin)* add localized summaries
- *(cli)* add localized plugin surfaces
- *(plugin)* add typed plugin kinds
- *(runtime)* add typed prompt registrations
- *(runtime)* add typed template registrations
- *(runtime)* add typed agent registrations
- *(runtime)* add typed check registrations
- *(runtime)* add typed provider registrations
- *(runtime)* add typed policy registrations
- *(runtime)* add typed doctor reporting
- *(runtime)* include hooks in runtime health
- *(runtime)* add typed hook registrations
- *(runtime)* add typed action planning
- *(runtime)* add typed issue reporting
- *(runtime)* add typed health status
- *(runtime)* add typed capability registrations
- *(mcp)* add typed launch contracts
- *(runtime)* add typed topology registration
- *(core)* reboot ralph engine on a rust-first foundation

### Other

- *(i18n)* migrate re-cli to TOML-based locale generation
- *(i18n)* migrate re-core to TOML-based locale generation
- *(core)* consolidate 9 runtime enums into define_runtime_enum! macro
- *(runtime)* preserve agent identifiers in registrations
- *(runtime)* materialize patched config from snapshot
- *(core)* neutralize shared runtime fixtures
- *(core)* decouple runtime fixtures from official plugins
- *(core)* share runtime kind parsers
- *(i18n)* add typed shared locale contract
- *(core)* share surface capability contracts
- *(core)* share surface hook contracts
- *(core)* centralize runtime locale catalogs
- *(core)* share runtime hook mappings
- *(core)* share runtime capability mappings
- *(i18n)* share locale resolution across crates
- *(i18n)* normalize locale resolvers
- *(cli)* modularize localized command labels
- *(core)* refine pt-br runtime copy
- *(core)* modularize runtime locale catalogs
- *(cli)* modularize locale catalogs
