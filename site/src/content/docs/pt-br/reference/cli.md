---
title: "Comandos CLI"
description: "Referência completa de comandos CLI"
---

Todos os comandos aceitam `--locale <locale-id>` (ou `-L <locale-id>`) para trocar o idioma em uma única invocação. Sem esse flag, a CLI resolve o idioma a partir de `RALPH_ENGINE_LOCALE`, depois do locale do SO (`LC_ALL`, `LC_MESSAGES`, `LANG`), e por fim usa inglês como padrão.

## Opções Globais

Exibir ajuda:

```bash
ralph-engine --help
```

Exibir versão:

```bash
ralph-engine --version
```

Trocar idioma para uma invocação:

```bash
ralph-engine --locale <locale-id>
```

## agents

Gerenciar registros tipados de agent runtimes.

```bash
ralph-engine agents list
```

Listar todos os agent runtimes registrados.

```bash
ralph-engine agents show <agent-id>
```

Exibir o contrato tipado de um agent runtime.

```bash
ralph-engine agents plan <agent-id>
```

Exibir o plano executável de bootstrap de um agent runtime.

```bash
ralph-engine agents launch <agent-id>
```

Executar a implementação `PluginRuntime.bootstrap_agent()` do plugin. Para plugins agent-runtime (claude, claudebox, codex), verifica se o binário do agente existe no PATH.

## capabilities

Inspecionar o registro de capabilities do runtime.

```bash
ralph-engine capabilities list
```

Listar todas as capabilities registradas entre plugins.

```bash
ralph-engine capabilities show <capability-id>
```

Exibir detalhes de uma capability.

## checks

Gerenciar verificações de validação do runtime (em tempo de prepare e doctor).

```bash
ralph-engine checks list
```

Listar todas as verificações registradas.

```bash
ralph-engine checks show <check-id>
```

Exibir detalhes de uma verificação.

```bash
ralph-engine checks plan <check-id>
```

Exibir o plano de execução de uma verificação.

```bash
ralph-engine checks run <check-id>
```

Executar uma verificação contra a topologia resolvida. Retorna um resultado localizado de aprovação/reprovação com findings e ações de remediação.

## config

Inspecionar contratos de configuração.

```bash
ralph-engine config show-defaults
```

Exibir a configuração padrão completa do projeto (YAML).

```bash
ralph-engine config locale
```

Exibir o contrato padrão de locale.

```bash
ralph-engine config budgets
```

Exibir os limites de tokens de prompt e contexto.

```bash
ralph-engine config layers
```

Exibir a pilha de resolução de configuração.

```bash
ralph-engine config show-plugin <plugin-id>
```

Exibir a configuração resolvida de um plugin com proveniência.

```bash
ralph-engine config show-mcp-server <server-id>
```

Exibir a configuração resolvida de um servidor MCP com proveniência.

## doctor

Diagnosticar e remediar a configuração do projeto.

```bash
ralph-engine doctor
```

Exibir um relatório de diagnóstico compondo status do runtime, issues pendentes e ações de remediação.

```bash
ralph-engine doctor runtime
```

Exibir o componente de runtime do relatório de diagnóstico.

```bash
ralph-engine doctor config
```

Renderizar a configuração de projeto resultante da aplicação da remediação sobre os defaults atuais.

```bash
ralph-engine doctor apply-config <output-path>
```

Persistir a configuração de remediação em um arquivo.

```bash
ralph-engine doctor write-config <output-path>
```

Alias de compatibilidade para `doctor apply-config`.

## hooks

Inspecionar registros de hooks de ciclo de vida do runtime.

```bash
ralph-engine hooks list
```

Listar todos os hooks registrados.

```bash
ralph-engine hooks show <hook-id>
```

Exibir detalhes de um hook.

```bash
ralph-engine hooks plan <hook-id>
```

Exibir o mapa de superfícies de um hook (quais templates, prompts, agentes, checks, providers, policies e registros MCP ele controla).

## locales

Inspecionar o catálogo de locales suportados.

```bash
ralph-engine locales list
```

Listar todos os locales suportados com nomes nativos.

```bash
ralph-engine locales show <locale-id>
```

Exibir detalhes do locale incluindo nome nativo e regras de fallback para inglês.

## mcp

Gerenciar registros de servidores Model Context Protocol.

```bash
ralph-engine mcp list
```

Listar todos os servidores MCP registrados.

```bash
ralph-engine mcp show <server-id>
```

Exibir o contrato tipado de lançamento MCP (modelo de processo, policy de lançamento, fronteiras de comando, policy de diretório de trabalho, policy de ambiente, disponibilidade).

