# Plugin Contract Rules (Model B)

Rules follow EARS syntax (SHALL keyword).
Applies to: `core/**/*.rs`, `plugins/**/*.rs`

## Model B: Mediator Pattern

Core orchestrates the FLOW between plugins but NEVER inspects, transforms,
or understands plugin-owned data. This is Rule 65 in AGENTS.md.

## Core Prohibitions

- Core SHALL NOT hardcode plugin IDs (`"official.*"` strings outside `plugins/`).
- Core SHALL NOT hardcode tool names, agent names, or workflow steps.
- Core SHALL NOT contain business logic belonging to workflow/agent/routing plugins.
- Core SHALL NOT know about specific data formats (diff syntax, agent streams).
- Core naming SHALL use generic terms: `Feed`, `Block`, `Indicator`, `Agent`.
- Core SHALL NEVER use: `workflow`, `sprint`, `story`, `test`, `build`, `CR`,
  `claude`, `codex`, `gemini`, `Read`, `Edit`, `Bash` as identifiers.

## Plugin Responsibilities

- Plugins classify their tools via `ToolKindMapping` at registration time.
- Plugins parse agent output into generic `BlockKind` types.
- Plugins declare indicators, labels, and state transitions.
- Plugins own their locale catalogs, manifests, and runtime hooks.

## Tool Kind Mapping (Model B)

Plugins register tool-name-to-block-kind mappings via `Feed::register_tool_mappings()`.
Core resolves unknown tools to `BlockKind::AgentText` (safe fallback).
Core NEVER hardcodes tool name → kind mappings.

## Code Review Checklist

Every CR SHALL verify Model B compliance:
1. No hardcoded plugin IDs in core
2. No hardcoded tool/agent/workflow names in core
3. No business logic leaking from plugins into core/TUI
4. No core knowledge of agent-specific data formats
5. Test fixtures use generic IDs, not official plugin names
