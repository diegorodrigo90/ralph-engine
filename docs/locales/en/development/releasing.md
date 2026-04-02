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
Before it publishes anything, the workflow verifies that the selected SHA is the current `origin/main` head and that the canonical `CI` workflow has already completed successfully for that exact push.
The workflow also rejects `publish_npm=true` or `publish_homebrew=true` unless `publish_github_release=true`, because both downstream channels depend on the reviewed GitHub Release asset set for the selected tag.
When `publish_npm=true`, the workflow also dry-runs `npm pack --json --dry-run` against the staged `ralph-engine` and `create-ralph-engine-plugin` payloads and rejects the publish if required entries, `bin` wiring, scripts, package names, or rewritten versions are wrong.
That same `CI` workflow builds cross-platform release candidates in parallel with the quality gates and publishes reusable approved release artifacts for the SHA only after `Quality`, `Security`, and `SonarCloud` have all passed.
Both the reviewed `dist-workspace.toml` contract and the generated release assets are validated explicitly before artifacts are approved or promoted.
The SonarCloud quality gate is also the hard release stop for coverage: if it falls below the configured `100%` target for analyzed code, the SHA is not approved for artifact publication or release promotion.

## Rules

- SemVer is mandatory.
- Conventional Commits are mandatory.
- Actions are pinned by SHA.
- Release tooling is pinned to reviewed versions.
- The release workflow SHALL verify the target `main` SHA against the canonical `CI` workflow before artifacts are published.
- The release workflow SHALL reuse prior green CI evidence for the target `main` SHA instead of rerunning the full validation contract inside the publish workflow.
- The release workflow SHALL reject downstream channel publication when `publish_github_release=false`.
- The release workflow SHALL verify that the GitHub Release for the selected tag exists before npm or Homebrew publication starts.
- The release workflow SHALL verify staged npm tarballs before publishing npm channels.
- The canonical `CI` workflow SHALL build cross-platform release candidates for the target `main` SHA in parallel with the quality gates.
- The canonical `CI` workflow SHALL publish reusable approved release artifacts for that SHA only after `Quality`, `Security`, and `SonarCloud` have all passed.
- The release workflow SHALL download and publish that approved artifact set instead of rebuilding it.
- `scripts/verify-dist-workspace.sh` SHALL validate the reviewed `cargo-dist` workspace contract before release-candidate or publish steps depend on it.
- `scripts/verify-release-assets.sh` SHALL validate candidate and assembled release assets, checksums, and target completeness before approval or publication.
- Pages SHALL publish from published releases and build from the release tag so the public site and docs stay aligned with published versions.
- `cargo-dist` SHALL be the Rust artifact builder for release distribution.
- `Quality`, `Security`, and `SonarCloud` SHALL all pass before a release tag is created.
- The SonarCloud quality gate SHALL enforce 100% coverage for analyzed code before reusable release artifacts are approved.
- `SONAR_TOKEN` SHALL resolve to a SonarCloud token that can browse and analyze the target project.
- Checksums, SBOMs, and artifact attestations are part of the target release contract.
- npm SHALL install from reviewed `cargo-dist` release assets and verify the published `.sha256` checksum before extraction.
- Homebrew SHALL be derived from the same `cargo-dist` release assets and checksums used by the npm channel.
- Automatic publication SHALL NOT happen from `main` until GitHub Releases, npm, and Homebrew are connected to the Rust pipeline.
