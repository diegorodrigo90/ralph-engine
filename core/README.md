# core/

Rust runtime crates that form the Ralph Engine foundation. All crates are workspace members compiled together.

## Crates

| Crate | Purpose |
|-------|---------|
| `re-plugin` | Plugin trait, types, capabilities. The contract all plugins implement. |
| `re-config` | Config parsing, locale resolution, typed layered configuration. |
| `re-core` | Runtime topology, registrations, health checks, action plans. |
| `re-mcp` | MCP server descriptors, launch policy, transport contracts. |
| `re-official` | Built-in plugin registry. Auto-generated from plugin crate dependencies. |
| `re-build-utils` | Build-time i18n codegen (TOML locale files to Rust structs). |
| `re-tui` | Terminal dashboard framework (ratatui). Layout, events, keybindings, logging. |
| `re-cli` | CLI binary (`ralph-engine`), command registry, i18n, orchestration. |

## Architecture

```
re-cli (binary)
  ├── re-official (plugin registry)
  │     ├── re-plugin (trait contracts)
  │     ├── re-config (configuration)
  │     ├── re-core (runtime state)
  │     └── re-mcp (MCP contracts)
  ├── re-tui (terminal UI)
  └── re-build-utils (build-time only)
```

Core follows Model B: it orchestrates the flow between plugins but never inspects plugin-owned data.
