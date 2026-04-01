# Arquitetura

## Posicionamento

Ralph Engine é um runtime open source, orientado a plugins, para fluxos de desenvolvimento com agentes.

## Estrutura do repositório

- `core/` — crates Rust do runtime
- `plugins/official/` — plugins oficiais com base em Rust
- `docs/` — docs em VitePress
- `site/` — assets do site público
- `catalog/` — metadados do catálogo
- `packaging/` — superfícies de npm e Homebrew
- `tools/create-ralph-engine/` — scaffolding para desenvolvedores
- `scripts/` — bootstrap, validação e automação de release

## Workspace Rust

- `re-core` — fundações compartilhadas do runtime
- `re-cli` — crate CLI que produz `ralph-engine`
- crates de plugins oficiais vivem em `plugins/official/*`

## Regras arquiteturais

- o core continua plugin-first e agnóstico de workflow
- MCP externo continua como parte nativa da arquitetura
- plugins oficiais são Rust
- plugins de terceiros continuam agnósticos de linguagem
- prompt, contexto, governança de MCP, segurança e diagnósticos continuam sendo preocupações do core
