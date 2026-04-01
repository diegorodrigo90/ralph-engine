//! Immutable built-in catalog for official plugins and MCP contributions.

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
