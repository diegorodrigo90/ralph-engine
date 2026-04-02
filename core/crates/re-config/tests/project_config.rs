//! Integration tests for the shared Ralph Engine config contract.

use re_config::{
    CANONICAL_CONFIG_LAYERS, CANONICAL_SUPPORTED_LOCALES, ConfigScope, DEFAULT_LOCALE, McpConfig,
    McpDiscovery, PluginActivation, PluginConfig, ProjectConfig, ProjectConfigLayer,
    ResolvedPluginConfig, RuntimeBudgetConfig, SupportedLocale, canonical_config_layers,
    default_project_config, default_project_config_layer, find_locale_descriptor,
    find_plugin_config, parse_supported_locale, render_config_layers_yaml,
    render_default_locale_yaml, render_locale_descriptor_yaml, render_project_config_yaml,
    render_resolved_plugin_config_yaml, render_runtime_budgets_yaml, render_supported_locales_yaml,
    resolve_locale_or_default, resolve_plugin_config, resolve_supported_locale_or_default,
    supported_locales,
};

const TEST_DEFAULT_PLUGIN_ID: &str = "test.defaults";
const TEST_OVERRIDE_PLUGIN_ID: &str = "test.override";
const TEST_UNKNOWN_PLUGIN_ID: &str = "test.unknown";
const TEST_DEFAULT_PLUGINS: &[PluginConfig] = &[PluginConfig::new(
    TEST_DEFAULT_PLUGIN_ID,
    PluginActivation::Enabled,
)];

#[test]
fn default_project_config_uses_stable_schema_defaults() {
    // Arrange
    let config = default_project_config();

    // Act
    let is_expected = config.schema_version == 1
        && config.default_locale == DEFAULT_LOCALE
        && config.mcp.discovery == McpDiscovery::OfficialOnly
        && config.budgets.prompt_tokens == 8_192
        && config.budgets.context_tokens == 32_768;

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
    assert!(yaml.contains("budgets:"));
    assert!(yaml.contains("  prompt_tokens: 8192"));
    assert!(yaml.contains("  context_tokens: 32768"));
}

#[test]
fn supported_locales_returns_stable_catalog() {
    let locales = supported_locales();

    assert_eq!(locales, CANONICAL_SUPPORTED_LOCALES);
    assert_eq!(locales.len(), 2);
    assert_eq!(locales[0].id, "en");
    assert_eq!(locales[1].id, "pt-br");
    assert!(locales[1].falls_back_to_english);
}

#[test]
fn find_locale_descriptor_returns_matching_locale() {
    let locale = find_locale_descriptor("pt-br");

    assert_eq!(
        locale.map(|entry| entry.native_name),
        Some("Português (Brasil)")
    );
}

#[test]
fn find_locale_descriptor_accepts_case_insensitive_lookup() {
    let locale = find_locale_descriptor("PT-BR");

    assert_eq!(locale.map(|entry| entry.id), Some("pt-br"));
}

#[test]
fn find_locale_descriptor_returns_none_for_unknown_locale() {
    assert!(find_locale_descriptor("es").is_none());
}

#[test]
fn resolve_locale_or_default_returns_supported_locale() {
    assert_eq!(resolve_locale_or_default("PT-BR"), "pt-br");
}

#[test]
fn resolve_locale_or_default_falls_back_to_english() {
    assert_eq!(resolve_locale_or_default("es"), "en");
}

#[test]
fn parse_supported_locale_returns_typed_locale() {
    assert_eq!(parse_supported_locale("pt-BR"), Some(SupportedLocale::PtBr));
    assert_eq!(parse_supported_locale("en"), Some(SupportedLocale::En));
    assert_eq!(parse_supported_locale("es"), None);
}

#[test]
fn resolve_supported_locale_or_default_returns_typed_locale() {
    assert_eq!(
        resolve_supported_locale_or_default("pt-BR"),
        SupportedLocale::PtBr
    );
    assert_eq!(
        resolve_supported_locale_or_default("es"),
        SupportedLocale::En
    );
}

#[test]
fn render_default_locale_yaml_is_human_readable() {
    let yaml = render_default_locale_yaml(&default_project_config());

    assert_eq!(yaml, "default_locale: en");
}

