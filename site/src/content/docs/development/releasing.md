---
title: "Releasing"
description: "Release process and CI pipeline"
---

## Release Flow

1. Merge reviewed Conventional Commit changes into `main`.
2. `release-plz release-pr` opens or updates the release PR.
3. Merge the release PR.
4. Release publication stays disabled until the Rust distribution pipeline is wired and validated end to end.
5. The hardened publish workflow creates the SemVer tag only after the required gates are green for the target `main` commit.

## Manual Publish Workflow

The `Release` workflow is manual and expects these inputs:

- `tag` — must include the leading `v` (e.g., `v0.2.0-alpha.1`). The workflow strips the prefix before preparing npm package versions.
- `publish_github_release`
- `publish_npm`
- `publish_homebrew`
- `homebrew_tap_repository` — when the tap should not be inferred elsewhere

### Required Secrets

- `NPM_TOKEN` — when `publish_npm=true`
- `HOMEBREW_TAP_TOKEN` — when `publish_homebrew=true`

### Pre-Publish Checks

Before publishing anything, the workflow:

1. Verifies the selected SHA is the current `origin/main` head.
2. Verifies the canonical `CI` workflow has completed successfully for that exact push.
3. Rejects `publish_npm=true` or `publish_homebrew=true` unless `publish_github_release=true` (both downstream channels depend on the reviewed GitHub Release asset set).

## npm Verification

When `publish_npm=true`, the workflow:

1. Dry-runs `npm pack --json --dry-run` against the staged `ralph-engine` and `create-ralph-engine-plugin` payloads.
2. Rejects the publish if required entries, `bin` wiring, scripts, package names, or rewritten versions are wrong.
3. Installs the staged tarballs into a throwaway consumer project and executes their public binaries before publish.

## Homebrew Verification

When `publish_homebrew=true`, the workflow:

1. Renders the formula from the approved release assets.
2. Validates it on macOS with `brew audit`, `brew install`, and `brew test`.
3. Only then updates the tap repository.

## CI Pipeline

The canonical `CI` workflow builds cross-platform release candidates in parallel with the quality gates and publishes reusable approved release artifacts for the SHA only after `Quality`, `Security`, and `SonarCloud` have all passed.

Both the reviewed `dist-workspace.toml` contract and the generated release assets are validated explicitly before artifacts are approved or promoted.

The SonarCloud quality gate is the hard release stop for coverage: if it falls below the configured `100%` target for analyzed code, the SHA is not approved for artifact publication or release promotion.

## Rules

- SemVer is mandatory.
- Conventional Commits are mandatory.
- Actions are pinned by SHA.
- Release tooling is pinned to reviewed versions.
- The release workflow verifies the target `main` SHA against the canonical `CI` workflow before artifacts are published.
- The release workflow reuses prior green CI evidence instead of rerunning the full validation contract inside the publish workflow.
- The release workflow rejects downstream channel publication when `publish_github_release=false`.
- The release workflow verifies that the GitHub Release for the selected tag exists before npm or Homebrew publication starts.
- The release workflow verifies staged npm tarballs before publishing npm channels.
- The release workflow smoke-tests staged npm install flows before publishing npm channels.
- The canonical `CI` workflow builds cross-platform release candidates in parallel with the quality gates.
- The canonical `CI` workflow publishes reusable approved release artifacts only after `Quality`, `Security`, and `SonarCloud` have all passed.
- The release workflow downloads and publishes that approved artifact set instead of rebuilding it.
- `scripts/verify-dist-workspace.sh` validates the reviewed `cargo-dist` workspace contract before release-candidate or publish steps depend on it.
- `scripts/verify-release-assets.sh` validates candidate and assembled release assets, checksums, and target completeness before approval or publication.
- `scripts/verify-homebrew-formula.sh` validates the rendered Homebrew formula with `brew audit`, `brew install`, and `brew test` before the tap is updated.
- Pages publish from published releases and build from the release tag so the public site and docs stay aligned with published versions.
- `cargo-dist` is the Rust artifact builder for release distribution.
- `Quality`, `Security`, and `SonarCloud` must all pass before a release tag is created.
- The SonarCloud quality gate enforces 100% coverage for analyzed code before reusable release artifacts are approved.
- `SONAR_TOKEN` must resolve to a SonarCloud token that can browse and analyze the target project.
- Checksums, SBOMs, and artifact attestations are part of the target release contract.
- npm installs from reviewed `cargo-dist` release assets and verifies the published `.sha256` checksum before extraction.
- Homebrew is derived from the same `cargo-dist` release assets and checksums used by the npm channel.
- Automatic publication does not happen from `main` until GitHub Releases, npm, and Homebrew are connected to the Rust pipeline.
