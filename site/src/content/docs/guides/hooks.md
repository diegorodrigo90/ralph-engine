---
title: "Hooks"
description: "Lifecycle hooks for agent workflows"
---

Runtime hooks are the extension points where plugins contribute behavior to the Ralph Engine lifecycle. Each hook represents a specific phase in the runtime — from scaffolding a new project to enforcing policies during an agent session.

Hooks are not called directly by users. They fire automatically when the runtime reaches the corresponding lifecycle phase. For example, `prepare` hooks run before any workflow starts, `doctor` hooks run during diagnostics, and `agent_bootstrap` hooks run when launching an agent.

## Available Hooks

Each plugin declares which hooks it contributes. The runtime tracks these registrations.

List all registered hooks:

```bash
ralph-engine hooks list
```

Show details for a specific hook:

```bash
ralph-engine hooks show <hook-id>
```

Show the execution plan for a hook (which plugins contribute, in what order):

```bash
ralph-engine hooks plan <hook-id>
```

## Hook Types

| Hook | When it runs | What it does |
|------|-------------|-------------|
| `scaffold` | During `templates scaffold` / `templates materialize` | Project scaffolding (template materialization) |
| `prepare` | Before any workflow starts (`checks run prepare`) | Pre-flight validation |
| `doctor` | During `doctor` diagnostics | System health checks |
| `prompt_assembly` | When building the prompt for an agent session | Prompt fragment composition |
| `agent_bootstrap` | During `agents launch` | Agent runtime initialization |
| `mcp_registration` | During runtime startup | MCP server registration |
| `data_source_registration` | During runtime startup | Data source provider registration |
| `context_provider_registration` | During runtime startup | Context provider registration |
| `forge_provider_registration` | During runtime startup | Forge automation registration |
| `remote_control_bootstrap` | During runtime startup | Remote control initialization |
| `policy_enforcement` | During agent sessions | Policy guardrail enforcement |

## Hook Execution

Hooks are executed through the `PluginRuntime` trait. When you run commands like `checks run prepare` or `agents launch`, the runtime dispatches to the appropriate plugin's hook implementation.

Plugins that provide a runtime can respond to hook invocations with real validation, binary probing, or process management — depending on their capabilities.

## Project Hooks File

The BMAD plugin template includes a `.ralph-engine/hooks.yaml` file for project-level hook configuration. This file is part of the template scaffolding and is consumed by the BMAD workflow, not by the core runtime directly.
