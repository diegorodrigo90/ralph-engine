# Roadmap

Updated: 2026-04-02

## Done

- [x] Reboot the repository onto a Rust-first foundation.
- [x] Pin Rust, Node, commitlint, hooks, and core validation tooling.
- [x] Establish SemVer + Conventional Commits + release-plz as the release model.
- [x] Move the validation contract to repository-level scripts.
- [x] Establish the first `cargo-dist` workspace foundation for Rust release artifacts.
- [x] Introduce first-class bilingual support for the CLI, docs, site, and plugins surface in English and pt-BR.
- [x] Establish one coherent UX system across site, docs, and plugins with clear navigation and A-grade accessibility, performance, and SEO baselines.

## Next

- [ ] Rebuild the runtime core in Rust under TDD and 100% meaningful coverage.
- [ ] Reintroduce configuration, state, MCP, and plugin lifecycle on the new core.
- [ ] Wire npm and Homebrew publication to the Rust release pipeline.
- [ ] Harden `cargo-dist` publishing end to end with GitHub Releases, attestations, SBOMs, npm, and Homebrew.
- [ ] Rebuild the official plugins on the new Rust contracts.
