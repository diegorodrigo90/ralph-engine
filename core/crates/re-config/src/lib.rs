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

/// Typed MCP server configuration entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct McpServerConfig {
    /// Stable MCP server identifier.
    pub id: &'static str,
    /// Whether the server is enabled in the effective configuration.
    pub enabled: bool,
}

/// Resolved MCP server configuration entry with source scope metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedMcpServerConfig {
    /// Stable MCP server identifier.
    pub id: &'static str,
    /// Effective enabled state after typed resolution.
    pub enabled: bool,
    /// Scope that supplied the effective configuration.
    pub resolved_from: ConfigScope,
}

impl ResolvedMcpServerConfig {
    /// Creates a new immutable resolved MCP server config entry.
    #[must_use]
    pub const fn new(id: &'static str, enabled: bool, resolved_from: ConfigScope) -> Self {
        Self {
            id,
            enabled,
            resolved_from,
        }
    }
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

impl McpServerConfig {
    /// Creates a new immutable MCP server config entry.
    #[must_use]
    pub const fn new(id: &'static str, enabled: bool) -> Self {
        Self { id, enabled }
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
    /// Per-server activation entries.
    pub servers: &'static [McpServerConfig],
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

/// Owned MCP configuration document with patchable entries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedMcpConfig {
    /// Whether MCP support is enabled.
    pub enabled: bool,
    /// Discovery policy for built-in MCP contributions.
    pub discovery: McpDiscovery,
    /// Per-server activation entries.
    pub servers: Vec<McpServerConfig>,
}

/// Run command configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunConfig {
    /// Workflow plugin that resolves work items and builds prompts.
    pub workflow_plugin: Option<&'static str>,
    /// Agent plugin that launches the agent process.
    pub agent_plugin: Option<&'static str>,
    /// Stable agent identifier to launch.
    pub agent_id: Option<&'static str>,
    /// Run mode: `"loop"` (default), `"chat"`, or `"task-routed"`.
    pub mode: &'static str,
}

impl Default for RunConfig {
    fn default() -> Self {
        Self {
            workflow_plugin: None,
            agent_plugin: None,
            agent_id: None,
            mode: "loop",
        }
    }
}

/// Context management configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContextConfig {
    /// Whether to persist sessions to disk automatically.
    pub persistence: bool,
    /// Whether to auto-compact context before agent transfer.
    pub auto_compact: bool,
    /// Compact when context exceeds this fraction of the window (0.0-1.0).
    /// Stored as percentage (0-100) to avoid floats.
    pub compact_threshold_pct: u8,
}

/// A project entry for multi-project mode.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectEntry {
    /// Filesystem path to the project root.
    pub path: String,
    /// Optional display label for the TUI tab.
    pub label: Option<String>,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            persistence: true,
            auto_compact: true,
            compact_threshold_pct: 80,
        }
    }
}

/// Owned project configuration document with patchable entries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedProjectConfig {
    /// Stable config schema version.
    pub schema_version: u8,
    /// Default locale for runtime-facing surfaces.
    pub default_locale: &'static str,
    /// Default plugin entries.
    pub plugins: Vec<PluginConfig>,
    /// MCP configuration defaults.
    pub mcp: OwnedMcpConfig,
    /// Runtime budget defaults.
    pub budgets: RuntimeBudgetConfig,
    /// Run command configuration (optional).
    pub run: RunConfig,
    /// Context management configuration.
    pub context: ContextConfig,
    /// Multi-project entries (empty = single project mode).
    pub projects: Vec<ProjectEntry>,
}

