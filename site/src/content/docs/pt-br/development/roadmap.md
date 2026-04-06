---
title: "Roadmap"
description: "Roadmap de desenvolvimento do Ralph Engine — marcos planejados e concluídos"
---

## Concluído

### Core Runtime (v0.2.0-alpha)

- Sistema de plugins com 17 capabilities e auto-discovery
- 13 plugins oficiais (agente, workflow, policy, TUI, contexto, roteamento)
- Comando `run` — loop autônomo com resolução de work items
- Dashboard TUI com ratatui — atividade do agente em tempo real, painéis sidebar, blocos de feed
- `zone_hint` — plugins posicionam painéis na sidebar ou área central do feed
- `FeedContribution` — plugins injetam blocos de status no feed em tempo real
- Sistema de widgets `TuiBlock` — barras, métricas, indicadores, pares, listas
- Sistema de temas via ratatui-themekit — 11 temas, slots de cor semânticos
- Lançamento de agente com modo `-p` (Claude, Codex, ClaudeBox)
- Auto-discovery e registro de servidores MCP
- i18n — Inglês e Português Brasileiro (baseado em TOML)
- Scaffolder de plugins — `npx create-ralph-engine-plugin`
- Plugin findings — loop de feedback injetando findings anteriores nos prompts
- Roteador de agentes — classificação de tarefas e seleção de agente
- Barra de input do modo guiado — feedback human-in-the-loop
- Pausar/continuar sessões de agente
- Modo autônomo com gate de aceitação de segurança
- 994 testes, 72 Golden Rules, zero warnings do clippy

### Site de Documentação (v1.0)

- Astro + Starlight com EN/PT-BR
- Guia de plugins com contribuições TUI
- Referência de configuração
- Referência CLI
- Visão geral da arquitetura
- Política de privacidade e termos

## Em Progresso

### Polish do Modo Guiado

- Toggle do chat (mostrar/ocultar barra de input)
- Melhorias no comportamento de pausa
- Gravação de demo

### Plugin Demo

- Plugin da comunidade criado via scaffolder
- Demonstra TuiBlock, feed_contributions, zone_hint

## Planejado

### Empacotamento e Distribuição

- Pacote npm (`npx ralph-engine`)
- Fórmula Homebrew
- Binary crate no crates.io
- Deploy Cloudflare Pages para o site de docs

### Ecossistema de Plugins da Comunidade

- Loader de plugins da comunidade (scan do filesystem)
- Validação de plugins antes de listar no catálogo
- Tiers de confiança (oficial, verificado, comunidade)
- Registro de plugins

### Funcionalidades Avançadas

- Troca de contexto multi-agente
- Compartilhamento de contexto entre agentes
- Plugins de memória para persistência entre sessões
- Plugins preset (bundles de config + hooks)
- Modo multi-projeto
- Modo equipe

### Qualidade e Polish

- Gravação GIF/VHS da TUI em ação
- Exemplo interativo de TUI no ratatui-themekit
- Benchmarks para resolução de temas
- Avaliação `no_std` para themekit
