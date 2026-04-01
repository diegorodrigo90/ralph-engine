# AGENTS.md — Ralph Engine

Universal instructions for AI coding assistants working on Ralph Engine.
This file is the operational source of truth for the repository.

## Project

Ralph Engine is an open-source plugin-first runtime for agentic coding workflows.
It is being rebuilt on a Rust-first foundation as the core runtime of an agentic coding platform.

## Canonical Contracts

- `AGENTS.md` SHALL be the canonical assistant contract.
- `scripts/validate.sh` SHALL be the canonical validation contract for local work, CI, hooks, and release.
- `scripts/validate-ci-local.sh` SHALL be the supported local smoke check for GitHub Actions workflow behavior.
- `rust-toolchain.toml` SHALL pin the canonical Rust toolchain.
- `.tool-versions` SHALL pin the canonical asdf tool versions.
- GitHub Actions SHALL be pinned to full SHAs.
- Tooling installed by scripts SHALL use explicit reviewed versions, never `latest`.

## Golden Rules

1. Ralph Engine SHALL stay generic, configurable, and public-safe.
2. The core runtime and official plugins SHALL be implemented in Rust.
3. Third-party plugin contracts SHALL stay language-agnostic.
4. Tests SHALL be written before implementation. TDD is mandatory.
5. Core and official plugin code SHALL target 100% meaningful coverage.
6. `cargo fmt`, `clippy`, tests, coverage, deny, audit, rustdoc, docs build, CR, and quality gates SHALL be treated as mandatory, not optional.
7. Repository code, tests, and commit messages SHALL be in English.
8. Public-facing surfaces SHALL be designed for bilingual operation in English and pt-BR, including the CLI, docs, site, and plugins surface.
9. Site, docs, and plugins SHALL share a coherent UX system: clear menus, stable public paths, predictable navigation, and consistent brand language across the three surfaces.
10. Public surfaces SHALL target A-grade accessibility, performance, and SEO baselines through semantic HTML, strong contrast, low-friction navigation, lightweight assets, and metadata discipline.
11. Public Rust APIs SHALL use `rustdoc` comments. Public undocumented items SHALL fail the quality contract.
12. Rust tests SHALL follow the Arrange, Act, Assert structure when practical. AAA clarity SHALL be enforced through examples, CR, and repository rules.
13. Library code SHALL NOT use `unwrap`, `expect`, `panic!`, `todo!`, or `unimplemented!` outside tests.
14. Unsafe Rust SHALL be forbidden unless explicitly documented, isolated, and justified.
15. Modules, functions, traits, and structs SHALL stay small, explicit, and single-purpose.
16. DDD, SOLID, object calisthenics, early returns, strong typing, and clear names SHALL be applied where they improve maintainability in idiomatic Rust.
17. The repository SHALL optimize for low token cost and high signal: prompt/context control, MCP governance, and plugin contracts are core responsibilities.
18. Pre-1.0 cleanup MAY break compatibility when it improves the final architecture. Compatibility debt SHALL not block necessary refactors.
19. Selective validation MAY skip checks only when the changed files fit an explicit, reviewed safe scope. If the change set crosses domains, touches tooling, or falls outside a known-safe scope, validation SHALL fall back to the full contract.
20. CI, hooks, and local validation SHALL use the same selective-validation rules. The optimization SHALL be conservative: skip only for clearly docs-only or clearly Rust-only change sets; uncertainty SHALL resolve to full validation.
21. Local GitHub Actions simulation MAY be used to catch workflow failures before push, but it SHALL complement `scripts/validate.sh`, not replace it.

## Structure

- `core/` SHALL own the Rust runtime crates.
- `plugins/official/` SHALL own Rust-first official plugins.
- `docs/` SHALL remain a distinct top-level owned surface.
- `site/` SHALL own the public web surfaces, including `landing/`, `plugins/`, and `ui/`.
- `packaging/` SHALL own npm and Homebrew packaging.
- `tools/create-ralph-engine/` SHALL own developer scaffolding.
- `scripts/` SHALL own shared bootstrap, install, validation, and release scripts.

## Commands

```bash
./scripts/bootstrap-dev.sh
./scripts/install-dev-tools.sh
./scripts/validate.sh --mode local
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo llvm-cov --workspace --all-features --lcov --output-path coverage/lcov.info
cargo doc --workspace --no-deps
cargo deny check
cargo audit
```

## Release and Git Flow

- `main` SHALL stay releasable.
- Feature work SHALL happen on short-lived branches and merge through PRs.
- Conventional Commits SHALL be enforced by hooks and CI.
- release-plz SHALL manage version bumps, changelog updates, and tags.
- Merge to `main` SHALL update the release PR. Merging the release PR SHALL create the release tag.

## Documentation Sync

- `README.md`, `docs/`, and `llms.txt` SHALL be updated together when durable user-facing behavior changes.
- `docs/development/roadmap.md` SHALL stay strategic and current.
- `docs/development/backlog.md` SHALL stay tactical.
