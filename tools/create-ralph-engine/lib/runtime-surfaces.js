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
  "tui_extension",
  "context_manager",
  "agent_router",
  "preset",
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
  "context_management",
  "session_persistence",
  "agent_routing",
  "preset",
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
  ["tui_extension", ["tui_widgets"]],
  ["context_manager", ["context_management", "session_persistence"]],
  ["agent_router", ["agent_routing"]],
  ["preset", ["preset"]],
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
  ["context_management", "CONTEXT_MANAGEMENT"],
  ["session_persistence", "SESSION_PERSISTENCE"],
  ["agent_routing", "AGENT_ROUTING"],
  ["preset", "PRESET"],
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
  ["context_management", "PluginRuntimeHook::ContextManagement"],
  ["session_persistence", "PluginRuntimeHook::SessionPersistence"],
  ["agent_routing", "PluginRuntimeHook::AgentRouting"],
  ["preset", "PluginRuntimeHook::PresetApplication"],
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
  ["tui_extension", "PluginKind::TuiExtension"],
  ["context_manager", "PluginKind::ContextManager"],
  ["agent_router", "PluginKind::AgentRouter"],
  ["preset", "PluginKind::Preset"],
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
