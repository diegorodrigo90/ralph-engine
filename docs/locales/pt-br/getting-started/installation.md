# Instalação

O Ralph Engine está, neste momento, em um fluxo priorizando código-fonte.

Isso é intencional nesta fase do reboot em Rust: o próprio repositório é o contrato principal do runtime, dos plugins oficiais, das regras de validação e do pipeline de release.

## Pré-requisitos

A base atual assume:

- Git
- Rust `1.91.1`
- Node.js `20.19.0`
- `asdf` se você quiser o caminho mais simples para um ambiente local pinado

O repositório fixa a toolchain canônica por meio de:

- `rust-toolchain.toml`
- `.tool-versions`

## Instalação via código-fonte

```bash
git clone https://github.com/diegorodrigo90/ralph-engine.git
cd ralph-engine
./scripts/bootstrap-dev.sh
cargo run -p re-cli -- --version
```

`bootstrap-dev.sh` é o ponto de entrada suportado para setup local. Ele instala dependências do repositório, dependências das docs, hooks e o conjunto revisado de ferramentas que faz parte do contrato atual.

## O que rodar em seguida

Depois do bootstrap, os próximos comandos úteis são:

```bash
./scripts/validate.sh --mode local
cargo run -p re-cli
./scripts/validate-ci-local.sh
```

Use nessa ordem:

1. `validate.sh` prova que a base local está alinhada ao contrato do repositório.
2. `cargo run -p re-cli` confirma que a base atual da CLI em Rust está funcionando.
3. `validate-ci-local.sh` roda um smoke local do workflow do GitHub Actions quando `act` estiver instalado.

## Canais oficiais planejados

Esses canais continuam fazendo parte do contrato público do produto e estão sendo reconstruídos sobre a nova base em Rust:

- GitHub Releases
- npm
- Homebrew

Até eles estarem totalmente religados, o caminho canônico continua sendo via código-fonte.
