//! Immutable built-in catalog for official plugins, MCP contributions, and runtime topology.

use re_config::{
    ConfigScope, PluginActivation, ResolvedPluginConfig, default_project_config_layer,
    resolve_plugin_config,
};
use re_core::{RuntimeMcpRegistration, RuntimePhase, RuntimePluginRegistration, RuntimeTopology};
use re_mcp::McpServerDescriptor;
use re_plugin::PluginDescriptor;

/// Returns the immutable catalog of official plugins.
#[must_use]
pub fn official_plugins() -> [PluginDescriptor; 8] {
    [
        re_plugin_basic::descriptor(),
        re_plugin_bmad::descriptor(),
        re_plugin_claude::descriptor(),
        re_plugin_claudebox::descriptor(),
        re_plugin_codex::descriptor(),
        re_plugin_github::descriptor(),
        re_plugin_ssh::descriptor(),
        re_plugin_tdd_strict::descriptor(),
    ]
}

/// Returns one immutable official plugin descriptor by identifier.
#[must_use]
pub fn find_official_plugin(plugin_id: &str) -> Option<PluginDescriptor> {
    official_plugins()
        .into_iter()
        .find(|plugin| plugin.id == plugin_id)
}

/// Returns the immutable catalog of official MCP server contributions.
#[must_use]
pub fn official_mcp_servers() -> [McpServerDescriptor; 4] {
    [
        re_plugin_claude::mcp_servers()[0],
        re_plugin_claudebox::mcp_servers()[0],
        re_plugin_codex::mcp_servers()[0],
        re_plugin_github::mcp_servers()[0],
    ]
}

/// Returns one immutable official MCP server descriptor by identifier.
#[must_use]
pub fn find_official_mcp_server(server_id: &str) -> Option<McpServerDescriptor> {
    official_mcp_servers()
        .into_iter()
        .find(|server| server.id == server_id)
}

fn resolved_plugin_entry(plugin: PluginDescriptor) -> ResolvedPluginConfig {
    let layers = [default_project_config_layer()];

    resolve_plugin_config(&layers, plugin.id).unwrap_or(ResolvedPluginConfig::new(
        plugin.id,
        PluginActivation::Disabled,
        ConfigScope::BuiltInDefaults,
    ))
}

fn resolved_plugin_entry_by_id(plugin_id: &'static str) -> ResolvedPluginConfig {
    find_official_plugin(plugin_id)
        .map(resolved_plugin_entry)
        .unwrap_or(ResolvedPluginConfig::new(
            plugin_id,
            PluginActivation::Disabled,
            ConfigScope::BuiltInDefaults,
        ))
}

/// Returns the resolved runtime plugin registrations for the official catalog.
#[must_use]
pub fn official_runtime_plugins() -> [RuntimePluginRegistration; 8] {
    let plugins = official_plugins();

    plugins.map(|plugin| {
        let resolved = resolved_plugin_entry(plugin);

        RuntimePluginRegistration::new(plugin, resolved.activation, resolved.resolved_from)
    })
}

/// Returns the resolved runtime MCP registrations for the official catalog.
#[must_use]
pub fn official_runtime_mcp_registrations() -> [RuntimeMcpRegistration; 4] {
    let servers = official_mcp_servers();

    servers.map(|server| {
        let resolved = resolved_plugin_entry_by_id(server.plugin_id);
        let enabled = matches!(resolved.activation, PluginActivation::Enabled);

        RuntimeMcpRegistration::new(server, enabled)
    })
}

/// Returns the resolved runtime topology for the official catalog.
#[must_use]
pub fn official_runtime_topology<'a>(
    plugins: &'a [RuntimePluginRegistration],
    mcp_servers: &'a [RuntimeMcpRegistration],
) -> RuntimeTopology<'a> {
    RuntimeTopology {
        phase: RuntimePhase::Ready,
        locale: default_project_config_layer().config.default_locale,
        plugins,
        mcp_servers,
    }
}
