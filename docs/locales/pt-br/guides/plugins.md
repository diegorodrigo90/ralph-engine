# Plugins

Plugins continuam sendo a unidade de distribuição.

O reboot mantém estas regras arquiteturais:

- plugins oficiais são implementados em Rust
- plugins de terceiros continuam agnósticos de linguagem
- confiança de plugin continua explícita: plugins oficiais são first-party, enquanto manifests de terceiros ficam em escopo `community` até o core definir algo diferente
- capabilities continuam sendo o modelo de extensibilidade
- templates são capabilities de plugin, não um tipo de artefato separado
- MCP pode ser configurado externamente e ampliado por plugins
- manifests de plugins de terceiros seguem um contrato versionado de `manifest.yaml` mantido em `tools/create-ralph-engine/`
- metadados de exibição de plugins suportam nomes e resumos localizados, começando por `en` e `pt-br`
- quando um locale de plugin não existir, as superfícies do runtime fazem fallback para o nome e o resumo em inglês em vez de falhar
- crates que renderizam saída pública voltada a plugins devem manter strings de locale em módulos ou arquivos por idioma, para que adicionar um novo locale continue sendo uma mudança aditiva em vez de reescrever handlers de comando
