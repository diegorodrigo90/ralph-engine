# CLI Reference

The current Rust foundation exposes a minimal CLI surface while the runtime is rebuilt.

## Commands

```bash
ralph-engine
ralph-engine --locale <locale-id>
ralph-engine --version
ralph-engine --locale <locale-id> --version
ralph-engine agents
ralph-engine agents list
ralph-engine agents show <agent-id>
ralph-engine agents plan <agent-id>
ralph-engine capabilities
ralph-engine capabilities list
ralph-engine capabilities show <capability-id>
ralph-engine checks
ralph-engine checks list
ralph-engine checks show <check-id>
ralph-engine checks plan <check-id>
ralph-engine checks run <check-id>
ralph-engine doctor
ralph-engine doctor runtime
ralph-engine doctor config
ralph-engine doctor apply-config <output-path>
ralph-engine doctor write-config <output-path>
ralph-engine hooks
ralph-engine hooks list
ralph-engine hooks show <hook-id>
ralph-engine hooks plan <hook-id>
ralph-engine locales
ralph-engine locales list
ralph-engine locales show <locale-id>
ralph-engine policies
ralph-engine policies list
ralph-engine policies show <policy-id>
ralph-engine policies plan <policy-id>
ralph-engine providers
ralph-engine providers list
ralph-engine providers show <provider-id>
ralph-engine providers plan <provider-id>
ralph-engine config
ralph-engine config budgets
ralph-engine config layers
ralph-engine config locale
ralph-engine config show-budgets
ralph-engine config show-defaults
ralph-engine config show-layers
ralph-engine config show-locale
ralph-engine config show-mcp-server <server-id>
ralph-engine config show-plugin <plugin-id>
ralph-engine plugins
ralph-engine plugins list
ralph-engine plugins show <plugin-id>
ralph-engine runtime
ralph-engine runtime show
ralph-engine runtime status
ralph-engine runtime issues
ralph-engine runtime plan
ralph-engine runtime agent-plans
ralph-engine runtime provider-plans
ralph-engine runtime check-plans
ralph-engine runtime policy-plans
ralph-engine runtime mcp-plans
ralph-engine runtime patch
ralph-engine runtime patched-config
ralph-engine runtime apply-config <output-path>
ralph-engine runtime write-patched-config <output-path>
ralph-engine prompts
ralph-engine prompts list
ralph-engine prompts show <prompt-id>
ralph-engine prompts asset <prompt-id> <asset-path>
ralph-engine prompts materialize <prompt-id> <output-dir>
ralph-engine templates
ralph-engine templates list
ralph-engine templates show <template-id>
ralph-engine templates asset <template-id> <asset-path>
ralph-engine templates materialize <template-id> <output-dir>
ralph-engine mcp
ralph-engine mcp list
ralph-engine mcp show <server-id>
ralph-engine mcp plan <server-id>
```

The CLI also accepts the global `--locale <locale-id>` or `-L <locale-id>` flag so one invocation can switch language explicitly without depending on environment state. When no explicit flag is passed, the CLI still falls back to `RALPH_ENGINE_LOCALE` and then to the typed default locale contract.

The `plugins show` command prints the immutable plugin contract, including lifecycle, load boundary, runtime hooks, and resolved activation state.

The `agents` command family prints the typed agent runtime registry so official agent integrations stay explicit instead of hiding only inside generic capability listings.

The `agents plan` command prints the executable bootstrap plan for one typed agent runtime, so operator-facing startup steps stay attached to the agent surface that owns them instead of leaking only through aggregate runtime output.

The `capabilities` command family prints the typed runtime capability registry so capability providers remain explicit and modular.

The `templates` command family prints the typed runtime template registry so template providers stay explicit and separate from generic capability listings while scaffolding ownership remains tooling-owned.

The `templates materialize` command writes the embedded asset bundle owned by one typed template into an output directory, so official scaffolding remains explicit, plugin-owned, and inspectable instead of hiding behind implicit generator behavior.

The `prompts` command family prints the typed runtime prompt registry so prompt providers stay explicit and separate from generic capability listings while prompt assembly remains a modular runtime surface instead of implicit command-local behavior.

The `prompts materialize` command writes the embedded asset bundle owned by one typed prompt surface into an output directory, so reusable prompt assets remain explicit, plugin-owned, and executable instead of hiding behind ad hoc command behavior.

The `checks` command family prints the typed runtime check registry so prepare-time and doctor-time validation contributions stay explicit instead of hiding only as generic capabilities.

The `checks plan` command prints the typed execution plan for one runtime check contribution, so prepare and doctor execution steps stay attached to the check surface that owns them instead of leaking only through aggregate runtime output.

The `checks run` command executes one typed runtime check against the canonical resolved topology and returns a localized pass/fail result with the current runtime findings and remediation actions, so official check providers stop at being metadata and become an executable runtime surface.

