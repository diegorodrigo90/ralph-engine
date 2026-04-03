---
title: "Padrões de código"
description: "Estilo de código e regras de qualidade para contribuidores"
---

O Ralph Engine usa uma base de qualidade rígida em Rust, mas o repositório também precisa continuar acolhedor para quem está entrando em Rust ou em open source.

Estas regras existem para manter o código:

- claro de ler
- fácil de testar
- fácil de revisar
- consistente em todo o repositório

Se você vem de TypeScript, Go, Java ou outra linguagem, leia esta página como um guia de tradução para entender como esperamos que o código Rust do Ralph Engine se comporte na prática.

## Regras Centrais

- APIs públicas em Rust usam comentários `rustdoc` com `///` ou `//!`.
- Itens públicos sem documentação falham no contrato de lint do repositório.
- `cargo fmt`, `clippy`, testes, coverage, `rustdoc`, `cargo deny`, `cargo audit`, verificação cross-language de contratos de plugins, build de docs e montagem de superfícies públicas são obrigatórios.
- Crates de plugins oficiais devem possuir os testes de contrato mais próximos localmente: consistência do descriptor, alinhamento do manifesto e comportamento localizado de contribuições devem falhar dentro do próprio crate antes que uma camada de smoke compartilhada detecte a divergência.
- Manifestos de plugins oficiais devem localizar toda contribuição pública que entregam. Templates, prompts, agentes, checks, providers e policies devem manter `display_name_locales` e `summary_locales` alinhados com todos os locales suportados, em vez de depender de revisão manual.
- Os caches da CI devem ser indexados por runner, toolchain e lockfiles relevantes, em vez de usar um cache global cego.
- Etapas caras devem rodar uma vez no job correto, não ficar duplicadas no grafo do workflow.
- O comportamento cross-platform do produto deve ser provado na matriz de qualidade, enquanto scanners de segurança independentes de plataforma podem ficar centralizados no runner Linux canônico.
- `unsafe` é proibido por padrão.
- `unwrap`, `expect`, `panic!`, `todo!` e `unimplemented!` são proibidos em código de produção.

## Regras de Design

- Prefira nomes claros a nomes espertos.
- Prefira funções pequenas com uma responsabilidade.
- Prefira tipos fortes a contratos baseados em string.
- Prefira retornos antecipados a aninhamento profundo.
- Aplique DDD, SOLID e object calisthenics somente quando isso melhorar a manutenibilidade em Rust idiomático.
- Mantenha domínio, aplicação e infraestrutura bem separados.

## Como Interpretar Esses Princípios em Rust

- DDD significa fronteiras claras de domínio, não burocracia.
- SOLID significa responsabilidades pequenas e contratos explícitos, frequentemente por meio de traits enxutas e tipos fortes.
- Object calisthenics significa disciplina de legibilidade, não forçar Rust a parecer OOP clássica.

Na prática, prefira:

- structs com responsabilidades claras
- enums para estados e resultados explícitos
- traits pequenas em vez de interfaces largas
- funções auxiliares com um único papel
- parsing, validação e I/O separados das regras centrais de negócio

## Regras de Teste

Testes em Rust preferem a estrutura Arrange, Act, Assert.

- Testes de contrato em crates compartilhadas devem preferir fixtures sintéticas e neutras em vez de depender de identificadores de plugins oficiais quando o comportamento testado é genérico.
- Cada crate de plugin oficial deve possuir os testes mais próximos do seu próprio manifesto, metadata localizada e detalhes de contribuição.
- Verificações contratuais de plugins oficiais devem falhar quando uma contribuição publicada divergir do conjunto de locales suportados ou perder seu namespace próprio.
- Testes de integração e smoke podem continuar exercitando o catálogo oficial empacotado quando o objetivo for validar o runtime público distribuído, não um contrato compartilhado genérico.

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

AAA é uma regra do repositório e uma expectativa de revisão. Onde linting nativo não está disponível, o Ralph Engine usa convenções, exemplos e code review para manter os testes legíveis e consistentes.

O objetivo é simples: alguém deve conseguir bater o olho num teste e entender rapidamente o preparo, a ação e a verificação, sem precisar adivinhar.

## Comandos

Execute o contrato completo de validação:

```bash
./scripts/validate.sh --mode local
```

Verifique contratos cross-language de plugins:

```bash
npm run contracts:verify
```

Verifique a formatação:

```bash
cargo fmt --all --check
```

Execute o clippy:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Execute todos os testes:

```bash
cargo test --workspace --all-targets --all-features
```

Gere coverage:

```bash
cargo llvm-cov --workspace --all-features --lcov --output-path coverage/lcov.info
```

Compile o rustdoc:

```bash
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```

Verifique licenças e vulnerabilidades de dependências:

```bash
cargo deny check
```

```bash
cargo audit
```
