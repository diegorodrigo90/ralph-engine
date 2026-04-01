# GitHub Copilot Instructions

This is ralph-engine — an autonomous AI development loop engine in Go.

See [AGENTS.md](../AGENTS.md) for full coding standards.

## Quick Reference

- Language: Go 1.24+, ALL English
- Test: `go test ./... -count=1` (TDD, table-driven)
- Lint: `golangci-lint run ./...` (21 linters)
- Build: `go build -o bin/ralph-engine ./cmd/ralph-engine/`
- Style: Godoc on exports, ≤ 20 line functions, error wrapping, no panics
- Commits: `type(scope): description` (conventional)
- Cross-platform: use build tags for OS-specific code
