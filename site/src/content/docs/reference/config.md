---
title: "Config Reference"
description: "All configuration options"
---


The configuration system exposes typed contracts through the CLI:

## Commands

```bash
ralph-engine config show-defaults          # Full default project config (YAML)
ralph-engine config locale                 # Default locale settings
ralph-engine config budgets                # Prompt and context token budgets
ralph-engine config layers                 # Configuration resolution stack
ralph-engine config show-plugin <id>       # Resolved plugin config with provenance
ralph-engine config show-mcp-server <id>   # Resolved MCP server config with provenance
```

## Locale Management

```bash
ralph-engine locales list                  # Supported locales catalog
ralph-engine locales show <locale-id>      # Locale details (native name, fallback)
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
