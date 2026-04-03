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

Executar `ralph-engine templates scaffold official.basic.starter .` cria um diretório `.ralph-engine/` com os arquivos de configuração do projeto:

- `.ralph-engine/config.yaml` — configuração do projeto
- `.ralph-engine/prompt.md` — conteúdo de prompt específico do projeto
- `.ralph-engine/hooks.yaml` — configuração de hooks (ao usar o plugin BMAD)

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
