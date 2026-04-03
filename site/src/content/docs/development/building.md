---
title: "Building"
description: "Build the project from source"
---

## Toolchain

- Rust 1.91.1 via `rust-toolchain.toml`
- Node.js 20.19.0 via `.tool-versions`

## Debug Build

Build all workspace crates in debug mode:

```bash
cargo build --workspace
```

## Release Build

Build all workspace crates with optimizations:

```bash
cargo build --workspace --release
```

## Tests

Run the full test suite:

```bash
cargo test --workspace --all-targets
```

Run tests with all feature flags enabled:

```bash
cargo test --workspace --all-targets --all-features
```

Run a specific test by name:

```bash
cargo test --workspace <test-name>
```

## Code Coverage

Generate LCOV coverage report:

```bash
cargo llvm-cov --workspace --all-features --lcov --output-path coverage/lcov.info
```

## Linting

Check formatting:

```bash
cargo fmt --all --check
```

Run clippy with warnings as errors:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Documentation

Build rustdoc with warnings as errors:

```bash
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```

## Dependency Auditing

Check for known vulnerabilities:

```bash
cargo audit
```

Check license and supply chain policies:

```bash
cargo deny check
```

## Cross-Language Contract Verification

Verify plugin contracts across Rust and Node boundaries:

```bash
npm run contracts:verify
```

## Full Validation

Run the complete repository validation contract (formatting, linting, tests, coverage, docs, audits, contracts):

```bash
./scripts/validate.sh --mode local
```

Run a CI-equivalent smoke check locally:

```bash
./scripts/validate-ci-local.sh
```
