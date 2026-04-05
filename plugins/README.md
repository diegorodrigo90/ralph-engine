# plugins/

Official and community plugins for Ralph Engine.

## Official Plugins (`official/`)

| Plugin | Kind | Purpose |
|--------|------|---------|
| `basic` | template | Starter template for new projects |
| `bmad` | template | BMAD workflow integration (work items, sprint tracking) |
| `claude` | agent_runtime | Claude Code agent (launch, MCP, context export) |
| `claudebox` | agent_runtime | ClaudeBox agent variant |
| `codex` | agent_runtime | OpenAI Codex agent (exec mode, skills) |
| `context` | context_manager | Session persistence, context compaction, cross-agent transfer |
| `findings` | context_provider | Findings file injection into agent prompts |
| `github` | data_source | GitHub repository integration |
| `guided` | tui_extension | Chat input bar for the TUI dashboard |
| `hello-world` | template | Example plugin for community developers |
| `ssh` | remote_control | SSH remote execution |
| `tdd-strict` | policy | TDD enforcement policy |

## Creating a Plugin

```bash
npx create-ralph-engine-plugin my-plugin
```

See the [Plugin Development Guide](https://ralphengine.com/docs/plugin-development/) for details.

## Architecture

Each plugin is a standalone Rust crate with:
- `manifest.yaml` — metadata, capabilities, kind
- `src/lib.rs` — `PluginRuntime` trait implementation
- `locales/` — i18n TOML files (EN + PT-BR)
- `build.rs` — locale codegen

Plugins own their behavior. Core only calls trait methods and auto-discovers contributions.
