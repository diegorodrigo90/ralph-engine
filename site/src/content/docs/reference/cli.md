---
title: "CLI Commands"
description: "Complete CLI command reference"
---

All commands accept `--locale <locale-id>` (or `-L <locale-id>`) to switch language for a single invocation. Without it, the CLI resolves locale from `RALPH_ENGINE_LOCALE`, then OS locale (`LC_ALL`, `LC_MESSAGES`, `LANG`), then defaults to English.

## Global Options

Print help:

```bash
ralph-engine --help
```

Print version:

```bash
ralph-engine --version
```

Override locale for one invocation:

```bash
ralph-engine --locale <locale-id>
```

## agents

Manage typed agent runtime registrations.

```bash
ralph-engine agents list
```

List all registered agent runtimes.

```bash
ralph-engine agents show <agent-id>
```

Show the typed agent contract for one agent runtime.

```bash
ralph-engine agents plan <agent-id>
```

Show the executable bootstrap plan for one agent runtime.

```bash
ralph-engine agents launch <agent-id>
```

Dispatch to the plugin's `PluginRuntime.bootstrap_agent()` implementation. For agent-runtime plugins (claude, claudebox, codex), this probes for the agent binary on PATH.

## capabilities

Inspect the runtime capability registry.

```bash
ralph-engine capabilities list
```

List all registered capabilities across plugins.

```bash
ralph-engine capabilities show <capability-id>
```

Show details for one capability.

## checks

Manage runtime validation checks (prepare-time and doctor-time).

```bash
ralph-engine checks list
```

List all registered checks.

```bash
ralph-engine checks show <check-id>
```

Show details for one check.

```bash
ralph-engine checks plan <check-id>
```

Show the execution plan for one check.

```bash
ralph-engine checks run <check-id>
```

Execute one check against the resolved topology. Returns a localized pass/fail result with findings and remediation actions.

```bash
ralph-engine checks asset <check-id> <asset-path>
```

Print a specific asset from a check's embedded bundle.

```bash
ralph-engine checks materialize <check-id> <output-dir>
```

Write the check's embedded asset bundle to an output directory.

## config

Inspect configuration contracts.

```bash
ralph-engine config show-defaults
```

Print the full default project config (YAML).

```bash
ralph-engine config locale
```

Print the default locale contract.

```bash
ralph-engine config budgets
```

Print prompt and context token budget ceilings.

```bash
ralph-engine config layers
```

Print the configuration resolution stack.

```bash
ralph-engine config show-plugin <plugin-id>
```

Print resolved config for one plugin with provenance.

```bash
ralph-engine config show-mcp-server <server-id>
```

Print resolved config for one MCP server with provenance.

## doctor

Diagnose and remediate project configuration.

```bash
ralph-engine doctor
```

Print a diagnostic report composing runtime status, unresolved issues, and remediation actions.

```bash
ralph-engine doctor runtime
```

Print the runtime component of the diagnostic report.

```bash
ralph-engine doctor config
```

Render the project configuration that results from applying remediation to current defaults.

```bash
ralph-engine doctor apply-config <output-path>
```

Persist the remediation config to a file.

```bash
ralph-engine doctor write-config <output-path>
```

Compatibility alias for `doctor apply-config`.

## hooks

Inspect runtime lifecycle hook registrations.

```bash
ralph-engine hooks list
```

List all registered hooks.

```bash
ralph-engine hooks show <hook-id>
```

Show details for one hook.

```bash
ralph-engine hooks plan <hook-id>
```

Show the surface map for one hook (which templates, prompts, agents, checks, providers, policies, and MCP registrations it owns).

## locales

Inspect supported locale catalog.

```bash
ralph-engine locales list
```

List all supported locales with native names.

```bash
ralph-engine locales show <locale-id>
```

Show locale details including native name and English fallback rules.

## mcp

Manage Model Context Protocol server registrations.

```bash
ralph-engine mcp list
```

List all registered MCP servers.

```bash
ralph-engine mcp show <server-id>
```

Print the typed MCP launch contract (process model, launch policy, command boundaries, working-directory policy, environment policy, availability).

```bash
ralph-engine mcp plan <server-id>
```

Print the typed MCP launch plan derived from the contract.

