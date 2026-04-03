---
title: "Hooks"
description: "Hooks de ciclo de vida para workflows de agente"
---


Hooks são os pontos de extensão onde plugins contribuem comportamento ao ciclo de vida do Ralph Engine.

## Hooks Disponíveis

Cada plugin declara quais hooks contribui. O runtime rastreia esses registros:

```bash
ralph-engine hooks list              # Listar todos os hooks registrados
ralph-engine hooks show <hook-id>    # Detalhes de um hook
ralph-engine hooks plan <hook-id>    # Plano de execução de um hook
```

## Tipos de Hook

| Hook | O que faz |
|------|-----------|
| `scaffold` | Scaffolding de projeto (materialização de templates) |
| `prepare` | Validação de pré-requisitos antes de workflows |
| `doctor` | Diagnóstico e verificação de saúde do sistema |
| `prompt_assembly` | Composição de fragmentos de prompt |
| `agent_bootstrap` | Inicialização de runtimes de agente |
| `mcp_registration` | Registro de servidores MCP |
| `data_source_registration` | Registro de fontes de dados |
| `context_provider_registration` | Registro de provedores de contexto |
| `forge_provider_registration` | Registro de provedores forge |
| `remote_control_bootstrap` | Inicialização de controle remoto |
| `policy_enforcement` | Enforcement de guardrails de policy |

## Execução de Hooks

Hooks são executados pelo trait `PluginRuntime`. Quando você roda comandos como `checks run prepare` ou `agents launch`, o runtime encaminha a execução para a implementação do plugin correspondente.

Plugins que fornecem um runtime podem responder com validação real, detecção de binários ou gerenciamento de processos — de acordo com suas capabilities.

## Arquivo de Hooks do Projeto

O template do plugin BMAD inclui um `.ralph-engine/hooks.yaml` para configuração de hooks no nível do projeto. Esse arquivo faz parte do scaffolding do template e é consumido pelo workflow BMAD, não diretamente pelo core do runtime.
