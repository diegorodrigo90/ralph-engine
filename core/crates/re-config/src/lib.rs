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

/// Typed plugin configuration entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginConfig {
    /// Stable plugin identifier.
    pub id: &'static str,
    /// Default activation state for the plugin.
    pub activation: PluginActivation,
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

/// Returns the default project configuration contract.
#[must_use]
pub const fn default_project_config() -> ProjectConfig {
    DEFAULT_PROJECT_CONFIG
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
