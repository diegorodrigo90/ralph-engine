# Hooks

Hooks define custom quality gate steps that run at specific points in the engine loop. Defined in `.ralph-engine/hooks.yaml`.

## hooks.yaml Format

```yaml
preflight:
  steps:
    - name: "Check dependencies"
      run: "npm install"
      timeout: 120

pre_story:
  steps:
    - name: "Pull latest"
      run: "git pull --rebase"

quality_gates:
  steps:
    - name: "Unit tests"
      run: "npm test"
      timeout: 300
      required: true

    - name: "Type check"
      run: "npm run type-check"
      required: true

    - name: "Build"
      run: "npm run build"
      required: true

    - name: "Lint"
      run: "npm run lint"
      required: false

post_story:
  steps:
    - name: "Update docs"
      run: "npm run docs:generate"
      required: false

post_session:
  steps:
    - name: "Clean artifacts"
      run: "rm -rf .tmp/"
```

## Hook Phases

| Phase           | When it runs                    | Failure behavior               |
| --------------- | ------------------------------- | ------------------------------ |
| `preflight`     | Once, before the loop starts    | Blocks execution               |
| `pre_story`     | Before each story               | Blocks story                   |
| `quality_gates` | After each story implementation | `required: true` blocks commit |
| `post_story`    | After story commit              | Warning only                   |
| `post_session`  | Once, when engine stops         | Warning only                   |

## Step Properties

| Property   | Type     | Required | Description                                      |
| ---------- | -------- | -------- | ------------------------------------------------ |
| `name`     | string   | yes      | Human-readable step name                         |
| `run`      | string   | yes      | Shell command to execute                         |
| `timeout`  | int      | no       | Timeout in seconds (default: 120)                |
| `required` | bool     | no       | If true, failure blocks progress (default: true) |
| `paths`    | string[] | no       | Only run if changed files match these globs      |
| `ssh`      | bool     | no       | Run via remote executor _(not yet implemented)_  |

## Path-Based Filtering

Quality gates can run conditionally based on which files changed:

```yaml
quality_gates:
  steps:
    - name: "TypeScript tests"
      run: "pnpm test"
      paths:
        - "apps/**/*.ts"
        - "packages/**/*.ts"

    - name: "Python tests"
      run: "pytest"
      paths:
        - "**/*.py"

    - name: "Storybook tests"
      run: "npx test-storybook"
      paths:
        - "packages/ui/**"
        - "**/*.stories.tsx"
```

If no files match the `paths` glob, the step is skipped.

## Example: Monorepo with multiple languages

```yaml
preflight:
  steps:
    - name: "Install dependencies"
      run: "npm install"

quality_gates:
  steps:
    # Always run
    - name: "Type check"
      run: "npm run type-check"
      required: true

    # TypeScript only
    - name: "Unit tests"
      run: "npm test"
      paths: ["**/*.ts", "**/*.tsx"]
      required: true

    - name: "Build"
      run: "npm run build"
      paths: ["**/*.ts", "**/*.tsx"]
      required: true

    # Python only
    - name: "Python lint"
      run: "ruff check ."
      paths: ["**/*.py"]
      required: true

    - name: "Python tests"
      run: "pytest"
      paths: ["**/*.py"]
      required: true

    # E2E
    - name: "E2E tests"
      run: "npx playwright test"
      timeout: 600
      required: true

post_session:
  steps:
    - name: "Clean build cache"
      run: "npm run clean"
      required: false
```

## Status: Implementation Progress

> **Note:** Hook execution is defined but not yet wired into the engine loop. The hooks.yaml format is stable — your configs will work once execution is implemented. See [GitHub Issues](https://github.com/diegorodrigo90/ralph-engine/issues) for progress.
