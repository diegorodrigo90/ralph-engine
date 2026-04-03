# ADR-001: Bootstrap script over DevContainer

**Date:** 2026-04-03
**Status:** Accepted
**Context:** Evaluate whether a DevContainer setup makes sense for Ralph Engine development.

## Decision

Use a bootstrap script (`scripts/setup.sh`), not a DevContainer.

## Rationale

- RE is a small Rust + Node project with no databases or services to orchestrate
- Zero major Rust CLI projects (ripgrep, bat, fd, nushell, starship) use DevContainers
- Rust's toolchain (rustup + cargo) is already self-contained and cross-platform
- DevContainer adds Docker overhead with no benefit for this project type
- Rust compilation in containers is 10-30% slower due to I/O overhead on bind mounts
- A bootstrap script is editor-agnostic (works with VS Code, Cursor, Neovim, JetBrains)

## What the bootstrap script should install

- `rustup` (if missing) — handles the Rust toolchain
- `cargo-binstall` — downloads pre-built binaries instead of compiling
- `cargo-nextest` — faster test runner
- `cargo-watch` — auto-rebuild on file changes
- Node.js via `fnm` — for the site build (Astro + Starlight)
- `lefthook` — git hooks

## When to reconsider

Only if RE grows to need databases, message queues, or other services for development. Unlikely for a CLI tool.

## Research sources

- [Microsoft Rust DevContainer Image](https://hub.docker.com/r/microsoft/devcontainers-rust)
- [Tips for Faster Rust Compile Times](https://corrode.dev/blog/tips-for-faster-rust-compile-times/)
- [ripgrep](https://github.com/BurntSushi/ripgrep), [bat](https://github.com/sharkdp/bat), [nushell](https://github.com/nushell/nushell) — none use DevContainers
