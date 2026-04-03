---
title: "Configuração"
description: "Configure seu projeto com .ralph-engine/config.yaml"
---

A configuração do Ralph Engine é gerenciada por contratos tipados em Rust. A CLI expõe toda a superfície de configuração.

## Inspecionando a Configuração

Exiba a configuração padrão completa do projeto (YAML):

```bash
ralph-engine config show-defaults
```

Exiba as configurações padrão de idioma:

```bash
ralph-engine config locale
```

Exiba os limites de tokens de prompt e contexto:

```bash
ralph-engine config budgets
```

Exiba a pilha de camadas de resolução de configuração:

```bash
ralph-engine config layers
```

Exiba a configuração resolvida de um plugin específico:

```bash
ralph-engine config show-plugin <id>
```

Exiba a configuração resolvida de um servidor MCP específico:

```bash
ralph-engine config show-mcp-server <id>
```

## Configuração do Projeto

Executar `ralph-engine templates materialize official.basic.starter .` cria um diretório `.ralph-engine/` com os arquivos de configuração do projeto:

- `.ralph-engine/config.yaml` — configuração do projeto
- `.ralph-engine/prompt.md` — conteúdo de prompt específico do projeto
- `.ralph-engine/hooks.yaml` — configuração de hooks (ao usar o plugin BMAD)

### Padrões do runtime

A configuração padrão do runtime (exibida por `ralph-engine config show-defaults`):

```yaml
schema_version: 1
default_locale: en
plugins:
  - id: official.basic
    activation: enabled
mcp:
  enabled: true
  discovery: official_only
  servers:
budgets:
  prompt_tokens: 8192
  context_tokens: 32768
```

### Config do template

O template starter cria um `.ralph-engine/config.yaml` com configurações específicas de workflow que estendem os padrões do runtime:

```yaml
agent:
  type: "claude"           # claude | codex | claudebox
  cooldown_seconds: 10
  max_work_items_per_session: 1

workflow:
  instructions: |
    Follow a minimal implementation loop.
    Read the work item, implement the change, run tests,
    and leave the tree reviewable.

execution:
  max_post_agent_retries: 0
  max_retry_output_chars: 800

tracker:
  type: "file"
  status_file: "sprint-status.yaml"

circuit_breaker:
  max_failures: 3
  cooldown_minutes: 5
```

## Camadas de Configuração

A configuração é resolvida por um sistema em camadas:

1. **Padrões embutidos** — distribuídos com o runtime
2. **Padrões dos plugins** — declarados por cada plugin
3. **Config do projeto** — de `.ralph-engine/config.yaml`

Use `ralph-engine config layers` para inspecionar a cadeia completa de resolução.

## Diagnóstico

Exiba um relatório de diagnóstico:

```bash
ralph-engine doctor
```

Grave uma configuração corrigida em arquivo:

```bash
ralph-engine doctor apply-config config.yaml
```
