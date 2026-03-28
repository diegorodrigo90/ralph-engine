# Creating a Release

ralph-engine uses semantic versioning and automated releases. One git tag triggers publishing to all channels.

## Semantic Versioning

```
v{MAJOR}.{MINOR}.{PATCH}

v0.x.x  — Pre-release (API may change)
v1.0.0  — First stable release
v1.1.0  — New feature (backward compatible)
v1.1.1  — Bug fix
v2.0.0  — Breaking change
```

## Release Process

### 1. Ensure all checks pass

```bash
make check    # fmt + vet + lint + test + build
make cross    # Cross-platform compilation
```

### 2. Tag the release

```bash
# Create annotated tag
git tag -a v1.0.0 -m "feat: first stable release"

# Push tag — triggers CI release pipeline
git push origin v1.0.0
```

### 3. CI does the rest automatically

The `release.yml` workflow:

1. **Tests** — Runs full test suite with race detector
2. **GoReleaser** — Builds 6 binaries (Linux/macOS/Windows × amd64/arm64)
3. **GitHub Releases** — Publishes binaries + checksums + changelog
4. **Homebrew** — Updates tap formula (if `HOMEBREW_TAP_TOKEN` secret is set)
5. **npm** — Publishes wrapper package (if `NPM_PUBLISH_ENABLED` var is set)

## One-Time Setup (secrets)

### GitHub Releases

Works automatically with `GITHUB_TOKEN` (built-in).

### npm

1. Create account at [npmjs.com](https://www.npmjs.com/signup)
2. Generate access token: npmjs.com → Settings → Access Tokens → Generate → Automation
3. Add to GitHub: repo → Settings → Secrets → Actions → `NPM_TOKEN`
4. Add variable: repo → Settings → Variables → Actions → `NPM_PUBLISH_ENABLED` = `true`

### Homebrew

1. Create repo `your-username/homebrew-tap` on GitHub (public, empty)
2. Generate Personal Access Token (PAT): GitHub → Settings → Developer Settings → Fine-grained tokens → create with `contents: write` on the tap repo
3. Add to GitHub: repo → Settings → Secrets → Actions → `HOMEBREW_TAP_TOKEN`

### go install

Works automatically — users install from the tagged source.

### curl installer

Works automatically — reads from GitHub Releases.

## Pre-release versions

Tags with `-` suffix are marked as pre-release:

```bash
git tag -a v1.0.0-beta.1 -m "feat: beta release"
git push origin v1.0.0-beta.1
```

GoReleaser detects pre-release tags and marks them accordingly on GitHub.

## Local release (testing)

```bash
# Dry run — see what GoReleaser would do
goreleaser release --snapshot --clean

# Check output
ls dist/
```

## Changelog

GoReleaser auto-generates changelog from conventional commits:

- `feat:` → Features section
- `fix:` → Bug Fixes section
- `docs:`, `test:`, `chore:` → excluded from changelog

## Rollback

If a release has issues:

1. Delete the tag: `git tag -d v1.0.1 && git push origin :v1.0.1`
2. Delete the GitHub Release from the Releases page
3. npm: `npm unpublish ralph-engine@1.0.1` (within 72 hours)
4. Homebrew: Push a new tag — the tap auto-updates
