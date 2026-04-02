//! Shared plugin contracts for Ralph Engine.

use std::fmt;

mod i18n;

/// Extensible plugin capability identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginCapability(&'static str);

impl PluginCapability {
    /// Creates a new plugin capability identifier.
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    /// Returns the stable capability identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for PluginCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Template generation capability.
pub const TEMPLATE: PluginCapability = PluginCapability::new("template");
/// Prompt fragment contribution capability.
pub const PROMPT_FRAGMENTS: PluginCapability = PluginCapability::new("prompt_fragments");
/// Prepare-time validation contribution capability.
pub const PREPARE_CHECKS: PluginCapability = PluginCapability::new("prepare_checks");
/// Doctor-time validation contribution capability.
pub const DOCTOR_CHECKS: PluginCapability = PluginCapability::new("doctor_checks");
/// Agent runtime integration capability.
pub const AGENT_RUNTIME: PluginCapability = PluginCapability::new("agent_runtime");
/// MCP contribution capability.
pub const MCP_CONTRIBUTION: PluginCapability = PluginCapability::new("mcp_contribution");
/// Data source capability.
pub const DATA_SOURCE: PluginCapability = PluginCapability::new("data_source");
/// Context provider capability.
pub const CONTEXT_PROVIDER: PluginCapability = PluginCapability::new("context_provider");
/// Forge provider capability.
pub const FORGE_PROVIDER: PluginCapability = PluginCapability::new("forge_provider");
/// Remote control capability.
pub const REMOTE_CONTROL: PluginCapability = PluginCapability::new("remote_control");
/// Policy enforcement capability.
pub const POLICY: PluginCapability = PluginCapability::new("policy");

/// Canonical ordered list of reviewed plugin capabilities.
pub const ALL_PLUGIN_CAPABILITIES: &[PluginCapability] = &[
    TEMPLATE,
    PROMPT_FRAGMENTS,
    PREPARE_CHECKS,
    DOCTOR_CHECKS,
    AGENT_RUNTIME,
    MCP_CONTRIBUTION,
    DATA_SOURCE,
    CONTEXT_PROVIDER,
    FORGE_PROVIDER,
    REMOTE_CONTROL,
    POLICY,
];

/// Parses one reviewed plugin capability identifier.
#[must_use]
pub fn parse_reviewed_plugin_capability(value: &str) -> Option<PluginCapability> {
    match value {
        "template" => Some(TEMPLATE),
        "prompt_fragments" => Some(PROMPT_FRAGMENTS),
        "prepare_checks" => Some(PREPARE_CHECKS),
        "doctor_checks" => Some(DOCTOR_CHECKS),
        "agent_runtime" => Some(AGENT_RUNTIME),
        "mcp_contribution" => Some(MCP_CONTRIBUTION),
        "data_source" => Some(DATA_SOURCE),
        "context_provider" => Some(CONTEXT_PROVIDER),
        "forge_provider" => Some(FORGE_PROVIDER),
        "remote_control" => Some(REMOTE_CONTROL),
        "policy" => Some(POLICY),
        _ => None,
    }
}

/// Typed runtime surface identifier owned by reviewed plugin capabilities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PluginRuntimeSurface {
    /// Template-provider runtime surface.
    Templates,
    /// Prompt-provider runtime surface.
    Prompts,
    /// Runtime-check surface.
    Checks,
    /// Agent-runtime surface.
    Agents,
    /// MCP server contribution surface.
    Mcp,
    /// Shared provider surface for data, context, forge, and remote control.
    Providers,
    /// Policy-provider surface.
    Policies,
}

impl PluginRuntimeSurface {
    /// Returns the stable runtime-surface identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Templates => "templates",
            Self::Prompts => "prompts",
            Self::Checks => "checks",
            Self::Agents => "agents",
            Self::Mcp => "mcp",
            Self::Providers => "providers",
            Self::Policies => "policies",
        }
    }
}

