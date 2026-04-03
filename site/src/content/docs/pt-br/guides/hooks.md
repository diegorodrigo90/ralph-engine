---
title: "Hooks"
description: "Hooks de ciclo de vida para workflows de agente"
---

Runtime hooks são os pontos de extensão onde plugins contribuem comportamento ao ciclo de vida do Ralph Engine. Cada hook representa uma fase específica no runtime — desde o scaffolding de um novo projeto até a aplicação de policies durante uma sessão de agente.

Hooks não são chamados diretamente pelo usuário. Eles disparam automaticamente quando o runtime atinge a fase correspondente do ciclo de vida. Por exemplo, hooks `prepare` rodam antes de qualquer workflow iniciar, hooks `doctor` rodam durante diagnósticos, e hooks `agent_bootstrap` rodam ao lançar um agente.

## Hooks Disponíveis

Cada plugin declara quais hooks contribui. O runtime rastreia esses registros.

Liste todos os hooks registrados:

```bash
ralph-engine hooks list
```

Exiba detalhes de um hook específico:

```bash
ralph-engine hooks show <hook-id>
```

Exiba o plano de execução de um hook (quais plugins contribuem, em que ordem):

```bash
ralph-engine hooks plan <hook-id>
```

## Tipos de Hook

| Hook | Quando executa | O que faz |
|------|---------------|-----------|
| `scaffold` | Durante `templates scaffold` / `templates materialize` | Scaffolding de projeto (materialização de templates) |
| `prepare` | Antes de qualquer workflow iniciar (`checks run prepare`) | Validação de pré-requisitos |
| `doctor` | Durante diagnósticos com `doctor` | Verificações de saúde do sistema |
| `prompt_assembly` | Ao construir o prompt para uma sessão de agente | Composição de fragmentos de prompt |
| `agent_bootstrap` | Durante `agents launch` | Inicialização do runtime de agente |
| `mcp_registration` | Durante a inicialização do runtime | Registro de servidores MCP |
| `data_source_registration` | Durante a inicialização do runtime | Registro de provedores de fonte de dados |
| `context_provider_registration` | Durante a inicialização do runtime | Registro de provedores de contexto |
| `forge_provider_registration` | Durante a inicialização do runtime | Registro de automação forge |
| `remote_control_bootstrap` | Durante a inicialização do runtime | Inicialização de controle remoto |
| `policy_enforcement` | Durante sessões de agente | Enforcement de guardrails de policy |

## Execução de Hooks

Hooks são executados pelo trait `PluginRuntime`. Quando você roda comandos como `checks run prepare` ou `agents launch`, o runtime encaminha a execução para a implementação do hook no plugin correspondente.

Plugins que fornecem um runtime podem responder a invocações de hook com validação real, detecção de binários ou gerenciamento de processos — de acordo com suas capabilities.

## Arquivo de Hooks do Projeto

O template do plugin BMAD inclui um arquivo `.ralph-engine/hooks.yaml` para configuração de hooks no nível do projeto. Esse arquivo faz parte do scaffolding do template e é consumido pelo workflow BMAD, não diretamente pelo core do runtime.
