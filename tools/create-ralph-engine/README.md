# create-ralph-engine

Developer scaffolder for Ralph Engine plugins and related starter assets.

This package is the separate `npx create-ralph-engine` entrypoint so runtime
users do not need to install or think about authoring workflows.

## Usage

```bash
npx create-ralph-engine plugin jira-suite --publisher acme
```

Interactive mode works when running in a TTY without `--yes`. Non-interactive
mode is driven by flags and is suitable for automation.
