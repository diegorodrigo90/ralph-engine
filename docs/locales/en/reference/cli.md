# CLI Reference

The current Rust foundation exposes a minimal CLI surface while the runtime is rebuilt.

## Commands

```bash
ralph-engine
ralph-engine --version
ralph-engine capabilities
ralph-engine capabilities list
ralph-engine capabilities show <capability-id>
ralph-engine config
ralph-engine config show-defaults
ralph-engine config show-plugin <plugin-id>
ralph-engine plugins
ralph-engine plugins list
ralph-engine plugins show <plugin-id>
ralph-engine runtime
ralph-engine runtime show
ralph-engine mcp
ralph-engine mcp list
ralph-engine mcp show <server-id>
```

The `plugins show` command prints the immutable plugin contract, including lifecycle, load boundary, runtime hooks, and resolved activation state.

The `capabilities` command family prints the typed runtime capability registry so capability providers remain explicit and modular.

The `mcp show` command prints the typed MCP launch contract, including process model, launch policy, command boundaries, working-directory policy, environment policy, and availability.

The `runtime show` command prints the resolved runtime topology, including effective plugin activation, capability registration, and MCP enablement.