const DEFAULT_PLUGINS: &[PluginConfig] = &[PluginConfig::new(
    "official.basic",
    PluginActivation::Enabled,
)];
const SUPPORTED_LOCALES: &[LocaleDescriptor] = &[
    SupportedLocale::En.descriptor(),
    SupportedLocale::PtBr.descriptor(),
];
const DEFAULT_MCP_SERVERS: &[McpServerConfig] = &[];
const DEFAULT_MCP: McpConfig = McpConfig {
    enabled: true,
    discovery: McpDiscovery::OfficialOnly,
    servers: DEFAULT_MCP_SERVERS,
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

/// Attempts to parse an OS locale string (e.g., `pt_BR.UTF-8`, `en_US`)
/// into a supported Ralph Engine locale. Returns `None` when the OS locale
/// does not map to any supported language.
///
/// The parser strips the encoding suffix (`.UTF-8`), replaces `_` with `-`,
/// and tries both the full tag (`pt-br`) and the language-only prefix (`pt`).
#[must_use]
pub fn parse_os_locale(os_locale: &str) -> Option<SupportedLocale> {
    let trimmed = os_locale.trim();
    if trimmed.is_empty() || trimmed == "C" || trimmed == "POSIX" {
        return None;
    }

    // Strip encoding suffix: "pt_BR.UTF-8" → "pt_BR"
    let without_encoding = trimmed.split('.').next().unwrap_or(trimmed);

    // Normalize separator: "pt_BR" → "pt-br"
    let normalized = without_encoding.replace('_', "-").to_ascii_lowercase();

    // Try full tag first: "pt-br"
    if let Some(locale) = parse_supported_locale(&normalized) {
        return Some(locale);
    }

    // Try language-only prefix: "pt-br" → "pt", then check if any
    // supported locale starts with that prefix
    let language_prefix = normalized.split('-').next().unwrap_or(&normalized);
    supported_locales()
        .iter()
        .find(|descriptor| {
            descriptor.id.split('-').next().unwrap_or(descriptor.id) == language_prefix
        })
        .and_then(|descriptor| parse_supported_locale(descriptor.id))
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

/// Returns one immutable MCP server config entry by identifier.
#[must_use]
pub fn find_mcp_server_config(config: &ProjectConfig, server_id: &str) -> Option<McpServerConfig> {
    config
        .mcp
        .servers
        .iter()
        .find(|server| server.id == server_id)
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

/// Resolves one MCP server config entry from ordered layers.
///
/// Layers SHALL be passed from lowest precedence to highest precedence.
#[must_use]
pub fn resolve_mcp_server_config(
    layers: &[ProjectConfigLayer],
    server_id: &str,
) -> Option<ResolvedMcpServerConfig> {
    layers.iter().rev().find_map(|layer| {
        find_mcp_server_config(&layer.config, server_id)
            .map(|entry| ResolvedMcpServerConfig::new(entry.id, entry.enabled, layer.scope))
    })
}

fn merge_plugin_config_entries(base: &[PluginConfig], patch: &[PluginConfig]) -> Vec<PluginConfig> {
    let mut merged = base.to_vec();

    for entry in patch {
        if let Some(existing) = merged.iter_mut().find(|candidate| candidate.id == entry.id) {
            *existing = *entry;
        } else {
            merged.push(*entry);
        }
    }

    merged
}

fn merge_mcp_server_config_entries(
    base: &[McpServerConfig],
    patch: &[McpServerConfig],
) -> Vec<McpServerConfig> {
    let mut merged = base.to_vec();

    for entry in patch {
        if let Some(existing) = merged.iter_mut().find(|candidate| candidate.id == entry.id) {
            *existing = *entry;
        } else {
            merged.push(*entry);
        }
    }

    merged
}

/// Materializes the static project configuration into an owned patchable document.
#[must_use]
pub fn materialize_project_config(config: &ProjectConfig) -> OwnedProjectConfig {
    OwnedProjectConfig {
        schema_version: config.schema_version,
        default_locale: config.default_locale,
        plugins: config.plugins.to_vec(),
        mcp: OwnedMcpConfig {
            enabled: config.mcp.enabled,
            discovery: config.mcp.discovery,
            servers: config.mcp.servers.to_vec(),
        },
        budgets: config.budgets,
        run: RunConfig::default(),
        context: ContextConfig::default(),
        projects: Vec::new(),
    }
}

/// Applies plugin and MCP overrides to one base project configuration.
#[must_use]
pub fn apply_project_config_patch(
    config: &ProjectConfig,
    plugin_patch: &[PluginConfig],
    mcp_server_patch: &[McpServerConfig],
) -> OwnedProjectConfig {
    let mut owned = materialize_project_config(config);
    owned.plugins = merge_plugin_config_entries(&owned.plugins, plugin_patch);
    owned.mcp.servers = merge_mcp_server_config_entries(&owned.mcp.servers, mcp_server_patch);
    owned
}

/// Renders the project configuration contract as YAML.
#[must_use]
pub fn render_project_config_yaml(config: &ProjectConfig) -> String {
    render_owned_project_config_yaml(&materialize_project_config(config))
}

/// Renders an owned project configuration document as YAML.
#[must_use]
pub fn render_owned_project_config_yaml(config: &OwnedProjectConfig) -> String {
    let mut lines = vec![
        format!("schema_version: {}", config.schema_version),
        format!("default_locale: {}", config.default_locale),
        "plugins:".to_owned(),
    ];

    for plugin in &config.plugins {
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
    lines.push("  servers:".to_owned());

    for server in &config.mcp.servers {
        lines.push(format!("    - id: {}", server.id));
        lines.push(format!("      enabled: {}", server.enabled));
    }
    lines.push("budgets:".to_owned());
    lines.push(format!("  prompt_tokens: {}", config.budgets.prompt_tokens));
    lines.push(format!(
        "  context_tokens: {}",
        config.budgets.context_tokens
    ));

    if config.run.workflow_plugin.is_some()
        || config.run.agent_plugin.is_some()
        || config.run.agent_id.is_some()
        || config.run.mode != "loop"
    {
        lines.push("run:".to_owned());
        if let Some(wp) = config.run.workflow_plugin {
            lines.push(format!("  workflow_plugin: {wp}"));
        }
        if let Some(ap) = config.run.agent_plugin {
            lines.push(format!("  agent_plugin: {ap}"));
        }
        if let Some(ai) = config.run.agent_id {
            lines.push(format!("  agent_id: {ai}"));
        }
        if config.run.mode != "loop" {
            lines.push(format!("  mode: {}", config.run.mode));
        }
    }

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

/// Renders one resolved MCP server configuration block as YAML.
#[must_use]
pub fn render_resolved_mcp_server_config_yaml(config: &ResolvedMcpServerConfig) -> String {
    [
        format!("id: {}", config.id),
        format!("enabled: {}", config.enabled),
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
            "    mcp_server_count: {}",
            layer.config.mcp.servers.len()
        ));
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

// ── YAML config parsing ──────────────────────────────────────────

/// Errors returned by the project configuration parser.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConfigParseError {
    /// A required top-level key is missing.
    MissingKey(&'static str),
    /// A value could not be parsed into the expected type.
    InvalidValue {
        /// The key that contained the bad value.
        key: &'static str,
        /// Human-readable description of what went wrong.
        detail: String,
    },
    /// A list entry is malformed (missing required sub-key).
    MalformedEntry {
        /// Parent section name.
        section: &'static str,
        /// Sub-key that was expected.
        expected_key: &'static str,
        /// Zero-based index of the entry in the list.
        index: usize,
    },
}

impl core::fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingKey(key) => write!(f, "missing required key: {key}"),
            Self::InvalidValue { key, detail } => {
                write!(f, "invalid value for '{key}': {detail}")
            }
            Self::MalformedEntry {
                section,
                expected_key,
                index,
            } => write!(
                f,
                "malformed entry in '{section}' at index {index}: missing '{expected_key}'"
            ),
        }
    }
}

/// Parses a project configuration YAML document into an owned config.
///
/// The parser accepts the exact format produced by
/// [`render_owned_project_config_yaml`] and tolerates comments, blank
/// lines, and trailing whitespace.  It does **not** implement a
/// general-purpose YAML parser — only the Ralph Engine config contract.
///
/// # Errors
///
/// Returns [`ConfigParseError`] when required keys are missing or values
/// are not in the expected format.
pub fn parse_owned_project_config_yaml(
    content: &str,
) -> Result<OwnedProjectConfig, ConfigParseError> {
    let lines: Vec<&str> = content.lines().collect();

    let schema_version = extract_scalar(&lines, "schema_version")
        .ok_or(ConfigParseError::MissingKey("schema_version"))?
        .parse::<u8>()
        .map_err(|_| ConfigParseError::InvalidValue {
            key: "schema_version",
            detail: "expected integer 1–255".to_owned(),
        })?;

    let default_locale_raw = extract_scalar(&lines, "default_locale")
        .ok_or(ConfigParseError::MissingKey("default_locale"))?;
    let default_locale =
        canonical_locale_id(default_locale_raw).ok_or_else(|| ConfigParseError::InvalidValue {
            key: "default_locale",
            detail: format!("unsupported locale '{default_locale_raw}'"),
        })?;

    let plugins = parse_plugin_entries(&lines)?;
    let mcp = parse_mcp_section(&lines)?;
    let budgets = parse_budgets_section(&lines)?;
    let run = parse_run_section(&lines);

    Ok(OwnedProjectConfig {
        schema_version,
        default_locale,
        plugins,
        mcp,
        budgets,
        run,
        context: ContextConfig::default(),
        projects: Vec::new(),
    })
}

// ── private helpers ──────────────────────────────────────────────

/// Extracts the value of a top-level `key: value` line.
fn extract_scalar<'a>(lines: &[&'a str], key: &str) -> Option<&'a str> {
    let prefix = format!("{key}:");
    lines.iter().find_map(|line| {
        let trimmed = line.trim();
        trimmed.strip_prefix(&prefix).map(str::trim)
    })
}

