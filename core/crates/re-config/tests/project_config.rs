//! Integration tests for the shared Ralph Engine config contract.

use re_config::{
    CANONICAL_CONFIG_LAYERS, ConfigScope, DEFAULT_LOCALE, McpConfig, McpDiscovery,
    PluginActivation, PluginConfig, ProjectConfig, ProjectConfigLayer, ResolvedPluginConfig,
    canonical_config_layers, default_project_config, default_project_config_layer,
    find_plugin_config, render_config_layers_yaml, render_project_config_yaml,
    render_resolved_plugin_config_yaml, resolve_plugin_config,
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
    assert_eq!(
        basic_plugin.map(|plugin| plugin.activation),
        Some(PluginActivation::Enabled)
    );
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
    assert!(yaml.contains("    activation: enabled"));
    assert!(yaml.contains("mcp:"));
    assert!(yaml.contains("  discovery: official_only"));
}

#[test]
fn plugin_config_constructor_is_stable() {
    // Arrange
    let plugin = PluginConfig::new("official.github", PluginActivation::Disabled);

    // Act
    let matches = plugin.id == "official.github" && !plugin.is_enabled();

    // Assert
    assert!(matches);
}

#[test]
fn plugin_activation_as_str_is_stable() {
    // Arrange
    let enabled = PluginActivation::Enabled;
    let disabled = PluginActivation::Disabled;

    // Act
    let rendered = [enabled.as_str(), disabled.as_str()];

    // Assert
    assert_eq!(rendered, ["enabled", "disabled"]);
}

#[test]
fn config_scope_as_str_is_stable() {
    // Arrange
    let scopes = [
        ConfigScope::BuiltInDefaults,
        ConfigScope::Workspace,
        ConfigScope::Project,
        ConfigScope::User,
    ];

    // Act
    let rendered = scopes
        .into_iter()
        .map(ConfigScope::as_str)
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(
        rendered,
        vec!["built_in_defaults", "workspace", "project", "user"]
    );
}

#[test]
fn find_plugin_config_returns_matching_entry() {
    // Arrange
    let config = default_project_config();

    // Act
    let plugin = find_plugin_config(&config, "official.basic");

    // Assert
    assert_eq!(
        plugin.map(|entry| entry.activation),
        Some(PluginActivation::Enabled)
    );
}

#[test]
fn find_plugin_config_returns_none_for_unknown_plugin() {
    // Arrange
    let config = default_project_config();

    // Act
    let plugin = find_plugin_config(&config, "official.unknown");

    // Assert
    assert!(plugin.is_none());
}

#[test]
fn default_project_config_layer_uses_built_in_scope() {
    // Arrange
    let layer = default_project_config_layer();

    // Act
    let is_default_scope = layer.scope == ConfigScope::BuiltInDefaults;

    // Assert
    assert!(is_default_scope);
}

#[test]
fn canonical_config_layers_returns_stable_defaults_stack() {
    // Arrange
    let layers = canonical_config_layers();

    // Act
    let matches = layers == CANONICAL_CONFIG_LAYERS
        && layers.len() == 1
        && layers[0].scope == ConfigScope::BuiltInDefaults;

    // Assert
    assert!(matches);
}

#[test]
fn resolve_plugin_config_returns_effective_entry_from_highest_precedence_layer() {
    // Arrange
    const DEFAULT_PLUGINS: &[PluginConfig] = &[PluginConfig::new(
        "official.github",
        PluginActivation::Disabled,
    )];
    const PROJECT_PLUGINS: &[PluginConfig] = &[PluginConfig::new(
        "official.github",
        PluginActivation::Enabled,
    )];
    let layers = [
        ProjectConfigLayer::new(
            ConfigScope::BuiltInDefaults,
            ProjectConfig {
                schema_version: 1,
                default_locale: DEFAULT_LOCALE,
                plugins: DEFAULT_PLUGINS,
                mcp: McpConfig {
                    enabled: true,
                    discovery: McpDiscovery::OfficialOnly,
                },
            },
        ),
        ProjectConfigLayer::new(
            ConfigScope::Project,
            ProjectConfig {
                schema_version: 1,
                default_locale: DEFAULT_LOCALE,
                plugins: PROJECT_PLUGINS,
                mcp: McpConfig {
                    enabled: true,
                    discovery: McpDiscovery::OfficialOnly,
                },
            },
        ),
    ];

    // Act
    let resolved = resolve_plugin_config(&layers, "official.github");

    // Assert
    assert_eq!(
        resolved,
        Some(ResolvedPluginConfig::new(
            "official.github",
            PluginActivation::Enabled,
            ConfigScope::Project,
        ))
    );
}

#[test]
fn resolve_plugin_config_returns_none_for_unknown_plugin() {
    // Arrange
    let layers = [default_project_config_layer()];

    // Act
    let resolved = resolve_plugin_config(&layers, "official.unknown");

    // Assert
    assert!(resolved.is_none());
}

#[test]
fn plugin_config_is_enabled_reflects_enabled_state() {
    // Arrange
    let plugin = PluginConfig::new("official.basic", PluginActivation::Enabled);

    // Act
    let enabled = plugin.is_enabled();

    // Assert
    assert!(enabled);
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
    assert!(!yaml.contains("    activation:"));
    assert!(yaml.contains("  discovery: official_only"));
}

#[test]
fn render_resolved_plugin_config_yaml_is_human_readable() {
    // Arrange
    let resolved = ResolvedPluginConfig::new(
        "official.basic",
        PluginActivation::Enabled,
        ConfigScope::BuiltInDefaults,
    );

    // Act
    let yaml = render_resolved_plugin_config_yaml(&resolved);

    // Assert
    assert!(yaml.contains("id: official.basic"));
    assert!(yaml.contains("activation: enabled"));
    assert!(yaml.contains("resolved_from: built_in_defaults"));
}

#[test]
fn render_config_layers_yaml_is_human_readable() {
    // Arrange
    let layers = canonical_config_layers();

    // Act
    let yaml = render_config_layers_yaml(layers);

    // Assert
    assert!(yaml.contains("layers:"));
    assert!(yaml.contains("scope: built_in_defaults"));
    assert!(yaml.contains("schema_version: 1"));
    assert!(yaml.contains("plugin_count: 1"));
    assert!(yaml.contains("mcp_enabled: true"));
}
