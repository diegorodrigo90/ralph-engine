//! Shared configuration contracts for Ralph Engine.

/// Supported locale identifiers for runtime defaults.
pub const DEFAULT_LOCALE: &str = "en";

/// Typed supported locale descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocaleDescriptor {
    /// Stable locale identifier.
    pub id: &'static str,
    /// English display name for the locale.
    pub english_name: &'static str,
    /// Native display name for the locale.
    pub native_name: &'static str,
    /// Whether English is the fallback source for this locale catalog.
    pub falls_back_to_english: bool,
}

impl LocaleDescriptor {
    /// Creates a new immutable locale descriptor.
    #[must_use]
    pub const fn new(
        id: &'static str,
        english_name: &'static str,
        native_name: &'static str,
        falls_back_to_english: bool,
    ) -> Self {
        Self {
            id,
            english_name,
            native_name,
            falls_back_to_english,
        }
    }
}

/// Canonical supported runtime locales.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SupportedLocale {
    /// English locale.
    En,
    /// Portuguese (Brazil) locale.
    PtBr,
}

impl SupportedLocale {
    /// Returns the canonical locale identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::PtBr => "pt-br",
        }
    }

    /// Returns the immutable locale descriptor.
    #[must_use]
    pub const fn descriptor(self) -> LocaleDescriptor {
        match self {
            Self::En => LocaleDescriptor::new("en", "English", "English", false),
            Self::PtBr => {
                LocaleDescriptor::new("pt-br", "Portuguese (Brazil)", "Português (Brasil)", true)
            }
        }
    }
}

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
    /// Runtime budget defaults.
    pub budgets: RuntimeBudgetConfig,
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

/// Typed runtime budget defaults.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeBudgetConfig {
    /// Maximum prompt tokens per assembly cycle.
    pub prompt_tokens: u32,
    /// Maximum context tokens retained for one agent cycle.
    pub context_tokens: u32,
}

const DEFAULT_PLUGINS: &[PluginConfig] = &[PluginConfig::new(
    "official.basic",
    PluginActivation::Enabled,
)];
const SUPPORTED_LOCALES: &[LocaleDescriptor] = &[
    SupportedLocale::En.descriptor(),
    SupportedLocale::PtBr.descriptor(),
];
const DEFAULT_MCP: McpConfig = McpConfig {
    enabled: true,
    discovery: McpDiscovery::OfficialOnly,
};
const DEFAULT_BUDGETS: RuntimeBudgetConfig = RuntimeBudgetConfig {
    prompt_tokens: 8_192,
    context_tokens: 32_768,
};
const DEFAULT_PROJECT_CONFIG: ProjectConfig = ProjectConfig {
    schema_version: 1,
    default_locale: DEFAULT_LOCALE,
    plugins: DEFAULT_PLUGINS,
    mcp: DEFAULT_MCP,
    budgets: DEFAULT_BUDGETS,
};
const DEFAULT_PROJECT_CONFIG_LAYER: ProjectConfigLayer =
    ProjectConfigLayer::new(ConfigScope::BuiltInDefaults, DEFAULT_PROJECT_CONFIG);

/// Canonical typed configuration layers in resolution order.
pub const CANONICAL_CONFIG_LAYERS: &[ProjectConfigLayer] = &[DEFAULT_PROJECT_CONFIG_LAYER];

/// Canonical supported locale catalog for runtime-facing surfaces.
pub const CANONICAL_SUPPORTED_LOCALES: &[LocaleDescriptor] = SUPPORTED_LOCALES;

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

/// Returns the canonical typed configuration layers in resolution order.
#[must_use]
pub const fn canonical_config_layers() -> &'static [ProjectConfigLayer] {
    CANONICAL_CONFIG_LAYERS
}

/// Returns the canonical supported locale catalog.
#[must_use]
pub const fn supported_locales() -> &'static [LocaleDescriptor] {
    CANONICAL_SUPPORTED_LOCALES
}

