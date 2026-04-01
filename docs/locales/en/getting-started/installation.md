# Installation

Ralph Engine is currently source-first.

That is intentional in this phase of the Rust-first reboot: the repository itself is the primary contract for the runtime, official plugins, validation rules, and release pipeline.

## Prerequisites

The current baseline assumes:

- Git
- Rust `1.91.1`
- Node.js `20.19.0`
- `asdf` if you want the easiest path to a pinned local environment

The repository pins its canonical toolchain through:

- `rust-toolchain.toml`
- `.tool-versions`

## Source install

```bash
git clone https://github.com/diegorodrigo90/ralph-engine.git
cd ralph-engine
./scripts/bootstrap-dev.sh
cargo run -p re-cli -- --version
```

`bootstrap-dev.sh` is the supported entry point for local setup. It installs repository dependencies, docs dependencies, hooks, and the reviewed developer tooling required by the current contract.

## What to run next

Once the environment is bootstrapped, the next useful commands are:

```bash
./scripts/validate.sh --mode local
cargo run -p re-cli
./scripts/validate-ci-local.sh
```

Use them in this order:

1. `validate.sh` proves the local foundation matches the repository contract.
2. `cargo run -p re-cli` confirms the CLI is wired correctly.
3. `validate-ci-local.sh` gives you a local smoke run of the GitHub Actions workflow when `act` is installed.

## Planned official channels

These channels remain part of the public product contract and are being rebuilt on top of the Rust-first foundation:

- GitHub Releases
- npm
- Homebrew

Until those channels are fully reconnected, the source install path is the canonical one.
