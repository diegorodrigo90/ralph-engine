# Config Reference

The Rust-first reboot has reintroduced the first typed configuration slice on the new core.

The current default contract now exposes:

- `schema_version`
- `default_locale`
- default plugin entries with typed activation state
- typed configuration scopes and resolved plugin activation provenance
- MCP defaults

The broader target architecture still includes:

- project config
- plugin defaults
- user overrides
- MCP configuration
- prompt and context budgets

The current CLI can render the default typed contract with:

```bash
ralph-engine config layers
ralph-engine config show-defaults
ralph-engine config show-layers
ralph-engine config show-plugin <plugin-id>
```

The `config layers` command renders the canonical typed configuration stack in resolution order so defaults, future workspace settings, project settings, and user overrides stay explicit in the runtime contract.

The `config show-plugin` command now renders the effective plugin activation together with the scope that supplied it.

Those contracts will continue to expand on top of the new Rust core under TDD.
