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

```bash
ralph-engine checks asset <check-id> <asset-path>
```

Exibir um asset específico do pacote embutido de uma verificação.

```bash
ralph-engine checks materialize <check-id> <output-dir>
```

Gravar o pacote de assets embutidos da verificação em um diretório de saída.

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

```bash
ralph-engine policies run <policy-id>
```

Executar o enforcement de uma policy. Retorna um resultado localizado de aprovação/reprovação.

```bash
ralph-engine policies asset <policy-id> <asset-path>
```

Exibir um asset específico do pacote embutido de uma policy.

```bash
ralph-engine policies materialize <policy-id> <output-dir>
```

Gravar o pacote de assets embutidos da policy em um diretório de saída.

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

## run

Executar itens de trabalho através do pipeline de workflow + plugin de agente.

```bash
ralph-engine run <id>
```

Resolve um item de trabalho, monta o prompt e lança o agente configurado. No primeiro uso, solicita aceitação do modo autônomo (salvo em `.ralph-engine/.accepted-autonomous`).

O fluxo de execução tem cinco etapas:

1. **Verificar agente** — verificar se o binário do agente está disponível
2. **Resolver item de trabalho** — o plugin de workflow localiza o item pelo ID
3. **Montar prompt** — reunir contexto da tarefa, contribuições de plugins e ferramentas descobertas
4. **Exibir info de lançamento** — mostrar o item de trabalho e o agente sendo utilizado
5. **Lançar agente** — iniciar o processo do agente com o prompt montado

```bash
ralph-engine run --list
```

Listar itens de trabalho acionáveis do plugin de workflow. Cada item exibe seu ID, título e status.

```bash
ralph-engine run plan <id>
```

Dry-run: resolver o item de trabalho e exibir o plano de execução sem lançar o agente. Mostra o plugin de workflow, plugin de agente, metadados do item de trabalho, tamanho do prompt, contagem de arquivos de contexto e status de prontidão do agente.

```bash
ralph-engine run --verbose <id>
ralph-engine run --verbose --list
ralph-engine run --verbose plan <id>
```

Habilitar logging de debug. Exibe detalhes passo a passo no stderr: resolução de config, seleção de runtime do plugin, resolução do item de trabalho, discovery de ferramentas, tamanho do prompt montado e resultados do processo do agente.

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