impl fmt::Display for PluginRuntimeSurface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Canonical ordered list of reviewed runtime surfaces owned by plugin capabilities.
pub const ALL_PLUGIN_RUNTIME_SURFACES: &[PluginRuntimeSurface] = &[
    PluginRuntimeSurface::Templates,
    PluginRuntimeSurface::Prompts,
    PluginRuntimeSurface::Checks,
    PluginRuntimeSurface::Agents,
    PluginRuntimeSurface::Mcp,
    PluginRuntimeSurface::Providers,
    PluginRuntimeSurface::Policies,
];

/// Resolves the dedicated runtime surface that owns one reviewed capability.
#[must_use]
pub fn runtime_surface_for_capability(
    capability: PluginCapability,
) -> Option<PluginRuntimeSurface> {
    match capability {
        TEMPLATE => Some(PluginRuntimeSurface::Templates),
        PROMPT_FRAGMENTS => Some(PluginRuntimeSurface::Prompts),
        PREPARE_CHECKS | DOCTOR_CHECKS => Some(PluginRuntimeSurface::Checks),
        AGENT_RUNTIME => Some(PluginRuntimeSurface::Agents),
        MCP_CONTRIBUTION => Some(PluginRuntimeSurface::Mcp),
        DATA_SOURCE | CONTEXT_PROVIDER | FORGE_PROVIDER | REMOTE_CONTROL => {
            Some(PluginRuntimeSurface::Providers)
        }
        POLICY => Some(PluginRuntimeSurface::Policies),
        _ => None,
    }
}

/// Typed primary plugin kind identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PluginKind {
    /// Template-oriented plugin.
    Template,
    /// Agent runtime plugin.
    AgentRuntime,
    /// Forge provider plugin.
    ForgeProvider,
    /// Context provider plugin.
    ContextProvider,
    /// Data source plugin.
    DataSource,
    /// Remote control plugin.
    RemoteControl,
    /// MCP contribution plugin.
    McpContribution,
    /// Policy plugin.
    Policy,
}

impl PluginKind {
    /// Returns the stable plugin kind identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Template => "template",
            Self::AgentRuntime => "agent_runtime",
            Self::ForgeProvider => "forge_provider",
            Self::ContextProvider => "context_provider",
            Self::DataSource => "data_source",
            Self::RemoteControl => "remote_control",
            Self::McpContribution => "mcp_contribution",
            Self::Policy => "policy",
        }
    }
}

impl fmt::Display for PluginKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Canonical ordered list of reviewed plugin kinds.
pub const ALL_PLUGIN_KINDS: &[PluginKind] = &[
    PluginKind::Template,
    PluginKind::AgentRuntime,
    PluginKind::ForgeProvider,
    PluginKind::ContextProvider,
    PluginKind::DataSource,
    PluginKind::RemoteControl,
    PluginKind::McpContribution,
    PluginKind::Policy,
];

/// Typed plugin trust-level identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PluginTrustLevel {
    /// Official first-party plugin.
    Official,
    /// Community plugin outside the trusted official set.
    Community,
}

impl PluginTrustLevel {
    /// Returns the stable plugin trust-level identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Official => "official",
            Self::Community => "community",
        }
    }
}

impl fmt::Display for PluginTrustLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Canonical ordered list of reviewed plugin trust levels.
pub const ALL_PLUGIN_TRUST_LEVELS: &[PluginTrustLevel] =
    &[PluginTrustLevel::Official, PluginTrustLevel::Community];

/// One localized plugin text entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginLocalizedText {
    /// Stable locale identifier.
    pub locale: &'static str,
    /// Localized text value.
    pub value: &'static str,
}

impl PluginLocalizedText {
    /// Creates a new immutable localized plugin text entry.
    #[must_use]
    pub const fn new(locale: &'static str, value: &'static str) -> Self {
        Self { locale, value }
    }
}

