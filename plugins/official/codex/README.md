# official.codex

Codex runtime and MCP session integration.

## Overview

Integrates OpenAI's Codex CLI as an agent runtime for Ralph Engine workflows. This plugin allows teams using Codex to get the same Ralph Engine experience — plugins, checks, prompt assembly, and MCP servers — regardless of which AI coding assistant they prefer.

## How it works

1. `ralph-engine agents launch official.codex.session` probes for the `codex` binary
2. If found, it loads your project configuration and bootstraps a Codex session
3. All enabled plugins, checks, and prompt fragments are available to the agent
4. MCP servers defined in your config are registered for the session

## Requirements

- Codex CLI installed and on your PATH (`codex` command available)
- A valid Ralph Engine project

## When to use

Use this plugin when:

- Your team uses OpenAI Codex as the primary AI coding assistant
- You want to switch between Claude and Codex without changing your project configuration
- You're evaluating different AI assistants and want a consistent workflow layer

Ralph Engine is agent-agnostic — the same config works with Claude, Claude Box, or Codex.
