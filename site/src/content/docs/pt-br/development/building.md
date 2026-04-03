---
title: "Compilação"
description: "Compile o Ralph Engine para contribuidores"
---

## Toolchain

- Rust 1.91.1 via `rust-toolchain.toml`
- Node.js 20.19.0 via `.tool-versions`

## Build de Debug

Compile todos os crates do workspace em modo debug:

```bash
cargo build --workspace
```

## Build de Release

Compile todos os crates do workspace com otimizações:

```bash
cargo build --workspace --release
```

## Testes

Execute a suíte completa de testes:

```bash
cargo test --workspace --all-targets
```

Execute os testes com todas as feature flags habilitadas:

```bash
cargo test --workspace --all-targets --all-features
```

Execute um teste específico por nome:

```bash
cargo test --workspace <test-name>
```

## Coverage

Gere o relatório de coverage em LCOV:

```bash
cargo llvm-cov --workspace --all-features --lcov --output-path coverage/lcov.info
```

## Linting

Verifique a formatação:

```bash
cargo fmt --all --check
```

Execute o clippy com warnings como erros:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Documentação

Compile o rustdoc com warnings como erros:

```bash
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```

## Auditoria de Dependências

Verifique vulnerabilidades conhecidas:

```bash
cargo audit
```

Verifique licenças e políticas de supply chain:

```bash
cargo deny check
```

## Verificação Cross-Language de Contratos

Verifique contratos de plugins entre fronteiras Rust e Node:

```bash
npm run contracts:verify
```

## Validação Completa

Execute o contrato completo de validação do repositório (formatação, linting, testes, coverage, docs, auditorias, contratos):

```bash
./scripts/validate.sh --mode local
```

Execute um smoke check equivalente ao CI localmente:

```bash
./scripts/validate-ci-local.sh
```
