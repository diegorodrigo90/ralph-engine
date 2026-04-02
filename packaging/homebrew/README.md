# packaging/homebrew

[![CI](https://github.com/diegorodrigo90/ralph-engine/actions/workflows/ci.yml/badge.svg)](https://github.com/diegorodrigo90/ralph-engine/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/diegorodrigo90/ralph-engine?display_name=tag)](https://github.com/diegorodrigo90/ralph-engine/releases)
[![Homebrew Channel](https://img.shields.io/badge/homebrew-gated-lightgrey.svg)](./README.md)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSE)

The Homebrew channel remains part of the official Ralph Engine distribution contract.

This directory holds the reviewed formula template and release notes needed to wire Homebrew
to Rust artifacts once the hardened publish workflow is enabled.

Current status:

- GitHub release artifacts are built with `cargo-dist`.
- Homebrew publication is not automated yet.
- Formula updates SHALL be derived from the same release assets and checksums used by npm.
