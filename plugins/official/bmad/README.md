# official.bmad

Workflow plugin for BMAD scaffolding and prompts.

## Overview

Full BMAD Method integration for Ralph Engine. The BMAD (Business-Method-Architecture-Development) method is a structured approach to AI-assisted software development with specialized agents for each phase: analysis, planning, architecture, implementation, and review.

## What it ships

- A starter template with pre-configured BMAD agents and workflow definitions
- Prompt fragments that get assembled into agent sessions for BMAD workflows
- Prepare-time checks that validate your project is ready for BMAD workflows
- Doctor checks that diagnose BMAD configuration health

## How to use

Start a new BMAD project:

```
ralph-engine templates materialize official.bmad.starter ./my-project
```

This creates a `.ralph-engine/` directory with BMAD-specific configuration including agent definitions, workflow templates, and prompt bundles.

## Checks

The plugin includes two check types:

- `official.bmad.prepare` — validates that required files exist and configuration is valid before starting a workflow
- `official.bmad.doctor` — comprehensive diagnostic that checks agent definitions, prompt integrity, and workflow consistency

## When to use

Use this plugin when:

- You want structured AI-assisted development with defined phases
- Your team follows a sprint-based workflow with stories and acceptance criteria
- You need multiple specialized agents (analyst, architect, PM, dev, QA)

For a simpler setup without BMAD, use `official.basic` instead.
