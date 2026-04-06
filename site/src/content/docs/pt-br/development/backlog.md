---
title: "Backlog"
description: "Backlog de funcionalidades e melhorias conhecidas para o Ralph Engine"
---

## Sistema de Plugins

- Loader de plugins da comunidade — scan do filesystem + validação de manifest
- Tiers de confiança de plugins — oficial, verificado, comunidade
- Registro de plugins — catálogo pesquisável com versionamento
- CI de validação de plugins — verificações automatizadas antes de listar no catálogo
- Scaffolder: adicionar exemplo end-to-end do kind `workflow`

## Dashboard TUI

- Modo guiado: toggle do chat, polish da UX de pausa
- Gravação GIF/VHS para README e docs
- Seletor de temas na TUI (troca de tema em runtime)
- Layout configurável (tamanhos e posições dos painéis)

## Integração de Agentes

- Troca de contexto multi-agente
- Protocolo de compartilhamento de contexto entre agentes
- Plugins de memória para persistência entre sessões
- Protocolo bidirecional de comandos de agente (Claude stdin NDJSON)
- Plugin de agente Gemini

## CLI

- Subcomando `ralph-engine exec` (atalho headless)
- Alias `re` no shell via script de bootstrap
- Flag `--headless` para pipelines CI/CD

## Empacotamento

- npm: `npx ralph-engine`
- Homebrew: `brew install ralph-engine`
- crates.io: `cargo install ralph-engine`
- Cloudflare Pages: deploy do site de docs
- Artefatos de release assinados

## Documentação

- Tutorial de desenvolvimento de plugins (passo a passo)
- Tutorial de integração de servidor MCP
- Deep dive de configuração
- Guia de migração (upgrades de versão)
- Vídeo walkthrough

## ratatui-themekit

- Exemplo interativo de TUI (não apenas println)
- Benchmarks para resolução de temas
- Avaliação `no_std`
- Mais temas: Kanagawa, Everforest, Ayu
- `impl Theme for &T` blanket impl
