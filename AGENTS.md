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
6. The SonarCloud quality gate SHALL enforce 100% coverage for the analyzed code, and CI SHALL NOT approve reusable release artifacts or allow publication when that gate fails.
7. `cargo fmt`, `clippy`, tests, coverage, deny, audit, rustdoc, docs build, CR, and quality gates SHALL be treated as mandatory, not optional.
8. Repository code, tests, and commit messages SHALL be in English.
9. Public-facing surfaces SHALL be designed for bilingual operation in English and pt-BR, including the CLI, docs, site, and plugins surface.
10. Site, docs, and plugins SHALL share a coherent UX system: clear menus, stable public paths, predictable navigation, and consistent brand language across the three surfaces.
11. Public surfaces SHALL target A-grade accessibility, performance, and SEO baselines through semantic HTML, strong contrast, low-friction navigation, lightweight assets, and metadata discipline.
12. Public Rust APIs SHALL use `rustdoc` comments. Public undocumented items SHALL fail the quality contract.
13. Rust tests SHALL follow the Arrange, Act, Assert structure when practical. AAA clarity SHALL be enforced through examples, CR, and repository rules.
14. Library code SHALL NOT use `unwrap`, `expect`, `panic!`, `todo!`, or `unimplemented!` outside tests.
15. Unsafe Rust SHALL be forbidden unless explicitly documented, isolated, and justified.
16. Modules, functions, traits, and structs SHALL stay small, explicit, and single-purpose.
17. DDD, SOLID, object calisthenics, early returns, strong typing, and clear names SHALL be applied where they improve maintainability in idiomatic Rust.
18. The repository SHALL optimize for low token cost and high signal: prompt/context control, MCP governance, and plugin contracts are core responsibilities.
19. The CLI SHALL stay modular. New command families or behaviors SHALL be introduced through isolated command modules or registries rather than by growing one central dispatcher function.
20. Plugin and MCP capabilities SHALL remain extensible by contract. Adding a new capability or contribution SHALL be possible through local module changes and typed descriptors rather than string parsing spread across the runtime.
21. Plugin lifecycle SHALL remain typed and explicit. Discovery, configuration, validation, loading, and future lifecycle stages SHALL evolve through shared contracts instead of ad hoc booleans or scattered command-specific checks.
22. Plugin runtime hooks SHALL remain typed and explicit. Prepare, doctor, prompt, agent, MCP, policy, and future hook surfaces SHALL evolve through shared contracts instead of capability-specific strings or ad hoc dispatch.
23. Configuration resolution SHALL remain typed and layered. Built-in defaults, workspace settings, project settings, user overrides, prompt and context budgets, and future scopes SHALL evolve through shared contracts instead of implicit precedence rules spread across commands.
24. Typed configuration layers SHALL remain inspectable. The CLI and docs SHALL expose the canonical layer stack and resolved plugin configuration so configuration precedence stays visible instead of being inferred from implementation details.
25. Runtime registration and state orchestration SHALL remain typed and explicit. Plugin activation, capability registration, check registration, provider registration, policy registration, runtime-hook registration, runtime health, runtime issues, runtime doctor reporting, runtime action plans, runtime topology, MCP enablement, and future runtime state transitions SHALL evolve through shared contracts instead of implicit catalog traversal in command handlers. Disabled capabilities, disabled checks, disabled providers, disabled policies, and disabled runtime hooks SHALL remain visible in runtime health and remediation output; they SHALL NOT be treated as invisible metadata.
26. Pre-1.0 cleanup MAY break compatibility when it improves the final architecture. Compatibility debt SHALL not block necessary refactors.
27. Selective validation MAY skip checks only when the changed files fit an explicit, reviewed safe scope. If the change set crosses domains, touches tooling, or falls outside a known-safe scope, validation SHALL fall back to the full contract.
28. CI, hooks, and local validation SHALL use the same selective-validation rules. The optimization SHALL be conservative: skip only for clearly public-surface-only or clearly Rust-only change sets; uncertainty SHALL resolve to full validation.
29. Local GitHub Actions simulation MAY be used to catch workflow failures before push, but it SHALL complement `scripts/validate.sh`, not replace it.
30. CI caches SHALL be keyed and scoped by the inputs that actually affect correctness, including operating system, toolchain, dependency lockfiles, and job purpose. Broad blind caches SHALL be avoided.
31. Cache strategy SHALL optimize by domain where it improves reuse without increasing drift, such as separate dependency caches for repository Node tooling, docs tooling, and Rust build artifacts.
32. Workflows SHALL avoid duplicate heavy work across jobs. Expensive steps such as coverage generation, scanner installs, and release-only tooling SHALL run only in the jobs that need them.
33. Cross-platform quality SHALL be proven through an OS matrix, while platform-independent security scanners MAY run once on a canonical runner when that avoids duplicated cost without reducing coverage.
34. CI workflows SHALL cancel superseded in-progress runs for the same branch or pull request whenever the older run no longer provides unique value.
35. SonarCloud configuration SHALL fail fast with a clear preflight error when the configured token cannot browse or analyze the target project.
36. SonarCloud scans SHALL resolve the project key and organization from `sonar-project.properties` and pass them explicitly to the scanner so CI logs and behavior stay unambiguous.
37. Coverage used by SonarCloud SHALL be generated once in the canonical Linux quality job, uploaded as an artifact, and reused by the SonarCloud job instead of rerunning test coverage.
38. The hardened release workflow SHALL verify that the target SHA is the current `origin/main` head and that the canonical `CI` workflow has already completed successfully for that exact push before any tag or publication step begins.
39. GitHub Actions checkouts SHALL disable persisted credentials unless a later step in that same job explicitly needs to push or publish.
40. The canonical `CI` workflow on `main` SHALL build cross-platform release candidate artifacts in parallel with the code-quality gates for the same SHA, and SHALL publish reusable approved release artifacts only after `Quality`, `Security`, and `SonarCloud` have all passed. The publish workflow SHALL promote those approved artifacts instead of rebuilding them.
41. Reviewed pinned tool binaries that are installed by repository scripts MAY be cached in CI only when the cache key stays scoped by operating system, installer definition, and job purpose. Tool caches SHALL NOT be shared blindly across unrelated jobs or platforms.
42. Workflows SHALL avoid no-op cache restores and unnecessary setup steps. If a job does not install or consume a dependency set, it SHALL NOT restore that cache just for symmetry.
43. Matrix fail-fast behavior SHALL match the purpose of the matrix. Quality matrices SHOULD keep `fail-fast: false` to surface cross-platform regressions in one run, while release-artifact matrices SHOULD keep `fail-fast: true` because one failed platform already invalidates the publishable set.
44. Pages publication SHALL happen from published releases and SHALL build from the release tag so the public site, docs, and plugins surface reflect published versions rather than unreleased `main` state.

