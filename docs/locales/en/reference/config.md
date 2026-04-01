# Config Reference

The Rust-first reboot has reintroduced the first typed configuration slice on the new core.

The current default contract now exposes:

- `schema_version`
- `default_locale`
- default plugin entries
- MCP defaults

The broader target architecture still includes:

- project config
- plugin defaults
- user overrides
- MCP configuration
- prompt and context budgets

The current CLI can render the default typed contract with:

```bash
ralph-engine config show-defaults
```

Those contracts will continue to expand on top of the new Rust core under TDD.