/// Parses the `plugins:` list into typed entries.
fn parse_plugin_entries(lines: &[&str]) -> Result<Vec<PluginConfig>, ConfigParseError> {
    let entries = collect_list_entries(lines, "plugins:");
    let mut plugins = Vec::with_capacity(entries.len());

    for (i, entry) in entries.iter().enumerate() {
        let id = entry_value(entry, "id").ok_or(ConfigParseError::MalformedEntry {
            section: "plugins",
            expected_key: "id",
            index: i,
        })?;
        let activation_raw =
            entry_value(entry, "activation").ok_or(ConfigParseError::MalformedEntry {
                section: "plugins",
                expected_key: "activation",
                index: i,
            })?;
        let activation = match activation_raw {
            "enabled" => PluginActivation::Enabled,
            "disabled" => PluginActivation::Disabled,
            other => {
                return Err(ConfigParseError::InvalidValue {
                    key: "plugins[].activation",
                    detail: format!("expected 'enabled' or 'disabled', got '{other}'"),
                });
            }
        };
        plugins.push(PluginConfig {
            id: leak_str(id),
            activation,
        });
    }

    Ok(plugins)
}

/// Parses the `mcp:` section.
fn parse_mcp_section(lines: &[&str]) -> Result<OwnedMcpConfig, ConfigParseError> {
    let mcp_start = lines
        .iter()
        .position(|l| l.trim() == "mcp:")
        .ok_or(ConfigParseError::MissingKey("mcp"))?;
    let mcp_lines = &lines[mcp_start..];

    let enabled = extract_scalar(mcp_lines, "enabled")
        .ok_or(ConfigParseError::MissingKey("mcp.enabled"))?
        == "true";

    let discovery_raw = extract_scalar(mcp_lines, "discovery")
        .ok_or(ConfigParseError::MissingKey("mcp.discovery"))?;
    let discovery = match discovery_raw {
        "official_only" => McpDiscovery::OfficialOnly,
        other => {
            return Err(ConfigParseError::InvalidValue {
                key: "mcp.discovery",
                detail: format!("unsupported discovery policy '{other}'"),
            });
        }
    };

    let server_entries = collect_list_entries(mcp_lines, "servers:");
    let mut servers = Vec::with_capacity(server_entries.len());

    for (i, entry) in server_entries.iter().enumerate() {
        let id = entry_value(entry, "id").ok_or(ConfigParseError::MalformedEntry {
            section: "mcp.servers",
            expected_key: "id",
            index: i,
        })?;
        let enabled_raw =
            entry_value(entry, "enabled").ok_or(ConfigParseError::MalformedEntry {
                section: "mcp.servers",
                expected_key: "enabled",
                index: i,
            })?;
        servers.push(McpServerConfig {
            id: leak_str(id),
            enabled: enabled_raw == "true",
        });
    }

    Ok(OwnedMcpConfig {
        enabled,
        discovery,
        servers,
    })
}

