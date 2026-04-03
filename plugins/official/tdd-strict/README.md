# official.tdd-strict

Strict TDD policy and template guardrails.

## Overview

Enforces strict Test-Driven Development discipline in Ralph Engine projects. This plugin combines a starter template pre-configured with TDD guardrails and a policy that validates the RED-GREEN-REFACTOR cycle is followed for every acceptance criterion.

## What it enforces

The TDD strict policy validates that:

- Tests are written before implementation (RED phase)
- Each acceptance criterion has at least one corresponding test
- Test names follow naming conventions
- The RED-GREEN-REFACTOR cycle is documented in the development record

## How to use

Start a TDD-strict project:

```
ralph-engine templates materialize official.tdd-strict.starter ./my-project
```

Or enable the policy in an existing project by adding `official.tdd-strict` to your plugin list in `.ralph-engine/config.yaml`.

## Template vs Policy

This plugin ships two things:

- **Starter template** (`official.tdd-strict.starter`) — creates a project pre-configured with TDD guardrails enabled
- **Policy** (`official.tdd-strict.guardrails`) — the enforcement rules that can be added to any existing project

You can use the template for new projects, or just enable the policy in an existing project that uses `official.basic` or `official.bmad` as its base.

## When to use

Use this plugin when:

- Your team practices TDD and wants automated enforcement
- You want to ensure AI-generated code always has tests written first
- Code review should verify that tests exist for every acceptance criterion
- You want to prevent "I'll add tests later" from ever happening

## Combining with other plugins

TDD strict works alongside any agent runtime (Claude, Codex, Claude Box) and alongside the BMAD workflow plugin. The policy enforcement happens at check time, not at agent level.
