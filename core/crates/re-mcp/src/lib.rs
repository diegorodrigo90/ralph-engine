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
}

impl McpServerDescriptor {
    /// Creates a new immutable MCP server descriptor.
    #[must_use]
    pub const fn new(
        id: &'static str,
        plugin_id: &'static str,
        name: &'static str,
        transport: McpTransport,
    ) -> Self {
        Self {
            id,
            plugin_id,
            name,
            transport,
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