/// Parses the `budgets:` section.
fn parse_budgets_section(lines: &[&str]) -> Result<RuntimeBudgetConfig, ConfigParseError> {
    let budgets_start = lines
        .iter()
        .position(|l| l.trim() == "budgets:")
        .ok_or(ConfigParseError::MissingKey("budgets"))?;
    let budget_lines = &lines[budgets_start..];

    let prompt_tokens = extract_scalar(budget_lines, "prompt_tokens")
        .ok_or(ConfigParseError::MissingKey("budgets.prompt_tokens"))?
        .parse::<u32>()
        .map_err(|_| ConfigParseError::InvalidValue {
            key: "budgets.prompt_tokens",
            detail: "expected positive integer".to_owned(),
        })?;

    let context_tokens = extract_scalar(budget_lines, "context_tokens")
        .ok_or(ConfigParseError::MissingKey("budgets.context_tokens"))?
        .parse::<u32>()
        .map_err(|_| ConfigParseError::InvalidValue {
            key: "budgets.context_tokens",
            detail: "expected positive integer".to_owned(),
        })?;

    Ok(RuntimeBudgetConfig {
        prompt_tokens,
        context_tokens,
    })
}

/// Parses the optional `run:` section.
fn parse_run_section(lines: &[&str]) -> RunConfig {
    let Some(run_start) = lines.iter().position(|l| l.trim() == "run:") else {
        return RunConfig::default();
    };
    let run_lines = section_lines(lines, run_start);

    RunConfig {
        workflow_plugin: extract_scalar(run_lines, "workflow_plugin").map(leak_str),
        agent_plugin: extract_scalar(run_lines, "agent_plugin").map(leak_str),
        agent_id: extract_scalar(run_lines, "agent_id").map(leak_str),
        mode: extract_scalar(run_lines, "mode")
            .map(leak_str)
            .unwrap_or("loop"),
    }
}