fn resolve_localized_text<'a>(
    localized_values: &'a [PluginLocalizedText],
    locale: &str,
    fallback: &'a str,
) -> &'a str {
    let locale = re_config::resolve_supported_locale_or_default(locale.trim()).as_str();

    localized_values
        .iter()
        .find(|entry| entry.locale == locale)
        .map_or(fallback, |entry| entry.value)
}

/// One immutable template asset entry owned by one template contribution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginTemplateAsset {
    /// Stable relative asset path exposed by the template.
    pub path: &'static str,
    /// Immutable embedded asset contents.
    pub contents: &'static str,
}

impl PluginTemplateAsset {
    /// Creates one immutable template asset entry.
    #[must_use]
    pub const fn new(path: &'static str, contents: &'static str) -> Self {
        Self { path, contents }
    }
}

/// One immutable prompt asset entry owned by one prompt contribution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginPromptAsset {
    /// Stable relative asset path exposed by the prompt contribution.
    pub path: &'static str,
    /// Immutable embedded prompt contents.
    pub contents: &'static str,
}

impl PluginPromptAsset {
    /// Creates one immutable prompt asset entry.
    #[must_use]
    pub const fn new(path: &'static str, contents: &'static str) -> Self {
        Self { path, contents }
    }
}

/// One immutable check asset entry owned by one check contribution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginCheckAsset {
    /// Stable relative asset path exposed by the check contribution.
    pub path: &'static str,
    /// Immutable embedded check contents.
    pub contents: &'static str,
}

impl PluginCheckAsset {
    /// Creates a new immutable check asset entry.
    #[must_use]
    pub const fn new(path: &'static str, contents: &'static str) -> Self {
        Self { path, contents }
    }
}

/// One immutable policy asset entry owned by one policy contribution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginPolicyAsset {
    /// Stable relative asset path exposed by the policy contribution.
    pub path: &'static str,
    /// Immutable embedded policy contents.
    pub contents: &'static str,
}

impl PluginPolicyAsset {
    /// Creates a new immutable policy asset entry.
    #[must_use]
    pub const fn new(path: &'static str, contents: &'static str) -> Self {
        Self { path, contents }
    }
}

/// Typed plugin-owned check kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PluginCheckKind {
    /// Prepare-time validation contribution.
    Prepare,
    /// Doctor-time validation contribution.
    Doctor,
}

impl PluginCheckKind {
    /// Returns the stable check-kind identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Prepare => "prepare",
            Self::Doctor => "doctor",
        }
    }
}

impl fmt::Display for PluginCheckKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Typed plugin-owned provider kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PluginProviderKind {
    /// Data-source provider contribution.
    DataSource,
    /// Context-provider contribution.
    ContextProvider,
    /// Forge-provider contribution.
    ForgeProvider,
    /// Remote-control contribution.
    RemoteControl,
}

impl PluginProviderKind {
    /// Returns the stable provider-kind identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DataSource => "data_source",
            Self::ContextProvider => "context_provider",
            Self::ForgeProvider => "forge_provider",
            Self::RemoteControl => "remote_control",
        }
    }
}

impl fmt::Display for PluginProviderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Immutable check contribution owned by one plugin.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginCheckDescriptor {
    /// Stable check identifier.
    pub id: &'static str,
    /// Plugin that owns the check.
    pub plugin_id: &'static str,
    /// Typed check kind.
    pub kind: PluginCheckKind,
    /// Human-readable check name.
    pub name: &'static str,
    /// Optional localized check names keyed by locale.
    pub localized_names: &'static [PluginLocalizedText],
    /// Human-readable English summary.
    pub summary: &'static str,
    /// Optional localized check summaries keyed by locale.
    pub localized_summaries: &'static [PluginLocalizedText],
    /// Immutable embedded assets exposed by this check contribution.
    pub assets: &'static [PluginCheckAsset],
}

