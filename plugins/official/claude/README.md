# official.claude

Claude agent runtime and MCP session integration.

## Overview

Integrates Anthropic's Claude as an agent runtime for Ralph Engine. When you launch a session, this plugin detects the `claude` CLI on your PATH, validates it's ready, and bootstraps a coding session with your project's configuration pre-loaded.

## How it works

1. `ralph-engine agents launch official.claude.session` probes for the Claude CLI binary
2. If found, it loads your project's `.ralph-engine/config.yaml` and `prompt.md`
3. The agent session starts with all enabled plugins, checks, and prompt fragments available
4. MCP servers defined in your config are registered and available to the agent

## Requirements

- Claude CLI installed and on your PATH (`claude` command available)
- A valid Ralph Engine project (`.ralph-engine/config.yaml` exists)

## MCP contribution

This plugin also contributes an MCP server that agents can use for session management — starting, stopping, and querying active sessions.

## When to use

This is the default agent runtime for most Ralph Engine projects. If you're using Claude Code or Claude Desktop as your AI coding assistant, this is the plugin you want.

For sandboxed environments, use `official.claudebox` instead. For OpenAI Codex users, use `official.codex`.
