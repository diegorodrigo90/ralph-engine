use super::RuntimeLocaleCatalog;

pub(super) const LOCALE: RuntimeLocaleCatalog = RuntimeLocaleCatalog {
    runtime_phase: "Runtime phase",
    runtime_health: "Runtime health",
    locale: "Locale",
    plugins: "Plugins",
    capabilities: "Capabilities",
    templates: "Templates",
    prompts: "Prompts",
    agent_runtimes: "Agent runtimes",
    checks: "Checks",
    providers: "Providers",
    policies: "Policies",
    runtime_hooks: "Runtime hooks",
    mcp_servers: "MCP servers",
    runtime_mcp_launch_plans: "Runtime MCP launch plans",
    runtime_issues: "Runtime issues",
    runtime_action_plan: "Runtime action plan",
    runtime_doctor: "Runtime doctor",
    translate_runtime_reason,
};

fn translate_runtime_reason(reason: &str) -> String {
    reason.to_owned()
}
