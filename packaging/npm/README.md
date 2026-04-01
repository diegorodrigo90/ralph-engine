# packaging/npm

The npm packaging channel remains part of the official Ralph Engine distribution contract.

Current status:

- the package remains private during the Rust-first reboot
- `postinstall` downloads the reviewed `cargo-dist` release asset for the current platform
- downloaded archives are verified against the published `.sha256` asset before extraction
- automatic publication remains disabled until GitHub Releases, npm provenance, and Homebrew are wired together
