# official/tdd-strict

Strict TDD policy plugin with starter-template guardrails.

## Surfaces

- Kind: `policy`
- Runtime hooks:
  - `scaffold`
  - `policy_enforcement`
- Template contributions:
  - `official.tdd-strict.starter`
- Policy contributions:
  - `official.tdd.strict`

## What it owns

- the strict starter template bundle under `template/`
- the typed policy descriptor and enforcement surface
- localized plugin, template, and policy metadata

This plugin is the catalog guardrail provider for strict TDD flows. It combines a starter template with typed policy enforcement instead of leaving those rules implicit in ad hoc tooling.
