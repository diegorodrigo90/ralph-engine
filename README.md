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
- `tools/create-ralph-engine/` — plugin scaffolder (`npx create-ralph-engine-plugin`)
- `scripts/` — shared bootstrap, validation, and release scripts

## Development baseline

```bash
./scripts/bootstrap-dev.sh
./scripts/validate.sh --mode local
./scripts/validate-ci-local.sh
npm run contracts:verify
cargo test --workspace --all-targets --all-features
```

## Coding standards

- Public Rust APIs are documented with `rustdoc`
- Rust tests prefer Arrange, Act, Assert
- Plugin and MCP extensibility stays typed: capabilities, lifecycle stages, runtime hooks, configuration scopes, launch policy, command boundaries, and future contributions are expected to evolve through shared contracts rather than string-coupled runtime branches
- Configuration layering stays typed and explicit: canonical defaults, future workspace settings, project settings, and user overrides are expected to evolve through shared layer contracts and inspectable CLI output rather than hidden precedence rules
- Prompt and context budgets stay typed and explicit: shared runtime budget contracts and CLI inspection are expected to carry token ceilings instead of scattering implicit defaults through providers
- Runtime registration stays typed: the resolved runtime topology, capability registry, template registry, prompt registry, agent registry, check registry, provider registry, policy registry, runtime-hook registry, runtime health, runtime issues, runtime action plans, and MCP contributions are expected to evolve through shared contracts rather than implicit command-local catalog traversal, and disabled capabilities, templates, prompt providers, agent runtimes, checks, providers, policies, or hooks remain visible in health plus remediation output
- Developer scaffolding stays tooling-owned: `tools/create-ralph-engine/` is the home of plugin scaffold generation through `npx create-ralph-engine-plugin`, while runtime surfaces stay focused on typed runtime contracts instead of turning scaffolding into a generic runtime responsibility
- The plugin scaffolder only accepts kinds and capabilities that the typed runtime already defines; future surfaces stay rejected until the core contracts exist for them
- Cross-language plugin contracts are verified explicitly so Rust capability contracts and `create-ralph-engine-plugin` supported surfaces cannot drift silently
- Runtime diagnostics stay typed: doctor-style reporting is expected to compose status, unresolved issues, and remediation actions from one shared runtime snapshot instead of re-deriving them ad hoc in separate commands
- The repository enforces `fmt`, `clippy`, tests, coverage, `rustdoc`, `cargo deny`, `cargo audit`, docs build, and public-surface assembly from the same validation contract
- SonarCloud is configured as the final coverage gate for analyzed code, and the release path is blocked unless that gate stays at `100%`
- `./scripts/validate-ci-local.sh` provides a supported local smoke run for the GitHub Actions CI workflow when `act` is installed

## Release model

- SemVer 2.0.0
- Conventional Commits
- commitlint + lefthook
- release-plz opens release PRs from `main`
- merging the release PR prepares the versioned release state
- the canonical `CI` workflow on `main` builds cross-platform release candidates in parallel and only publishes reusable approved release artifacts for the SHA after `Quality`, `Security`, and `SonarCloud` have passed
- the manual hardened release workflow verifies green CI for the current `main` SHA and promotes those same artifacts instead of rebuilding them
- the public Pages deployment publishes from published releases and builds from the release tag so docs and site stay aligned with published releases

## Status

This is the Rust-first reboot baseline. The release pipeline is being hardened on top of the new Rust foundation. npm now targets reviewed `cargo-dist` artifacts, the Homebrew channel has a tracked formula template, and the release flow now follows a build-once promote-later model: `CI` builds reusable artifacts for each approved `main` SHA, and the manual publish workflow promotes that exact artifact set. Automatic publication still remains disabled until GitHub Releases, npm, and Homebrew are connected end to end with provenance.
