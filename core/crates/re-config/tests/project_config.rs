//! Integration tests for the shared Ralph Engine config contract.

use re_config::{
    DEFAULT_LOCALE, McpConfig, McpDiscovery, PluginConfig, ProjectConfig, default_project_config,
    render_project_config_yaml,
};

#[test]
fn default_project_config_uses_stable_schema_defaults() {
    // Arrange
    let config = default_project_config();

    // Act
    let is_expected = config.schema_version == 1
        && config.default_locale == DEFAULT_LOCALE
        && config.mcp.discovery == McpDiscovery::OfficialOnly;

    // Assert
    assert!(is_expected);
}

#[test]
fn default_project_config_enables_basic_plugin_by_default() {
    // Arrange
    let config = default_project_config();

    // Act
    let basic_plugin = config.plugins.first().copied();

    // Assert
    assert_eq!(basic_plugin.map(|plugin| plugin.id), Some("official.basic"));
    assert_eq!(basic_plugin.map(|plugin| plugin.enabled), Some(true));
}

#[test]
fn render_project_config_yaml_is_human_readable() {
    // Arrange
    let config = default_project_config();

    // Act
    let yaml = render_project_config_yaml(&config);

    // Assert
    assert!(yaml.contains("schema_version: 1"));
    assert!(yaml.contains("default_locale: en"));
    assert!(yaml.contains("plugins:"));
    assert!(yaml.contains("  - id: official.basic"));
    assert!(yaml.contains("mcp:"));
    assert!(yaml.contains("  discovery: official_only"));
}

#[test]
fn plugin_config_constructor_is_stable() {
    // Arrange
    let plugin = PluginConfig::new("official.github", false);

    // Act
    let matches = plugin.id == "official.github" && !plugin.enabled;

    // Assert
    assert!(matches);
}

#[test]
fn render_project_config_yaml_handles_empty_plugin_sets() {
    // Arrange
    let config = ProjectConfig {
        schema_version: 1,
        default_locale: DEFAULT_LOCALE,
        plugins: &[],
        mcp: McpConfig {
            enabled: true,
            discovery: McpDiscovery::OfficialOnly,
        },
    };

    // Act
    let yaml = render_project_config_yaml(&config);

    // Assert
    assert!(yaml.contains("plugins:"));
    assert!(!yaml.contains("  - id:"));
    assert!(yaml.contains("  discovery: official_only"));
}
