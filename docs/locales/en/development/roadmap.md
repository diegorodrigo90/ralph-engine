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
- [x] Rebuild the public CLI and runtime inspection surface around typed Rust contracts for capabilities, templates, prompts, agents, checks, providers, policies, hooks, MCP, health, issues, and remediation plans.
- [x] Establish a typed shared locale contract in `re-config` so CLI, runtime crates, official plugins, and scaffolded plugins all grow locales from one canonical base with English fallback.
- [x] Harden the GitHub Actions pipeline around `Quality`, `Security`, `SonarCloud`, reusable approved release artifacts, and promote-later releases.

## Next

- [ ] Move the runtime from typed metadata and diagnostics into richer executable orchestration and state handling under TDD and 100% meaningful coverage.
- [ ] Rebuild official plugins beyond typed descriptors so they execute real behavior on top of the new Rust runtime contracts.
- [ ] Finish the end-to-end release pipeline for GitHub Releases, npm, and Homebrew with provenance, checksums, attestations, and final publish gates.
- [ ] Add explicit validation for generated release artifacts and `dist-workspace.toml`.
- [ ] Restore richer runtime examples in docs once the executable runtime layer is further along.
