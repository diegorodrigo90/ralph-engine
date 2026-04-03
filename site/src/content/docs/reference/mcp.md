---
title: "MCP"
description: "Model Context Protocol integration"
---

Ralph Engine exposes a typed MCP (Model Context Protocol) contract on the core runtime. Each MCP server is a plugin contribution with a structured descriptor.

## MCP Server Descriptor

Every registered MCP server carries a typed contract including:

- **Server identifier** — unique ID scoped to the owning plugin
- **Owning plugin identifier** — which plugin contributes this server
- **Transport** — communication protocol (stdio, HTTP, etc.)
- **Launch policy** — when and how the server should be started
- **Process model** — `SpawnProcess` (external binary) or `PluginRuntime` (in-process via plugin trait)
- **Command contract** — the binary and arguments to execute (for SpawnProcess servers)
- **Working-directory policy** — where the spawned process runs
- **Environment policy** — environment variables passed to the process
- **Availability policy** — conditions under which the server is considered available

## Process Models

### SpawnProcess

The server is an external binary. Ralph Engine spawns it as a child process, manages its lifecycle, and communicates via the declared transport. The `mcp launch` command starts it in foreground if the binary is found on PATH.

Use case: wrapping third-party tools (e.g., `npx @anthropic/mcp-server-github`) where the server binary exists outside the Ralph Engine process.

### PluginRuntime

The server is implemented inside a plugin's `PluginRuntime` trait. The `mcp launch` command dispatches to `register_mcp_server()` to check readiness instead of spawning a process.

Use case: servers that are part of a plugin's internal implementation, where readiness depends on the plugin's own validation and state rather than an external binary.

## Commands

List all registered MCP servers:

```bash
ralph-engine mcp list
```

Show the full launch contract for a server:

```bash
ralph-engine mcp show <server-id>
```

Show the launch plan derived from the contract:

```bash
ralph-engine mcp plan <server-id>
```

Validate and optionally start an MCP server:

```bash
ralph-engine mcp launch <server-id>
```

Check launch readiness for all MCP servers:

```bash
ralph-engine mcp status
```

Check detailed status for one MCP server:

```bash
ralph-engine mcp status <server-id>
```

## Runtime Integration

MCP servers are part of the runtime topology. Use `runtime mcp-plans` to see all enabled MCP launch plans:

```bash
ralph-engine runtime mcp-plans
```

Use `runtime issues` to see disabled or unhealthy MCP servers:

```bash
ralph-engine runtime issues
```