impl PluginCheckDescriptor {
    /// Creates a new immutable check descriptor.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        id: &'static str,
        plugin_id: &'static str,
        kind: PluginCheckKind,
        name: &'static str,
        localized_names: &'static [PluginLocalizedText],
        summary: &'static str,
        localized_summaries: &'static [PluginLocalizedText],
        assets: &'static [PluginCheckAsset],
    ) -> Self {
        Self {
            id,
            plugin_id,
            kind,
            name,
            localized_names,
            summary,
            localized_summaries,
            assets,
        }
    }

    /// Resolves the display name for a locale with English fallback.
    #[must_use]
    pub fn display_name_for_locale(&self, locale: &str) -> &'static str {
        resolve_localized_text(self.localized_names, locale, self.name)
    }

    /// Resolves the summary for a locale with English fallback.
    #[must_use]
    pub fn summary_for_locale(&self, locale: &str) -> &'static str {
        resolve_localized_text(self.localized_summaries, locale, self.summary)
    }

    /// Returns whether the check exposes embedded assets.
    #[must_use]
    pub const fn has_assets(&self) -> bool {
        !self.assets.is_empty()
    }
}

/// Immutable provider contribution owned by one plugin.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginProviderDescriptor {
    /// Stable provider identifier.
    pub id: &'static str,
    /// Plugin that owns the provider.
    pub plugin_id: &'static str,
    /// Typed provider kind.
    pub kind: PluginProviderKind,
    /// Human-readable provider name.
    pub name: &'static str,
    /// Optional localized provider names keyed by locale.
    pub localized_names: &'static [PluginLocalizedText],
    /// Human-readable English summary.
    pub summary: &'static str,
    /// Optional localized provider summaries keyed by locale.
    pub localized_summaries: &'static [PluginLocalizedText],
}

impl PluginProviderDescriptor {
    /// Creates a new immutable provider descriptor.
    #[must_use]
    pub const fn new(
        id: &'static str,
        plugin_id: &'static str,
        kind: PluginProviderKind,
        name: &'static str,
        localized_names: &'static [PluginLocalizedText],
        summary: &'static str,
        localized_summaries: &'static [PluginLocalizedText],
    ) -> Self {
        Self {
            id,
            plugin_id,
            kind,
            name,
            localized_names,
            summary,
            localized_summaries,
        }
    }

    /// Resolves the display name for a locale with English fallback.
    #[must_use]
    pub fn display_name_for_locale(&self, locale: &str) -> &'static str {
        resolve_localized_text(self.localized_names, locale, self.name)
    }

    /// Resolves the summary for a locale with English fallback.
    #[must_use]
    pub fn summary_for_locale(&self, locale: &str) -> &'static str {
        resolve_localized_text(self.localized_summaries, locale, self.summary)
    }
}

/// Immutable template contribution owned by one plugin.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginTemplateDescriptor {
    /// Stable template identifier.
    pub id: &'static str,
    /// Plugin that owns the template.
    pub plugin_id: &'static str,
    /// Human-readable template name.
    pub name: &'static str,
    /// Optional localized template names keyed by locale.
    pub localized_names: &'static [PluginLocalizedText],
    /// Human-readable English summary.
    pub summary: &'static str,
    /// Optional localized template summaries keyed by locale.
    pub localized_summaries: &'static [PluginLocalizedText],
    /// Immutable assets exposed by the template contribution.
    pub assets: &'static [PluginTemplateAsset],
}

impl PluginTemplateDescriptor {
    /// Creates a new immutable template descriptor.
    #[must_use]
    pub const fn new(
        id: &'static str,
        plugin_id: &'static str,
        name: &'static str,
        localized_names: &'static [PluginLocalizedText],
        summary: &'static str,
        localized_summaries: &'static [PluginLocalizedText],
        assets: &'static [PluginTemplateAsset],
    ) -> Self {
        Self {
            id,
            plugin_id,
            name,
            localized_names,
            summary,
            localized_summaries,
            assets,
        }
    }

