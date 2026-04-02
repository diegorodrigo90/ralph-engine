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
]);

function capabilityImportName(capability) {
  return CAPABILITY_IMPORT_NAMES.get(capability) || capability.toUpperCase();
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
  CAPABILITY_IMPORT_NAMES,
  CAPABILITY_RUNTIME_HOOKS,
  capabilityImportName,
  runtimeHooksForCapabilities,
};
