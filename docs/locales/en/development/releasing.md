# Releasing

## Flow

1. Merge reviewed Conventional Commit changes into `main`.
2. `release-plz release-pr` opens or updates the release PR.
3. Merge the release PR.
4. Release publication stays disabled until the Rust distribution pipeline is wired and validated end to end.
5. The hardened publish workflow will create the SemVer tag only after the required gates are green for the target `main` commit.

## Rules

- SemVer is mandatory.
- Conventional Commits are mandatory.
- Actions are pinned by SHA.
- Release tooling is pinned to reviewed versions.
- The release contract SHALL run through `./scripts/validate.sh --mode release` before artifacts are published.
- `Quality`, `Security`, and `SonarCloud` SHALL all pass before a release tag is created.
- Checksums, SBOMs, and artifact attestations are part of the target release contract.
- npm and Homebrew remain official channels, but they are not wired yet.
- Automatic publication SHALL NOT happen from `main` until GitHub Releases, npm, and Homebrew are connected to the Rust pipeline.
