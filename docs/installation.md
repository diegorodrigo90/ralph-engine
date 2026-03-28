# Installation

ralph-engine runs on Linux, macOS, and Windows (WSL2). Pick the method that fits your workflow — all are automatically updated on every release.

## npm (recommended for JS/TS developers)

```bash
# Global install
npm install -g ralph-engine

# Or run without installing
npx ralph-engine run --dry-run
```

**Requirements:** Node.js 16+

The npm package downloads the correct pre-built binary for your platform during install. No Go required.

## Homebrew (macOS and Linux)

```bash
brew install diegorodrigo90/tap/ralph-engine
```

Updates automatically with `brew upgrade`.

## curl (one-line install)

```bash
curl -fsSL https://raw.githubusercontent.com/diegorodrigo90/ralph-engine/main/scripts/install.sh | bash
```

Downloads the latest binary from GitHub Releases. Falls back to building from source if download fails.

**Installs to:** `/usr/local/bin/ralph-engine` (configurable via `INSTALL_DIR` env var).

## Go

```bash
go install github.com/diegorodrigo90/ralph-engine/cmd/ralph-engine@latest
```

**Requirements:** Go 1.24+

Binary goes to `$GOPATH/bin/`. Make sure it's in your `PATH`:

```bash
export PATH=$PATH:$(go env GOPATH)/bin
```

## Binary download

Download from [GitHub Releases](https://github.com/diegorodrigo90/ralph-engine/releases):

| Platform            | File                                       |
| ------------------- | ------------------------------------------ |
| Linux amd64         | `ralph-engine_VERSION_linux_amd64.tar.gz`  |
| Linux arm64         | `ralph-engine_VERSION_linux_arm64.tar.gz`  |
| macOS Intel         | `ralph-engine_VERSION_darwin_amd64.tar.gz` |
| macOS Apple Silicon | `ralph-engine_VERSION_darwin_arm64.tar.gz` |
| Windows amd64       | `ralph-engine_VERSION_windows_amd64.zip`   |

```bash
# Example: Linux amd64
curl -LO https://github.com/diegorodrigo90/ralph-engine/releases/latest/download/ralph-engine_0.1.0_linux_amd64.tar.gz
tar -xzf ralph-engine_0.1.0_linux_amd64.tar.gz
sudo mv ralph-engine /usr/local/bin/
```

## Verify installation

```bash
ralph-engine version
```

## Uninstall

```bash
# npm
npm uninstall -g ralph-engine

# Homebrew
brew uninstall ralph-engine

# Manual
rm $(which ralph-engine)
```

## Next steps

- [Quick Start](quickstart.md) — First run in 3 commands
- [Configuration](configuration.md) — Customize for your project
