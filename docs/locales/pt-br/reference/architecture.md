# Arquitetura

## Posicionamento

Ralph Engine é um runtime open source, orientado a plugins, para fluxos de desenvolvimento com agentes.

## Estrutura do repositório

- `core/` — crates Rust do runtime
- `plugins/official/` — plugins oficiais com base em Rust
- `docs/` — docs em VitePress
- `site/` — superfícies públicas da web, UI compartilhada e metadados de plugins
- `packaging/` — superfícies de npm e Homebrew
- `tools/create-ralph-engine/` — scaffolding para desenvolvedores
- `scripts/` — bootstrap, validação e automação de release

## Workspace Rust

- `re-core` — fundações compartilhadas do runtime, topologia e contratos de estado
- `re-config` — contratos, escopos, padrões e regras de resolução compartilhados de configuração do runtime
- `re-mcp` — contratos compartilhados de contribuições MCP, policy de lançamento, modelo de processo, fronteiras de comando e policy
- `re-plugin` — contratos compartilhados de metadados, lifecycle, runtime hooks, fronteira de carregamento e capabilities de plugin
- `re-cli` — crate CLI que produz `ralph-engine`
- crates de plugins oficiais vivem em `plugins/official/*`

## Regras arquiteturais

- o core continua plugin-first e agnóstico de workflow
- MCP externo continua como parte nativa da arquitetura
- plugins oficiais são Rust
- plugins de terceiros continuam agnósticos de linguagem
- prompt, contexto, governança de MCP, segurança e diagnósticos continuam sendo preocupações do core
- famílias de comandos da CLI evoluem por módulos e registries isolados, não por um dispatcher central cada vez maior
- capabilities de plugin e contribuições MCP evoluem por descritores tipados, para que novas capabilities entrem sem lógica acoplada por string espalhada no runtime
- o lifecycle de plugin evolui por estágios tipados compartilhados, para que descoberta, configuração, validação e carregamento continuem explícitos e extensíveis
- runtime hooks de plugin evoluem por descritores tipados compartilhados, para que prepare, doctor, prompt, agent, MCP e policy permaneçam modulares sem dispatch ad hoc
- a resolução de configuração evolui por escopos tipados em camadas, para que defaults e futuros overrides permaneçam explícitos em vez de inferidos dentro dos comandos
- a topologia do runtime evolui por registros tipados, para que ativação de plugin e enablement de MCP permaneçam explícitos em vez de reconstruídos ad hoc por comando
- fronteiras de carregamento de plugin permanecem tipadas, para que integração in-process, subprocess e remota evoluam sem branch ad hoc espalhada no runtime
