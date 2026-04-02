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
- plugin display metadata supports localized names, starting with `en` and `pt-br`
- when a plugin locale is missing, runtime-facing surfaces fall back to the English display name instead of failing closed
