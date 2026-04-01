# Ralph Engine

Ralph Engine is an open-source plugin-first runtime for agentic coding workflows.

This repository has been rebooted onto a Rust-first foundation. The core runtime and official plugins now evolve in Rust, while docs, site, and developer scaffolding keep the stacks that fit them best.

Public product surfaces are being prepared for bilingual operation in English and pt-BR, including the CLI, docs, and site.
Those public surfaces also follow a shared UX contract: consistent navigation, stable public paths, and A-grade accessibility, performance, and SEO targets.

## Repository shape

- `core/` — Rust crates for the runtime and CLI
- `plugins/official/` — Rust-first official plugins
- `docs/` — VitePress documentation
- `site/` — public web surfaces, shared UI, and plugin metadata
- `packaging/` — npm and Homebrew packaging surfaces
- `tools/create-ralph-engine/` — developer scaffolder
- `scripts/` — shared bootstrap, validation, and release scripts

## Development baseline

```bash
./scripts/bootstrap-dev.sh
./scripts/validate.sh --mode local
./scripts/validate-ci-local.sh
cargo test --workspace --all-targets --all-features
```

## Coding standards

- Public Rust APIs are documented with `rustdoc`
- Rust tests prefer Arrange, Act, Assert
- The repository enforces `fmt`, `clippy`, tests, coverage, `rustdoc`, `cargo deny`, `cargo audit`, docs build, and public-surface assembly from the same validation contract
- `./scripts/validate-ci-local.sh` provides a supported local smoke run for the GitHub Actions CI workflow when `act` is installed

## Release model

- SemVer 2.0.0
- Conventional Commits
- commitlint + lefthook
- release-plz opens release PRs from `main`
- merging the release PR creates the version tag
- tag workflows build release artifacts

## Status

This is the Rust-first reboot baseline. The release pipeline is being hardened on top of the new Rust foundation. npm now targets reviewed `cargo-dist` artifacts, and the Homebrew channel has a tracked formula template, but automatic publication remains disabled until GitHub Releases, npm, and Homebrew are connected end to end with provenance.
