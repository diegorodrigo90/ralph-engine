---
title: "Troubleshooting"
description: "Common issues when using Ralph Engine and how to fix them"
---

## ralph-engine: command not found

The CLI is not on your PATH. Reinstall or check your installation method:

```bash
npm list -g ralph-engine
```

If installed via Cargo:

```bash
which ralph-engine
```

Make sure `~/.cargo/bin` (Cargo) or the npm global bin directory is in your `PATH`.

## doctor reports issues

Run the doctor command to see what needs attention:

```bash
ralph-engine doctor
```

To automatically generate a fixed configuration:

```bash
ralph-engine doctor apply-config .ralph-engine/config.yaml
```

## Plugin not found

If a plugin command says the plugin doesn't exist:

```bash
ralph-engine plugins list
```

Check that the plugin ID is correct (e.g., `official.claude`, not just `claude`).

## MCP server not available

Check the status of all MCP servers:

```bash
ralph-engine mcp status
```

For a specific server:

```bash
ralph-engine mcp status <server-id>
```

If a SpawnProcess server is unavailable, the required binary may not be on your PATH.

## Configuration not loading

Verify the configuration file exists and is valid:

```bash
ralph-engine config show-defaults
```

```bash
ralph-engine config layers
```

Check that `.ralph-engine/config.yaml` exists in your project root.

## Wrong language

The CLI respects locale in this order:
1. `--locale` flag (e.g., `--locale pt-br`)
2. `RALPH_ENGINE_LOCALE` environment variable
3. System locale (`LC_ALL`, `LC_MESSAGES`, `LANG`)
4. English (default)

To force Portuguese:

```bash
ralph-engine --locale pt-br --help
```

## Runtime issues

List all unresolved issues detected by the runtime:

```bash
ralph-engine runtime issues
```

See the remediation plan:

```bash
ralph-engine runtime plan
```
