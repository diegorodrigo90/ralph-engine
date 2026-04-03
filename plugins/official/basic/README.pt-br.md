# official.basic

Plugin base para templates iniciais.

## Visao geral

O plugin Basic oferece o ponto de partida padrao para novos projetos Ralph Engine. Ele inclui um unico template inicial que cria a estrutura minima do diretorio `.ralph-engine/` necessaria para integrar com qualquer assistente de codigo com IA.

## O que ele cria

Ao executar o comando materialize, ele cria:

- `.ralph-engine/config.yaml` — configuracao do projeto com lista de plugins, definicoes de checks e preferencias de agente
- `.ralph-engine/prompt.md` — conteudo de prompt especifico do projeto que e injetado nas sessoes do agente

Esses dois arquivos sao o contrato entre o seu projeto e o Ralph Engine. O config define o que roda; o prompt define o que o agente sabe sobre o seu projeto.

## Quando usar

Use este plugin quando:

- Estiver iniciando um novo projeto com Ralph Engine pela primeira vez
- Quiser uma configuracao minima e sem opiniao
- Pretender personalizar a configuracao por conta propria

Se preferir uma configuracao mais direcionada com integracao ao workflow BMAD, use o `official.bmad`.

## Configuracao

O `config.yaml` gerado inclui:

- Lista de plugins com `official.basic` habilitado por padrao
- Doctor checks habilitados para validacao de configuracao
- Runtime de agente padrao definido como `official.claude` (pode ser alterado para `official.codex` ou `official.claudebox`)

## Personalizacao

Apos materializar, edite o `.ralph-engine/config.yaml` para:

- Habilitar plugins adicionais
- Configurar limites de checks
- Definir o runtime de agente de sua preferencia
- Adicionar servidores MCP especificos do projeto
