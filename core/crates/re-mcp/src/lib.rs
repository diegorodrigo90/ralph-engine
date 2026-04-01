//! Shared MCP contribution contracts for Ralph Engine.

use std::fmt;

/// Supported MCP transport kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpTransport {
    /// Standard I/O transport.
    Stdio,
}

impl fmt::Display for McpTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stdio => f.write_str("stdio"),
        }
    }
}

/// Supported MCP process models.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpProcessModel {
    /// The plugin manages the server process internally.
    PluginManaged,
    /// The server runs as an external binary or service.
    ExternalBinary,
}

impl McpProcessModel {
    /// Returns the stable process model identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PluginManaged => "plugin_managed",
            Self::ExternalBinary => "external_binary",
        }
    }
}

impl fmt::Display for McpProcessModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Supported MCP availability policies.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpAvailability {
    /// The server may be started on demand by the runtime.
    OnDemand,
    /// The server requires explicit operator opt-in before activation.
    ExplicitOptIn,
}

impl McpAvailability {
    /// Returns the stable availability identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnDemand => "on_demand",
            Self::ExplicitOptIn => "explicit_opt_in",
        }
    }
}

impl fmt::Display for McpAvailability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Immutable metadata for an MCP server contribution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct McpServerDescriptor {
    /// Stable server identifier.
    pub id: &'static str,
    /// Owning plugin identifier.
    pub plugin_id: &'static str,
    /// Human-readable server name.
    pub name: &'static str,
    /// Declared transport kind.
    pub transport: McpTransport,
    /// Declared server process model.
    pub process_model: McpProcessModel,
    /// Declared runtime availability policy.
    pub availability: McpAvailability,
}

impl McpServerDescriptor {
    /// Creates a new immutable MCP server descriptor.
    #[must_use]
    pub const fn new(
        id: &'static str,
        plugin_id: &'static str,
        name: &'static str,
        transport: McpTransport,
        process_model: McpProcessModel,
        availability: McpAvailability,
    ) -> Self {
        Self {
            id,
            plugin_id,
            name,
            transport,
            process_model,
            availability,
        }
    }

    /// Returns whether the server identifier uses a namespace prefix.
    #[must_use]
    pub fn is_namespaced(&self) -> bool {
        self.id.contains('.')
    }

    /// Returns whether the server is attached to a namespaced plugin identifier.
    #[must_use]
    pub fn has_plugin_namespace(&self) -> bool {
        self.plugin_id.contains('.')
    }

    /// Returns whether the runtime may start the server on demand.
    #[must_use]
    pub fn is_on_demand(&self) -> bool {
        matches!(self.availability, McpAvailability::OnDemand)
    }

    /// Returns whether the server uses plugin-managed execution.
    #[must_use]
    pub fn is_plugin_managed(&self) -> bool {
        matches!(self.process_model, McpProcessModel::PluginManaged)
    }
}

/// Renders a human-readable MCP server listing.
#[must_use]
pub fn render_mcp_server_listing(servers: &[McpServerDescriptor]) -> String {
    let mut lines = Vec::with_capacity(servers.len() + 1);
    lines.push(format!("Official MCP servers ({})", servers.len()));

    for server in servers {
        lines.push(format!(
            "- {} | {} | {} | {}",
            server.id, server.name, server.plugin_id, server.transport
        ));
    }

    lines.join("\n")
}

/// Renders a human-readable MCP server detail block.
#[must_use]
pub fn render_mcp_server_detail(server: &McpServerDescriptor) -> String {
    format!(
        "MCP server: {}\nName: {}\nPlugin: {}\nTransport: {}\nProcess model: {}\nAvailability: {}",
        server.id,
        server.name,
        server.plugin_id,
        server.transport,
        server.process_model,
        server.availability
    )
}
