# packaging/homebrew

The Homebrew channel remains part of the official Ralph Engine distribution contract.

This directory holds the reviewed formula template and release notes needed to wire Homebrew
to Rust artifacts once the hardened publish workflow is enabled.

Current status:

- GitHub release artifacts are built with `cargo-dist`.
- Homebrew publication is not automated yet.
- Formula updates SHALL be derived from the same release assets and checksums used by npm.
