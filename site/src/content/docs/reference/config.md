---
title: "Config Reference"
description: "All configuration options"
---

The configuration system exposes typed contracts through the CLI.

## Commands

Print the full default project config (YAML):

```bash
ralph-engine config show-defaults
```

Print default locale settings:

```bash
ralph-engine config locale
```

Print prompt and context token budgets:

```bash
ralph-engine config budgets
```

Print the configuration resolution stack:

```bash
ralph-engine config layers
```

Print resolved config for a specific plugin with provenance:

```bash
ralph-engine config show-plugin <id>
```

Print resolved config for a specific MCP server with provenance:

```bash
ralph-engine config show-mcp-server <id>
```

## Locale Management

List all supported locales:

```bash
ralph-engine locales list
```

Show details for a specific locale (native name, fallback rules):

```bash
ralph-engine locales show <locale-id>
```

The `--locale <id>` flag (or `-L <id>`) switches language for a single invocation. Without it, the CLI resolves locale from `RALPH_ENGINE_LOCALE`, then OS locale (`LC_ALL`, `LC_MESSAGES`, `LANG`), then defaults to English.

## What the Config Contains

| Section | Description |
|---------|-------------|
| `schema_version` | Config schema version |
| `default_locale` | Default locale for the project |
| `plugins` | Plugin activation state (enabled/disabled per plugin) |
| `mcp_servers` | MCP server activation state |
| `budgets` | Prompt and context token ceilings |

## Configuration Resolution

Config is resolved through a layered stack (inspectable via `config layers`):

1. **Built-in defaults** — compiled into the binary
2. **Plugin defaults** — declared by each plugin crate
3. **Project config** — `.ralph-engine/config.yaml` (when present)

Use `ralph-engine doctor apply-config <path>` to generate a patched config that enables all recommended plugins and servers.
