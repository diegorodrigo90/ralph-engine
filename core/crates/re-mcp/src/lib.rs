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

/// Supported MCP working-directory policies.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpWorkingDirectoryPolicy {
    /// The runtime chooses the working directory internally.
    RuntimeManaged,
    /// The server expects the current project root as working directory.
    ProjectRoot,
    /// The server expects the owning plugin workspace as working directory.
    PluginWorkspace,
}

impl McpWorkingDirectoryPolicy {
    /// Returns the stable working-directory policy identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeManaged => "runtime_managed",
            Self::ProjectRoot => "project_root",
            Self::PluginWorkspace => "plugin_workspace",
        }
    }
}

impl fmt::Display for McpWorkingDirectoryPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Supported MCP environment policies.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpEnvironmentPolicy {
    /// The runtime passes a minimal inherited environment.
    MinimalRuntime,
    /// The runtime passes plugin-scoped environment variables.
    PluginScoped,
}

impl McpEnvironmentPolicy {
    /// Returns the stable environment policy identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MinimalRuntime => "minimal_runtime",
            Self::PluginScoped => "plugin_scoped",
        }
    }
}

impl fmt::Display for McpEnvironmentPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Typed process invocation contract for externally spawned MCP servers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct McpCommandDescriptor {
    /// Stable executable or script entrypoint.
    pub program: &'static str,
    /// Stable process arguments.
    pub args: &'static [&'static str],
    /// Working-directory policy used for the process launch.
    pub working_directory: McpWorkingDirectoryPolicy,
    /// Environment policy used for the process launch.
    pub environment: McpEnvironmentPolicy,
}

impl McpCommandDescriptor {
    /// Creates a new immutable MCP process invocation contract.
    #[must_use]
    pub const fn new(
        program: &'static str,
        args: &'static [&'static str],
        working_directory: McpWorkingDirectoryPolicy,
        environment: McpEnvironmentPolicy,
    ) -> Self {
        Self {
            program,
            args,
            working_directory,
            environment,
        }
    }

    /// Returns whether the command declares at least one argument.
    #[must_use]
    pub fn has_args(&self) -> bool {
        !self.args.is_empty()
    }

    /// Renders a stable human-readable process invocation.
    #[must_use]
    pub fn render_invocation(&self) -> String {
        if self.args.is_empty() {
            self.program.to_owned()
        } else {
            format!("{} {}", self.program, self.args.join(" "))
        }
    }
}

/// Supported MCP launch policies.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpLaunchPolicy {
    /// The plugin runtime owns bootstrap and process orchestration.
    PluginRuntime,
    /// The runtime spawns an external binary using a typed command contract.
    SpawnProcess(McpCommandDescriptor),
}

impl McpLaunchPolicy {
    /// Returns the stable launch-policy identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PluginRuntime => "plugin_runtime",
            Self::SpawnProcess(_) => "spawn_process",
        }
    }

    /// Returns the derived process model for this launch policy.
    #[must_use]
    pub const fn process_model(self) -> McpProcessModel {
        match self {
            Self::PluginRuntime => McpProcessModel::PluginManaged,
            Self::SpawnProcess(_) => McpProcessModel::ExternalBinary,
        }
    }

    /// Returns the typed command descriptor when process spawning is explicit.
    #[must_use]
    pub const fn command(self) -> Option<McpCommandDescriptor> {
        match self {
            Self::PluginRuntime => None,
            Self::SpawnProcess(command) => Some(command),
        }
    }
}

impl fmt::Display for McpLaunchPolicy {
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
    /// Declared server launch policy.
    pub launch_policy: McpLaunchPolicy,
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
        launch_policy: McpLaunchPolicy,
        availability: McpAvailability,
    ) -> Self {
        Self {
            id,
            plugin_id,
            name,
            transport,
            launch_policy,
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
        matches!(self.launch_policy, McpLaunchPolicy::PluginRuntime)
    }

    /// Returns the derived process model for this server.
    #[must_use]
    pub const fn process_model(&self) -> McpProcessModel {
        self.launch_policy.process_model()
    }

    /// Returns the typed spawn command when the runtime launches a process directly.
    #[must_use]
    pub const fn command(&self) -> Option<McpCommandDescriptor> {
        self.launch_policy.command()
    }
}

