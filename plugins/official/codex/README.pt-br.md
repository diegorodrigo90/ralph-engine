# official.codex

Runtime Codex e integracao com sessoes MCP.

## Visao geral

Integra o Codex CLI da OpenAI como runtime de agente para workflows do Ralph Engine. Este plugin permite que equipes que usam Codex tenham a mesma experiencia do Ralph Engine — plugins, checks, montagem de prompt e servidores MCP — independente de qual assistente de codigo com IA preferem.

## Como funciona

1. `ralph-engine agents launch official.codex.session` procura o binario `codex`
2. Se encontrado, carrega a configuracao do projeto e inicializa uma sessao Codex
3. Todos os plugins habilitados, checks e fragmentos de prompt ficam disponiveis para o agente
4. Servidores MCP definidos na configuracao sao registrados para a sessao

## Requisitos

- Codex CLI instalado e no seu PATH (comando `codex` disponivel)
- Um projeto Ralph Engine valido

## Quando usar

Use este plugin quando:

- Sua equipe usar o OpenAI Codex como assistente de codigo principal
- Quiser alternar entre Claude e Codex sem mudar a configuracao do projeto
- Estiver avaliando diferentes assistentes de IA e quiser uma camada de workflow consistente

O Ralph Engine e agnostico em relacao ao agente — a mesma configuracao funciona com Claude, Claude Box ou Codex.
