# create-ralph-engine-plugin

[![CI](https://github.com/diegorodrigo90/ralph-engine/actions/workflows/ci.yml/badge.svg)](https://github.com/diegorodrigo90/ralph-engine/actions/workflows/ci.yml)
[![Sonar Quality Gate](https://sonarcloud.io/api/project_badges/measure?project=ralph-engine_ralph-engine&metric=alert_status)](https://sonarcloud.io/project/overview?id=ralph-engine_ralph-engine)
[![Coverage](https://sonarcloud.io/api/project_badges/measure?project=ralph-engine_ralph-engine&metric=coverage)](https://sonarcloud.io/project/overview?id=ralph-engine_ralph-engine)
[![npx](https://img.shields.io/badge/npx-create--ralph--engine--plugin-blue.svg)](https://github.com/diegorodrigo90/ralph-engine/tree/main/tools/create-ralph-engine)
[![Node](https://img.shields.io/badge/node-20.19%2B-339933.svg)](package.json)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSE)
[![Repository](https://img.shields.io/badge/source-ralph--engine-black)](https://github.com/diegorodrigo90/ralph-engine/tree/main/tools/create-ralph-engine)

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

## What it gives you

- namespaced plugin id scaffold such as `acme.jira-suite`
- versioned `manifest.yaml` aligned with the runtime contract
- Rust crate with per-locale metadata catalogs in `src/i18n/`
- generated runtime hooks based on reviewed capability contracts
- validation before writing the scaffold to disk

## Usage

```bash
npx create-ralph-engine-plugin plugin jira-suite --publisher acme
```

Interactive mode works when running in a TTY without `--yes`. Non-interactive
mode is driven by flags and is suitable for automation.

The scaffolder resolves user-facing CLI text through locale catalogs and
currently supports `en` plus `pt-br` via `RALPH_ENGINE_LOCALE`, falling back to
English when an unsupported locale is requested. That locale resolution now
follows the same typed shared locale contract owned by `re-config`, so runtime,
official plugins, and scaffolded plugins add new locales on one canonical base.

Generated Rust plugin crates also start with per-locale catalog modules in
`src/i18n/`, so localized names and summaries can grow by adding locale files
instead of rewriting runtime-facing handlers.

## Open Source Notes

- this package is the only supported scaffold entrypoint for plugin creation
- future plugin kinds or capabilities stay rejected until the core runtime defines them
- locale growth is additive; `en` remains the fallback when a locale is missing
- the generated manifest and crate layout are designed to stay compatible with the typed Rust contracts in the main repository
