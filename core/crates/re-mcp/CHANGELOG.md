# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0-alpha.1](https://github.com/diegorodrigo90/ralph-engine/releases/tag/re-mcp-v0.2.0-alpha.1) - 2026-04-10

### Added

- community plugin discovery, audit skill, crates.io prep
- *(mcp)* add typed launch plans
- *(mcp)* localize official server catalogs
- *(i18n)* naturalize pt-br runtime labels
- *(cli)* add localized plugin surfaces
- *(mcp)* add typed launch contracts
- *(mcp)* add typed process and policy contracts
- *(core)* add mcp contribution registry

### Other

- *(i18n)* migrate re-mcp to TOML-based locale generation
- *(mcp)* consolidate 5 MCP enums into define_mcp_enum! macro
- *(mcp)* neutralize shared server fixtures
- *(mcp)* decouple shared fixtures from official servers
- *(i18n)* add typed shared locale contract
- *(i18n)* centralize plugin and mcp catalogs
- *(i18n)* share locale resolution across crates
- *(i18n)* normalize locale resolvers
- *(mcp)* modularize locale catalogs
