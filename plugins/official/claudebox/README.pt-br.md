# official.claudebox

Runtime Claude Box e integracao com sessoes MCP.

## Visao geral

Integra o Claude Box — ambiente isolado (sandbox) do Claude pela Anthropic — como runtime de agente. O Claude Box roda em um container Docker isolado com acesso ao sistema de arquivos, automacao de navegador via Playwright e capacidades de sudo, sem afetar o seu sistema host.

## Como funciona

1. `ralph-engine agents launch official.claudebox.session` procura o binario `claudebox`
2. Se encontrado, inicia uma sessao isolada com o projeto montado como volume compartilhado
3. O agente tem acesso completo ao sistema dentro do container (instalar pacotes, rodar servidores, usar navegadores)
4. Servidores MCP sao registrados e ficam disponiveis dentro do ambiente isolado

## Requisitos

- Claude Box instalado e no seu PATH (comando `claudebox` disponivel)
- Docker rodando (o Claude Box usa containers para isolamento)
- Um projeto Ralph Engine valido

## Quando usar

Use este plugin quando:

- Precisar que o agente instale pacotes do sistema, execute comandos Docker ou acesse navegadores
- Quiser acesso completo ao sistema sem arriscar o ambiente host
- Seu workflow exigir desenvolvimento isolado no estilo DevContainer

Para sessoes Claude padrao sem sandbox, use o `official.claude`.
