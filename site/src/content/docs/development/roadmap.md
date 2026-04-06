---
title: "Roadmap"
description: "Ralph Engine development roadmap — planned and completed milestones"
---

## Completed

### Core Runtime (v0.2.0-alpha)

- Plugin system with 17 capabilities and auto-discovery
- 13 official plugins (agent, workflow, policy, TUI, context, routing)
- `run` command — autonomous loop with work item resolution
- TUI dashboard with ratatui — live agent activity, sidebar panels, feed blocks
- `zone_hint` — plugins place panels in sidebar or main feed area
- `FeedContribution` — plugins inject status blocks into the live feed
- `TuiBlock` widget system — bars, metrics, indicators, pairs, lists
- Theme system via ratatui-themekit — 11 themes, semantic color slots
- Agent launch with `-p` mode (Claude, Codex, ClaudeBox)
- MCP server auto-discovery and registration
- i18n — English and Brazilian Portuguese (TOML-based)
- Plugin scaffolder — `npx create-ralph-engine-plugin`
- Findings plugin — feedback loop injecting past findings into prompts
- Agent router — task classification and agent selection
- Guided mode input bar — human-in-the-loop feedback
- Pause/resume agent sessions
- Autonomous mode with safety acceptance gate
- 994 tests, 72 Golden Rules, zero clippy warnings

### Documentation Site (v1.0)

- Astro + Starlight with EN/PT-BR
- Plugin guide with TUI contributions
- Configuration reference
- CLI reference
- Architecture overview
- Privacy policy and terms

## In Progress

### Guided Mode Polish

- Chat toggle (show/hide input bar)
- Pause behavior improvements
- Demo recording

### Demo Plugin

- Community plugin created via scaffolder
- Demonstrates TuiBlock, feed_contributions, zone_hint

## Planned

### Packaging & Distribution

- npm package (`npx ralph-engine`)
- Homebrew formula
- crates.io binary crate
- Cloudflare Pages deployment for docs site

### Community Plugin Ecosystem

- Community plugin loader (filesystem scan)
- Plugin validation before catalog listing
- Trust tiers (official, verified, community)
- Plugin registry

### Advanced Features

- Multi-agent context switching
- Context sharing between agents
- Memory plugins for cross-session persistence
- Preset plugins (config + hooks bundles)
- Multi-project mode
- Team mode

### Quality & Polish

- GIF/VHS recording of TUI in action
- Interactive TUI example in ratatui-themekit
- Benchmarks for theme resolution
- `no_std` evaluation for themekit
