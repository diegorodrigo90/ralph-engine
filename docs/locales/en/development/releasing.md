# Releasing

## Flow

1. Merge reviewed Conventional Commit changes into `main`.
2. `release-plz release-pr` opens or updates the release PR.
3. Merge the release PR.
4. Release publication stays disabled until the Rust distribution pipeline is wired and validated end to end.
5. The hardened publish workflow will create the SemVer tag only after the required gates are green for the target `main` commit.

## Manual publish workflow

The `Release` workflow is manual and expects:

- `tag`
- `publish_github_release`
- `publish_npm`
- `publish_homebrew`
- `homebrew_tap_repository` when the tap should not be inferred elsewhere

Secrets used by this workflow:

- `NPM_TOKEN` when `publish_npm=true`
- `HOMEBREW_TAP_TOKEN` when `publish_homebrew=true`

The `tag` input SHALL include the leading `v`, for example `v0.2.0-alpha.1`. The workflow strips that prefix before preparing npm package versions.

## Rules

- SemVer is mandatory.
- Conventional Commits are mandatory.
- Actions are pinned by SHA.
- Release tooling is pinned to reviewed versions.
- The release contract SHALL run through `./scripts/validate.sh --mode release` before artifacts are published.
- `cargo-dist` SHALL be the Rust artifact builder for release distribution.
- `Quality`, `Security`, and `SonarCloud` SHALL all pass before a release tag is created.
- `SONAR_TOKEN` SHALL resolve to a SonarCloud token that can browse and analyze the target project.
- Checksums, SBOMs, and artifact attestations are part of the target release contract.
- npm SHALL install from reviewed `cargo-dist` release assets and verify the published `.sha256` checksum before extraction.
- Homebrew SHALL be derived from the same `cargo-dist` release assets and checksums used by the npm channel.
- Automatic publication SHALL NOT happen from `main` until GitHub Releases, npm, and Homebrew are connected to the Rust pipeline.
