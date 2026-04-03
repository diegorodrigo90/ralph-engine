# official.claudebox

Claude Box runtime and MCP session integration.

## Overview

Integrates Claude Box — Anthropic's sandboxed Claude environment — as an agent runtime. Claude Box runs in an isolated Docker container with filesystem access, browser automation via Playwright, and sudo capabilities, without affecting your host system.

## How it works

1. `ralph-engine agents launch official.claudebox.session` probes for the `claudebox` binary
2. If found, it launches a sandboxed session with your project mounted as a shared volume
3. The agent has full system access inside the container (install packages, run servers, use browsers)
4. MCP servers are registered and available within the sandboxed environment

## Requirements

- Claude Box installed and on your PATH (`claudebox` command available)
- Docker running (Claude Box uses containers for isolation)
- A valid Ralph Engine project

## When to use

Use this plugin when:

- You need the agent to install system packages, run Docker commands, or access browsers
- You want full system access without risking your host environment
- Your workflow requires DevContainer-style isolated development

For standard Claude sessions without sandboxing, use `official.claude` instead.
