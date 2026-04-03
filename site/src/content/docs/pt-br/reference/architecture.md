---
title: "Arquitetura"
description: "Arquitetura interna e decisões de design"
---

## Posicionamento

Ralph Engine é um runtime open-source, orientado a plugins, para fluxos de desenvolvimento com agentes.

## Estrutura do Repositório

- `core/` — crates Rust do runtime
- `plugins/official/` — plugins oficiais em Rust
- `site/` — site de documentação Astro Starlight, superfícies públicas da web, UI compartilhada e metadados de plugins
- `site/src/content/docs/` — fonte da documentação (EN + PT-BR)
- `packaging/` — superfícies de empacotamento npm e Homebrew
- `tools/create-ralph-engine/` — scaffolding de plugin para `npx create-ralph-engine-plugin`
- `scripts/` — bootstrap, validação e automação de release

## Workspace Rust

- `re-core` — fundações compartilhadas do runtime, topologia e contratos de estado
- `re-config` — contratos, escopos, padrões e regras de resolução compartilhados de configuração do runtime
- `re-mcp` — contratos compartilhados de contribuições MCP, policy de lançamento, modelo de processo, fronteiras de comando e policy
- `re-plugin` — contratos compartilhados de metadados, lifecycle, runtime hooks, fronteira de carregamento e capabilities de plugin
- `re-official` — catálogo tipado embutido que conecta plugins oficiais e servidores MCP em um snapshot reutilizável do runtime
- `re-cli` — crate CLI que produz `ralph-engine`
- Crates de plugins oficiais ficam em `plugins/official/*`

## Regras Arquiteturais

- O core permanece plugin-first e agnóstico de workflow.
- MCP externo permanece como parte nativa da arquitetura.
- Plugins oficiais são em Rust.
- Plugins de terceiros permanecem agnósticos de linguagem.
- Prompt, contexto, governança de MCP, segurança e diagnósticos permanecem como preocupações do core.
- Famílias de comandos da CLI evoluem por módulos e registries isolados, não por um dispatcher central cada vez maior.
- Capabilities de plugin e contribuições MCP evoluem por descritores tipados para que novas capabilities possam ser adicionadas sem lógica acoplada por string no runtime.
- O lifecycle de plugin evolui por estágios tipados compartilhados para que descoberta, configuração, validação e carregamento permaneçam explícitos e extensíveis.
- Runtime hooks de plugin evoluem por descritores tipados compartilhados para que prepare, doctor, prompt, agent, MCP e policy permaneçam modulares sem dispatch ad hoc.
- A resolução de configuração evolui por escopos tipados em camadas para que defaults e futuros overrides permaneçam explícitos em vez de inferidos dentro dos comandos.
- Topologia do runtime, saúde, reporting de issues, reporting de doctor, plano de ações do runtime e registro de runtime hooks evoluem por registros tipados e contratos compartilhados para que ativação de plugin, registro de capability, registro de hook e enablement de MCP permaneçam explícitos em vez de reconstruídos ad hoc por comando.
- Capabilities desabilitadas e runtime hooks desabilitados permanecem visíveis no health e na remediação do runtime; não se tornam metadado invisível só porque a topologia resolveu.
- Fronteiras de carregamento de plugin permanecem tipadas para que integração in-process, subprocess e remota possam evoluir sem branching ad hoc no runtime.
