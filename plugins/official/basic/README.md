# official/basic

Foundation plugin for starter project templates.

## Surfaces

- Kind: `template`
- Runtime hook: `scaffold`
- Template contributions:
  - `official.basic.starter`

## What it owns

- the baseline starter template bundle under `template/`
- localized plugin and template metadata
- the typed template descriptor exported by the crate

This plugin is the minimal official provider for project bootstrap assets. It owns starter-template content; it does not turn generic scaffolding into a core-runtime responsibility.