    /// Resolves the display name for a locale with English fallback.
    #[must_use]
    pub fn display_name_for_locale(&self, locale: &str) -> &'static str {
        resolve_localized_text(self.localized_names, locale, self.name)
    }

    /// Resolves the summary for a locale with English fallback.
    #[must_use]
    pub fn summary_for_locale(&self, locale: &str) -> &'static str {
        resolve_localized_text(self.localized_summaries, locale, self.summary)
    }

    /// Returns whether the template exposes embedded assets.
    #[must_use]
    pub const fn has_assets(&self) -> bool {
        !self.assets.is_empty()
    }
}

/// Immutable prompt contribution owned by one plugin.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginPromptDescriptor {
    /// Stable prompt identifier.
    pub id: &'static str,
    /// Plugin that owns the prompt.
    pub plugin_id: &'static str,
    /// Human-readable prompt name.
    pub name: &'static str,
    /// Optional localized prompt names keyed by locale.
    pub localized_names: &'static [PluginLocalizedText],
    /// Human-readable English summary.
    pub summary: &'static str,
    /// Optional localized prompt summaries keyed by locale.
    pub localized_summaries: &'static [PluginLocalizedText],
    /// Immutable prompt assets exposed by this contribution.
    pub assets: &'static [PluginPromptAsset],
}

impl PluginPromptDescriptor {
    /// Creates a new immutable prompt descriptor.
    #[must_use]
    pub const fn new(
        id: &'static str,
        plugin_id: &'static str,
        name: &'static str,
        localized_names: &'static [PluginLocalizedText],
        summary: &'static str,
        localized_summaries: &'static [PluginLocalizedText],
        assets: &'static [PluginPromptAsset],
    ) -> Self {
        Self {
            id,
            plugin_id,
            name,
            localized_names,
            summary,
            localized_summaries,
            assets,
        }
    }

    /// Resolves the display name for a locale with English fallback.
    #[must_use]
    pub fn display_name_for_locale(&self, locale: &str) -> &'static str {
        resolve_localized_text(self.localized_names, locale, self.name)
    }

    /// Resolves the summary for a locale with English fallback.
    #[must_use]
    pub fn summary_for_locale(&self, locale: &str) -> &'static str {
        resolve_localized_text(self.localized_summaries, locale, self.summary)
    }

    /// Returns whether the prompt exposes embedded assets.
    #[must_use]
    pub const fn has_assets(&self) -> bool {
        !self.assets.is_empty()
    }
}

/// Immutable agent runtime contribution owned by one plugin.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginAgentDescriptor {
    /// Stable agent identifier.
    pub id: &'static str,
    /// Plugin that owns the agent runtime.
    pub plugin_id: &'static str,
    /// Human-readable agent name.
    pub name: &'static str,
    /// Optional localized agent names keyed by locale.
    pub localized_names: &'static [PluginLocalizedText],
    /// Human-readable English summary.
    pub summary: &'static str,
    /// Optional localized agent summaries keyed by locale.
    pub localized_summaries: &'static [PluginLocalizedText],
}

impl PluginAgentDescriptor {
    /// Creates a new immutable agent descriptor.
    #[must_use]
    pub const fn new(
        id: &'static str,
        plugin_id: &'static str,
        name: &'static str,
        localized_names: &'static [PluginLocalizedText],
        summary: &'static str,
        localized_summaries: &'static [PluginLocalizedText],
    ) -> Self {
        Self {
            id,
            plugin_id,
            name,
            localized_names,
            summary,
            localized_summaries,
        }
    }

    /// Resolves the display name for a locale with English fallback.
    #[must_use]
    pub fn display_name_for_locale(&self, locale: &str) -> &'static str {
        resolve_localized_text(self.localized_names, locale, self.name)
    }

    /// Resolves the summary for a locale with English fallback.
    #[must_use]
    pub fn summary_for_locale(&self, locale: &str) -> &'static str {
        resolve_localized_text(self.localized_summaries, locale, self.summary)
    }
}