The `doctor` command family prints the typed runtime diagnostic report by composing runtime status, unresolved issues, and remediation actions from one shared runtime snapshot instead of spreading diagnosis across ad hoc command logic.

The `doctor config` command renders the same owned project configuration that results from applying the runtime remediation patch to the current defaults, so the doctor flow can point directly at an inspectable remediation target instead of stopping at diagnosis.

The `doctor apply-config` command persists that same remediation target to one output path, so the diagnostic flow can produce a concrete next-step artifact instead of stopping at rendered YAML.

The `doctor write-config` command remains available as a compatibility alias for `doctor apply-config`.

The `config locale` command prints the typed default locale contract so CLI i18n stays inspectable instead of remaining implicit in the runtime defaults.

The `locales` command family prints the typed supported locale catalog so runtime locale coverage, native names, and English fallback rules stay explicit and versioned.

The `config budgets` command prints the typed prompt and context budget contract so token ceilings remain explicit in the runtime foundation instead of hiding in future provider-specific logic.

The `hooks` command family prints the typed runtime-hook registry so hook providers remain explicit and modular.

The `hooks plan` command prints the typed surface map owned by one runtime hook, so templates, prompts, agents, checks, providers, policies, and MCP registrations remain inspectable from the hook boundary that orchestrates them instead of leaking only through aggregate runtime output.

The `policies` command family prints the typed runtime policy registry so policy providers stay explicit, inspectable, and separate from generic capability listings.

The `policies plan` command prints the typed enforcement plan for one policy contribution, so guardrail execution steps stay attached to the policy surface that owns them instead of leaking only through aggregate runtime output.

The `providers` command family prints the typed runtime provider registry so data-source, context-provider, forge-provider, and remote-control contributions stay explicit instead of hiding only inside generic capability output.

The `providers plan` command prints the executable registration plan for one typed provider contribution, so provider-managed registration steps stay attached to the provider surface that owns them instead of leaking only through aggregate runtime output.

The `mcp show` command prints the typed MCP launch contract, including process model, launch policy, command boundaries, working-directory policy, environment policy, and availability.

The `mcp plan` command prints the typed MCP launch plan derived from that contract, so plugin-managed bootstrap and spawn-process execution stay reusable outside command-local formatting.

The `runtime show` command prints the resolved runtime topology, including effective plugin activation, capability registration, template registration, prompt registration, agent registration, check registration, provider registration, policy registration, runtime-hook registration, and MCP enablement.

The `runtime status` command prints the typed runtime health summary, including enabled and disabled providers across plugins, capabilities, templates, prompts, agent registrations, check registrations, provider registrations, policies, runtime hooks, and MCP servers.

The `runtime issues` command prints the typed list of unresolved runtime issues and recommended actions, including disabled template, prompt, agent, check, provider, policy, and runtime-hook registrations, instead of requiring command-local heuristics.

The `runtime plan` command prints the typed runtime remediation plan derived from the resolved topology, including template-provider, prompt-provider, agent-provider, check-provider, provider, policy-provider, and hook-provider enablement, so the next enablement step stays explicit and modular instead of being inferred ad hoc in the CLI.

The `runtime agent-plans` command prints the executable agent bootstrap plans that remain enabled in the resolved runtime snapshot, so operational agent startup steps stay visible next to topology, health, issues, and remediation.

The `runtime provider-plans` command prints the executable provider registration plans that remain enabled in the resolved runtime snapshot, so runtime-managed provider registration stays visible next to topology, health, issues, remediation, and agent bootstrap.

The `runtime check-plans` command prints the executable runtime-check plans that remain enabled in the resolved runtime snapshot, so prepare and doctor execution steps stay visible next to topology, health, issues, remediation, and other runtime plans.

The `runtime policy-plans` command prints the executable policy-enforcement plans that remain enabled in the resolved runtime snapshot, so guardrail enforcement steps stay visible next to topology, health, issues, remediation, and other runtime plans.

The `runtime mcp-plans` command prints the executable MCP launch plans that remain enabled in the resolved runtime snapshot, so operational launch steps stay visible next to topology, health, issues, and remediation.

The `runtime patch` command renders the typed configuration patch that would remediate the current degraded topology, including plugin activation entries and per-server MCP enablement, so runtime recovery stays explicit and reusable instead of remaining only a textual plan.

The `runtime patched-config` command renders the owned project configuration produced by applying the typed runtime patch to the current defaults, so operators can inspect the effective remediation result before persisting it elsewhere.

The `runtime apply-config` command persists that fully materialized remediation target to one output path, so runtime recovery can move from inspection into an explicit, repeatable file-writing step.

The `runtime write-patched-config` command remains available as a compatibility alias for `runtime apply-config`.
