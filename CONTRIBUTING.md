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
npm run contracts:verify
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
- `npm run contracts:verify`
- `npm --prefix docs run build`
- `./scripts/assemble-public-surfaces.sh .site-dist`
- `./scripts/validate.sh --mode local`
- `./scripts/validate-ci-local.sh`

## How to add a new locale

1. Add a variant to `SupportedLocale` in `core/crates/re-config/src/lib.rs`
   (enum variant, `as_str`, `descriptor`, `parse_supported_locale`, `parse_os_locale`)
2. Create `locales/{locale}.toml` in each crate (copy `en.toml` and translate):
   - 4 core crates: `re-mcp`, `re-plugin`, `re-core`, `re-cli`
   - 8 plugins: `basic`, `bmad`, `claude`, `claudebox`, `codex`, `github`, `ssh`, `tdd-strict`
3. For `re-core` and `re-cli` only: create `src/i18n/fn_{locale}.rs` (copy `fn_en.rs`, translate)
4. Run `cargo build` — missing keys cause build failures automatically

The `scripts/add-locale.sh` helper lists all files that need to be created.

## How to add a new official plugin

1. Create the plugin crate in `plugins/official/{name}/` with:
   - `Cargo.toml`, `src/lib.rs`, `manifest.yaml`, `locales/en.toml`, `locales/pt-br.toml`, `build.rs`
2. Add `"plugins/official/{name}"` to workspace `members` in root `Cargo.toml`
3. Add `re-plugin-{name} = { path = "...", version = "..." }` to `core/crates/re-official/Cargo.toml`
4. Run `cargo build` — the plugin is auto-registered from its `manifest.yaml`

No manual editing of `re-official/src/lib.rs` needed.

## Commit messages

Conventional Commits are mandatory.

Examples:

- `feat(core): add mcp registry foundations`
- `fix(plugin): reject duplicate capability ids`
- `docs(architecture): align rust workspace model`
- `build(ci): pin release-plz workflow`
