# Building from Source

## Prerequisites

| Dependency          | Version | Check            | Install                         |
| ------------------- | ------- | ---------------- | ------------------------------- |
| **Go**              | 1.24+   | `go version`     | [go.dev/dl](https://go.dev/dl/) |
| **Git**             | any     | `git --version`  | Your package manager            |
| **Make** (optional) | any     | `make --version` | Your package manager            |

### Installing Go

| OS            | Command                                                             |
| ------------- | ------------------------------------------------------------------- |
| Arch Linux    | `sudo pacman -S go`                                                 |
| Ubuntu/Debian | `sudo snap install go --classic` or [go.dev/dl](https://go.dev/dl/) |
| Fedora        | `sudo dnf install golang`                                           |
| macOS         | `brew install go`                                                   |
| Windows       | `winget install GoLang.Go` or [go.dev/dl](https://go.dev/dl/)       |

**Note:** Ubuntu's `apt install golang-go` may give an older version. Use snap or download from go.dev for 1.24+.

## Clone and build

```bash
git clone https://github.com/diegorodrigo90/ralph-engine.git
cd ralph-engine

# Quick build
go build -o bin/ralph-engine ./cmd/ralph-engine/

# Or use the build script
./scripts/build-local.sh

# Or use Make
make build
```

## Run tests

```bash
# All tests
go test ./... -count=1

# Verbose
go test ./... -count=1 -v

# Single package
go test ./internal/tracker/ -count=1 -v

# With race detector
go test ./... -count=1 -race

# With coverage
go test ./... -count=1 -coverprofile=coverage.out
go tool cover -html=coverage.out
```

## Lint

```bash
# Install golangci-lint (one-time)
go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest

# Run linter (21 rules configured in .golangci.yml)
golangci-lint run ./...

# Or via Make
make lint
```

## Cross-compile

ralph-engine compiles for 5 platform/arch combinations:

```bash
# macOS Apple Silicon
GOOS=darwin GOARCH=arm64 go build -o bin/ralph-engine-darwin-arm64 ./cmd/ralph-engine/

# macOS Intel
GOOS=darwin GOARCH=amd64 go build -o bin/ralph-engine-darwin-amd64 ./cmd/ralph-engine/

# Linux amd64
GOOS=linux GOARCH=amd64 go build -o bin/ralph-engine-linux-amd64 ./cmd/ralph-engine/

# Linux arm64
GOOS=linux GOARCH=arm64 go build -o bin/ralph-engine-linux-arm64 ./cmd/ralph-engine/

# Windows amd64
GOOS=windows GOARCH=amd64 go build -o bin/ralph-engine-windows-amd64.exe ./cmd/ralph-engine/

# Or all at once via Make
make cross
```

CGO is disabled (`CGO_ENABLED=0`) for all builds — no C compiler needed.

## Makefile targets

```bash
make build     # Build for current platform
make test      # Run tests
make cover     # Run tests with coverage report
make lint      # Run golangci-lint
make fmt       # Format code
make vet       # Run go vet
make security  # Run govulncheck + gosec
make check     # Run all checks (fmt + vet + lint + test + build)
make cross     # Cross-compile all platforms
make tools     # Install development tools
make clean     # Remove build artifacts
```

## Project structure

```
ralph-engine/
├── cmd/ralph-engine/main.go     # Entry point
├── internal/
│   ├── cli/                     # Cobra command tree (run, prepare, status, config, init)
│   ├── claude/                  # AI agent subprocess client + stream parser
│   ├── config/                  # 4-level config cascade (Viper)
│   ├── dashboard/               # Bubbletea TUI model/view/update
│   ├── deps/                    # Runtime dependency checker
│   ├── engine/                  # Core loop + prompt builder
│   ├── logger/                  # Structured logging (human + JSON formats)
│   ├── runner/                  # Circuit breaker (stagnation detection)
│   ├── security/                # First-run security notice
│   ├── ssh/                     # SSH health + self-healing
│   ├── state/                   # Persistent state.json (atomic writes)
│   ├── system/                  # Resource monitoring (cross-platform, build tags)
│   └── tracker/                 # Pluggable task tracking (file, flat, command)
├── presets/                     # Config templates (basic, bmad-v6, tdd-strict)
├── npm/                         # npm distribution wrapper
├── docs/                        # Documentation (this site)
├── scripts/                     # Install + build scripts
├── testdata/                    # Test fixtures
├── .golangci.yml                # Linter config (21 rules)
├── .goreleaser.yaml             # Cross-platform release automation
├── Makefile                     # Build automation
└── go.mod / go.sum              # Go module definition
```

## Version injection

The binary version is injected at build time via ldflags:

```bash
go build -ldflags "-X github.com/diegorodrigo90/ralph-engine/internal/cli.Version=1.2.3" -o bin/ralph-engine ./cmd/ralph-engine/
```

GoReleaser does this automatically from the git tag.

## Next steps

- [Creating a Release](releasing.md) — Tag, build, publish
- [Writing Plugins](../guides/plugins.md) — Add custom trackers and workflows
