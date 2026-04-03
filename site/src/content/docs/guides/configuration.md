---
title: "Configuration"
description: "Configure Ralph Engine with YAML files"
---


Ralph Engine configuration is managed through typed Rust contracts. The CLI exposes the full configuration surface:

```bash
ralph-engine config show-defaults    # Default project config (YAML)
ralph-engine config locale           # Default locale settings
ralph-engine config budgets          # Token and context budgets
ralph-engine config layers           # Configuration resolution layers
ralph-engine config show-plugin <id> # Plugin-specific resolved config
ralph-engine config show-mcp-server <id> # MCP server resolved config
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

The `doctor` command analyzes your project configuration and suggests fixes:

```bash
ralph-engine doctor                         # Show diagnostic report
ralph-engine doctor apply-config config.yaml # Write patched config to file
```
