# Hooks

Hooks define generic runtime triggers and step execution in `.ralph-engine/hooks.yaml`.

Ralph Engine uses hooks as a generic mechanism. The core runtime does **not** assign workflow meaning to trigger names beyond the technical trigger points it supports.

If a workflow needs concepts such as `prepare`, `review`, or `validation`, those meanings belong to the plugin or boilerplate that generates and validates the project configuration.

## Runtime Triggers

The core currently recognizes these runtime triggers:

| Trigger         | When it runs                              |
| --------------- | ----------------------------------------- |
| `session_start` | Once before the loop starts               |
| `work_item_start` | Before the current work item begins     |
| `after_agent`   | After the agent session completes         |
| `work_item_finish` | After the work item completes successfully |
| `session_end`   | Once when the engine stops                |

## hooks.yaml Format

```yaml
session_start:
  steps:
    - name: "Bootstrap environment"
      run: "npm install"
      required: true

work_item_start:
  steps:
    - name: "Refresh generated files"
      run: "npm run codegen"
      required: false

after_agent:
  steps:
    - name: "Unit tests"
      run: "npm test"
      required: true
      paths: ["src/**"]

    - name: "Build"
      run: "npm run build"
      required: true

work_item_finish:
  steps:
    - name: "Update docs"
      run: "npm run docs:generate"
      required: false

session_end:
  steps:
    - name: "Clean temporary files"
      run: "npm run clean"
      required: false
```

## Step Properties

| Property   | Type     | Required | Description                                              |
| ---------- | -------- | -------- | -------------------------------------------------------- |
| `name`     | string   | yes      | Human-readable step name                                 |
| `run`      | string   | yes      | Shell command to execute                                 |
| `timeout`  | string   | no       | Duration string, e.g. `"5m"` or `"30s"`                  |
| `required` | bool     | no       | If true, failure blocks progress                         |
| `paths`    | string[] | no       | Only run if changed files match these globs              |

## Path Filtering

Steps under `after_agent` can run conditionally based on changed files:

```yaml
after_agent:
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
```

If no changed file matches the configured globs, the step is skipped.

## Workflow Ownership

The core runtime owns:

- loading `.ralph-engine/hooks.yaml`
- running trigger steps
- path filtering
- timeout handling
- required-step blocking

Plugins or boilerplates own:

- deciding which files and triggers a workflow requires
- deciding what a trigger means in that workflow
- validating whether the project is ready for that workflow

## Cross-Platform Behavior

Commands in `run` are executed through the built-in shell abstraction:

- Linux/macOS: `sh -c`
- Windows: `cmd /c`

This keeps trigger execution cross-platform without hardcoding shell paths in the config.
