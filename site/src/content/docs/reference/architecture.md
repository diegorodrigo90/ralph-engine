---
title: "Architecture"
description: "Internal architecture and design decisions"
---


## Positioning

Ralph Engine is an open-source plugin-first runtime for agentic coding workflows.

## Repository layout

- `core/` — Rust runtime crates
- `plugins/official/` — Rust-first official plugins
- `docs/` — VitePress docs
- `site/` — public web surfaces, shared UI, and plugin metadata
- `packaging/` — npm and Homebrew packaging surfaces
- `tools/create-ralph-engine/` — plugin scaffolding for `npx create-ralph-engine-plugin`
- `scripts/` — bootstrap, validation, and release automation

## Rust workspace

- `re-core` — shared runtime foundations, topology, and state contracts
- `re-config` — shared runtime configuration contracts, scopes, defaults, and resolution rules
- `re-mcp` — shared MCP contribution, launch-policy, process-model, command-boundary, and policy contracts
- `re-plugin` — shared plugin metadata, lifecycle, runtime-hook, loading-boundary, and capability contracts
- `re-official` — typed built-in catalog wiring official plugins and MCP servers into one reusable runtime snapshot
- `re-cli` — CLI crate producing `ralph-engine`
- official plugin crates live under `plugins/official/*`

## Architectural rules

- the core remains plugin-first and workflow-agnostic
- external MCP remains first-class
- official plugins are Rust
- third-party plugins remain language-agnostic
- prompt, context, MCP governance, security, and diagnostics remain core concerns
- CLI command families evolve through isolated modules and registries rather than one growing central dispatcher
- plugin capabilities and MCP contributions evolve through typed descriptors so new capabilities can be added without string-coupled runtime logic
- plugin lifecycle evolves through shared typed stages so discovery, configuration, validation, and loading stay explicit and extensible
- plugin runtime hooks evolve through shared typed descriptors so prepare, doctor, prompt, agent, MCP, and policy contributions stay modular without ad hoc dispatch
- configuration resolution evolves through typed layered scopes so defaults and future overrides stay explicit instead of being inferred inside commands
- runtime topology, health, issue reporting, doctor reporting, runtime action planning, and runtime-hook registration evolve through typed registrations and shared contracts so plugin activation, capability registration, hook registration, and MCP enablement stay explicit instead of being reconstructed ad hoc per command
- disabled capabilities and disabled runtime hooks stay visible in runtime health plus remediation output; they do not become invisible metadata just because the topology resolved
- plugin load boundaries stay typed so in-process, subprocess, and remote integration can evolve without ad hoc runtime branching