/// Renders a human-readable MCP server listing.
#[must_use]
pub fn render_mcp_server_listing(servers: &[McpServerDescriptor]) -> String {
    render_mcp_server_listing_for_locale(servers, "en")
}

/// Renders a human-readable MCP server listing for one locale.
#[must_use]
pub fn render_mcp_server_listing_for_locale(
    servers: &[McpServerDescriptor],
    locale: &str,
) -> String {
    let mut lines = Vec::with_capacity(servers.len() + 1);
    lines.push(format!(
        "{} ({})",
        if locale.eq_ignore_ascii_case("pt-br") {
            "Servidores MCP oficiais"
        } else {
            "Official MCP servers"
        },
        servers.len()
    ));

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
    render_mcp_server_detail_for_locale(server, "en")
}

/// Renders a human-readable MCP server detail block for one locale.
#[must_use]
pub fn render_mcp_server_detail_for_locale(server: &McpServerDescriptor, locale: &str) -> String {
    let mut lines = vec![
        format!(
            "{}: {}",
            if locale.eq_ignore_ascii_case("pt-br") {
                "Servidor MCP"
            } else {
                "MCP server"
            },
            server.id
        ),
        format!(
            "{}: {}",
            if locale.eq_ignore_ascii_case("pt-br") {
                "Nome"
            } else {
                "Name"
            },
            server.name
        ),
        format!("{}: {}", "Plugin", server.plugin_id),
        format!(
            "{}: {}",
            if locale.eq_ignore_ascii_case("pt-br") {
                "Transporte"
            } else {
                "Transport"
            },
            server.transport
        ),
        format!(
            "{}: {}",
            if locale.eq_ignore_ascii_case("pt-br") {
                "Modelo de processo"
            } else {
                "Process model"
            },
            server.process_model()
        ),
        format!(
            "{}: {}",
            if locale.eq_ignore_ascii_case("pt-br") {
                "Política de launch"
            } else {
                "Launch policy"
            },
            server.launch_policy
        ),
        format!(
            "{}: {}",
            if locale.eq_ignore_ascii_case("pt-br") {
                "Disponibilidade"
            } else {
                "Availability"
            },
            server.availability
        ),
    ];

    match server.command() {
        Some(command) => {
            lines.push(format!(
                "{}: {}",
                if locale.eq_ignore_ascii_case("pt-br") {
                    "Comando"
                } else {
                    "Command"
                },
                command.render_invocation()
            ));
            lines.push(format!(
                "{}: {}",
                if locale.eq_ignore_ascii_case("pt-br") {
                    "Diretório de trabalho"
                } else {
                    "Working directory"
                },
                command.working_directory
            ));
            lines.push(format!(
                "{}: {}",
                if locale.eq_ignore_ascii_case("pt-br") {
                    "Ambiente"
                } else {
                    "Environment"
                },
                command.environment
            ));
        }
        None => {
            lines.push(format!(
                "{}: {}",
                if locale.eq_ignore_ascii_case("pt-br") {
                    "Comando"
                } else {
                    "Command"
                },
                if locale.eq_ignore_ascii_case("pt-br") {
                    "gerenciado pelo runtime do plugin"
                } else {
                    "managed by plugin runtime"
                }
            ));
            lines.push(format!(
                "{}: runtime_managed",
                if locale.eq_ignore_ascii_case("pt-br") {
                    "Diretório de trabalho"
                } else {
                    "Working directory"
                }
            ));
            lines.push(format!(
                "{}: minimal_runtime",
                if locale.eq_ignore_ascii_case("pt-br") {
                    "Ambiente"
                } else {
                    "Environment"
                }
            ));
        }
    }

    lines.join("\n")
}
