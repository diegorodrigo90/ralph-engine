# MCP Reference

The Rust-first reboot now exposes a typed MCP contract on the new core.

The current shared descriptor already models:

- server identifier
- owning plugin identifier
- transport
- process model
- availability policy

The current CLI can inspect the built-in MCP catalog with:

```bash
ralph-engine mcp list
ralph-engine mcp show <server-id>
```

These contracts will keep expanding under TDD so process launching and policy boundaries stay typed instead of being spread across runtime-specific branches.
