# create-ralph-engine-plugin

Developer scaffolder for Ralph Engine plugins.

This package is the separate `npx create-ralph-engine-plugin` entrypoint so
plugin authors can scaffold plugin projects without turning scaffold generation
into a generic runtime concern.

## Usage

```bash
npx create-ralph-engine-plugin plugin jira-suite --publisher acme
```

Interactive mode works when running in a TTY without `--yes`. Non-interactive
mode is driven by flags and is suitable for automation.
