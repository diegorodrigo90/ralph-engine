# Plugins

Plugins remain the unit of distribution.

The reboot keeps these architectural rules:

- official plugins are implemented in Rust
- third-party plugins stay language-agnostic
- plugin trust stays explicit: official plugins are first-party, while third-party manifests stay community-scoped until the core defines otherwise
- capabilities remain the extensibility model
- templates are plugin capabilities, not a separate artifact kind
- MCP can be configured externally and enhanced by plugins
- third-party plugin manifests follow a versioned `manifest.yaml` contract owned by `tools/create-ralph-engine/`
- plugin display metadata supports localized names and summaries, starting with `en` and `pt-br`
- when a plugin locale is missing, runtime-facing surfaces fall back to the English name and summary instead of failing closed
- crates that render public plugin-facing output should keep locale strings in per-locale modules or files, so adding a new locale stays additive instead of rewriting command handlers
- each locale module should expose one locale catalog object instead of scattered constants or locale branching across handlers
- official plugin crates now follow that rule with `src/i18n/en.rs`, `src/i18n/pt_br.rs`, and `src/i18n/mod.rs`; scaffolded community crates should follow the same layout
- `npx create-ralph-engine-plugin` is the scaffolding entrypoint and should own project generation concerns instead of pushing scaffolding into the runtime core
- the scaffolder generates a Rust plugin crate skeleton plus localized `manifest.yaml` metadata so new plugin projects start aligned with the typed runtime contract
