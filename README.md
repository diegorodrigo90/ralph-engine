# Ralph Engine

[![CI](https://github.com/diegorodrigo90/ralph-engine/actions/workflows/ci.yml/badge.svg)](https://github.com/diegorodrigo90/ralph-engine/actions/workflows/ci.yml)
[![Pages](https://github.com/diegorodrigo90/ralph-engine/actions/workflows/pages.yml/badge.svg)](https://github.com/diegorodrigo90/ralph-engine/actions/workflows/pages.yml)
[![Release Automation](https://github.com/diegorodrigo90/ralph-engine/actions/workflows/release-plz.yml/badge.svg)](https://github.com/diegorodrigo90/ralph-engine/actions/workflows/release-plz.yml)
[![Sonar Quality Gate](https://sonarcloud.io/api/project_badges/measure?project=ralph-engine_ralph-engine&metric=alert_status)](https://sonarcloud.io/project/overview?id=ralph-engine_ralph-engine)
[![Coverage](https://sonarcloud.io/api/project_badges/measure?project=ralph-engine_ralph-engine&metric=coverage)](https://sonarcloud.io/project/overview?id=ralph-engine_ralph-engine)
[![Latest Release](https://img.shields.io/github/v/release/diegorodrigo90/ralph-engine?display_name=tag)](https://github.com/diegorodrigo90/ralph-engine/releases)
[![Rust](https://img.shields.io/badge/rust-1.91.1-93450a.svg)](rust-toolchain.toml)
[![Node](https://img.shields.io/badge/node-20.19%2B-339933.svg)](package.json)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![npm Channel](https://img.shields.io/badge/npm-gated-lightgrey.svg)](packaging/npm/README.md)
[![Homebrew Channel](https://img.shields.io/badge/homebrew-gated-lightgrey.svg)](packaging/homebrew/README.md)

Ralph Engine is an open-source plugin-first runtime for agentic coding workflows.

This repository has been rebooted onto a Rust-first foundation. The core runtime and official plugins now evolve in Rust, while docs, site, and developer scaffolding keep the stacks that fit them best.

Public product surfaces are being prepared for bilingual operation in English and pt-BR, including the CLI, docs, and site.
Those public surfaces also follow a shared UX contract: consistent navigation, stable public paths, and A-grade accessibility, performance, and SEO targets.

- Website: https://ralphengine.com
- Docs: https://ralphengine.com/docs/
- Plugins: https://ralphengine.com/plugins/
- Releases: https://github.com/diegorodrigo90/ralph-engine/releases

## Why Ralph Engine

- Rust-first runtime and CLI foundation
- plugin-first architecture with typed capability contracts
- MCP-aware official integrations
- hardened CI and release gates with build-once promote-later publishing
- bilingual public surfaces for `en` and `pt-BR`

## Install

Current public install channels are being wired through the hardened release pipeline.

- GitHub Releases are the canonical reviewed artifact source
- npm and Homebrew are prepared but still intentionally gated during the reboot
- `npx create-ralph-engine-plugin` is the supported scaffolder entrypoint for plugin authors

## Repository Layout

- `core/` — Rust crates for the runtime and CLI
- `plugins/official/` — Rust-first official plugins
- `docs/` — VitePress documentation
- `site/` — public web surfaces, shared UI, and plugin metadata
- `packaging/` — npm and Homebrew packaging surfaces
- `tools/create-ralph-engine/` — plugin scaffolder (`npx create-ralph-engine-plugin`)
- `scripts/` — shared bootstrap, validation, and release scripts

## Quickstart

```bash
./scripts/bootstrap-dev.sh
./scripts/validate.sh --mode local
cargo run -p re-cli -- --locale pt-br
./scripts/validate-ci-local.sh
npm run contracts:verify
cargo test --workspace --all-targets --all-features
```

## Open Source Contract

- typed runtime and plugin contracts come before string-coupled branching
- changes are gated by `fmt`, `clippy`, tests, coverage, `rustdoc`, `cargo deny`, `cargo audit`, docs build, and public-surface assembly
- release artifacts are only promoted from a green `main` SHA after `Quality`, `Security`, and `SonarCloud`
- docs and public site publish from published releases so the public surface follows released application state
- locale expansion is additive and English remains the fallback when a locale catalog is incomplete

## Engineering Rules

- Public Rust APIs are documented with `rustdoc`
- Rust tests prefer Arrange, Act, Assert
- Plugin and MCP extensibility stays typed: plugin kinds, capabilities, lifecycle stages, runtime hooks, configuration scopes, launch policy, command boundaries, and future contributions are expected to evolve through shared contracts rather than string-coupled runtime branches
- Shared contract tests stay neutral by default: shared crates use synthetic fixtures for contract coverage, while official plugin crates own the nearest manifest and contribution checks for their own behavior
- Plugin trust stays typed and explicit: official/runtime-owned plugins and community manifests are expected to flow through shared trust-level contracts instead of ad hoc YAML strings or CLI-only labels
- Configuration layering stays typed and explicit: canonical defaults, future workspace settings, project settings, and user overrides are expected to evolve through shared layer contracts and inspectable CLI output rather than hidden precedence rules
- Prompt and context budgets stay typed and explicit: shared runtime budget contracts and CLI inspection are expected to carry token ceilings instead of scattering implicit defaults through providers
- Runtime registration stays typed: the resolved runtime topology, capability registry, template registry, prompt registry, agent registry, check registry, provider registry, policy registry, runtime-hook registry, runtime health, runtime issues, runtime action plans, and MCP contributions are expected to evolve through shared contracts rather than implicit command-local catalog traversal, and disabled capabilities, templates, prompt providers, agent runtimes, checks, providers, policies, or hooks remain visible in health plus remediation output
- Official runtime catalog assembly stays plugin-owned: the CLI derives templates, prompts, agents, checks, providers, policies, and MCP servers from plugin-provided bundles instead of matching on plugin identifiers in shared catalog code
- The built-in runtime catalog lives in `re-official`, so official plugin wiring stays reusable and outside the CLI crate
- Template and prompt contributions now carry typed embedded asset contracts, so the CLI can expose the real files a plugin delivers instead of stopping at metadata-only listings
- `ralph-engine templates asset <template-id> <path>` and `ralph-engine prompts asset <prompt-id> <path>` expose embedded plugin assets directly from the typed official catalog
- Developer scaffolding stays tooling-owned: `tools/create-ralph-engine/` is the home of plugin scaffold generation through `npx create-ralph-engine-plugin`, while runtime surfaces stay focused on typed runtime contracts instead of turning scaffolding into a generic runtime responsibility
- The plugin scaffolder only accepts kinds and capabilities that the typed runtime already defines; future surfaces stay rejected until the core contracts exist for them
- Official and scaffolded plugin identifiers use the same dotted namespace contract, such as `official.basic` and `acme.jira-suite`
- Third-party plugin manifests stay contract-driven: `tools/create-ralph-engine/` owns a versioned `manifest.yaml` schema plus validation helpers so language-agnostic plugin metadata cannot drift away from the typed runtime contracts
- Cross-language plugin contracts are verified explicitly so Rust capability contracts, manifest schema enums, and `create-ralph-engine-plugin` supported surfaces cannot drift silently
- CLI output SHALL stay locale-aware for `en` and `pt-br` through modular locale catalogs, and plugin-facing metadata SHALL support locale-specific names and summaries with English fallback when a requested locale is missing
- Official plugins and scaffolded plugins use per-locale catalog modules, so adding a new locale stays additive across runtime, plugin metadata, and generated manifests
- `re-config` owns the shared typed locale contract and supported-locale catalog, so runtime crates do not need to invent locale parsing rules independently
- Runtime diagnostics stay typed: doctor-style reporting is expected to compose status, unresolved issues, and remediation actions from one shared runtime snapshot instead of re-deriving them ad hoc in separate commands
- The repository enforces `fmt`, `clippy`, tests, coverage, `rustdoc`, `cargo deny`, `cargo audit`, docs build, and public-surface assembly from the same validation contract
- SonarCloud is configured as the final coverage gate for analyzed code, and the release path is blocked unless that gate stays at `100%`
- `./scripts/validate-ci-local.sh` provides a supported local smoke run for the GitHub Actions CI workflow when `act` is installed

## Release Model

- SemVer 2.0.0
- Conventional Commits
- commitlint + lefthook
- release-plz opens release PRs from `main`
- merging the release PR prepares the versioned release state
- the canonical `CI` workflow on `main` builds cross-platform release candidates in parallel and only publishes reusable approved release artifacts for the SHA after `Quality`, `Security`, and `SonarCloud` have passed
- `dist-workspace.toml` and generated release assets are explicitly validated before release candidates are approved or promoted
- the manual hardened release workflow verifies green CI for the current `main` SHA and promotes those same artifacts instead of rebuilding them
- the public Pages deployment publishes from published releases and builds from the release tag so docs and site stay aligned with published releases

## Status

This is the Rust-first reboot baseline. The release pipeline is being hardened on top of the new Rust foundation. npm now targets reviewed `cargo-dist` artifacts, the Homebrew channel has a tracked formula template, and the release flow now follows a build-once promote-later model: `CI` builds reusable artifacts for each approved `main` SHA, and the manual publish workflow promotes that exact artifact set. Automatic publication still remains disabled until GitHub Releases, npm, and Homebrew are connected end to end with provenance.
