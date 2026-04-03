# official.github

GitHub data, context, forge, and MCP integration.

## Overview

Comprehensive GitHub integration for Ralph Engine. Provides typed access to repository data, CI/CD status, and forge operations (creating PRs, managing releases). Also contributes an MCP server for direct GitHub API access from agent sessions.

## Providers

This plugin ships three typed providers:

### Data source

`official.github.data` exposes repository metadata to workflows — issues, pull requests, branches, tags, and commit history. Agents can query this data without needing direct GitHub API access.

### Context provider

`official.github.context` provides CI/CD status, workflow run information, and environment context. Useful for agents that need to check if CI is passing before merging or deploying.

### Forge provider

`official.github.forge` enables automated GitHub operations — creating pull requests, managing releases, updating issues, and triggering workflows. This is what makes Ralph Engine workflows capable of end-to-end automation.

## MCP server

The plugin contributes an MCP server that gives agents direct access to the GitHub API. This is used by agent runtimes (Claude, Codex) to interact with GitHub during coding sessions.

## Requirements

- A GitHub repository (public or private)
- `gh` CLI installed for authentication, or a `GITHUB_TOKEN` environment variable

## When to use

This plugin is recommended for any project hosted on GitHub. It enables agents to understand your repository context, check CI status, and automate Git operations.
