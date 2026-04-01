# Padrões de código

O Ralph Engine usa uma base em Rust com verificações rígidas de qualidade.

## Regras centrais

- APIs públicas em Rust usam comentários `rustdoc` com `///` ou `//!`.
- Itens públicos sem documentação falham no contrato de lint do repositório.
- `cargo fmt`, `clippy`, testes, cobertura, `rustdoc`, `cargo deny`, `cargo audit`, build das docs e montagem das superfícies públicas são obrigatórios.
- `unsafe` é proibido por padrão.
- `unwrap`, `expect`, `panic!`, `todo!` e `unimplemented!` são proibidos em código de produção.

## Regras de design

- Prefira nomes claros a nomes espertos.
- Prefira funções pequenas com uma responsabilidade.
- Prefira tipos fortes a contratos baseados em string.
- Prefira retornos antecipados a aninhamento profundo.
- Aplique DDD, SOLID e object calisthenics quando melhorarem a manutenibilidade em Rust idiomático.
- Separe bem domínio, aplicação e infraestrutura.

## Regras de teste

Testes em Rust preferem a estrutura Arrange, Act, Assert.

```rust
#[test]
fn example() {
    // Arrange
    let input = "value";

    // Act
    let output = do_work(input);

    // Assert
    assert_eq!(output, "expected");
}
```

AAA é uma regra do repositório e também uma expectativa de revisão.

## Comandos

```bash
./scripts/validate.sh --mode local
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo llvm-cov --workspace --all-features --lcov --output-path coverage/lcov.info
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo deny check
cargo audit
```
