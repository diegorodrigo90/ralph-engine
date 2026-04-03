# official.hello-world

Minimal example plugin — use as a reference for building your own.

## Overview

This is the simplest possible Ralph Engine plugin. It demonstrates the plugin architecture with a single starter template and nothing else. If you're learning how plugins work, start here.

## What it does

Ships a single starter template (`official.hello-world.starter`) that creates a basic `.ralph-engine/` directory with:

- `config.yaml` — minimal project configuration
- `hooks.yaml` — empty hook definitions
- `prompt.md` — placeholder project prompt

## How to use

```
ralph-engine templates materialize official.hello-world.starter ./my-project
```

## Why it exists

This plugin serves as a living reference for the Plugin Development tutorial. Every file in this directory maps directly to a section in the docs. When you scaffold a new plugin with `npx create-ralph-engine-plugin`, the generated code follows this same structure.

## Files explained

| File | Purpose |
|------|---------|
| `manifest.yaml` | Plugin metadata — ID, kind, capabilities, i18n |
| `src/lib.rs` | Plugin descriptor + PluginRuntime trait implementation |
| `src/i18n/mod.rs` | Includes build-generated i18n code |
| `build.rs` | Reads locales/*.toml and generates i18n Rust code |
| `locales/en.toml` | English translations |
| `locales/pt-br.toml` | Portuguese translations |
| `template/` | Files materialized by the starter template |
| `Cargo.toml` | Rust crate manifest with workspace deps |
