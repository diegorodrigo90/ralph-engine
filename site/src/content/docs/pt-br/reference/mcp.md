---
title: "MCP"
description: "Integração com Model Context Protocol"
---

O Ralph Engine expõe um contrato tipado de MCP (Model Context Protocol) no core do runtime. Cada servidor MCP é uma contribuição de plugin com um descritor estruturado.

## Descritor de Servidor MCP

Todo servidor MCP registrado carrega um contrato tipado que inclui:

- **Identificador do servidor** — ID único no escopo do plugin proprietário
- **Identificador do plugin proprietário** — qual plugin contribui este servidor
- **Transporte** — protocolo de comunicação (stdio, HTTP, etc.)
- **Policy de lançamento** — quando e como o servidor deve ser iniciado
- **Modelo de processo** — `SpawnProcess` (binário externo) ou `PluginRuntime` (in-process via trait do plugin)
- **Contrato de comando** — o binário e argumentos a executar (para servidores SpawnProcess)
- **Policy de diretório de trabalho** — onde o processo spawned executa
- **Policy de ambiente** — variáveis de ambiente passadas ao processo
- **Policy de disponibilidade** — condições sob as quais o servidor é considerado disponível

## Modelos de Processo

### SpawnProcess

O servidor é um binário externo. O Ralph Engine o inicia como processo filho, gerencia seu ciclo de vida e se comunica via o transporte declarado. O comando `mcp launch` o inicia em foreground se o binário for encontrado no PATH.

Caso de uso: encapsular ferramentas de terceiros (ex: `npx @anthropic/mcp-server-github`) onde o binário do servidor existe fora do processo do Ralph Engine.

### PluginRuntime

O servidor é implementado dentro do trait `PluginRuntime` de um plugin. O comando `mcp launch` executa `register_mcp_server()` para verificar prontidão em vez de iniciar um processo.

Caso de uso: servidores que fazem parte da implementação interna de um plugin, onde a prontidão depende da validação e estado do próprio plugin em vez de um binário externo.

## Comandos

Liste todos os servidores MCP registrados:

```bash
ralph-engine mcp list
```

Exiba o contrato completo de lançamento de um servidor:

```bash
ralph-engine mcp show <server-id>
```

Exiba o plano de lançamento derivado do contrato:

```bash
ralph-engine mcp plan <server-id>
```

Valide e opcionalmente inicie um servidor MCP:

```bash
ralph-engine mcp launch <server-id>
```

Verifique a prontidão de lançamento de todos os servidores MCP:

```bash
ralph-engine mcp status
```

Verifique o status detalhado de um servidor MCP:

```bash
ralph-engine mcp status <server-id>
```

## Integração com o Runtime

Servidores MCP fazem parte da topologia do runtime. Use `runtime mcp-plans` para ver todos os planos de lançamento MCP habilitados:

```bash
ralph-engine runtime mcp-plans
```

Use `runtime issues` para ver servidores MCP desabilitados ou com problemas:

```bash
ralph-engine runtime issues
```
