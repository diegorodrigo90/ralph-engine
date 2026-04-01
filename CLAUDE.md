# CLAUDE.md — Ralph Engine companion notes

Use `AGENTS.md` as the primary contract.

## Working model

- Repository root validation contract: `./scripts/validate.sh`
- Rust toolchain contract: `rust-toolchain.toml`
- asdf contract: `.tool-versions`
- Hooks: `lefthook.yml`
- Versioning: Conventional Commits + release-plz + SemVer

## Important commands

```bash
./scripts/bootstrap-dev.sh
./scripts/validate.sh --mode local
cargo test --workspace --all-targets --all-features
npm --prefix docs run build
```