```bash
ralph-engine mcp plan <server-id>
```

Exibir o plano tipado de lançamento derivado do contrato.

```bash
ralph-engine mcp launch <server-id>
```

Validar e opcionalmente iniciar um servidor MCP. Servidores `SpawnProcess` são iniciados em foreground. Servidores `PluginRuntime` executam `register_mcp_server()` para verificar prontidão.

```bash
ralph-engine mcp status
```

Avaliar a prontidão de lançamento de todos os servidores MCP registrados (prontidão, saúde, estado de habilitação, transporte, issues, ações).

```bash
ralph-engine mcp status <server-id>
```

Exibir status detalhado de um servidor MCP específico.

## plugins

Inspecionar o registro de plugins.

```bash
ralph-engine plugins list
```

Listar todos os plugins registrados.

```bash
ralph-engine plugins show <plugin-id>
```

Exibir o contrato imutável do plugin (lifecycle, fronteira de carregamento, runtime hooks, estado de ativação resolvido).

## policies

Inspecionar registros de policies do runtime.

```bash
ralph-engine policies list
```

Listar todas as policies registradas.

```bash
ralph-engine policies show <policy-id>
```

Exibir detalhes de uma policy.

```bash
ralph-engine policies plan <policy-id>
```

Exibir o plano de enforcement de uma policy.

## prompts

Gerenciar registros de prompts do runtime.

```bash
ralph-engine prompts list
```

Listar todos os prompts registrados.

```bash
ralph-engine prompts show <prompt-id>
```

Exibir detalhes de um prompt.

```bash
ralph-engine prompts asset <prompt-id> <asset-path>
```

Exibir um asset específico do bundle embutido de um prompt.

```bash
ralph-engine prompts materialize <prompt-id> <output-dir>
```

Gravar o bundle de assets embutido de um prompt em um diretório de saída.

## providers

Inspecionar registros de providers do runtime (fontes de dados, provedores de contexto, provedores forge, controle remoto).

```bash
ralph-engine providers list
```

Listar todos os providers registrados.

```bash
ralph-engine providers show <provider-id>
```

Exibir detalhes de um provider.

```bash
ralph-engine providers plan <provider-id>
```

Exibir o plano de registro de um provider.

## runtime

Inspecionar e remediar a topologia resolvida do runtime.

```bash
ralph-engine runtime show
```

Exibir a topologia resolvida do runtime (ativação de plugins, registro de capabilities, registro de templates/prompts/agentes/checks/providers/policies/hooks, enablement de MCP).

```bash
ralph-engine runtime status
```

Exibir o resumo de saúde do runtime (contagens de habilitados/desabilitados em todos os tipos de registro).

```bash
ralph-engine runtime issues
```

Exibir issues pendentes do runtime e ações recomendadas.

```bash
ralph-engine runtime plan
```

Exibir o plano de remediação do runtime (passos de enablement para todos os tipos de provider).

```bash
ralph-engine runtime agent-plans
```

Exibir planos executáveis de bootstrap de agentes habilitados.

```bash
ralph-engine runtime provider-plans
```

Exibir planos executáveis de registro de providers habilitados.

```bash
ralph-engine runtime check-plans
```

Exibir planos executáveis de verificações habilitadas.

```bash
ralph-engine runtime policy-plans
```

Exibir planos executáveis de enforcement de policies habilitadas.

```bash
ralph-engine runtime mcp-plans
```

Exibir planos executáveis de lançamento MCP para servidores habilitados.

```bash
ralph-engine runtime patch
```

Renderizar o patch de configuração que remedia a topologia degradada atual.

```bash
ralph-engine runtime patched-config
```

Renderizar a configuração de projeto resultante da aplicação do patch sobre os defaults atuais.

```bash
ralph-engine runtime apply-config <output-path>
```

Persistir a configuração corrigida em um arquivo.

```bash
ralph-engine runtime write-patched-config <output-path>
```

Alias de compatibilidade para `runtime apply-config`.

## templates

Gerenciar registros de templates do runtime.

```bash
ralph-engine templates list
```

Listar todos os templates registrados.

```bash
ralph-engine templates show <template-id>
```

Exibir detalhes de um template.

```bash
ralph-engine templates asset <template-id> <asset-path>
```

Exibir um asset específico do bundle embutido de um template.

```bash
ralph-engine templates scaffold <template-id> <output-dir>
```

Gravar o bundle de assets embutido de um template em um diretório de saída. Alias para `templates materialize`.

```bash
ralph-engine templates materialize <template-id> <output-dir>
```

Gravar o bundle de assets embutido de um template em um diretório de saída.
