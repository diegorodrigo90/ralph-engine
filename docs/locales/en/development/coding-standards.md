# Coding Standards

Ralph Engine uses a strict Rust-first quality baseline, but the repository should still feel approachable to contributors who are newer to Rust or open source work.

These standards exist to keep code:

- clear to read
- easy to test
- easy to review
- consistent across the repository

If you come from TypeScript, Go, Java, or another language, read these notes as a translation guide for how Ralph Engine expects Rust code to feel in practice.

## Core rules

- Public Rust APIs use `rustdoc` comments with `///` or `//!`.
- Public undocumented items fail the repository lint contract.
- `cargo fmt`, `clippy`, tests, coverage, `rustdoc`, `cargo deny`, `cargo audit`, docs build, and public-surface assembly are mandatory.
- CI caches should be keyed by runner, toolchain, and lockfile inputs instead of using one blind global cache.
- Expensive checks should run once in the right job instead of being duplicated across the workflow graph.
- Cross-platform product behavior should be proven in the quality matrix, while platform-independent security scanners may stay centralized on the canonical Linux runner.
- `unsafe` is forbidden by default.
- `unwrap`, `expect`, `panic!`, `todo!`, and `unimplemented!` are forbidden in production code.

## Design rules

- Prefer clear names over clever names.
- Prefer small functions with one responsibility.
- Prefer strong types over stringly-typed contracts.
- Prefer early returns over deep nesting.
- Apply DDD, SOLID, and object calisthenics only where they improve maintainability in idiomatic Rust.
- Keep domain, application, and infrastructure concerns separated.

## How to interpret these principles in Rust

- DDD means clear domain boundaries, not ceremony.
- SOLID means small responsibilities and explicit contracts, often through focused traits and strong types.
- Object calisthenics means disciplined readability, not forcing Rust into classic OOP shapes.

In practice, prefer:

- structs with clear responsibilities
- enums for explicit states and outcomes
- small traits instead of wide interfaces
- helper functions with one job
- parsing, validation, and I/O separated from core business rules

## Test rules

Rust tests prefer the Arrange, Act, Assert structure.

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

AAA is a repository rule and review expectation. Where native linting is not available, Ralph Engine uses repository conventions, examples, and code review to keep tests readable and consistent.

The goal is simple: a contributor should be able to scan a test and understand setup, action, and assertion without guessing.

## Commands

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
