//! Shared configuration contracts for Ralph Engine.

/// Supported locale identifiers for runtime defaults.
pub const DEFAULT_LOCALE: &str = "en";

/// Typed project configuration contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProjectConfig {
    /// Stable config schema version.
    pub schema_version: u8,
    /// Default locale for runtime-facing surfaces.
    pub default_locale: &'static str,
    /// Default plugin entries.
    pub plugins: &'static [PluginConfig],
    /// MCP configuration defaults.
    pub mcp: McpConfig,
}

/// Typed configuration scope identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigScope {
    /// Built-in repository defaults.
    BuiltInDefaults,
    /// Workspace-level configuration.
    Workspace,
    /// Project-level configuration.
    Project,
    /// User-level overrides.
    User,
}

impl ConfigScope {
    /// Returns the stable configuration-scope identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuiltInDefaults => "built_in_defaults",
            Self::Workspace => "workspace",
            Self::Project => "project",
            Self::User => "user",
        }
    }
}

/// One typed configuration layer in resolution order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProjectConfigLayer {
    /// Scope represented by the layer.
    pub scope: ConfigScope,
    /// Immutable configuration payload for the scope.
    pub config: ProjectConfig,
}

impl ProjectConfigLayer {
    /// Creates a new immutable configuration layer.
    #[must_use]
    pub const fn new(scope: ConfigScope, config: ProjectConfig) -> Self {
        Self { scope, config }
    }
}

/// Typed plugin configuration entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginConfig {
    /// Stable plugin identifier.
    pub id: &'static str,
    /// Default activation state for the plugin.
    pub activation: PluginActivation,
}

/// Resolved plugin configuration entry with source scope metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedPluginConfig {
    /// Stable plugin identifier.
    pub id: &'static str,
    /// Effective activation state after typed resolution.
    pub activation: PluginActivation,
    /// Scope that supplied the effective configuration.
    pub resolved_from: ConfigScope,
}

impl ResolvedPluginConfig {
    /// Creates a new immutable resolved plugin config entry.
    #[must_use]
    pub const fn new(
        id: &'static str,
        activation: PluginActivation,
        resolved_from: ConfigScope,
    ) -> Self {
        Self {
            id,
            activation,
            resolved_from,
        }
    }
}

impl PluginConfig {
    /// Creates a new immutable plugin config entry.
    #[must_use]
    pub const fn new(id: &'static str, activation: PluginActivation) -> Self {
        Self { id, activation }
    }

    /// Returns whether the plugin is enabled by default.
    #[must_use]
    pub const fn is_enabled(self) -> bool {
        matches!(self.activation, PluginActivation::Enabled)
    }
}

/// Typed plugin activation states.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PluginActivation {
    /// The plugin is enabled by default.
    Enabled,
    /// The plugin is disabled by default.
    Disabled,
}

impl PluginActivation {
    /// Returns the stable activation identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
        }
    }
}

/// Typed MCP configuration defaults.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct McpConfig {
    /// Whether MCP support is enabled.
    pub enabled: bool,
    /// Discovery policy for built-in MCP contributions.
    pub discovery: McpDiscovery,
}

/// Supported MCP discovery policies.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpDiscovery {
    /// Only built-in official contributions are enabled by default.
    OfficialOnly,
}

const DEFAULT_PLUGINS: &[PluginConfig] = &[PluginConfig::new(
    "official.basic",
    PluginActivation::Enabled,
)];
const DEFAULT_MCP: McpConfig = McpConfig {
    enabled: true,
    discovery: McpDiscovery::OfficialOnly,
};
const DEFAULT_PROJECT_CONFIG: ProjectConfig = ProjectConfig {
    schema_version: 1,
    default_locale: DEFAULT_LOCALE,
    plugins: DEFAULT_PLUGINS,
    mcp: DEFAULT_MCP,
};
const DEFAULT_PROJECT_CONFIG_LAYER: ProjectConfigLayer =
    ProjectConfigLayer::new(ConfigScope::BuiltInDefaults, DEFAULT_PROJECT_CONFIG);

/// Returns the default project configuration contract.
#[must_use]
pub const fn default_project_config() -> ProjectConfig {
    DEFAULT_PROJECT_CONFIG
}

/// Returns the default project configuration as a typed resolution layer.
#[must_use]
pub const fn default_project_config_layer() -> ProjectConfigLayer {
    DEFAULT_PROJECT_CONFIG_LAYER
}

/// Returns one immutable plugin config entry by identifier.
#[must_use]
pub fn find_plugin_config(config: &ProjectConfig, plugin_id: &str) -> Option<PluginConfig> {
    config
        .plugins
        .iter()
        .find(|plugin| plugin.id == plugin_id)
        .copied()
}

/// Resolves one plugin config entry from ordered layers.
///
/// Layers SHALL be passed from lowest precedence to highest precedence.
#[must_use]
pub fn resolve_plugin_config(
    layers: &[ProjectConfigLayer],
    plugin_id: &str,
) -> Option<ResolvedPluginConfig> {
    layers.iter().rev().find_map(|layer| {
        find_plugin_config(&layer.config, plugin_id)
            .map(|entry| ResolvedPluginConfig::new(entry.id, entry.activation, layer.scope))
    })
}

/// Renders the project configuration contract as YAML.
#[must_use]
pub fn render_project_config_yaml(config: &ProjectConfig) -> String {
    let mut lines = vec![
        format!("schema_version: {}", config.schema_version),
        format!("default_locale: {}", config.default_locale),
        "plugins:".to_owned(),
    ];

    for plugin in config.plugins {
        lines.push(format!("  - id: {}", plugin.id));
        lines.push(format!("    activation: {}", plugin.activation.as_str()));
    }

    lines.push("mcp:".to_owned());
    lines.push(format!("  enabled: {}", config.mcp.enabled));
    lines.push(format!(
        "  discovery: {}",
        match config.mcp.discovery {
            McpDiscovery::OfficialOnly => "official_only",
        }
    ));

    lines.join("\n")
}

/// Renders one resolved plugin configuration block as YAML.
#[must_use]
pub fn render_resolved_plugin_config_yaml(config: &ResolvedPluginConfig) -> String {
    [
        format!("id: {}", config.id),
        format!("activation: {}", config.activation.as_str()),
        format!("resolved_from: {}", config.resolved_from.as_str()),
    ]
    .join("\n")
}
