# CLI Reference

The current Rust foundation exposes a minimal CLI surface while the runtime is rebuilt.

## Commands

```bash
ralph-engine
ralph-engine --version
ralph-engine agents
ralph-engine agents list
ralph-engine agents show <plugin-id>
ralph-engine capabilities
ralph-engine capabilities list
ralph-engine capabilities show <capability-id>
ralph-engine checks
ralph-engine checks list
ralph-engine checks show <check-id>
ralph-engine doctor
ralph-engine doctor runtime
ralph-engine hooks
ralph-engine hooks list
ralph-engine hooks show <hook-id>
ralph-engine policies
ralph-engine policies list
ralph-engine policies show <policy-id>
ralph-engine providers
ralph-engine providers list
ralph-engine providers show <provider-id>
ralph-engine config
ralph-engine config budgets
ralph-engine config layers
ralph-engine config show-budgets
ralph-engine config show-defaults
ralph-engine config show-layers
ralph-engine config show-plugin <plugin-id>
ralph-engine plugins
ralph-engine plugins list
ralph-engine plugins show <plugin-id>
ralph-engine runtime
ralph-engine runtime show
ralph-engine runtime status
ralph-engine runtime issues
ralph-engine runtime plan
ralph-engine templates
ralph-engine templates list
ralph-engine templates show <plugin-id>
ralph-engine mcp
ralph-engine mcp list
ralph-engine mcp show <server-id>
```

The `plugins show` command prints the immutable plugin contract, including lifecycle, load boundary, runtime hooks, and resolved activation state.

The `agents` command family prints the typed agent runtime registry so official agent integrations stay explicit instead of hiding only inside generic capability listings.

The `capabilities` command family prints the typed runtime capability registry so capability providers remain explicit and modular.

The `templates` command family prints the typed runtime template registry so template providers stay explicit and separate from generic capability listings while scaffolding ownership remains tooling-owned.

The `checks` command family prints the typed runtime check registry so prepare-time and doctor-time validation contributions stay explicit instead of hiding only as generic capabilities.

The `doctor` command family prints the typed runtime diagnostic report by composing runtime status, unresolved issues, and remediation actions from one shared runtime snapshot instead of spreading diagnosis across ad hoc command logic.

The `config budgets` command prints the typed prompt and context budget contract so token ceilings remain explicit in the runtime foundation instead of hiding in future provider-specific logic.

The `hooks` command family prints the typed runtime-hook registry so hook providers remain explicit and modular.

The `policies` command family prints the typed runtime policy registry so policy providers stay explicit, inspectable, and separate from generic capability listings.

The `providers` command family prints the typed runtime provider registry so data-source, context-provider, forge-provider, and remote-control contributions stay explicit instead of hiding only inside generic capability output.

The `mcp show` command prints the typed MCP launch contract, including process model, launch policy, command boundaries, working-directory policy, environment policy, and availability.

The `runtime show` command prints the resolved runtime topology, including effective plugin activation, capability registration, template registration, agent registration, check registration, provider registration, policy registration, runtime-hook registration, and MCP enablement.

The `runtime status` command prints the typed runtime health summary, including enabled and disabled providers across plugins, capabilities, templates, agent registrations, check registrations, provider registrations, policies, runtime hooks, and MCP servers.

The `runtime issues` command prints the typed list of unresolved runtime issues and recommended actions, including disabled template, agent, check, provider, policy, and runtime-hook registrations, instead of requiring command-local heuristics.

The `runtime plan` command prints the typed runtime remediation plan derived from the resolved topology, including template-provider, agent-provider, check-provider, provider, policy-provider, and hook-provider enablement, so the next enablement step stays explicit and modular instead of being inferred ad hoc in the CLI.
