# create-ralph-engine-plugin

Developer scaffolder for Ralph Engine plugins.

This package is the separate `npx create-ralph-engine-plugin` entrypoint so
plugin authors can scaffold plugin projects without turning scaffold generation
into a generic runtime concern.

The scaffolder only accepts plugin kinds and capabilities that already exist in
the typed Ralph Engine contracts. Future surfaces stay rejected until the core
runtime defines them explicitly.

## Usage

```bash
npx create-ralph-engine-plugin plugin jira-suite --publisher acme
```

Interactive mode works when running in a TTY without `--yes`. Non-interactive
mode is driven by flags and is suitable for automation.
