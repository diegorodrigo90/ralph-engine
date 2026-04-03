# official.claude

Runtime de agente Claude e integracao com sessoes MCP.

## Visao geral

Integra o Claude da Anthropic como runtime de agente para o Ralph Engine. Ao iniciar uma sessao, este plugin detecta o CLI `claude` no seu PATH, valida se esta pronto e inicializa uma sessao de codificacao com a configuracao do projeto pre-carregada.

## Como funciona

1. `ralph-engine agents launch official.claude.session` procura o binario do Claude CLI
2. Se encontrado, carrega o `config.yaml` e o `prompt.md` do seu projeto em `.ralph-engine/`
3. A sessao do agente inicia com todos os plugins habilitados, checks e fragmentos de prompt disponiveis
4. Servidores MCP definidos na sua configuracao sao registrados e ficam disponiveis para o agente

## Requisitos

- Claude CLI instalado e no seu PATH (comando `claude` disponivel)
- Um projeto Ralph Engine valido (`.ralph-engine/config.yaml` existente)

## Contribuicao MCP

Este plugin tambem disponibiliza um servidor MCP que os agentes podem usar para gerenciamento de sessoes — iniciar, parar e consultar sessoes ativas.

## Quando usar

Este e o runtime de agente padrao para a maioria dos projetos Ralph Engine. Se voce usa Claude Code ou Claude Desktop como assistente de codigo com IA, este e o plugin que voce quer.

Para ambientes isolados (sandbox), use o `official.claudebox`. Para usuarios do OpenAI Codex, use o `official.codex`.