/// Returns the slice of lines belonging to a top-level section, stopping
/// at the next peer-level key (a non-blank, non-comment line with zero indent).
fn section_lines<'a>(lines: &'a [&str], section_start: usize) -> &'a [&'a str] {
    let end = lines[section_start + 1..]
        .iter()
        .position(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with('#') && indent_level(line) == 0
        })
        .map_or(lines.len(), |offset| section_start + 1 + offset);
    &lines[section_start..end]
}

/// Collects YAML list entries (lines starting with `- `) under a section header.
///
/// Each entry is a slice of consecutive indented lines following a `- ` marker.
fn collect_list_entries<'a>(lines: &[&'a str], header: &str) -> Vec<Vec<&'a str>> {
    let Some(start) = lines
        .iter()
        .position(|l| l.trim_start().starts_with(header))
    else {
        return Vec::new();
    };

    let header_indent = indent_level(lines[start]);
    let entry_indent = header_indent + 2;
    let mut entries: Vec<Vec<&'a str>> = Vec::new();
    let mut current: Option<Vec<&'a str>> = None;

    for line in &lines[start + 1..] {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let line_indent = indent_level(line);
        if line_indent < entry_indent {
            break; // left the section
        }

        if trimmed.starts_with("- ") && line_indent == entry_indent {
            if let Some(entry) = current.take() {
                entries.push(entry);
            }
            current = Some(vec![trimmed]);
        } else if let Some(ref mut entry) = current {
            entry.push(trimmed);
        }
    }

    if let Some(entry) = current {
        entries.push(entry);
    }

    entries
}

/// Extracts a value from within a list entry (e.g., `"id"` from `"- id: foo"` or `"id: foo"`).
fn entry_value<'a>(entry: &[&'a str], key: &str) -> Option<&'a str> {
    let dash_prefix = format!("- {key}:");
    let plain_prefix = format!("{key}:");

    entry.iter().find_map(|line| {
        let trimmed = line.trim();
        trimmed
            .strip_prefix(&dash_prefix)
            .or_else(|| trimmed.strip_prefix(&plain_prefix))
            .map(str::trim)
    })
}

