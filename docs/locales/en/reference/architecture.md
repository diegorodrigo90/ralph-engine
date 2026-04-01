# Architecture

## Positioning

Ralph Engine is an open-source plugin-first runtime for agentic coding workflows.

## Repository layout

- `core/` — Rust runtime crates
- `plugins/official/` — Rust-first official plugins
- `docs/` — VitePress docs
- `site/` — public web surfaces, shared UI, and plugin metadata
- `packaging/` — npm and Homebrew packaging surfaces
- `tools/create-ralph-engine/` — developer scaffolding
- `scripts/` — bootstrap, validation, and release automation

## Rust workspace

- `re-core` — shared runtime foundations
- `re-cli` — CLI crate producing `ralph-engine`
- official plugin crates live under `plugins/official/*`

## Architectural rules

- the core remains plugin-first and workflow-agnostic
- external MCP remains first-class
- official plugins are Rust
- third-party plugins remain language-agnostic
- prompt, context, MCP governance, security, and diagnostics remain core concerns
