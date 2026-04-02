# official/bmad

Workflow plugin for BMAD starter templates and prompt bundles.

## Surfaces

- Kind: `template`
- Runtime hooks:
  - `scaffold`
  - `prompt_assembly`
  - `prepare`
- Template contributions:
  - `official.bmad.starter`
- Prompt contributions:
  - `official.bmad.workflow`

## What it owns

- the BMAD starter template bundle under `template/`
- the reusable workflow prompt asset bundle
- localized plugin, template, and prompt metadata
- typed check and prompt descriptors exported by the crate

This plugin is the official workflow-oriented provider in the catalog. Template and prompt assets stay plugin-owned and can now be inspected or materialized directly through the CLI.
