# tools/

Developer tooling for the Ralph Engine ecosystem.

## create-ralph-engine

Plugin scaffolder — generates a new plugin project with all boilerplate:

```bash
npx create-ralph-engine-plugin my-plugin
```

Generates:
- `Cargo.toml` with workspace dependencies
- `manifest.yaml` with correct schema
- `src/lib.rs` with `PluginRuntime` trait skeleton
- `build.rs` with i18n codegen
- `locales/en.toml` + `locales/pt-br.toml`
- Tests matching the contract verifier expectations

The scaffolder validates against the same typed contracts used by the core runtime, ensuring generated plugins are always compatible.