/// Returns the leading whitespace count of a line.
fn indent_level(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

/// Leaks a string slice into a `&'static str`.
///
/// Config is loaded once at startup and lives for the process lifetime,
/// so this avoids the need for owned `String` fields in the typed config
/// structs that use `&'static str`.
fn leak_str(s: &str) -> &'static str {
    Box::leak(s.to_owned().into_boxed_str())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod parse_tests {
    use super::*;

    #[test]
    fn roundtrip_default_config() {
        let original = materialize_project_config(&default_project_config());
        let yaml = render_owned_project_config_yaml(&original);
        let parsed =
            parse_owned_project_config_yaml(&yaml).unwrap_or_else(|e| panic!("parse failed: {e}"));

        assert_eq!(parsed.schema_version, original.schema_version);
        assert_eq!(parsed.default_locale, original.default_locale);
        assert_eq!(parsed.plugins.len(), original.plugins.len());
        assert_eq!(parsed.budgets, original.budgets);
        assert_eq!(parsed.mcp.enabled, original.mcp.enabled);
        assert_eq!(parsed.mcp.discovery, original.mcp.discovery);
    }

    #[test]
    fn roundtrip_full_config_with_all_plugins() {
        let config = OwnedProjectConfig {
            schema_version: 1,
            default_locale: "pt-br",
            plugins: vec![
                PluginConfig::new("official.basic", PluginActivation::Enabled),
                PluginConfig::new("official.bmad", PluginActivation::Enabled),
                PluginConfig::new("official.claude", PluginActivation::Disabled),
            ],
            mcp: OwnedMcpConfig {
                enabled: true,
                discovery: McpDiscovery::OfficialOnly,
                servers: vec![
                    McpServerConfig::new("official.claude.session", true),
                    McpServerConfig::new("official.github.repository", false),
                ],
            },
            budgets: RuntimeBudgetConfig {
                prompt_tokens: 4096,
                context_tokens: 16384,
            },
            run: RunConfig::default(),
            context: ContextConfig::default(),
            projects: Vec::new(),
        };

        let yaml = render_owned_project_config_yaml(&config);
        let parsed =
            parse_owned_project_config_yaml(&yaml).unwrap_or_else(|e| panic!("parse failed: {e}"));

        assert_eq!(parsed.schema_version, 1);
        assert_eq!(parsed.default_locale, "pt-br");
        assert_eq!(parsed.plugins.len(), 3);
        assert_eq!(parsed.plugins[0].id, "official.basic");
        assert_eq!(parsed.plugins[0].activation, PluginActivation::Enabled);
        assert_eq!(parsed.plugins[2].id, "official.claude");
        assert_eq!(parsed.plugins[2].activation, PluginActivation::Disabled);
        assert_eq!(parsed.mcp.servers.len(), 2);
        assert!(parsed.mcp.servers[0].enabled);
        assert!(!parsed.mcp.servers[1].enabled);
        assert_eq!(parsed.budgets.prompt_tokens, 4096);
    }

    #[test]
    fn parse_tolerates_comments_and_blank_lines() {
        let yaml = "\
# Ralph Engine project configuration
schema_version: 1

default_locale: en

# Plugins
plugins:
  - id: official.basic
    activation: enabled

mcp:
  enabled: true
  discovery: official_only
  servers:
    - id: official.claude.session
      enabled: true

budgets:
  prompt_tokens: 8192
  context_tokens: 32768
";
        let parsed =
            parse_owned_project_config_yaml(yaml).unwrap_or_else(|e| panic!("parse failed: {e}"));
        assert_eq!(parsed.plugins.len(), 1);
        assert_eq!(parsed.plugins[0].id, "official.basic");
    }

    #[test]
    fn parse_rejects_missing_schema_version() {
        let yaml = "default_locale: en\nplugins:\nmcp:\n  enabled: true\n  discovery: official_only\n  servers:\nbudgets:\n  prompt_tokens: 8192\n  context_tokens: 32768";
        let err = parse_owned_project_config_yaml(yaml).unwrap_err();
        assert_eq!(err, ConfigParseError::MissingKey("schema_version"));
    }

    #[test]
    fn parse_rejects_invalid_activation() {
        let yaml = "schema_version: 1\ndefault_locale: en\nplugins:\n  - id: test.plugin\n    activation: maybe\nmcp:\n  enabled: true\n  discovery: official_only\n  servers:\nbudgets:\n  prompt_tokens: 8192\n  context_tokens: 32768";
        let err = parse_owned_project_config_yaml(yaml).unwrap_err();
        matches!(
            err,
            ConfigParseError::InvalidValue {
                key: "plugins[].activation",
                ..
            }
        );
    }

    #[test]
    fn parse_rejects_unsupported_locale() {
        let yaml = "schema_version: 1\ndefault_locale: fr\nplugins:\nmcp:\n  enabled: true\n  discovery: official_only\n  servers:\nbudgets:\n  prompt_tokens: 8192\n  context_tokens: 32768";
        let err = parse_owned_project_config_yaml(yaml).unwrap_err();
        matches!(
            err,
            ConfigParseError::InvalidValue {
                key: "default_locale",
                ..
            }
        );
    }
}
