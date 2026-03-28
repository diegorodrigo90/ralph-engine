# ralph-engine

Autonomous AI development loop engine. Orchestrates CLI-based AI agent sessions (Claude Code, Codex, Aider, custom) with quality gates, resource monitoring, and progress persistence.

## Install

```bash
npm install -g ralph-engine
```

## Usage

```bash
# Initialize project config
ralph-engine init --preset basic

# Run the autonomous loop
ralph-engine run

# Dry run (show plan without executing)
ralph-engine run --dry-run
```

## Other install methods

- **curl**: `curl -fsSL https://raw.githubusercontent.com/diegorodrigo90/ralph-engine/main/scripts/install.sh | bash`
- **Go**: `go install github.com/diegorodrigo90/ralph-engine/cmd/ralph-engine@latest`
- **Homebrew**: `brew install diegorodrigo90/tap/ralph-engine`
- **Binary**: [GitHub Releases](https://github.com/diegorodrigo90/ralph-engine/releases)

## Documentation

Full documentation at [github.com/diegorodrigo90/ralph-engine](https://github.com/diegorodrigo90/ralph-engine).
