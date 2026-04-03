# official.tdd-strict

Politica rigorosa de TDD e guardrails de template.

## Visao geral

Impoe disciplina rigorosa de Test-Driven Development em projetos Ralph Engine. Este plugin combina um template inicial pre-configurado com guardrails de TDD e uma politica que valida se o ciclo RED-GREEN-REFACTOR esta sendo seguido para cada criterio de aceitacao.

## O que ele impoe

A politica TDD strict valida que:

- Os testes sao escritos antes da implementacao (fase RED)
- Cada criterio de aceitacao tem pelo menos um teste correspondente
- Os nomes dos testes seguem convencoes de nomenclatura
- O ciclo RED-GREEN-REFACTOR esta documentado no registro de desenvolvimento

## Como usar

Inicie um projeto TDD-strict:

```
ralph-engine templates materialize official.tdd-strict.starter ./meu-projeto
```

Ou habilite a politica em um projeto existente adicionando `official.tdd-strict` na lista de plugins do `.ralph-engine/config.yaml`.

## Template vs Politica

Este plugin inclui duas coisas:

- **Template inicial** (`official.tdd-strict.starter`) — cria um projeto pre-configurado com guardrails de TDD habilitados
- **Politica** (`official.tdd-strict.guardrails`) — as regras de aplicacao que podem ser adicionadas a qualquer projeto existente

Voce pode usar o template para projetos novos, ou simplesmente habilitar a politica em um projeto existente que usa `official.basic` ou `official.bmad` como base.

## Quando usar

Use este plugin quando:

- Sua equipe pratica TDD e quer aplicacao automatizada das regras
- Quiser garantir que codigo gerado por IA sempre tenha testes escritos primeiro
- O code review deve verificar que existem testes para cada criterio de aceitacao
- Quiser impedir que o "vou adicionar testes depois" aconteca

## Combinando com outros plugins

O TDD strict funciona junto com qualquer runtime de agente (Claude, Codex, Claude Box) e junto com o plugin de workflow BMAD. A aplicacao da politica acontece no momento dos checks, nao no nivel do agente.
