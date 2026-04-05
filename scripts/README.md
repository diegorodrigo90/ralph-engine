# scripts/

Development, validation, and release scripts.

## Development

| Script | Purpose |
|--------|---------|
| `bootstrap-dev.sh` | First-time setup (installs Rust toolchain, tools, deps) |
| `bootstrap-dev.ps1` | Windows PowerShell variant |
| `install-dev-tools.sh` | Install pinned dev tools (cargo-deny, cargo-llvm-cov, etc.) |
| `install-dev-tools.ps1` | Windows variant |
| `add-locale.sh` | Add a new locale to all plugin TOML catalogs |

## Validation

| Script | Purpose |
|--------|---------|
| `validate.sh` | Canonical validation contract (fmt, clippy, test, deny, audit, docs, contracts) |
| `validate-ci-local.sh` | Local GitHub Actions simulation |
| `check-i18n-compliance.sh` | Verify no inline locale checks in code |

## Release

| Script | Purpose |
|--------|---------|
| `export-release-version.sh` | Extract version from Cargo.toml |
| `export-sonarcloud-metadata.sh` | Export SonarCloud config for CI |
| `verify-dist-workspace.sh` | Validate cargo-dist workspace contract |
| `verify-release-assets.sh` | Validate release checksums and targets |
| `prepare-npm-release.mjs` | Generate npm package for release |
| `publish-homebrew-tap.sh` | Publish Homebrew formula to tap repo |
| `render-homebrew-formula.mjs` | Generate Homebrew formula from template |
| `verify-plugin-contracts.mjs` | Verify plugin manifests match typed Rust contracts |
