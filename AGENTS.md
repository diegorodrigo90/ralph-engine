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
- `scripts/verify-dist-workspace.sh` SHALL validate the reviewed `cargo-dist` workspace contract before release-candidate or publish steps depend on it.
- `scripts/verify-release-assets.sh` SHALL validate generated release assets, checksums, and target completeness before artifacts are approved or published.
- `rust-toolchain.toml` SHALL pin the canonical Rust toolchain.
- `.tool-versions` SHALL pin the canonical asdf tool versions.
- GitHub Actions SHALL be pinned to full SHAs.
- Tooling installed by scripts SHALL use explicit reviewed versions, never `latest`.

## Golden Rules

1. Ralph Engine SHALL stay generic, configurable, and public-safe.
2. The core runtime and official plugins SHALL be implemented in Rust.
3. Third-party plugin contracts SHALL stay language-agnostic.
4. Tests SHALL be written before implementation (TDD). WHEN a new feature or fix is implemented, the test SHALL exist before the production code.
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
16a. Code SHALL be written for readability first, even for developers unfamiliar with Rust. Prefer descriptive names over abbreviations, short functions over dense chains, explicit `filter` + `map` over `filter_map` when the intent is clearer, and comments explaining "why" on any non-obvious logic. Clever Rust idioms SHALL NOT be used when a simpler alternative communicates the same intent.
17. DDD, SOLID, object calisthenics, early returns, strong typing, and clear names SHALL be applied where they improve maintainability in idiomatic Rust.
18. The repository SHALL optimize for low token cost and high signal: prompt/context control, MCP governance, and plugin contracts are core responsibilities.
19. The CLI SHALL stay modular. New command families or behaviors SHALL be introduced through isolated command modules or registries rather than by growing one central dispatcher function.
20. Plugin and MCP capabilities SHALL remain extensible by contract. Adding a new capability or contribution SHALL be possible through local module changes and typed descriptors rather than string parsing spread across the runtime.
20a. Shared contract tests SHALL prefer neutral synthetic fixtures over official plugin identifiers. Official plugin crates SHALL verify their own manifests, metadata, and contribution details locally, while integration and smoke tests MAY continue to exercise the shipped official catalog intentionally.
21. Plugin kinds SHALL remain typed and explicit. Template, agent-runtime, provider, remote-control, MCP-contribution, policy, and future kinds SHALL evolve through shared contracts instead of ad hoc manifest-only strings.
22. Plugin lifecycle SHALL remain typed and explicit. Discovery, configuration, validation, loading, and future lifecycle stages SHALL evolve through shared contracts instead of ad hoc booleans or scattered command-specific checks.
23. Plugin runtime hooks SHALL remain typed and explicit. Prepare, doctor, prompt, agent, MCP, policy, and future hook surfaces SHALL evolve through shared contracts instead of capability-specific strings or ad hoc dispatch.
24. Configuration resolution SHALL remain typed and layered. Built-in defaults, workspace settings, project settings, user overrides, prompt and context budgets, and future scopes SHALL evolve through shared contracts instead of implicit precedence rules spread across commands.
25. Typed configuration layers SHALL remain inspectable. The CLI and docs SHALL expose the canonical layer stack and resolved plugin configuration so configuration precedence stays visible instead of being inferred from implementation details.
26. Runtime registration and state orchestration SHALL remain typed and explicit. Plugin activation, capability registration, template registration, prompt registration, agent registration, check registration, provider registration, policy registration, runtime-hook registration, runtime health, runtime issues, runtime doctor reporting, runtime action plans, runtime topology, MCP enablement, and future runtime state transitions SHALL evolve through shared contracts instead of implicit catalog traversal in command handlers. Disabled capabilities, disabled templates, disabled prompt providers, disabled agent runtimes, disabled checks, disabled providers, disabled policies, and disabled runtime hooks SHALL remain visible in runtime health and remediation output; they SHALL NOT be treated as invisible metadata.
26a. Official plugins SHALL stay autonomous. When a plugin exposes metadata, manifests, MCP servers, templates, prompts, policies, or future contributions, the plugin crate itself SHALL own the closest tests for that behavior instead of relying on unrelated shared-crate fixtures.
27. Pre-1.0 cleanup MAY break compatibility when it improves the final architecture. Compatibility debt SHALL not block necessary refactors.
28. Selective validation MAY skip checks only when the changed files fit an explicit, reviewed safe scope. If the change set crosses domains, touches tooling, or falls outside a known-safe scope, validation SHALL fall back to the full contract.
29. CI, hooks, and local validation SHALL use the same selective-validation rules. The optimization SHALL be conservative: skip only for clearly public-surface-only or clearly Rust-only change sets; uncertainty SHALL resolve to full validation.
30. Local GitHub Actions simulation MAY be used to catch workflow failures before push, but it SHALL complement `scripts/validate.sh`, not replace it.
31. CI caches SHALL be keyed and scoped by the inputs that actually affect correctness, including operating system, toolchain, dependency lockfiles, and job purpose. Broad blind caches SHALL be avoided.
32. Cache strategy SHALL optimize by domain where it improves reuse without increasing drift, such as separate dependency caches for repository Node tooling, docs tooling, and Rust build artifacts.
33. Workflows SHALL avoid duplicate heavy work across jobs. Expensive steps such as coverage generation, scanner installs, and release-only tooling SHALL run only in the jobs that need them.
34. Cross-platform quality SHALL be proven through an OS matrix, while platform-independent security scanners MAY run once on a canonical runner when that avoids duplicated cost without reducing coverage.
35. CI workflows SHALL cancel superseded in-progress runs for the same branch or pull request whenever the older run no longer provides unique value.
36. SonarCloud configuration SHALL fail fast with a clear preflight error when the configured token cannot browse or analyze the target project.
37. SonarCloud scans SHALL resolve the project key and organization from `sonar-project.properties` and pass them explicitly to the scanner so CI logs and behavior stay unambiguous.
38. Coverage used by SonarCloud SHALL be generated once in the canonical Linux quality job, uploaded as an artifact, and reused by the SonarCloud job instead of rerunning test coverage.
39. The hardened release workflow SHALL verify that the target SHA is the current `origin/main` head and that the canonical `CI` workflow has already completed successfully for that exact push before any tag or publication step begins.
40. GitHub Actions checkouts SHALL disable persisted credentials unless a later step in that same job explicitly needs to push or publish.
41. The canonical `CI` workflow on `main` SHALL build cross-platform release candidate artifacts in parallel with the code-quality gates for the same SHA, and SHALL publish reusable approved release artifacts only after `Quality`, `Security`, and `SonarCloud` have all passed. The publish workflow SHALL promote those approved artifacts instead of rebuilding them.
42. Reviewed pinned tool binaries that are installed by repository scripts MAY be cached in CI only when the cache key stays scoped by operating system, installer definition, and job purpose. Tool caches SHALL NOT be shared blindly across unrelated jobs or platforms.
43. Workflows SHALL avoid no-op cache restores and unnecessary setup steps. If a job does not install or consume a dependency set, it SHALL NOT restore that cache just for symmetry.
44. Matrix fail-fast behavior SHALL match the purpose of the matrix. Quality matrices SHOULD keep `fail-fast: false` to surface cross-platform regressions in one run, while release-artifact matrices SHOULD keep `fail-fast: true` because one failed platform already invalidates the publishable set.
45. Pages publication SHALL happen from published releases and SHALL build from the release tag so the public site, docs, and plugins surface reflect published versions rather than unreleased `main` state.
46. `dist-workspace.toml` SHALL remain an explicit reviewed contract. CI and release workflows SHALL validate it before building or promoting `cargo-dist` artifacts.
47. Reusable release artifacts SHALL be treated as first-class contract outputs. Candidate and publishable asset sets SHALL pass explicit checksum and target-completeness validation before approval or publication.

