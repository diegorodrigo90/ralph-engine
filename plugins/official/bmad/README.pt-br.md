# official.bmad

Plugin de workflow para scaffolding e prompts BMAD.

## Visao geral

Integracao completa com o BMAD Method para o Ralph Engine. O BMAD (Business-Method-Architecture-Development) e uma abordagem estruturada para desenvolvimento de software assistido por IA, com agentes especializados para cada fase: analise, planejamento, arquitetura, implementacao e revisao.

## O que ele inclui

- Um template inicial pre-configurado com agentes BMAD e definicoes de workflow
- Fragmentos de prompt que sao montados nas sessoes do agente para workflows BMAD
- Checks de prepare-time que validam se o projeto esta pronto para workflows BMAD
- Doctor checks que diagnosticam a saude da configuracao BMAD

## Como usar

Inicie um novo projeto BMAD:

```
ralph-engine templates materialize official.bmad.starter ./meu-projeto
```

Isso cria um diretorio `.ralph-engine/` com configuracao especifica do BMAD, incluindo definicoes de agentes, templates de workflow e bundles de prompt.

## Checks

O plugin inclui dois tipos de check:

- `official.bmad.prepare` — valida que os arquivos necessarios existem e a configuracao esta correta antes de iniciar um workflow
- `official.bmad.doctor` — diagnostico completo que verifica definicoes de agentes, integridade de prompts e consistencia de workflows

## Quando usar

Use este plugin quando:

- Quiser desenvolvimento assistido por IA estruturado com fases definidas
- Sua equipe seguir um workflow baseado em sprints com stories e criterios de aceitacao
- Precisar de multiplos agentes especializados (analista, arquiteto, PM, dev, QA)

Para uma configuracao mais simples sem BMAD, use o `official.basic`.
