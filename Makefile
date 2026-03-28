# ralph-engine Makefile
# Build, test, and cross-compile the autonomous AI dev loop engine.

BINARY     := ralph-engine
VERSION    := $(shell git describe --tags --always --dirty 2>/dev/null || echo "dev")
BUILD_DIR  := bin
LDFLAGS    := -ldflags "-s -w -X github.com/diegorodrigo90/ralph-engine/internal/cli.Version=$(VERSION)"
GO         := go

.PHONY: all build test clean install lint fmt vet cross security check help

## help: Show this help message
help:
	@echo "ralph-engine $(VERSION)"
	@echo ""
	@echo "Usage:"
	@echo "  make build       Build binary for current platform"
	@echo "  make test        Run all tests"
	@echo "  make test-v      Run all tests (verbose)"
	@echo "  make test-race   Run tests with race detector"
	@echo "  make cover       Run tests with coverage report"
	@echo "  make lint        Run golangci-lint (ESLint equivalent)"
	@echo "  make fmt         Format code (Prettier equivalent)"
	@echo "  make vet         Run go vet"
	@echo "  make security    Run security scanners (gosec + govulncheck)"
	@echo "  make check       Run ALL checks (fmt + vet + lint + test + security + build)"
	@echo "  make clean       Remove build artifacts"
	@echo "  make install     Install to GOPATH/bin"
	@echo "  make cross       Cross-compile for all platforms"
	@echo ""

## all: Build and test
all: check build

## check: Run ALL validation (pre-commit equivalent)
check: fmt vet lint test-race security
	@echo "All checks passed."

## build: Compile binary for current OS/arch
build:
	@mkdir -p $(BUILD_DIR)
	$(GO) build $(LDFLAGS) -o $(BUILD_DIR)/$(BINARY) ./cmd/ralph-engine/

## test: Run all tests
test:
	$(GO) test ./... -count=1

## test-v: Run all tests with verbose output
test-v:
	$(GO) test ./... -count=1 -v

## test-race: Run tests with race detector
test-race:
	$(GO) test ./... -count=1 -race

## cover: Run tests with coverage
cover:
	$(GO) test ./... -count=1 -coverprofile=coverage.out
	$(GO) tool cover -html=coverage.out -o coverage.html
	@echo "Coverage report: coverage.html"

## fmt: Format Go source files (Prettier equivalent)
fmt:
	$(GO) fmt ./...
	@echo "Formatted."

## vet: Run go vet
vet:
	$(GO) vet ./...

## lint: Run golangci-lint (ESLint equivalent)
lint:
	@which golangci-lint > /dev/null 2>&1 \
		&& golangci-lint run ./... \
		|| echo "golangci-lint not installed. Install: go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest"

## security: Run security scanners
security:
	@echo "Running govulncheck (dependency CVEs)..."
	@which govulncheck > /dev/null 2>&1 \
		&& govulncheck ./... \
		|| echo "govulncheck not installed. Install: go install golang.org/x/vuln/cmd/govulncheck@latest"
	@echo "Running gosec (SAST)..."
	@which gosec > /dev/null 2>&1 \
		&& gosec -quiet ./... \
		|| echo "gosec not installed. Install: go install github.com/securego/gosec/v2/cmd/gosec@latest"

## clean: Remove build artifacts
clean:
	rm -rf $(BUILD_DIR) coverage.out coverage.html

## install: Install to GOPATH/bin
install:
	$(GO) install $(LDFLAGS) ./cmd/ralph-engine/

## cross: Cross-compile for Linux, macOS, Windows
cross:
	@mkdir -p $(BUILD_DIR)
	GOOS=linux   GOARCH=amd64 $(GO) build $(LDFLAGS) -o $(BUILD_DIR)/$(BINARY)-linux-amd64       ./cmd/ralph-engine/
	GOOS=linux   GOARCH=arm64 $(GO) build $(LDFLAGS) -o $(BUILD_DIR)/$(BINARY)-linux-arm64       ./cmd/ralph-engine/
	GOOS=darwin  GOARCH=amd64 $(GO) build $(LDFLAGS) -o $(BUILD_DIR)/$(BINARY)-darwin-amd64      ./cmd/ralph-engine/
	GOOS=darwin  GOARCH=arm64 $(GO) build $(LDFLAGS) -o $(BUILD_DIR)/$(BINARY)-darwin-arm64      ./cmd/ralph-engine/
	GOOS=windows GOARCH=amd64 $(GO) build $(LDFLAGS) -o $(BUILD_DIR)/$(BINARY)-windows-amd64.exe ./cmd/ralph-engine/
	@echo "Built binaries in $(BUILD_DIR)/"

## tools: Install development tools
tools:
	$(GO) install github.com/golangci/golangci-lint/cmd/golangci-lint@latest
	$(GO) install golang.org/x/vuln/cmd/govulncheck@latest
	$(GO) install github.com/securego/gosec/v2/cmd/gosec@latest
	@echo "Development tools installed."
