# Installation

Ralph Engine is currently installed from source. Binary distribution via npm and Homebrew is prepared but intentionally gated until the release pipeline is fully validated.

## Prerequisites

- Git
- Rust 1.91.1 (pinned via `rust-toolchain.toml`)
- Node.js 20.19.0 (pinned via `.tool-versions`)

## Source Install

```bash
git clone https://github.com/diegorodrigo90/ralph-engine.git
cd ralph-engine
./scripts/bootstrap-dev.sh
cargo run -p re-cli -- --version
```

`bootstrap-dev.sh` installs all dependencies, hooks, and developer tooling.

## Verify Installation

```bash
# Validate the full repository contract
./scripts/validate.sh --mode local

# Run the CLI
cargo run -p re-cli -- --help

# Run in Portuguese
cargo run -p re-cli -- --locale pt-br --help

# Run full test suite
cargo test --workspace --all-targets
```

## Distribution Channels

These channels are prepared and will be enabled once the release pipeline is fully connected:

| Channel | Status |
|---------|--------|
| **Source** | Available (canonical install method) |
| **GitHub Releases** | Infrastructure ready, gated |
| **npm** | Package prepared, publication gated |
| **Homebrew** | Formula prepared, tap gated |