/// Immutable policy contribution owned by one plugin.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginPolicyDescriptor {
    /// Stable policy identifier.
    pub id: &'static str,
    /// Plugin that owns the policy.
    pub plugin_id: &'static str,
    /// Human-readable policy name.
    pub name: &'static str,
    /// Optional localized policy names keyed by locale.
    pub localized_names: &'static [PluginLocalizedText],
    /// Human-readable English summary.
    pub summary: &'static str,
    /// Optional localized policy summaries keyed by locale.
    pub localized_summaries: &'static [PluginLocalizedText],
    /// Immutable embedded assets exposed by this policy contribution.
    pub assets: &'static [PluginPolicyAsset],
}

impl PluginPolicyDescriptor {
    /// Creates a new immutable policy descriptor.
    #[must_use]
    pub const fn new(
        id: &'static str,
        plugin_id: &'static str,
        name: &'static str,
        localized_names: &'static [PluginLocalizedText],
        summary: &'static str,
        localized_summaries: &'static [PluginLocalizedText],
        assets: &'static [PluginPolicyAsset],
    ) -> Self {
        Self {
            id,
            plugin_id,
            name,
            localized_names,
            summary,
            localized_summaries,
            assets,
        }
    }

    /// Resolves the display name for a locale with English fallback.
    #[must_use]
    pub fn display_name_for_locale(&self, locale: &str) -> &'static str {
        resolve_localized_text(self.localized_names, locale, self.name)
    }

    /// Resolves the summary for a locale with English fallback.
    #[must_use]
    pub fn summary_for_locale(&self, locale: &str) -> &'static str {
        resolve_localized_text(self.localized_summaries, locale, self.summary)
    }

    /// Returns whether the policy exposes embedded assets.
    #[must_use]
    pub const fn has_assets(&self) -> bool {
        !self.assets.is_empty()
    }
}

/// Typed plugin lifecycle stage identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PluginLifecycleStage {
    /// The runtime can discover the plugin and list it in catalogs.
    Discover,
    /// The runtime can configure the plugin through typed configuration.
    Configure,
    /// The runtime can validate the plugin before activation.
    Validate,
    /// The runtime can load the plugin into the active runtime.
    Load,
}

impl PluginLifecycleStage {
    /// Returns the stable lifecycle stage identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Discover => "discover",
            Self::Configure => "configure",
            Self::Validate => "validate",
            Self::Load => "load",
        }
    }
}

impl fmt::Display for PluginLifecycleStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Typed plugin loading boundary identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PluginLoadBoundary {
    /// The plugin is loaded in process with the runtime.
    InProcess,
    /// The plugin runs behind a subprocess boundary.
    Subprocess,
    /// The plugin is resolved through a remote boundary.
    Remote,
}

impl PluginLoadBoundary {
    /// Returns the stable loading-boundary identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InProcess => "in_process",
            Self::Subprocess => "subprocess",
            Self::Remote => "remote",
        }
    }
}

impl fmt::Display for PluginLoadBoundary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Typed runtime hook identifier for plugin contributions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PluginRuntimeHook {
    /// The plugin contributes project scaffolding behavior.
    Scaffold,
    /// The plugin contributes prepare-time checks.
    Prepare,
    /// The plugin contributes doctor-time checks.
    Doctor,
    /// The plugin contributes prompt assembly behavior.
    PromptAssembly,
    /// The plugin contributes agent runtime bootstrap behavior.
    AgentBootstrap,
    /// The plugin contributes MCP server registration.
    McpRegistration,
    /// The plugin contributes data-source registration.
    DataSourceRegistration,
    /// The plugin contributes context-provider registration.
    ContextProviderRegistration,
    /// The plugin contributes forge-provider registration.
    ForgeProviderRegistration,
    /// The plugin contributes remote-control bootstrap behavior.
    RemoteControlBootstrap,
    /// The plugin contributes policy enforcement behavior.
    PolicyEnforcement,
}

