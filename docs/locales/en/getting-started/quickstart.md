# Quick Start

If you want the shortest path from clone to a validated local baseline, use this sequence:

```bash
./scripts/bootstrap-dev.sh
./scripts/validate.sh --mode local
cargo run -p re-cli
```

## Why this order matters

- `bootstrap-dev.sh` installs the pinned local environment expected by the repository.
- `validate.sh --mode local` proves the project is healthy before you start changing it.
- `cargo run -p re-cli` confirms the Rust-first CLI baseline is working.

## After the first run

The next useful paths depend on what you want to do:

- Read [Architecture](../reference/architecture.md) if you want the system model first.
- Read [Plugins](../guides/plugins.md) if you want to understand the extension surface.
- Read [Coding Standards](../development/coding-standards.md) if you are going to contribute code.
- Read [Roadmap](../development/roadmap.md) if you want the current direction rather than just the current implementation.

## Local validation options

For normal development:

```bash
./scripts/validate.sh --mode local
```

For a local smoke check of the GitHub Actions workflow:

```bash
./scripts/validate-ci-local.sh
```

The GitHub Actions smoke path complements the main validation contract. It does not replace it.
