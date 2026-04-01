# Releasing

## Flow

1. Merge reviewed Conventional Commit changes into `main`.
2. `release-plz release-pr` opens or updates the release PR.
3. Merge the release PR.
4. `release-plz release` creates the SemVer tag and GitHub release from `main`.
5. Tag workflows run the release contract and build artifacts.

## Rules

- SemVer is mandatory.
- Conventional Commits are mandatory.
- Actions are pinned by SHA.
- Release tooling is pinned to reviewed versions.
- The release contract SHALL run through `./scripts/validate.sh --mode release` before artifacts are published.
- Checksums, SBOMs, and artifact attestations are part of the target release contract.
- npm and Homebrew remain official channels and will publish from the Rust release pipeline, not from ad hoc local packaging.