## Documentation Audiences (CRITICAL — AI agents MUST respect)

Documentation is split into three audiences. Content SHALL NOT cross boundaries:

| Section | Audience | Content scope |
|---------|----------|--------------|
| **Using Ralph Engine** | End users who install and run the CLI | Install, configure, CLI commands, hooks, MCP, troubleshooting. Assumes the binary is installed. |
| **Plugin Development** | Developers who create their own plugins | Scaffold, implement PluginRuntime, test, publish. Assumes familiarity with Rust. |
| **Contributing** | Open-source contributors to the core | Build from source, coding standards, architecture internals, release pipeline. |

When writing or editing docs:
- User docs SHALL NOT mention `cargo build`, `cargo test`, or internal crate structure
- Plugin dev docs SHALL NOT explain core architecture decisions or CI pipeline
- Contributing docs SHALL NOT explain how to use the CLI as an end user
- Each page SHALL state who it is for in the description frontmatter

## Engineering Discipline

48. WHEN implementing any code change, the assistant SHALL first search official documentation (Context7, web search, crate docs) for the libraries and APIs involved. Implementation from memory or stale knowledge SHALL be treated as a defect source.
49. WHEN fixing a bug, the fix SHALL include a regression test that fails without the fix and passes with it. The test and fix SHALL be committed together.
50. Bug fixes SHALL identify and address the root cause. IF a proposed fix suppresses symptoms (hiding errors, adding defensive checks around broken logic, commenting out failing code) without understanding why the failure happens, THEN that fix SHALL be rejected.
51. Tests SHALL verify real behavior with specific expected values. WHEN a test only asserts existence (`!is_empty()`) or success (`is_ok()`) without checking the actual content, THEN that test SHALL be strengthened or rejected. Tests that pass regardless of the returned value are worse than no test — they create false confidence. Every `assert!` SHALL answer: "What specific bug would this catch?" IF the answer is "none" THEN the assertion has no value.
52. WHEN writing assertions for error paths, the test SHALL verify the error code or error message content — not just that an error occurred. `assert!(result.is_err())` alone is insufficient; `assert_eq!(err.code, "expected_code")` is the minimum bar.
53. Mock-free testing SHALL be the default. WHEN a function reads files, creates temp directories, or parses strings, the test SHALL use real files in temp directories and real input strings — not mocked I/O. Mocking is ONLY acceptable at true I/O boundaries (network calls, subprocess spawn) where the real operation cannot run in CI.
54. WHEN code is genuinely untestable in unit tests (subprocess spawn, binary-dependent branches, `#[non_exhaustive]` enum wildcards), it SHALL be marked with `#[cfg_attr(coverage_nightly, coverage(off))]` and a comment explaining WHY it cannot be tested. The comment SHALL state: (a) what the code does, (b) why it cannot be unit-tested, and (c) how it IS validated (e.g., E2E runs, manual testing). Code SHALL NOT be excluded from coverage to hide laziness — only true I/O boundaries and compile-required unreachable branches qualify. Pure logic SHALL ALWAYS be extracted into testable functions first (see `agent_helpers::build_agent_command_config` as the reference pattern).
55. Tests SHALL NEVER call `stdin().read_line()` or any function that reads from stdin. In git hooks and CI pipelines, stdin is closed — `read_line()` blocks indefinitely, causing the entire hook to hang. Interactive functions SHALL be tested only via their non-interactive paths (e.g., `--help` flag, catalog verification). The interactive flow itself is validated by manual execution.

