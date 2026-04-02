# Config Reference

The Rust-first reboot has reintroduced the first typed configuration slice on the new core.

The current default contract now exposes:

- `schema_version`
- `default_locale`
- default plugin entries with typed activation state
- typed configuration scopes and resolved plugin activation provenance
- MCP defaults
- prompt and context budgets

The broader target architecture still includes:

- project config
- plugin defaults
- user overrides
- MCP configuration
- prompt and context budgets

The current CLI can render the default typed contract with:

```bash
ralph-engine locales
ralph-engine locales list
ralph-engine locales show <locale-id>
ralph-engine config budgets
ralph-engine config layers
ralph-engine config locale
ralph-engine config show-budgets
ralph-engine config show-defaults
ralph-engine config show-layers
ralph-engine config show-locale
ralph-engine config show-mcp-server <server-id>
ralph-engine config show-plugin <plugin-id>
```

The `config locale` command renders the typed default locale contract so CLI localization stays visible and versioned instead of remaining only an internal default.

`re-config` now also owns the shared typed locale contract used by `re-cli`, `re-core`, `re-plugin`, and `re-mcp`, so locale resolution grows by extending one canonical foundation instead of re-implementing locale parsing per crate.

The CLI now accepts a global `--locale <locale-id>` or `-L <locale-id>` flag for one-off language selection. When that flag is absent, locale resolution falls back to `RALPH_ENGINE_LOCALE` and then to the typed default locale declared by `re-config`.

The `locales` command family renders the canonical supported locale catalog, including the native name of each locale and whether it falls back to English. This keeps locale expansion explicit and versioned instead of scattering support assumptions across the runtime.

The `config budgets` command renders the canonical typed prompt and context budget contract so token ceilings stay explicit in the shared runtime configuration instead of being inferred later from provider-local defaults.

The `config layers` command renders the canonical typed configuration stack in resolution order so defaults, future workspace settings, project settings, and user overrides stay explicit in the runtime contract.

The `config show-plugin` command now renders the effective plugin activation together with the scope that supplied it.

The `config show-mcp-server` command renders the effective typed activation for one MCP server together with the scope that supplied it, so per-server opt-in stays explicit instead of being inferred only from plugin activation.

Those contracts will continue to expand on top of the new Rust core under TDD.
