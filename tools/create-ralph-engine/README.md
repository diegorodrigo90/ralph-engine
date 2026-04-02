# create-ralph-engine-plugin

Developer scaffolder for Ralph Engine plugins.

This package is the separate `npx create-ralph-engine-plugin` entrypoint so
plugin authors can scaffold plugin projects without turning scaffold generation
into a generic runtime concern.

The scaffolder only accepts plugin kinds and capabilities that already exist in
the typed Ralph Engine contracts. Future surfaces stay rejected until the core
runtime defines them explicitly.

Scaffolded plugin identifiers use the same dotted namespace contract as the
runtime and official manifests, for example `acme.jira-suite`.

The generated `manifest.yaml` follows the versioned manifest contract shipped in
`schema/plugin-manifest.schema.json`, and the scaffolder validates that manifest
before writing it to disk.

The generated Rust crate keeps locale-aware plugin metadata under `src/i18n/`
so new plugin projects start with the same additive locale structure used by
the runtime and official plugins.

## Usage

```bash
npx create-ralph-engine-plugin plugin jira-suite --publisher acme
```

Interactive mode works when running in a TTY without `--yes`. Non-interactive
mode is driven by flags and is suitable for automation.

The scaffolder resolves user-facing CLI text through locale catalogs and
currently supports `en` plus `pt-br` via `RALPH_ENGINE_LOCALE`, falling back to
English when an unsupported locale is requested.