impl PluginRuntimeHook {
    /// Returns the stable runtime-hook identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scaffold => "scaffold",
            Self::Prepare => "prepare",
            Self::Doctor => "doctor",
            Self::PromptAssembly => "prompt_assembly",
            Self::AgentBootstrap => "agent_bootstrap",
            Self::McpRegistration => "mcp_registration",
            Self::DataSourceRegistration => "data_source_registration",
            Self::ContextProviderRegistration => "context_provider_registration",
            Self::ForgeProviderRegistration => "forge_provider_registration",
            Self::RemoteControlBootstrap => "remote_control_bootstrap",
            Self::PolicyEnforcement => "policy_enforcement",
        }
    }
}

/// Canonical ordered list of reviewed runtime hooks.
pub const ALL_PLUGIN_RUNTIME_HOOKS: &[PluginRuntimeHook] = &[
    PluginRuntimeHook::Scaffold,
    PluginRuntimeHook::Prepare,
    PluginRuntimeHook::Doctor,
    PluginRuntimeHook::PromptAssembly,
    PluginRuntimeHook::AgentBootstrap,
    PluginRuntimeHook::McpRegistration,
    PluginRuntimeHook::DataSourceRegistration,
    PluginRuntimeHook::ContextProviderRegistration,
    PluginRuntimeHook::ForgeProviderRegistration,
    PluginRuntimeHook::RemoteControlBootstrap,
    PluginRuntimeHook::PolicyEnforcement,
];

/// Parses one stable runtime-hook identifier.
#[must_use]
pub fn parse_plugin_runtime_hook(value: &str) -> Option<PluginRuntimeHook> {
    match value {
        "scaffold" => Some(PluginRuntimeHook::Scaffold),
        "prepare" => Some(PluginRuntimeHook::Prepare),
        "doctor" => Some(PluginRuntimeHook::Doctor),
        "prompt_assembly" => Some(PluginRuntimeHook::PromptAssembly),
        "agent_bootstrap" => Some(PluginRuntimeHook::AgentBootstrap),
        "mcp_registration" => Some(PluginRuntimeHook::McpRegistration),
        "data_source_registration" => Some(PluginRuntimeHook::DataSourceRegistration),
        "context_provider_registration" => Some(PluginRuntimeHook::ContextProviderRegistration),
        "forge_provider_registration" => Some(PluginRuntimeHook::ForgeProviderRegistration),
        "remote_control_bootstrap" => Some(PluginRuntimeHook::RemoteControlBootstrap),
        "policy_enforcement" => Some(PluginRuntimeHook::PolicyEnforcement),
        _ => None,
    }
}

impl fmt::Display for PluginRuntimeHook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Immutable metadata for a Ralph Engine plugin.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginDescriptor {
    /// Stable plugin identifier.
    pub id: &'static str,
    /// Stable primary plugin kind.
    pub kind: PluginKind,
    /// Stable plugin trust level.
    pub trust_level: PluginTrustLevel,
    /// Human-readable plugin name.
    pub name: &'static str,
    /// Optional localized plugin names keyed by locale.
    pub localized_names: &'static [PluginLocalizedText],
    /// Human-readable English summary.
    pub summary: &'static str,
    /// Optional localized plugin summaries keyed by locale.
    pub localized_summaries: &'static [PluginLocalizedText],
    /// Published plugin version.
    pub version: &'static str,
    /// Declared plugin capabilities.
    pub capabilities: &'static [PluginCapability],
    /// Declared lifecycle stages supported by the plugin.
    pub lifecycle: &'static [PluginLifecycleStage],
    /// Declared runtime loading boundary for the plugin.
    pub load_boundary: PluginLoadBoundary,
    /// Declared runtime hooks contributed by the plugin.
    pub runtime_hooks: &'static [PluginRuntimeHook],
}

