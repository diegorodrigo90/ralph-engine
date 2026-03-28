# Contributing to ralph-engine

Thank you for your interest in contributing! This guide covers everything from fixing a typo to adding a major feature.

**New to open source?** That's great — everyone starts somewhere. This guide is written with you in mind. If anything is unclear, [open an issue](https://github.com/diegorodrigo90/ralph-engine/issues/new) and we'll help.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Making Your First Pull Request](#making-your-first-pull-request)
- [Code Standards](#code-standards)
- [Adding Features](#adding-features)
- [Commit Messages](#commit-messages)
- [Code Review](#code-review)

## Getting Started

### Prerequisites

- [Go 1.24+](https://go.dev/dl/) — [install guide](https://go.dev/doc/install)
- [Git](https://git-scm.com/downloads)
- A code editor (VS Code, GoLand, Cursor, etc.)

### Setup (3 commands)

```bash
# 1. Fork and clone
git clone https://github.com/YOUR_USERNAME/ralph-engine.git
cd ralph-engine

# 2. Verify it builds
go build -o bin/ralph-engine ./cmd/ralph-engine/

# 3. Run tests
go test ./... -count=1
```

If all tests pass, you're ready to contribute!

### Optional tools

```bash
# Linter (recommended)
go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest

# Security scanner
go install golang.org/x/vuln/cmd/govulncheck@latest
```

## Development Workflow

### 1. Find something to work on

- Browse [open issues](https://github.com/diegorodrigo90/ralph-engine/issues)
- Look for [`good first issue`](https://github.com/diegorodrigo90/ralph-engine/labels/good%20first%20issue) — these are beginner-friendly
- Look for [`help wanted`](https://github.com/diegorodrigo90/ralph-engine/labels/help%20wanted) — the maintainer needs help here

**Before starting work:** Comment on the issue to let others know you're working on it. This prevents duplicate effort.

### 2. Create a branch

```bash
git checkout -b feat/my-feature   # New feature
git checkout -b fix/my-bugfix     # Bug fix
git checkout -b docs/my-update    # Documentation
```

Branch naming convention:

- `feat/description` — New features
- `fix/description` — Bug fixes
- `docs/description` — Documentation
- `test/description` — Test improvements
- `refactor/description` — Code refactoring

### 3. Write code (TDD)

We follow Test-Driven Development:

```bash
# 1. Write a failing test
go test ./internal/mypackage/ -count=1 -v   # RED — should fail

# 2. Write minimal code to pass
go test ./internal/mypackage/ -count=1 -v   # GREEN — should pass

# 3. Refactor if needed
go test ./internal/mypackage/ -count=1 -v   # Still GREEN
```

### 4. Verify before committing

```bash
go fmt ./...                     # Format code
go vet ./...                     # Static analysis
go test ./... -count=1           # All tests pass
go build ./cmd/ralph-engine/     # Binary compiles

# Cross-platform check (important!)
GOOS=windows GOARCH=amd64 go build -o /dev/null ./cmd/ralph-engine/
GOOS=darwin GOARCH=arm64 go build -o /dev/null ./cmd/ralph-engine/
```

### 5. Commit

```bash
git add -A
git commit -m "feat(tracker): add GitHub Issues tracker"
```

See [Commit Messages](#commit-messages) for the format.

## Making Your First Pull Request

### Step-by-step

1. **Push your branch:**

   ```bash
   git push origin feat/my-feature
   ```

2. **Open a PR on GitHub:**
   - Go to your fork on GitHub
   - Click "Compare & pull request"
   - Fill in the PR template

3. **Wait for CI:**
   - Tests run on Linux, macOS, and Windows
   - Linter checks code quality
   - Security scanners check for vulnerabilities
   - All checks must pass (green ✓)

4. **Address review feedback:**
   - Maintainers may suggest changes
   - Push new commits to the same branch
   - CI re-runs automatically

5. **Merge:**
   - Once approved, a maintainer will merge your PR
   - Your contribution is now part of ralph-engine!

### PR tips

- **Keep PRs small** — One feature or fix per PR. Easier to review.
- **Write a clear description** — What changed and why.
- **Link the issue** — Add `Closes #123` in the PR description.
- **Add tests** — Every feature needs tests. Every bug fix needs a regression test.
- **Update docs** — If you change user-facing behavior, update README.md.

## Code Standards

All rules follow [EARS syntax](https://en.wikipedia.org/wiki/Easy_Approach_to_Requirements_Syntax) (SHALL keyword):

### Language

- ALL code, comments, docs, tests, commit messages SHALL be in **English**.

### Go conventions

- Exported types, functions, methods, interfaces SHALL have **Godoc comments** ending with a period.
- Functions SHALL have **≤ 20 lines** and **≤ 3 parameters**.
- Errors SHALL be wrapped with context: `fmt.Errorf("doing X: %w", err)`.
- Library code SHALL NOT call `panic()`. Return errors instead.
- Tests SHALL be **table-driven** and test **behavior**, not implementation.
- Test helpers SHALL call `t.Helper()`.
- Interfaces SHALL be defined in the **consumer** package, not the provider.
- OS-specific code SHALL use **build tags** (`//go:build !windows`).

### File organization

```
internal/mypackage/
├── mypackage.go          # Main implementation
├── mypackage_test.go     # Tests
├── helper.go             # Internal helpers (if needed)
└── helper_unix.go        # OS-specific (with build tag)
```

## Adding Features

### Adding a new tracker

Trackers let ralph-engine read stories from different sources.

1. Create `internal/tracker/github_tracker.go`:

   ```go
   package tracker

   // GitHubTracker reads stories from GitHub Issues.
   type GitHubTracker struct {
       owner string
       repo  string
   }

   func (gt *GitHubTracker) NextStory() (*Story, error) {
       // Implementation here.
   }
   // ... implement all TaskTracker methods
   ```

2. Create `internal/tracker/github_tracker_test.go` with table-driven tests.

3. Register in the tracker registry.

4. Update README.md trackers section.

### Adding a new agent

Agents are AI coding tools that ralph-engine can orchestrate.

1. Extend `ClientConfig` in `internal/claude/client.go`
2. Add agent-specific flag building in `buildArgs()`
3. Add tests for the new argument format

### Adding a new workflow

Workflows define the development process for each story.

1. Add a case to `sessionInstructions()` in `internal/engine/prompt.go`
2. Add a preset in `internal/config/config.go`
3. Add tests for prompt generation

## Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description
```

### Types

| Type       | When                                                |
| ---------- | --------------------------------------------------- |
| `feat`     | New feature                                         |
| `fix`      | Bug fix                                             |
| `docs`     | Documentation only                                  |
| `test`     | Adding or updating tests                            |
| `refactor` | Code change that doesn't fix a bug or add a feature |
| `perf`     | Performance improvement                             |
| `ci`       | CI/CD changes                                       |
| `chore`    | Maintenance (deps, configs)                         |
| `build`    | Build system changes                                |

### Examples

```
feat(tracker): add GitHub Issues tracker
fix(engine): prevent nil dereference when tracker returns empty
test(ssh): add reconnection timeout test
docs: add PR workflow to CONTRIBUTING.md
refactor(state): extract atomic write helper
chore(deps): update cobra to v1.11
```

### Rules

- Subject line SHALL be ≤ 72 characters.
- Use imperative mood: "add feature", not "added feature".
- Scope is optional but recommended (package name).
- Body is optional — use for complex changes.

## Code Review

### What we look for

- **Tests** — Does the PR include tests? Do they test behavior?
- **Cross-platform** — Will this work on Windows/macOS/Linux?
- **Error handling** — Are errors wrapped with context?
- **Naming** — Are names clear and idiomatic Go?
- **Godoc** — Are exported symbols documented?
- **Security** — No hardcoded secrets, no command injection?

### Review timeline

- We aim to review PRs within **48 hours**.
- Complex PRs may take longer.
- If you haven't heard back in a week, ping us in the PR.

## Questions?

- [Open an issue](https://github.com/diegorodrigo90/ralph-engine/issues/new) for questions
- Check existing issues and discussions first

Thank you for contributing! Every improvement helps make autonomous AI development safer and more reliable.
