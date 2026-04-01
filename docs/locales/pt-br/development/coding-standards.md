# Padrões de código

O Ralph Engine usa uma base em Rust com verificações rígidas de qualidade, mas o repositório também precisa continuar acolhedor para quem ainda está entrando em Rust ou em open source.

Estas regras existem para manter o código:

- claro de ler
- fácil de testar
- fácil de revisar
- consistente em todo o repositório

Se você vem de TypeScript, Go, Java ou outra linguagem, leia esta página como um guia de tradução para entender como esperamos que o código Rust do Ralph Engine se comporte na prática.

## Regras centrais

- APIs públicas em Rust usam comentários `rustdoc` com `///` ou `//!`.
- Itens públicos sem documentação falham no contrato de lint do repositório.
- `cargo fmt`, `clippy`, testes, cobertura, `rustdoc`, `cargo deny`, `cargo audit`, build das docs e montagem das superfícies públicas são obrigatórios.
- Os caches da CI devem ser indexados por runner, toolchain e lockfiles relevantes, em vez de usar um cache global cego.
- Etapas caras devem rodar uma vez no job certo, não ficar duplicadas ao longo do workflow.
- O comportamento cross-platform do produto deve ser provado na matriz de quality, enquanto scanners de segurança independentes de plataforma podem ficar centralizados no runner Linux canônico.
- `unsafe` é proibido por padrão.
- `unwrap`, `expect`, `panic!`, `todo!` e `unimplemented!` são proibidos em código de produção.

## Regras de design

- Prefira nomes claros a nomes espertos.
- Prefira funções pequenas com uma responsabilidade.
- Prefira tipos fortes a contratos baseados em string.
- Prefira retornos antecipados a aninhamento profundo.
- Aplique DDD, SOLID e object calisthenics só quando isso realmente melhorar a manutenibilidade em Rust idiomático.
- Separe bem domínio, aplicação e infraestrutura.

## Como interpretar esses princípios em Rust

- DDD aqui significa fronteiras claras de domínio, não burocracia.
- SOLID aqui significa responsabilidades pequenas e contratos explícitos, muitas vezes por meio de traits enxutas e tipos fortes.
- Object calisthenics aqui significa disciplina de legibilidade, não tentar fazer Rust parecer OOP clássica.

Na prática, prefira:

- structs com responsabilidades claras
- enums para estados e resultados explícitos
- traits pequenas em vez de interfaces largas
- funções auxiliares com um único papel
- parsing, validação e I/O separados das regras centrais

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

O objetivo é simples: alguém precisa bater o olho num teste e entender rapidamente o preparo, a ação e a verificação final, sem precisar adivinhar a intenção.

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