/// Returns one immutable supported locale entry by identifier.
#[must_use]
pub fn find_locale_descriptor(locale_id: &str) -> Option<LocaleDescriptor> {
    supported_locales()
        .iter()
        .find(|locale| locale.id.eq_ignore_ascii_case(locale_id))
        .copied()
}

/// Returns the canonical supported locale identifier when one exists.
#[must_use]
pub fn canonical_locale_id(locale_id: &str) -> Option<&'static str> {
    find_locale_descriptor(locale_id).map(|locale| locale.id)
}

/// Parses one locale identifier into the canonical typed locale when supported.
#[must_use]
pub fn parse_supported_locale(locale_id: &str) -> Option<SupportedLocale> {
    match canonical_locale_id(locale_id) {
        Some("en") => Some(SupportedLocale::En),
        Some("pt-br") => Some(SupportedLocale::PtBr),
        _ => None,
    }
}

/// Resolves one locale identifier to a supported value, falling back to English.
#[must_use]
pub fn resolve_locale_or_default(locale_id: &str) -> &'static str {
    resolve_supported_locale_or_default(locale_id).as_str()
}

/// Resolves one locale identifier to a supported typed value, falling back to English.
#[must_use]
pub fn resolve_supported_locale_or_default(locale_id: &str) -> SupportedLocale {
    parse_supported_locale(locale_id).unwrap_or(SupportedLocale::En)
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
    lines.push("budgets:".to_owned());
    lines.push(format!("  prompt_tokens: {}", config.budgets.prompt_tokens));
    lines.push(format!(
        "  context_tokens: {}",
        config.budgets.context_tokens
    ));

    lines.join("\n")
}

/// Renders the default locale contract as YAML.
#[must_use]
pub fn render_default_locale_yaml(config: &ProjectConfig) -> String {
    format!("default_locale: {}", config.default_locale)
}

/// Renders the supported locale catalog as YAML.
#[must_use]
pub fn render_supported_locales_yaml(locales: &[LocaleDescriptor]) -> String {
    let mut lines = vec!["supported_locales:".to_owned()];

    for locale in locales {
        lines.push(format!("  - id: {}", locale.id));
        lines.push(format!("    english_name: {}", locale.english_name));
        lines.push(format!("    native_name: {}", locale.native_name));
        lines.push(format!(
            "    falls_back_to_english: {}",
            locale.falls_back_to_english
        ));
    }

    lines.join("\n")
}

/// Renders one supported locale descriptor as YAML.
#[must_use]
pub fn render_locale_descriptor_yaml(locale: &LocaleDescriptor) -> String {
    [
        format!("id: {}", locale.id),
        format!("english_name: {}", locale.english_name),
        format!("native_name: {}", locale.native_name),
        format!("falls_back_to_english: {}", locale.falls_back_to_english),
    ]
    .join("\n")
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

/// Renders typed configuration layers in resolution order as YAML.
#[must_use]
pub fn render_config_layers_yaml(layers: &[ProjectConfigLayer]) -> String {
    let mut lines = vec!["layers:".to_owned()];

    for layer in layers {
        lines.push(format!("  - scope: {}", layer.scope.as_str()));
        lines.push(format!(
            "    schema_version: {}",
            layer.config.schema_version
        ));
        lines.push(format!(
            "    default_locale: {}",
            layer.config.default_locale
        ));
        lines.push(format!("    plugin_count: {}", layer.config.plugins.len()));
        lines.push(format!("    mcp_enabled: {}", layer.config.mcp.enabled));
        lines.push(format!(
            "    prompt_tokens: {}",
            layer.config.budgets.prompt_tokens
        ));
        lines.push(format!(
            "    context_tokens: {}",
            layer.config.budgets.context_tokens
        ));
    }

    lines.join("\n")
}

/// Renders typed runtime budgets as YAML.
#[must_use]
pub fn render_runtime_budgets_yaml(budgets: &RuntimeBudgetConfig) -> String {
    [
        "budgets:".to_owned(),
        format!("  prompt_tokens: {}", budgets.prompt_tokens),
        format!("  context_tokens: {}", budgets.context_tokens),
    ]
    .join("\n")
}
