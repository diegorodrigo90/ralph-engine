# Security Audit — Ralph Engine

Run a comprehensive security audit across all distribution surfaces: Rust binary, npm packages, Homebrew formula, GitHub Actions, dependencies, and plugin ecosystem.

## Pre-flight

Before running checks, verify tools are available:
```bash
cargo audit --version && cargo deny --version && echo "Rust tools OK" || echo "MISSING: run ./scripts/install-dev-tools.sh"
```

## 1. Rust Binary Security

### 1.1 Vulnerability scan
```bash
cargo audit
```
Report any CVEs found. CRITICAL and HIGH must be fixed before release.

### 1.2 License compliance
```bash
cargo deny check licenses
```
Only MIT, Apache-2.0, BSD-2/3-Clause, ISC, CC0-1.0, Unicode-3.0, CDLA-Permissive-2.0 are allowed.

### 1.3 Dependency sources
```bash
cargo deny check sources
```
No unknown registries or git dependencies allowed.

### 1.4 Ban check
```bash
cargo deny check bans
```
No wildcard dependencies. Report multiple-version warnings.

### 1.5 Unsafe code
Verify `unsafe_code = "forbid"` is in workspace Cargo.toml:
```bash
grep 'unsafe_code.*forbid' Cargo.toml && echo "PASS" || echo "FAIL: unsafe_code not forbidden"
```

### 1.6 Dependency freshness
Check for outdated dependencies with known issues:
```bash
cargo outdated --workspace --depth 1 2>/dev/null || echo "cargo-outdated not installed (optional)"
```

## 2. Secret Scanning

### 2.1 Gitleaks
```bash
gitleaks detect --source . --no-banner 2>&1 | tail -5
```

### 2.2 Hardcoded secrets grep
```bash
grep -rn "AKIA\|sk-\|ghp_\|glpat-\|password\s*=" core/ plugins/ --include="*.rs" | grep -v "test\|example\|mock" | head -10
```

## 3. GitHub Actions Security

### 3.1 SHA pinning
All actions must be pinned to full 40-char SHA:
```bash
bash scripts/check-i18n-compliance.sh 2>/dev/null; grep -rn 'uses:' .github/workflows/*.yml | grep -v '@[a-f0-9]\{40\}' | grep -v '#' | head -10
```
Any output = FAIL. All actions must use `owner/action@SHA # vX.Y.Z` format.

### 3.2 Harden-runner
Every job in release.yml should have harden-runner as first step:
```bash
grep -c "harden-runner" .github/workflows/release.yml
```

### 3.3 GITHUB_TOKEN permissions
Check for overly permissive token scopes:
```bash
grep -A 3 "permissions:" .github/workflows/*.yml | head -20
```

## 4. npm Package Security

### 4.1 npm audit
```bash
npm audit --audit-level high 2>/dev/null || echo "Run from project root with node_modules"
```

### 4.2 Lockfile integrity
```bash
test -f package-lock.json && echo "PASS: lockfile exists" || echo "WARN: no lockfile"
test -f pnpm-lock.yaml && echo "PASS: pnpm lockfile exists" || echo "INFO: no pnpm lockfile"
```

### 4.3 Package verification scripts
```bash
test -f scripts/verify-npm-release.mjs && echo "PASS" || echo "FAIL: missing verify script"
test -f scripts/verify-npm-install.mjs && echo "PASS" || echo "FAIL: missing verify script"
```

## 5. Homebrew Security

### 5.1 Formula verification
```bash
test -f scripts/verify-homebrew-formula.sh && echo "PASS" || echo "FAIL: missing verify script"
test -f scripts/render-homebrew-formula.mjs && echo "PASS" || echo "FAIL: missing render script"
```

### 5.2 SHA256 verification in formula template
```bash
grep -c "sha256" packaging/homebrew/*.rb.tmpl 2>/dev/null || echo "WARN: no formula template"
```

## 6. Filesystem Scan

### 6.1 Trivy
```bash
trivy fs . --severity CRITICAL,HIGH --skip-dirs target,node_modules,.git 2>&1 | tail -20
```

## 7. Plugin Ecosystem Security

### 7.1 Official plugin manifests validated
```bash
npm run contracts:verify 2>&1 | tail -3
```

### 7.2 Community plugin trust model
- Check that community plugins are installed to `.ralph-engine/plugins/` (sandboxed)
- Verify manifest.yaml validation exists in install command
- Check that community plugins cannot override core keybindings or access terminal raw mode

### 7.3 Plugin capabilities boundary
```bash
grep -c "tui_widgets\|workflow" core/crates/re-official/src/lib.rs | head -5
```
Verify hook-only capabilities are classified correctly.

## 8. Release Artifact Security

### 8.1 Release verification scripts
```bash
test -f scripts/verify-release-assets.sh && echo "PASS" || echo "FAIL"
test -f scripts/verify-release-readiness.sh && echo "PASS" || echo "FAIL"
test -f scripts/release-check.sh && echo "PASS" || echo "FAIL"
```

### 8.2 Build-once promote-later model
Verify release.yml uses artifacts from CI, not rebuilds:
```bash
grep -c "download-artifact\|upload-artifact" .github/workflows/release.yml
```

## Output Format

For each section, report:
- **PASS**: check passed with details
- **FAIL**: specific violation with file:line and remediation
- **WARN**: potential issue that needs manual review
- **SKIP**: tool not available (suggest install command)

Summarize with counts: X passed, Y failed, Z warnings, W skipped.
