# GEMINI.md

Instructions for Gemini CLI. Full coding standards in [AGENTS.md](AGENTS.md).

## Project

ralph-engine — autonomous AI development loop engine (Go).

## Commands

```bash
go test ./... -count=1      # Test
go vet ./...                # Vet
go fmt ./...                # Format
golangci-lint run ./...     # Lint
go build -o bin/ralph-engine ./cmd/ralph-engine/  # Build
```

## Rules

- English only. TDD. Godoc on exports. Functions ≤ 20 lines.
- Conventional commits. No panics. Error wrapping mandatory.
- Cross-platform (Linux/macOS/Windows).
- See AGENTS.md for full standards.
