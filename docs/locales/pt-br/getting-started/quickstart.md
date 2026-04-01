# Início rápido

Se você quer o caminho mais curto entre o clone e uma base local validada, use esta sequência:

```bash
./scripts/bootstrap-dev.sh
./scripts/validate.sh --mode local
cargo run -p re-cli
```

## Por que essa ordem importa

- `bootstrap-dev.sh` instala o ambiente pinado esperado pelo repositório.
- `validate.sh --mode local` prova que o projeto está saudável antes de você começar a mudar alguma coisa.
- `cargo run -p re-cli` confirma que a base atual da CLI em Rust está funcionando.

## Depois da primeira execução

Os próximos caminhos úteis dependem do que você quer fazer:

- Leia [Arquitetura](../reference/architecture.md) se quiser começar pelo modelo do sistema.
- Leia [Plugins](../guides/plugins.md) se quiser entender a superfície de extensão.
- Leia [Padrões de código](../development/coding-standards.md) se você vai contribuir com código.
- Leia [Roadmap](../development/roadmap.md) se quiser entender a direção atual, não só a implementação do momento.

## Opções de validação local

Para o fluxo normal de desenvolvimento:

```bash
./scripts/validate.sh --mode local
```

Para um smoke local do workflow do GitHub Actions:

```bash
./scripts/validate-ci-local.sh
```

O smoke do GitHub Actions complementa o contrato principal de validação. Ele não substitui esse contrato.
