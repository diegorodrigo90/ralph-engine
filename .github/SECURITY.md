# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| latest  | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

**Do NOT open a public issue for security vulnerabilities.**

Instead, please report security issues via:

1. **GitHub Security Advisories** (preferred): https://github.com/diegorodrigo90/ralph-engine/security/advisories/new

Please include:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

We will respond within 48 hours and aim to release a fix within 7 days for critical issues.

## Security Measures

ralph-engine includes several security measures:

1. **CI Security Scanning**: Every PR is scanned by gosec (SAST), govulncheck (dependency CVEs), and Trivy (filesystem scan)
2. **No secrets in code**: Gitleaks pre-commit hook prevents accidental secret commits
3. **Dependency auditing**: `govulncheck` runs on every CI build
4. **Minimal permissions**: GitHub Actions use least-privilege permissions
5. **Signed releases**: GoReleaser produces checksums for binary verification
6. **Code review required**: All PRs require review before merge

## Security Design

- ralph-engine **never manages API billing or credentials**
- The engine only invokes user-configured agent binaries
- Container isolation is recommended for production use
- First-run security notice requires explicit acceptance
- No network requests except through user-configured agents
