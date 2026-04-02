# ralph-engine

[![CI](https://github.com/diegorodrigo90/ralph-engine/actions/workflows/ci.yml/badge.svg)](https://github.com/diegorodrigo90/ralph-engine/actions/workflows/ci.yml)
[![Sonar Quality Gate](https://sonarcloud.io/api/project_badges/measure?project=ralph-engine_ralph-engine&metric=alert_status)](https://sonarcloud.io/project/overview?id=ralph-engine_ralph-engine)
[![Latest Release](https://img.shields.io/github/v/release/diegorodrigo90/ralph-engine?display_name=tag)](https://github.com/diegorodrigo90/ralph-engine/releases)
[![npm Channel](https://img.shields.io/badge/npm-gated-lightgrey.svg)](./README.md)
[![Node](https://img.shields.io/badge/node-20.19%2B-339933.svg)](package.json)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSE)
[![Repository](https://img.shields.io/badge/source-ralph--engine-black)](https://github.com/diegorodrigo90/ralph-engine/tree/main/packaging/npm)

Official npm packaging surface for Ralph Engine.

## Status

This package is still intentionally private while the Rust-first reboot finishes its end-to-end release hardening.

Current contract:

- the package remains private during the Rust-first reboot
- `postinstall` downloads the reviewed `cargo-dist` release asset for the current platform
- downloaded archives are verified against the published `.sha256` asset before extraction
- the release workflow installs the staged package into a throwaway consumer project and executes the public binary before publish
- automatic publication remains disabled until GitHub Releases, npm provenance, and Homebrew are wired together

## Packaging Model

- GitHub Releases are the canonical source of reviewed artifacts
- npm installs the platform-specific release artifact rather than rebuilding locally
- checksum verification happens before extraction
- publication will remain gated until provenance and the full release workflow are hardened

## Repository Source

- package source: `packaging/npm/`
- CLI binary target: `ralph-engine`
- scaffolder package: `tools/create-ralph-engine/`
