const SUPPORTED_KINDS = new Set([
  "agent_runtime",
  "forge_provider",
  "context_provider",
  "data_source",
  "template",
  "remote_control",
  "mcp_contribution",
  "policy",
  "workflow",
]);

const SUPPORTED_CAPABILITIES = new Set([
  "agent_runtime",
  "data_source",
  "context_provider",
  "forge_provider",
  "doctor_checks",
  "prepare_checks",
  "prompt_fragments",
  "template",
  "remote_control",
  "mcp_contribution",
  "policy",
  "workflow",
  "tui_widgets",
]);

const DEFAULT_CAPABILITIES_BY_KIND = new Map([
  ["template", ["template"]],
  ["agent_runtime", ["agent_runtime"]],
  ["remote_control", ["remote_control"]],
  ["mcp_contribution", ["mcp_contribution"]],
  ["forge_provider", ["forge_provider"]],
  ["context_provider", ["context_provider"]],
  ["data_source", ["data_source"]],
  ["policy", ["policy"]],
  ["workflow", ["workflow"]],
]);

const CAPABILITY_IMPORT_NAMES = new Map([
  ["template", "TEMPLATE"],
  ["prompt_fragments", "PROMPT_FRAGMENTS"],
  ["prepare_checks", "PREPARE_CHECKS"],
  ["doctor_checks", "DOCTOR_CHECKS"],
  ["agent_runtime", "AGENT_RUNTIME"],
  ["mcp_contribution", "MCP_CONTRIBUTION"],
  ["data_source", "DATA_SOURCE"],
  ["context_provider", "CONTEXT_PROVIDER"],
  ["forge_provider", "FORGE_PROVIDER"],
  ["remote_control", "REMOTE_CONTROL"],
  ["policy", "POLICY"],
  ["workflow", "WORKFLOW"],
  ["tui_widgets", "TUI_WIDGETS"],
]);

const CAPABILITY_RUNTIME_HOOKS = new Map([
  ["template", "PluginRuntimeHook::Scaffold"],
  ["prompt_fragments", "PluginRuntimeHook::PromptAssembly"],
  ["prepare_checks", "PluginRuntimeHook::Prepare"],
  ["doctor_checks", "PluginRuntimeHook::Doctor"],
  ["agent_runtime", "PluginRuntimeHook::AgentBootstrap"],
  ["mcp_contribution", "PluginRuntimeHook::McpRegistration"],
  ["data_source", "PluginRuntimeHook::DataSourceRegistration"],
  ["context_provider", "PluginRuntimeHook::ContextProviderRegistration"],
  ["forge_provider", "PluginRuntimeHook::ForgeProviderRegistration"],
  ["remote_control", "PluginRuntimeHook::RemoteControlBootstrap"],
  ["policy", "PluginRuntimeHook::PolicyEnforcement"],
  ["workflow", "PluginRuntimeHook::WorkItemResolution"],
  ["tui_widgets", "PluginRuntimeHook::TuiContribution"],
]);

const KIND_VARIANTS = new Map([
  ["template", "PluginKind::Template"],
  ["agent_runtime", "PluginKind::AgentRuntime"],
  ["forge_provider", "PluginKind::ForgeProvider"],
  ["context_provider", "PluginKind::ContextProvider"],
  ["data_source", "PluginKind::DataSource"],
  ["remote_control", "PluginKind::RemoteControl"],
  ["mcp_contribution", "PluginKind::McpContribution"],
  ["policy", "PluginKind::Policy"],
  ["workflow", "PluginKind::Workflow"],
]);

function capabilityImportName(capability) {
  return CAPABILITY_IMPORT_NAMES.get(capability) || capability.toUpperCase();
}

function defaultCapabilitiesForKind(kind) {
  return [...(DEFAULT_CAPABILITIES_BY_KIND.get(kind) || [])];
}

function pluginKindVariant(kind) {
  return KIND_VARIANTS.get(kind) || "PluginKind::McpContribution";
}

function runtimeHooksForCapabilities(capabilities) {
  const hooks = new Set();

  for (const capability of capabilities) {
    const hook = CAPABILITY_RUNTIME_HOOKS.get(capability);
    if (hook) {
      hooks.add(hook);
    }
  }

  return [...hooks];
}

module.exports = {
  DEFAULT_CAPABILITIES_BY_KIND,
  KIND_VARIANTS,
  SUPPORTED_CAPABILITIES,
  SUPPORTED_KINDS,
  CAPABILITY_IMPORT_NAMES,
  CAPABILITY_RUNTIME_HOOKS,
  capabilityImportName,
  defaultCapabilitiesForKind,
  pluginKindVariant,
  runtimeHooksForCapabilities,
};
