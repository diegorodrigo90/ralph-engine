# Plugins

Plugins continuam sendo a unidade de distribuição.

O reboot mantém estas regras arquiteturais:

- plugins oficiais são implementados em Rust
- plugins de terceiros continuam agnósticos de linguagem
- capabilities continuam sendo o modelo de extensibilidade
- templates são capabilities de plugin, não um tipo de artefato separado
- MCP pode ser configurado externamente e ampliado por plugins
- manifests de plugins de terceiros seguem um contrato versionado de `manifest.yaml` mantido em `tools/create-ralph-engine/`
