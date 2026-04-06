---
title: "Architecture"
description: "Internal architecture for core contributors"
---

## Positioning

Ralph Engine is an open-source plugin-first runtime for agentic coding workflows.

## Repository Layout

- `core/` — Rust runtime crates
- `plugins/official/` — Rust-first official plugins
- `site/` — Astro Starlight docs site, public web surfaces, shared UI, and plugin metadata
- `site/src/content/docs/` — documentation source (EN + PT-BR)
- `packaging/` — npm and Homebrew packaging surfaces
- `tools/create-ralph-engine/` — plugin scaffolding for `npx create-ralph-engine-plugin`
- `scripts/` — bootstrap, validation, and release automation

## Rust Workspace

- `re-core` — shared runtime foundations, topology, and state contracts
- `re-config` — shared runtime configuration contracts, scopes, defaults, and resolution rules
- `re-mcp` — shared MCP contribution, launch-policy, process-model, command-boundary, and policy contracts
- `re-plugin` — shared plugin metadata, lifecycle, runtime-hook, loading-boundary, and capability contracts
- `re-official` — typed built-in catalog wiring official plugins and MCP servers into one reusable runtime snapshot
- `re-cli` — CLI crate producing `ralph-engine`
- Official plugin crates live under `plugins/official/*`

## Agent Integration Model

Agent plugins launch the agent's own CLI binary as a subprocess (`claude -p`, `codex exec`, etc.). The integration boundary is the agent's stdout stream — Ralph Engine reads stream-json events to display progress in the TUI. It never accesses agent credentials, intercepts API calls, or acts as a proxy. The agent binary handles its own authentication, billing, and API communication.

This is equivalent to a shell script running `claude -p "prompt"` — Ralph Engine adds orchestration (work item resolution, prompt assembly, quality gates) around that same subprocess call.

## Architectural Rules

- The core remains plugin-first and workflow-agnostic.
- External MCP remains first-class.
- Official plugins are Rust.
- Third-party plugins remain language-agnostic.
- Prompt, context, MCP governance, security, and diagnostics remain core concerns.
- CLI command families evolve through isolated modules and registries rather than one growing central dispatcher.
- Plugin capabilities and MCP contributions evolve through typed descriptors so new capabilities can be added without string-coupled runtime logic.
- Plugin lifecycle evolves through shared typed stages so discovery, configuration, validation, and loading stay explicit and extensible.
- Plugin runtime hooks evolve through shared typed descriptors so prepare, doctor, prompt, agent, MCP, and policy contributions stay modular without ad hoc dispatch.
- Configuration resolution evolves through typed layered scopes so defaults and future overrides stay explicit instead of being inferred inside commands.
- Runtime topology, health, issue reporting, doctor reporting, runtime action planning, and runtime-hook registration evolve through typed registrations and shared contracts so plugin activation, capability registration, hook registration, and MCP enablement stay explicit instead of being reconstructed ad hoc per command.
- Disabled capabilities and disabled runtime hooks stay visible in runtime health plus remediation output; they do not become invisible metadata just because the topology resolved.
- Plugin load boundaries stay typed so in-process, subprocess, and remote integration can evolve without ad hoc runtime branching.

## Run Command Pipeline

The `run` command orchestrates work item execution through a five-step pipeline:

1. **Probe agent** — call `bootstrap_agent()` on the agent plugin to verify the binary is installed and ready.
2. **Resolve work item** — call `resolve_work_item()` on the workflow plugin. Returns the canonical ID, title, source path, and metadata.
3. **Build prompt** — call `build_prompt_context()` on the workflow plugin, then enrich with auto-discovered tools and plugin prompt contributions.
4. **Print launch info** — display the work item and agent to the user.
5. **Launch agent** — call `launch_agent()` on the agent plugin with the assembled `PromptContext`.

### Prompt Assembly

The prompt is assembled in layers:

- **Task context** — the workflow plugin reads the work item file (story, issue, spec) and constructs the base prompt with task description, acceptance criteria, and relevant project rules.
- **Plugin contributions** — each enabled plugin's `prompt_contributions()` is called. Contributions are appended to the prompt text and tracked as context files. The `official.findings` plugin uses this to inject past findings.
- **Constraints** — workflow-defined constraints (quality gates, coding standards) are appended last.

### Tool Auto-Discovery

Instead of requiring users to list every tool an agent needs, the `run` command collects tools from all enabled plugins:

1. Each plugin implements `required_tools()` returning tool names or patterns it needs (e.g., MCP tool patterns).
2. The core collects from all enabled plugins, deduplicates, and merges with any tools configured in `.ralph-engine/config.yaml`.
3. The merged list is passed to the agent plugin via `PromptContext.discovered_tools`.

This means installing a plugin that needs specific MCP tools automatically makes those tools available to the agent.

### Feedback Loop

The `official.findings` plugin creates a feedback loop across agent sessions:

1. After a run, findings (code review issues, quality gate failures, learnings) are written to `.ralph-engine/findings.md`.
2. On the next run, the findings plugin reads this file and injects it as a `<findings>` prompt section.
3. The agent sees past mistakes before implementing, reducing repeated errors.

The file format is project-defined — the plugin reads and injects without parsing.