## Structure

- `core/` SHALL own the Rust runtime crates.
- `core/crates/re-core/` SHALL own shared runtime foundations.
- `core/crates/re-config/` SHALL own typed runtime configuration contracts and defaults.
- `core/crates/re-mcp/` SHALL own typed MCP contribution contracts, including launch policy, process model, command boundaries, working-directory policy, environment policy, and availability policy.
- `core/crates/re-plugin/` SHALL own typed plugin metadata, lifecycle, runtime-hook, loading-boundary, and capability contracts.
- `core/crates/re-cli/` SHALL own the modular CLI surface and command registry.
- `re-core` and `re-cli` SHALL expose typed runtime capability registration so new capabilities can be added through shared contracts instead of command-local branching.
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

For public-surface-only change sets, the `public` validation step SHALL cover both:

- `npm --prefix docs run build`
- `./scripts/assemble-public-surfaces.sh .site-dist`

CI cache design SHALL follow these rules:

- Rust build caches SHALL stay runner-specific.
- Node dependency caches SHALL stay lockfile-specific.
- Shared caches MAY span jobs only when the runner platform and toolchain remain compatible.
- Reviewed pinned tool caches SHALL stay purpose-specific, such as separate caches for coverage tooling, security tooling, and release-only tooling.
- Cache misses SHALL degrade safely to fresh installs; they SHALL NOT change validation behavior.
- Cross-platform correctness SHALL be checked in the quality matrix.
- Fail-fast SHOULD be used to stop matrices early only when additional running jobs would no longer add decision value for the workflow outcome.
- Platform-independent supply-chain and secret scanners MAY be centralized on the canonical Linux runner to avoid repeated installs and duplicate findings.
- SonarCloud tokens SHALL be dedicated to repository analysis and keep Browse plus Execute Analysis access to the target project.

## Release and Git Flow

- `main` SHALL stay releasable.
- Feature work SHALL happen on short-lived branches and merge through PRs.
- Conventional Commits SHALL be enforced by hooks and CI.
- release-plz SHALL manage version bumps and changelog updates through the release PR.
- Merge to `main` SHALL update the release PR.
- Automatic publication SHALL remain disabled until the Rust distribution pipeline is wired end to end for GitHub Releases, npm, and Homebrew.
- Release tags SHALL be created only by the hardened release workflow once `Quality`, `Security`, and `SonarCloud` have passed for the target `main` commit.
- The release workflow SHALL reuse prior green CI evidence for the target `main` SHA instead of rerunning the full validation contract inside the publish workflow.
- The release workflow SHALL download and publish the reusable release artifacts already produced by the canonical `CI` workflow for that same `main` SHA.

## Documentation Sync

- `README.md`, `docs/`, and `llms.txt` SHALL be updated together when durable user-facing behavior changes.
- `docs/development/roadmap.md` SHALL stay strategic and current.
- `docs/development/backlog.md` SHALL stay tactical.
