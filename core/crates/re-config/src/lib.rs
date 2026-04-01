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
    /// Whether the plugin is enabled by default.
    pub enabled: bool,
}

impl PluginConfig {
    /// Creates a new immutable plugin config entry.
    #[must_use]
    pub const fn new(id: &'static str, enabled: bool) -> Self {
        Self { id, enabled }
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

const DEFAULT_PLUGINS: &[PluginConfig] = &[PluginConfig::new("official.basic", true)];
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
        lines.push(format!("    enabled: {}", plugin.enabled));
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
