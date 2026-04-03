//! Shared MCP contribution contracts for Ralph Engine.

use std::fmt;

mod i18n;

/// Generates an enum with `as_str()` and `Display` from a single declaration.
macro_rules! define_mcp_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident => $str_val:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )+
        }

        impl $name {
            /// Returns the stable string identifier for this variant.
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $str_val,)+
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

/// One localized MCP text entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct McpLocalizedText {
    /// Stable locale identifier.
    pub locale: &'static str,
    /// Localized text value.
    pub value: &'static str,
}

impl McpLocalizedText {
    /// Creates a new immutable localized MCP text entry.
    #[must_use]
    pub const fn new(locale: &'static str, value: &'static str) -> Self {
        Self { locale, value }
    }
}

fn resolve_localized_text<'a>(
    localized_values: &'a [McpLocalizedText],
    locale: &str,
    fallback: &'a str,
) -> &'a str {
    let locale = re_config::resolve_supported_locale_or_default(locale.trim()).as_str();

    localized_values
        .iter()
        .find(|entry| entry.locale == locale)
        .map_or(fallback, |entry| entry.value)
}

define_mcp_enum! {
    /// Supported MCP transport kinds.
    pub enum McpTransport {
        /// Standard I/O transport.
        Stdio => "stdio",
    }
}

define_mcp_enum! {
    /// Supported MCP process models.
    pub enum McpProcessModel {
        /// The plugin manages the server process internally.
        PluginManaged => "plugin_managed",
        /// The server runs as an external binary or service.
        ExternalBinary => "external_binary",
    }
}

define_mcp_enum! {
    /// Supported MCP working-directory policies.
    pub enum McpWorkingDirectoryPolicy {
        /// The runtime chooses the working directory internally.
        RuntimeManaged => "runtime_managed",
        /// The server expects the current project root as working directory.
        ProjectRoot => "project_root",
        /// The server expects the owning plugin workspace as working directory.
        PluginWorkspace => "plugin_workspace",
    }
}

define_mcp_enum! {
    /// Supported MCP environment policies.
    pub enum McpEnvironmentPolicy {
        /// The runtime passes a minimal inherited environment.
        MinimalRuntime => "minimal_runtime",
        /// The runtime passes plugin-scoped environment variables.
        PluginScoped => "plugin_scoped",
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

define_mcp_enum! {
    /// Supported MCP availability policies.
    pub enum McpAvailability {
        /// The server may be started on demand by the runtime.
        OnDemand => "on_demand",
        /// The server requires explicit operator opt-in before activation.
        ExplicitOptIn => "explicit_opt_in",
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
    /// Optional localized server names keyed by locale.
    pub localized_names: &'static [McpLocalizedText],
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
        localized_names: &'static [McpLocalizedText],
        transport: McpTransport,
        launch_policy: McpLaunchPolicy,
        availability: McpAvailability,
    ) -> Self {
        Self {
            id,
            plugin_id,
            name,
            localized_names,
            transport,
            launch_policy,
            availability,
        }
    }

    /// Resolves the display name for a locale with English fallback.
    #[must_use]
    pub fn display_name_for_locale(&self, locale: &str) -> &'static str {
        resolve_localized_text(self.localized_names, locale, self.name)
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

/// One typed execution step for an MCP launch plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpLaunchStep {
    /// The plugin runtime owns bootstrap and process orchestration.
    PluginRuntimeBootstrap {
        /// Owning plugin identifier.
        plugin_id: &'static str,
    },
    /// The runtime spawns an external process from a typed command contract.
    SpawnProcess {
        /// Typed command contract.
        command: McpCommandDescriptor,
    },
}

impl McpLaunchStep {
    /// Returns the stable launch-step identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PluginRuntimeBootstrap { .. } => "plugin_runtime_bootstrap",
            Self::SpawnProcess { .. } => "spawn_process",
        }
    }
}

impl fmt::Display for McpLaunchStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Immutable execution plan derived from one MCP server descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct McpLaunchPlan {
    /// Target server identifier.
    pub server_id: &'static str,
    /// Owning plugin identifier.
    pub plugin_id: &'static str,
    /// Declared transport kind.
    pub transport: McpTransport,
    /// Derived launch step.
    pub step: McpLaunchStep,
    /// Availability policy preserved for operator-facing decisions.
    pub availability: McpAvailability,
}

