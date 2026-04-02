# Plugins

Plugins remain the unit of distribution.

The reboot keeps these architectural rules:

- official plugins are implemented in Rust
- third-party plugins stay language-agnostic
- capabilities remain the extensibility model
- templates are plugin capabilities, not a separate artifact kind
- MCP can be configured externally and enhanced by plugins
- third-party plugin manifests follow a versioned `manifest.yaml` contract owned by `tools/create-ralph-engine/`