```bash
ralph-engine mcp launch <server-id>
```

Validate and optionally start an MCP server. `SpawnProcess` servers are spawned in foreground. `PluginRuntime` servers dispatch to `register_mcp_server()` for readiness check.

```bash
ralph-engine mcp status
```

Evaluate launch readiness for all registered MCP servers (readiness, health, enabled state, transport, issues, actions).

```bash
ralph-engine mcp status <server-id>
```

Show detailed status for one specific MCP server.

## plugins

Inspect the plugin registry.

```bash
ralph-engine plugins list
```

List all registered plugins.

```bash
ralph-engine plugins show <plugin-id>
```

Print the immutable plugin contract (lifecycle, load boundary, runtime hooks, resolved activation state).

## policies

Inspect runtime policy registrations.

```bash
ralph-engine policies list
```

List all registered policies.

```bash
ralph-engine policies show <policy-id>
```

Show details for one policy.

```bash
ralph-engine policies plan <policy-id>
```

Show the enforcement plan for one policy.

```bash
ralph-engine policies run <policy-id>
```

Execute one policy enforcement. Returns a localized pass/fail result.

```bash
ralph-engine policies asset <policy-id> <asset-path>
```

Print a specific asset from a policy's embedded bundle.

```bash
ralph-engine policies materialize <policy-id> <output-dir>
```

Write the policy's embedded asset bundle to an output directory.

## prompts

Manage runtime prompt registrations.

```bash
ralph-engine prompts list
```

List all registered prompts.

```bash
ralph-engine prompts show <prompt-id>
```

Show details for one prompt.

```bash
ralph-engine prompts asset <prompt-id> <asset-path>
```

Print a specific asset from a prompt's embedded bundle.

```bash
ralph-engine prompts materialize <prompt-id> <output-dir>
```

Write the prompt's embedded asset bundle to an output directory.

## providers

Inspect runtime provider registrations (data sources, context providers, forge providers, remote control).

```bash
ralph-engine providers list
```

List all registered providers.

```bash
ralph-engine providers show <provider-id>
```

Show details for one provider.

```bash
ralph-engine providers plan <provider-id>
```

Show the registration plan for one provider.

## runtime

Inspect and remediate the resolved runtime topology.

```bash
ralph-engine runtime show
```

Print the resolved runtime topology (plugin activation, capability registration, template/prompt/agent/check/provider/policy/hook registration, MCP enablement).

```bash
ralph-engine runtime status
```

Print the runtime health summary (enabled/disabled counts across all registration types).

```bash
ralph-engine runtime issues
```

Print unresolved runtime issues and recommended actions.

```bash
ralph-engine runtime plan
```

Print the runtime remediation plan (enablement steps for all provider types).

```bash
ralph-engine runtime agent-plans
```

Print executable agent bootstrap plans for enabled agents.

```bash
ralph-engine runtime provider-plans
```

Print executable provider registration plans for enabled providers.

```bash
ralph-engine runtime check-plans
```

Print executable check plans for enabled checks.

```bash
ralph-engine runtime policy-plans
```

Print executable policy enforcement plans for enabled policies.

```bash
ralph-engine runtime mcp-plans
```

Print executable MCP launch plans for enabled servers.

```bash
ralph-engine runtime patch
```

Render the configuration patch that would remediate the current degraded topology.

```bash
ralph-engine runtime patched-config
```

Render the project configuration produced by applying the patch to current defaults.

```bash
ralph-engine runtime apply-config <output-path>
```

Persist the patched configuration to a file.

```bash
ralph-engine runtime write-patched-config <output-path>
```

Compatibility alias for `runtime apply-config`.

## templates

Manage runtime template registrations.

```bash
ralph-engine templates list
```

List all registered templates.

```bash
ralph-engine templates show <template-id>
```

Show details for one template.

```bash
ralph-engine templates asset <template-id> <asset-path>
```

Print a specific asset from a template's embedded bundle.

```bash
ralph-engine templates scaffold <template-id> <output-dir>
```

Write the template's embedded asset bundle to an output directory. Alias for `templates materialize`.

```bash
ralph-engine templates materialize <template-id> <output-dir>
```

Write the template's embedded asset bundle to an output directory.
