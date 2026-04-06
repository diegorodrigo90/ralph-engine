---
title: "Backlog"
description: "Feature backlog and known improvements for Ralph Engine"
---

## Plugin System

- Community plugin loader — filesystem scan + manifest validation
- Plugin trust tiers — official, verified, community
- Plugin registry — searchable catalog with versioning
- Plugin validation CI — automated checks before catalog listing
- Scaffolder: add `workflow` kind end-to-end example

## TUI Dashboard

- Guided mode: chat toggle, pause UX polish
- GIF/VHS recording for README and docs
- Theme switcher in TUI (runtime theme change)
- Configurable layout (panel sizes, positions)

## Agent Integration

- Multi-agent context switching
- Context sharing protocol between agents
- Memory plugins for cross-session persistence
- Agent command bidirectional protocol (Claude stdin NDJSON)
- Gemini agent plugin

## CLI

- `ralph-engine exec` subcommand (headless shorthand)
- `re` shell alias via bootstrap script
- `--headless` flag for CI/CD pipelines

## Packaging

- npm: `npx ralph-engine`
- Homebrew: `brew install ralph-engine`
- crates.io: `cargo install ralph-engine`
- Cloudflare Pages: docs site deployment
- Signed release artifacts

## Documentation

- Plugin development tutorial (step-by-step)
- MCP server integration tutorial
- Configuration deep dive
- Migration guide (version upgrades)
- Video walkthrough

## ratatui-themekit

- Interactive TUI example (not just println)
- Benchmarks for theme resolution
- `no_std` evaluation
- More themes: Kanagawa, Everforest, Ayu
- `impl Theme for &T` blanket impl
