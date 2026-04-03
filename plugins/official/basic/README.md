# official.basic

Foundation plugin for starter templates.

## Overview

The Basic plugin provides the default starting point for new Ralph Engine projects. It ships a single starter template that creates the minimal `.ralph-engine/` directory structure needed to integrate with any AI coding assistant.

## What it creates

Running the materialize command creates:

- `.ralph-engine/config.yaml` — project configuration with plugin list, check definitions, and agent preferences
- `.ralph-engine/prompt.md` — project-specific prompt content that gets injected into agent sessions

These two files are the contract between your project and Ralph Engine. The config defines what runs; the prompt defines what the agent knows about your project.

## When to use

Use this plugin when:

- Starting a new project with Ralph Engine for the first time
- You want a minimal, unopinionated setup
- You plan to customize the configuration yourself

If you want a more opinionated setup with BMAD workflow integration, use `official.bmad` instead.

## Configuration

The generated `config.yaml` includes:

- Plugin list with `official.basic` enabled by default
- Doctor checks enabled for config validation
- Default agent runtime set to `official.claude` (can be changed to `official.codex` or `official.claudebox`)

## Customization

After materializing, edit `.ralph-engine/config.yaml` to:

- Enable additional plugins
- Configure check thresholds
- Set your preferred agent runtime
- Add project-specific MCP servers
