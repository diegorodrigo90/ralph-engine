# Security Policy

## Supported Versions

| Version | Supported |
| ------- | --------- |
| current `main` | :white_check_mark: |
| archived Go-era refs | :x: |

## Reporting a Vulnerability

Do **not** open a public issue for security vulnerabilities.

Please use GitHub Security Advisories:

- https://github.com/diegorodrigo90/ralph-engine/security/advisories/new

Include:

- a clear description of the issue
- impact and affected surfaces
- reproduction steps
- suggested remediation, if known

## Security Baseline

Ralph Engine now enforces a Rust-first security baseline from day zero:

- pinned toolchains and reviewed tool versions
- GitHub Actions pinned by SHA
- least-privilege workflow permissions
- `cargo audit` for RustSec advisories
- `cargo deny` for dependency policy and advisories
- SonarCloud as a complementary signal
- Gitleaks configuration for repository secret scanning
- release automation through SemVer + release-please + reviewed release tooling

## Release Integrity

The target release contract for the Rust-first era includes:

- checksums
- SBOMs
- artifact attestations
- reviewed release tooling
- npm provenance and Homebrew distribution once the Rust pipeline is fully wired

## Current Exception

The current VitePress documentation stack still carries a moderate upstream `vite/esbuild` advisory in the latest stable line available to this repository. That exception is explicit, documented, and scoped to the docs toolchain rather than the Rust runtime.

## Design Notes

- Ralph Engine does not own user billing or provider credentials.
- The core runtime is plugin-first and keeps workflow semantics outside the core.
- Third-party plugins and external MCP servers are treated as untrusted until validated and explicitly enabled.
- Unsafe Rust is forbidden by default in the repository foundation.
