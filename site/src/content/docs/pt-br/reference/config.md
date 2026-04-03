---
title: "Referência de config"
description: "Todas as opções de configuração"
---


O sistema de configuração expõe contratos tipados pela CLI:

## Comandos

```bash
ralph-engine config show-defaults          # Config padrão completa (YAML)
ralph-engine config locale                 # Configuração de idioma
ralph-engine config budgets                # Limites de tokens de prompt e contexto
ralph-engine config layers                 # Pilha de resolução de configuração
ralph-engine config show-plugin <id>       # Config resolvida de um plugin
ralph-engine config show-mcp-server <id>   # Config resolvida de um servidor MCP
```

## Gerenciamento de Idiomas

```bash
ralph-engine locales list                  # Catálogo de idiomas suportados
ralph-engine locales show <locale-id>      # Detalhes do idioma (nome nativo, fallback)
```

O flag `--locale <id>` (ou `-L <id>`) troca o idioma para uma ��nica execução. Sem ele, a CLI resolve o idioma a partir de `RALPH_ENGINE_LOCALE`, depois do locale do SO (`LC_ALL`, `LC_MESSAGES`, `LANG`), e por fim usa inglês como padrão.

## Conteúdo da Configuração

| Seção | Descrição |
|-------|-----------|
| `schema_version` | Versão do schema de configuração |
| `default_locale` | Idioma padrão do projeto |
| `plugins` | Estado de ativação dos plugins (habilitado/desabilitado) |
| `mcp_servers` | Estado de ativação dos servidores MCP |
| `budgets` | Limites de tokens de prompt e contexto |

## Resolução de Configuração

A configuração é resolvida por uma pilha de camadas (inspecionável via `config layers`):

1. **Padrões embutidos** — compilados no binário
2. **Padrões dos plugins** — declarados por cada crate de plugin
3. **Config do projeto** — `.ralph-engine/config.yaml` (quando presente)

Use `ralph-engine doctor apply-config <caminho>` para gerar uma config corrigida que habilita todos os plugins e servidores recomendados.
