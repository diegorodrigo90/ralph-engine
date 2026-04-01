# Contributing to Ralph Engine

## Prerequisites

- Rust 1.91.1
- Node.js 20.19.0
- Git
- `asdf` is recommended but optional

## First setup

```bash
git clone https://github.com/diegorodrigo90/ralph-engine.git
cd ralph-engine
./scripts/bootstrap-dev.sh
./scripts/validate.sh --mode local
./scripts/validate-ci-local.sh
```

## Workflow

- Create a short-lived branch from `main`
- Follow TDD
- Keep code and docs aligned
- Document public Rust APIs with `rustdoc`
- Prefer Arrange, Act, Assert in tests
- Run the full validation contract before pushing
- When workflow changes are involved, run the local CI smoke check before pushing
- Open a PR with Conventional Commit messages
- Expect code review and quality gates before merge

## Quality contract

The repository contract is enforced through:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets --all-features`
- `cargo llvm-cov`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- `cargo deny check`
- `cargo audit`
- `npm --prefix docs run build`
- `./scripts/validate.sh --mode local`
- `./scripts/validate-ci-local.sh`

## Commit messages

Conventional Commits are mandatory.

Examples:

- `feat(core): add mcp registry foundations`
- `fix(plugin): reject duplicate capability ids`
- `docs(architecture): align rust workspace model`
- `build(ci): pin release-plz workflow`