#[test]
fn render_supported_locales_yaml_is_human_readable() {
    let yaml = render_supported_locales_yaml(supported_locales());

    assert!(yaml.contains("supported_locales:"));
    assert!(yaml.contains("  - id: en"));
    assert!(yaml.contains("    native_name: English"));
    assert!(yaml.contains("  - id: pt-br"));
    assert!(yaml.contains("    native_name: Português (Brasil)"));
    assert!(yaml.contains("    falls_back_to_english: true"));
}

#[test]
fn render_locale_descriptor_yaml_is_human_readable() {
    let locale = find_locale_descriptor("pt-br");

    assert!(locale.is_some());

    let yaml = locale
        .map(|entry| render_locale_descriptor_yaml(&entry))
        .unwrap_or_default();

    assert!(yaml.contains("id: pt-br"));
    assert!(yaml.contains("english_name: Portuguese (Brazil)"));
    assert!(yaml.contains("native_name: Português (Brasil)"));
    assert!(yaml.contains("falls_back_to_english: true"));
}

#[test]
fn plugin_config_constructor_is_stable() {
    // Arrange
    let plugin = PluginConfig::new(TEST_OVERRIDE_PLUGIN_ID, PluginActivation::Disabled);

    // Act
    let matches = plugin.id == TEST_OVERRIDE_PLUGIN_ID && !plugin.is_enabled();

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
    let config = ProjectConfig {
        schema_version: config.schema_version,
        default_locale: config.default_locale,
        plugins: TEST_DEFAULT_PLUGINS,
        mcp: config.mcp,
        budgets: config.budgets,
    };

    // Act
    let plugin = find_plugin_config(&config, TEST_DEFAULT_PLUGIN_ID);

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
    let plugin = find_plugin_config(&config, TEST_UNKNOWN_PLUGIN_ID);

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
        TEST_OVERRIDE_PLUGIN_ID,
        PluginActivation::Disabled,
    )];
    const PROJECT_PLUGINS: &[PluginConfig] = &[PluginConfig::new(
        TEST_OVERRIDE_PLUGIN_ID,
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
                budgets: RuntimeBudgetConfig {
                    prompt_tokens: 8_192,
                    context_tokens: 32_768,
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
                budgets: RuntimeBudgetConfig {
                    prompt_tokens: 8_192,
                    context_tokens: 32_768,
                },
            },
        ),
    ];

    // Act
    let resolved = resolve_plugin_config(&layers, TEST_OVERRIDE_PLUGIN_ID);

    // Assert
    assert_eq!(
        resolved,
        Some(ResolvedPluginConfig::new(
            TEST_OVERRIDE_PLUGIN_ID,
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
    let resolved = resolve_plugin_config(&layers, TEST_UNKNOWN_PLUGIN_ID);

    // Assert
    assert!(resolved.is_none());
}

#[test]
fn plugin_config_is_enabled_reflects_enabled_state() {
    // Arrange
    let plugin = PluginConfig::new(TEST_DEFAULT_PLUGIN_ID, PluginActivation::Enabled);

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
        budgets: RuntimeBudgetConfig {
            prompt_tokens: 8_192,
            context_tokens: 32_768,
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
        TEST_DEFAULT_PLUGIN_ID,
        PluginActivation::Enabled,
        ConfigScope::BuiltInDefaults,
    );

    // Act
    let yaml = render_resolved_plugin_config_yaml(&resolved);

    // Assert
    assert!(yaml.contains("id: test.defaults"));
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
    assert!(yaml.contains("prompt_tokens: 8192"));
    assert!(yaml.contains("context_tokens: 32768"));
}

#[test]
fn render_runtime_budgets_yaml_is_human_readable() {
    // Arrange
    let budgets = RuntimeBudgetConfig {
        prompt_tokens: 4_096,
        context_tokens: 16_384,
    };

    // Act
    let yaml = render_runtime_budgets_yaml(&budgets);

    // Assert
    assert!(yaml.contains("budgets:"));
    assert!(yaml.contains("prompt_tokens: 4096"));
    assert!(yaml.contains("context_tokens: 16384"));
}
