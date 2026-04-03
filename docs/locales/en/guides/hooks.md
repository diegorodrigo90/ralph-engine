# Hooks

Runtime hooks are the extension points where plugins contribute behavior to the Ralph Engine lifecycle.

## Available Hooks

Each plugin declares which hooks it contributes. The runtime tracks these registrations:

```bash
ralph-engine hooks list              # List all registered hooks
ralph-engine hooks show <hook-id>    # Show hook details
ralph-engine hooks plan <hook-id>    # Show execution plan for a hook
```

## Hook Types

| Hook | What it does |
|------|-------------|
| `scaffold` | Project scaffolding (template materialization) |
| `prepare` | Pre-flight validation before workflows |
| `doctor` | System diagnostics and health checks |
| `prompt_assembly` | Prompt fragment composition |
| `agent_bootstrap` | Agent runtime initialization |
| `mcp_registration` | MCP server registration |
| `data_source_registration` | Data source provider registration |
| `context_provider_registration` | Context provider registration |
| `forge_provider_registration` | Forge automation registration |
| `remote_control_bootstrap` | Remote control initialization |
| `policy_enforcement` | Policy guardrail enforcement |

## Hook Execution

Hooks are executed through the `PluginRuntime` trait. When you run commands like `checks run prepare` or `agents launch`, the runtime dispatches to the appropriate plugin's hook implementation.

Plugins that provide a runtime can respond to hook invocations with real validation, binary probing, or process management — depending on their capabilities.

## Project Hooks File

The BMAD plugin template includes a `.ralph-engine/hooks.yaml` file for project-level hook configuration. This file is part of the template scaffolding and is consumed by the BMAD workflow, not by the core runtime directly.
