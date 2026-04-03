---
title: "Referência de configuração"
description: "Todas as opções de configuração"
---

O sistema de configuração expõe contratos tipados pela CLI.

## Comandos

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

Exiba a pilha de resolução de configuração:

```bash
ralph-engine config layers
```

Exiba a configuração resolvida de um plugin específico com proveniência:

```bash
ralph-engine config show-plugin <id>
```

Exiba a configuração resolvida de um servidor MCP específico com proveniência:

```bash
ralph-engine config show-mcp-server <id>
```

## Gerenciamento de Idiomas

Liste todos os locales suportados:

```bash
ralph-engine locales list
```

Exiba detalhes de um locale específico (nome nativo, regras de fallback):

```bash
ralph-engine locales show <locale-id>
```

O flag `--locale <id>` (ou `-L <id>`) troca o idioma para uma única invocação. Sem ele, a CLI resolve o idioma a partir de `RALPH_ENGINE_LOCALE`, depois do locale do SO (`LC_ALL`, `LC_MESSAGES`, `LANG`), e por fim usa inglês como padrão.

## Conteúdo da Configuração

| Seção | Descrição |
|-------|-----------|
| `schema_version` | Versão do schema de configuração |
| `default_locale` | Idioma padrão do projeto |
| `plugins` | Estado de ativação dos plugins (habilitado/desabilitado por plugin) |
| `mcp_servers` | Estado de ativação dos servidores MCP |
| `budgets` | Limites de tokens de prompt e contexto |

## Resolução de Configuração

A configuração é resolvida por uma pilha de camadas (inspecionável via `config layers`):

1. **Padrões embutidos** — compilados no binário
2. **Padrões dos plugins** — declarados por cada crate de plugin
3. **Config do projeto** — `.ralph-engine/config.yaml` (quando presente)

Use `ralph-engine doctor apply-config <caminho>` para gerar uma configuração corrigida que habilita todos os plugins e servidores recomendados.
