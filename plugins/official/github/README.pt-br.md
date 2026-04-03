# official.github

Dados GitHub, contexto, forge e integracao MCP.

## Visao geral

Integracao completa com GitHub para o Ralph Engine. Fornece acesso tipado a dados do repositorio, status de CI/CD e operacoes de forge (criar PRs, gerenciar releases). Tambem disponibiliza um servidor MCP para acesso direto a API do GitHub a partir das sessoes do agente.

## Providers

Este plugin inclui tres providers tipados:

### Data source

`official.github.data` expoe metadados do repositorio para workflows — issues, pull requests, branches, tags e historico de commits. Os agentes podem consultar esses dados sem precisar de acesso direto a API do GitHub.

### Context provider

`official.github.context` fornece status de CI/CD, informacoes sobre execucoes de workflows e contexto do ambiente. Util para agentes que precisam verificar se o CI esta passando antes de fazer merge ou deploy.

### Forge provider

`official.github.forge` habilita operacoes automatizadas no GitHub — criar pull requests, gerenciar releases, atualizar issues e disparar workflows. E isso que torna os workflows do Ralph Engine capazes de automacao ponta a ponta.

## Servidor MCP

O plugin disponibiliza um servidor MCP que da aos agentes acesso direto a API do GitHub. Isso e usado pelos runtimes de agente (Claude, Codex) para interagir com o GitHub durante sessoes de codificacao.

## Requisitos

- Um repositorio GitHub (publico ou privado)
- CLI `gh` instalado para autenticacao, ou variavel de ambiente `GITHUB_TOKEN` configurada

## Quando usar

Este plugin e recomendado para qualquer projeto hospedado no GitHub. Ele permite que os agentes entendam o contexto do repositorio, verifiquem o status do CI e automatizem operacoes Git.
