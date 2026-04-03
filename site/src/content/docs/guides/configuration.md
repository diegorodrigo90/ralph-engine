---
title: "Configuration"
description: "Configure your project with .ralph-engine/config.yaml"
---

Ralph Engine configuration is managed through typed Rust contracts. The CLI exposes the full configuration surface.

## Inspecting Configuration

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

Print the configuration resolution layer stack:

```bash
ralph-engine config layers
```

Print resolved config for a specific plugin:

```bash
ralph-engine config show-plugin <id>
```

Print resolved config for a specific MCP server:

```bash
ralph-engine config show-mcp-server <id>
```

## Project Configuration

Running `ralph-engine templates scaffold official.basic.starter .` creates a `.ralph-engine/` directory with the project configuration files:

- `.ralph-engine/config.yaml` — project configuration
- `.ralph-engine/prompt.md` — project-specific prompt content
- `.ralph-engine/hooks.yaml` — hook configuration (when using BMAD plugin)

## Configuration Layers

Configuration is resolved through a layered system:

1. **Built-in defaults** — shipped with the runtime
2. **Plugin defaults** — declared by each plugin
3. **Project config** — from `.ralph-engine/config.yaml`

Use `ralph-engine config layers` to inspect the full resolution chain.

## Diagnostics

Print a diagnostic report:

```bash
ralph-engine doctor
```

Write a patched config to file:

```bash
ralph-engine doctor apply-config config.yaml
```
