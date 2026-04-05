# packaging/

Distribution packaging for Ralph Engine across package managers.

## Packages

| Directory | Manager | Description |
|-----------|---------|-------------|
| `npm/` | npm/npx | Node.js wrapper package for `npx ralph-engine` |
| `homebrew/` | Homebrew | Formula template for `brew install ralph-engine` |

## How it works

The CI release workflow (`cargo-dist`) builds platform binaries. These packaging scripts wrap or reference those binaries for each package manager:

- **npm**: Downloads the correct platform binary on `postinstall`
- **Homebrew**: Formula points to GitHub release assets with SHA256 verification

Crates.io publishing is handled directly by `cargo publish` (core crates only, plugins are `publish = false`).
