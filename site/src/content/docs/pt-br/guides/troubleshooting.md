---
title: "Solução de problemas"
description: "Problemas comuns ao usar o Ralph Engine e como resolvê-los"
---

## ralph-engine: command not found

A CLI não está no seu PATH. Reinstale ou verifique o método de instalação:

```bash
npm list -g ralph-engine
```

Se instalou via Cargo:

```bash
which ralph-engine
```

Certifique-se de que `~/.cargo/bin` (Cargo) ou o diretório global do npm está no seu `PATH`.

## doctor reporta problemas

Execute o comando doctor para ver o que precisa de atenção:

```bash
ralph-engine doctor
```

Para gerar automaticamente uma configuração corrigida:

```bash
ralph-engine doctor apply-config .ralph-engine/config.yaml
```

## Plugin não encontrado

Se um comando diz que o plugin não existe:

```bash
ralph-engine plugins list
```

Verifique se o ID do plugin está correto (ex: `official.claude`, não apenas `claude`).

## Servidor MCP não disponível

Verifique o status de todos os servidores MCP:

```bash
ralph-engine mcp status
```

Para um servidor específico:

```bash
ralph-engine mcp status <server-id>
```

Se um servidor SpawnProcess está indisponível, o binário necessário pode não estar no seu PATH.

## Configuração não carrega

Verifique se o arquivo de configuração existe e é válido:

```bash
ralph-engine config show-defaults
```

```bash
ralph-engine config layers
```

Confirme que `.ralph-engine/config.yaml` existe na raiz do seu projeto.

## Idioma errado

A CLI respeita o locale nesta ordem:
1. Flag `--locale` (ex: `--locale pt-br`)
2. Variável de ambiente `RALPH_ENGINE_LOCALE`
3. Locale do sistema (`LC_ALL`, `LC_MESSAGES`, `LANG`)
4. Inglês (padrão)

Para forçar português:

```bash
ralph-engine --locale pt-br --help
```

## Problemas no runtime

Liste todos os problemas não resolvidos detectados pelo runtime:

```bash
ralph-engine runtime issues
```

Veja o plano de remediação:

```bash
ralph-engine runtime plan
```