impl McpLaunchPlan {
    /// Returns the typed spawn command when direct process launch is required.
    #[must_use]
    pub const fn command(self) -> Option<McpCommandDescriptor> {
        match self.step {
            McpLaunchStep::PluginRuntimeBootstrap { .. } => None,
            McpLaunchStep::SpawnProcess { command } => Some(command),
        }
    }
}

/// Builds a typed launch plan for one MCP server descriptor.
#[must_use]
pub const fn build_mcp_launch_plan(server: &McpServerDescriptor) -> McpLaunchPlan {
    let step = match server.launch_policy {
        McpLaunchPolicy::PluginRuntime => McpLaunchStep::PluginRuntimeBootstrap {
            plugin_id: server.plugin_id,
        },
        McpLaunchPolicy::SpawnProcess(command) => McpLaunchStep::SpawnProcess { command },
    };

    McpLaunchPlan {
        server_id: server.id,
        plugin_id: server.plugin_id,
        transport: server.transport,
        step,
        availability: server.availability,
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
        i18n::official_servers_label(locale),
        servers.len()
    ));

    for server in servers {
        lines.push(format!(
            "- {} | {} | {} | {}",
            server.id,
            server.display_name_for_locale(locale),
            server.plugin_id,
            server.transport
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
        format!("{}: {}", i18n::server_label(locale), server.id),
        format!(
            "{}: {}",
            i18n::name_label(locale),
            server.display_name_for_locale(locale)
        ),
        format!("{}: {}", i18n::plugin_label(locale), server.plugin_id),
        format!("{}: {}", i18n::transport_label(locale), server.transport),
        format!(
            "{}: {}",
            i18n::process_model_label(locale),
            server.process_model()
        ),
        format!(
            "{}: {}",
            i18n::launch_policy_label(locale),
            server.launch_policy
        ),
        format!(
            "{}: {}",
            i18n::availability_label(locale),
            server.availability
        ),
    ];

    match server.command() {
        Some(command) => {
            lines.push(format!(
                "{}: {}",
                i18n::command_label(locale),
                command.render_invocation()
            ));
            lines.push(format!(
                "{}: {}",
                i18n::working_directory_label(locale),
                command.working_directory
            ));
            lines.push(format!(
                "{}: {}",
                i18n::environment_label(locale),
                command.environment
            ));
        }
        None => {
            lines.push(format!(
                "{}: {}",
                i18n::command_label(locale),
                i18n::runtime_managed_command_label(locale)
            ));
            lines.push(format!(
                "{}: runtime_managed",
                i18n::working_directory_label(locale)
            ));
            lines.push(format!(
                "{}: minimal_runtime",
                i18n::environment_label(locale)
            ));
        }
    }

    lines.join("\n")
}

/// Renders a human-readable MCP launch plan.
#[must_use]
pub fn render_mcp_launch_plan(plan: &McpLaunchPlan) -> String {
    render_mcp_launch_plan_for_locale(plan, "en")
}

/// Renders a human-readable MCP launch plan for one locale.
#[must_use]
pub fn render_mcp_launch_plan_for_locale(plan: &McpLaunchPlan, locale: &str) -> String {
    let mut lines = vec![
        format!("{}: {}", i18n::launch_plan_label(locale), plan.server_id),
        format!("{}: {}", i18n::plugin_label(locale), plan.plugin_id),
        format!("{}: {}", i18n::transport_label(locale), plan.transport),
        format!("{}: {}", i18n::launch_step_label(locale), plan.step),
        format!(
            "{}: {}",
            i18n::availability_label(locale),
            plan.availability
        ),
    ];

    match plan.command() {
        Some(command) => {
            lines.push(format!(
                "{}: {}",
                i18n::command_label(locale),
                command.render_invocation()
            ));
            lines.push(format!(
                "{}: {}",
                i18n::working_directory_label(locale),
                command.working_directory
            ));
            lines.push(format!(
                "{}: {}",
                i18n::environment_label(locale),
                command.environment
            ));
        }
        None => {
            lines.push(format!(
                "{}: {}",
                i18n::command_label(locale),
                i18n::runtime_managed_command_label(locale)
            ));
            lines.push(format!(
                "{}: runtime_managed",
                i18n::working_directory_label(locale)
            ));
            lines.push(format!(
                "{}: minimal_runtime",
                i18n::environment_label(locale)
            ));
        }
    }

    lines.join("\n")
}