## Structure

- `core/` SHALL own the Rust runtime crates.
- `core/crates/re-core/` SHALL own shared runtime foundations.
- `core/crates/re-config/` SHALL own typed runtime configuration contracts and defaults.
- `core/crates/re-mcp/` SHALL own typed MCP contribution contracts, including launch policy, process model, command boundaries, working-directory policy, environment policy, and availability policy.
- `core/crates/re-plugin/` SHALL own typed plugin metadata, lifecycle, runtime-hook, loading-boundary, and capability contracts.
- `core/crates/re-plugin/` SHALL also own typed plugin trust-level contracts so official and community plugin metadata stay aligned across runtime, CLI, and third-party manifests.
- `core/crates/re-official/` SHALL own the typed built-in runtime catalog so official plugin wiring stays reusable and outside the CLI crate.
- `core/crates/re-cli/` SHALL own the modular CLI surface and command registry.
- `core/crates/re-config/` SHALL also own the canonical typed locale contract and supported-locale catalog for runtime-facing surfaces.
- `re-core` and `re-cli` SHALL expose typed runtime capability, template, prompt, agent, check, provider, policy, and hook registration so new capabilities can be added through shared contracts instead of command-local branching.
- `plugins/official/` SHALL own Rust-first official plugins.
- `site/` SHALL own all public web surfaces: landing pages, plugin catalog, and documentation (Astro + Starlight).
- `site/src/content/docs/` SHALL own docs content (EN at root, PT-BR under `pt-br/`).
- `packaging/` SHALL own npm and Homebrew packaging.
- `tools/create-ralph-engine/` SHALL own plugin scaffolding for `npx create-ralph-engine-plugin`. Runtime catalog surfaces SHALL NOT turn scaffolding into a generic runtime responsibility.
- CLI surfaces SHALL support `en` and `pt-br` through typed locale catalogs, and new locales SHALL be additive rather than requiring handler rewrites.
- Locale-aware crates SHALL resolve locale selection through the shared typed locale contract in `re-config` instead of introducing crate-local locale-id branching rules.
- Locale-aware crates and public surfaces SHALL organize translation strings per locale module or file set instead of scattering inline locale branches across handlers.
- Plugin metadata SHALL support locale-aware display names and summaries with English fallback when a requested locale is missing.
- Plugins that expose public CLI-facing output SHALL own their locale catalogs alongside the plugin/runtime crate that renders that output, with English fallback when a requested locale is missing.
- Per-locale modules SHALL expose one typed locale catalog object instead of scattered string constants, so core, official plugins, and scaffolded plugins can add a new locale without rewriting handlers.
- Third-party plugin manifests SHALL carry locale-aware display metadata through the versioned `manifest.yaml` contract owned by `tools/create-ralph-engine/`, including required English summaries plus optional localized summaries.
- `core/crates/re-plugin/` SHALL own typed plugin kind contracts for the runtime and tooling.
- The plugin scaffolder SHALL only accept kinds and capabilities that already exist in the typed runtime and plugin contracts. Future surfaces SHALL stay rejected until the core defines them explicitly.
- Scaffolded and official plugin identifiers SHALL use the same dotted namespace contract, such as `official.basic` or `acme.jira-suite`.
- `tools/create-ralph-engine/` SHALL own the versioned `manifest.yaml` contract for third-party plugins. Manifest kinds, capabilities, identifier rules, and template requirements SHALL stay aligned with the typed runtime contracts.
- Cross-language plugin contracts SHALL stay verified. Changes to reviewed Rust capabilities, official plugin descriptors, manifest schema enums, or scaffolder-supported surfaces SHALL update the explicit contract checks rather than relying on drift-prone manual synchronization.
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
npm run contracts:verify
```

For public-surface-only change sets, the `public` validation step SHALL cover:

- `cd site && npm run build` (Astro + Starlight + Pagefind in a single build)

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

## Prompt Assembly (Agent-Agnostic)

Rules follow EARS syntax (SHALL keyword). These rules apply to ALL agent runtimes (Claude, Codex, or any future agent).

### Structure

Appended system prompt content goes at the END of the agent's built-in instructions. Research shows LLMs over-attend to beginning and end, under-attend to the middle ("lost in the middle" effect: 30%+ accuracy drop). Prompt sections SHALL be ordered by attention priority:

| Position | Section | Tag | Attention | Content |
|----------|---------|-----|-----------|---------|
| First | Task | `<task>` | HIGH (start of block) | Story with ACs |
| Middle | Rules | `<rules>` | MEDIUM (reference zone) | Condensed project rules |
| Middle | Context | `<context>` | MEDIUM (reference zone) | Tech stack, domain, test users |
| Last | Constraints | `<constraints>` | HIGHEST (end of block) | Workflow + tracking requirements |

### Principles

- Prompt instructions SHALL be outcome-based. WHEN multiple valid approaches exist, the prompt SHALL describe the expected outcome, not prescriptive steps. **Why:** outcome-based prompts let agents adapt to reality; prescriptive steps create brittleness when conditions differ from the script.
- Every prompt instruction SHALL carry its weight. IF the agent would produce the correct behavior without an instruction, THEN that instruction SHALL be removed. **Why:** noise dilutes signal; redundant instructions waste tokens and compete for attention.
- Prompt content SHALL NOT duplicate information already available in the agent's native config (CLAUDE.md, AGENTS.md, `.cursorrules`). WHEN a project provides `rules-digest.md`, it SHALL be included for agents that lack native config loading. **Why:** duplication wastes tokens and risks inconsistency.
- Prompt sections SHALL use XML tags (`<task>`, `<rules>`, `<context>`, `<constraints>`). **Why:** XML tags improve LLM parsing accuracy and enable Anthropic prompt caching on static prefixes.
- Non-negotiable requirements (tracking updates, quality gates) SHALL be placed in the `<constraints>` section at the END of the prompt. **Why:** the end of the system prompt is the highest-attention zone.
- Prompt content SHALL prefer structured formats (bullets, tables) over prose, references over full document inclusion, and RAG over document pasting. **Why:** token efficiency without sacrificing load-bearing context.
- Prompt content SHALL be agent-agnostic. Plugin-specific CLI flags (`--allowedTools`, `-p`, `--output-format`) SHALL live in the agent plugin implementation, not in the prompt text.

### Feedback Loop (Learnings)

- WHEN `.ralph-engine/findings.md` exists, the BMAD plugin SHALL read it and include it in the prompt as a `<findings>` section between `<rules>` and `<constraints>`.
- The `<constraints>` section SHALL instruct the agent to review past findings before implementing, and to update the file after code review.
- The format, categories, and content of `findings.md` SHALL be defined by the project, not by Ralph Engine. RE only reads and injects.

### Tool Auto-Discovery

- Plugins SHALL declare required agent tools by implementing `required_tools()` on `PluginRuntime`. The default implementation SHALL return an empty slice.
- WHEN the `run` command assembles a prompt context, it SHALL collect `required_tools()` from ALL enabled plugin runtimes, deduplicate, and populate `PromptContext.discovered_tools`.
- The agent plugin SHALL merge three tool sources in order: its own base tools, discovered tools from plugins, and user-configured extras from `run.allowed_tools` in project config. Duplicates SHALL be removed.
- WHEN a plugin contributes MCP servers, it SHOULD also declare the corresponding MCP tool patterns in `required_tools()` so agents receive them automatically.

## Documentation Sync

- `README.md`, `site/src/content/docs/`, and `llms.txt` SHALL be updated together when durable user-facing behavior changes.
- Roadmap and backlog docs SHALL stay strategic and current within the docs content collection.
- Public docs in this repository SHALL describe shipped behavior and reviewed public contracts. Internal handoffs, private progress notes, and process artifacts SHALL NOT be committed here.