impl PluginDescriptor {
    /// Creates a new immutable plugin descriptor.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        id: &'static str,
        kind: PluginKind,
        trust_level: PluginTrustLevel,
        name: &'static str,
        localized_names: &'static [PluginLocalizedText],
        summary: &'static str,
        localized_summaries: &'static [PluginLocalizedText],
        version: &'static str,
        capabilities: &'static [PluginCapability],
        lifecycle: &'static [PluginLifecycleStage],
        load_boundary: PluginLoadBoundary,
        runtime_hooks: &'static [PluginRuntimeHook],
    ) -> Self {
        Self {
            id,
            kind,
            trust_level,
            name,
            localized_names,
            summary,
            localized_summaries,
            version,
            capabilities,
            lifecycle,
            load_boundary,
            runtime_hooks,
        }
    }

    /// Returns whether the plugin identifier uses a namespace prefix.
    #[must_use]
    pub fn is_namespaced(&self) -> bool {
        self.id.contains('.')
    }

    /// Returns whether the plugin declares at least one capability.
    #[must_use]
    pub fn has_capabilities(&self) -> bool {
        !self.capabilities.is_empty()
    }

    /// Returns whether the plugin declares at least one lifecycle stage.
    #[must_use]
    pub fn has_lifecycle(&self) -> bool {
        !self.lifecycle.is_empty()
    }

    /// Returns whether the plugin declares at least one runtime hook.
    #[must_use]
    pub fn has_runtime_hooks(&self) -> bool {
        !self.runtime_hooks.is_empty()
    }

    /// Resolves the display name for a locale with English fallback.
    #[must_use]
    pub fn display_name_for_locale(&self, locale: &str) -> &'static str {
        resolve_localized_text(self.localized_names, locale, self.name)
    }

    /// Resolves the plugin summary for a locale with English fallback.
    #[must_use]
    pub fn summary_for_locale(&self, locale: &str) -> &'static str {
        resolve_localized_text(self.localized_summaries, locale, self.summary)
    }
}

/// Renders a human-readable plugin listing.
#[must_use]
pub fn render_plugin_listing(plugins: &[PluginDescriptor]) -> String {
    render_plugin_listing_for_locale(plugins, "en")
}

/// Renders a human-readable plugin listing for one locale.
#[must_use]
pub fn render_plugin_listing_for_locale(plugins: &[PluginDescriptor], locale: &str) -> String {
    let mut lines = Vec::with_capacity(plugins.len() + 1);
    lines.push(format!(
        "{} ({})",
        i18n::official_plugins_label(locale),
        plugins.len()
    ));

    for plugin in plugins {
        let capabilities = plugin
            .capabilities
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        lines.push(format!(
            "- {} | {} | {} | {} | v{} | {} | {}",
            plugin.id,
            plugin.kind,
            plugin.trust_level,
            plugin.display_name_for_locale(locale),
            plugin.version,
            capabilities,
            plugin.summary_for_locale(locale)
        ));
    }

    lines.join("\n")
}

/// Renders a human-readable plugin detail block.
#[must_use]
pub fn render_plugin_detail(plugin: &PluginDescriptor) -> String {
    render_plugin_detail_for_locale(plugin, "en")
}

/// Renders a human-readable plugin detail block for one locale.
#[must_use]
pub fn render_plugin_detail_for_locale(plugin: &PluginDescriptor, locale: &str) -> String {
    let capabilities = plugin
        .capabilities
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    let lifecycle = plugin
        .lifecycle
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" -> ");
    let runtime_hooks = plugin
        .runtime_hooks
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: v{}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}",
        i18n::plugin_label(locale),
        plugin.id,
        i18n::kind_label(locale),
        plugin.kind,
        i18n::trust_label(locale),
        plugin.trust_level,
        i18n::name_label(locale),
        plugin.display_name_for_locale(locale),
        i18n::version_label(locale),
        plugin.version,
        i18n::summary_label(locale),
        plugin.summary_for_locale(locale),
        i18n::capabilities_label(locale),
        capabilities,
        i18n::lifecycle_label(locale),
        lifecycle,
        i18n::load_boundary_label(locale),
        plugin.load_boundary,
        i18n::runtime_hooks_label(locale),
        runtime_hooks
    )
}
