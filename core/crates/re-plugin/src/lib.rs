//! Shared plugin contracts for Ralph Engine.

use std::fmt;
use std::path::Path;

pub mod agent_helpers;
mod i18n;

/// Generates a `#[non_exhaustive]` enum with `as_str()`, `Display`, and a
/// canonical `ALL_*` constant from a single declaration.
///
/// # Example
///
/// ```ignore
/// define_plugin_enum! {
///     /// Doc comment for the enum.
///     pub enum PluginKind => ALL_PLUGIN_KINDS {
///         /// Template-oriented plugin.
///         Template => "template",
///         /// Agent runtime plugin.
///         AgentRuntime => "agent_runtime",
///     }
/// }
/// ```
///
/// This expands to the enum definition, `as_str()` match, `Display` impl,
/// and a `pub const ALL_PLUGIN_KINDS: &[PluginKind]` array — all in one
/// place. Adding a new variant means adding one line.
macro_rules! define_plugin_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident => $all_const:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident => $str_val:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
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

        /// Canonical ordered list of all reviewed variants.
        pub const $all_const: &[$name] = &[
            $($name::$variant,)+
        ];
    };
}

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
/// Workflow orchestration capability (resolve work items, build prompts).
pub const WORKFLOW: PluginCapability = PluginCapability::new("workflow");
/// TUI dashboard widget contribution capability.
pub const TUI_WIDGETS: PluginCapability = PluginCapability::new("tui_widgets");
/// Context management capability (export, import, compact, persist sessions).
pub const CONTEXT_MANAGEMENT: PluginCapability = PluginCapability::new("context_management");
/// Session persistence capability (save/load sessions to disk).
pub const SESSION_PERSISTENCE: PluginCapability = PluginCapability::new("session_persistence");
/// Agent routing capability (task classification, agent/model selection).
pub const AGENT_ROUTING: PluginCapability = PluginCapability::new("agent_routing");

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
    WORKFLOW,
    TUI_WIDGETS,
    CONTEXT_MANAGEMENT,
    SESSION_PERSISTENCE,
    AGENT_ROUTING,
];

/// Parses one reviewed plugin capability identifier.
#[must_use]
pub fn parse_reviewed_plugin_capability(value: &str) -> Option<PluginCapability> {
    ALL_PLUGIN_CAPABILITIES
        .iter()
        .find(|cap| cap.as_str() == value)
        .copied()
}

define_plugin_enum! {
    /// Typed runtime surface identifier owned by reviewed plugin capabilities.
    pub enum PluginRuntimeSurface => ALL_PLUGIN_RUNTIME_SURFACES {
        /// Template-provider runtime surface.
        Templates => "templates",
        /// Prompt-provider runtime surface.
        Prompts => "prompts",
        /// Runtime-check surface.
        Checks => "checks",
        /// Agent-runtime surface.
        Agents => "agents",
        /// MCP server contribution surface.
        Mcp => "mcp",
        /// Shared provider surface for data, context, forge, and remote control.
        Providers => "providers",
        /// Policy-provider surface.
        Policies => "policies",
    }
}

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
        WORKFLOW => None, // Workflow uses hooks, not a dedicated surface
        _ => None,
    }
}

define_plugin_enum! {
    /// Typed primary plugin kind identifier.
    pub enum PluginKind => ALL_PLUGIN_KINDS {
        /// Template-oriented plugin.
        Template => "template",
        /// Agent runtime plugin.
        AgentRuntime => "agent_runtime",
        /// Forge provider plugin.
        ForgeProvider => "forge_provider",
        /// Context provider plugin.
        ContextProvider => "context_provider",
        /// Data source plugin.
        DataSource => "data_source",
        /// Remote control plugin.
        RemoteControl => "remote_control",
        /// MCP contribution plugin.
        McpContribution => "mcp_contribution",
        /// Policy plugin.
        Policy => "policy",
        /// Workflow orchestration plugin.
        Workflow => "workflow",
        /// TUI extension plugin (keybindings, panels, interactive controls).
        TuiExtension => "tui_extension",
        /// Context management plugin (session persistence, compaction, transfer).
        ContextManager => "context_manager",
        /// Agent routing plugin (task classification, agent selection).
        AgentRouter => "agent_router",
    }
}

define_plugin_enum! {
    /// Typed plugin trust-level identifier.
    pub enum PluginTrustLevel => ALL_PLUGIN_TRUST_LEVELS {
        /// Official first-party plugin.
        Official => "official",
        /// Community plugin outside the trusted official set.
        Community => "community",
    }
}

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

define_plugin_enum! {
    /// Typed plugin-owned check kind.
    pub enum PluginCheckKind => ALL_PLUGIN_CHECK_KINDS {
        /// Prepare-time validation contribution.
        Prepare => "prepare",
        /// Doctor-time validation contribution.
        Doctor => "doctor",
    }
}

define_plugin_enum! {
    /// Typed plugin-owned provider kind.
    pub enum PluginProviderKind => ALL_PLUGIN_PROVIDER_KINDS {
        /// Data-source provider contribution.
        DataSource => "data_source",
        /// Context-provider contribution.
        ContextProvider => "context_provider",
        /// Forge-provider contribution.
        ForgeProvider => "forge_provider",
        /// Remote-control contribution.
        RemoteControl => "remote_control",
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

define_plugin_enum! {
    /// Typed plugin lifecycle stage identifier.
    pub enum PluginLifecycleStage => ALL_PLUGIN_LIFECYCLE_STAGES {
        /// The runtime can discover the plugin and list it in catalogs.
        Discover => "discover",
        /// The runtime can configure the plugin through typed configuration.
        Configure => "configure",
        /// The runtime can validate the plugin before activation.
        Validate => "validate",
        /// The runtime can load the plugin into the active runtime.
        Load => "load",
    }
}

define_plugin_enum! {
    /// Typed plugin loading boundary identifier.
    pub enum PluginLoadBoundary => ALL_PLUGIN_LOAD_BOUNDARIES {
        /// The plugin is loaded in process with the runtime.
        InProcess => "in_process",
        /// The plugin runs behind a subprocess boundary.
        Subprocess => "subprocess",
        /// The plugin is resolved through a remote boundary.
        Remote => "remote",
    }
}

define_plugin_enum! {
    /// Typed runtime hook identifier for plugin contributions.
    pub enum PluginRuntimeHook => ALL_PLUGIN_RUNTIME_HOOKS {
        /// The plugin contributes project scaffolding behavior.
        Scaffold => "scaffold",
        /// The plugin contributes prepare-time checks.
        Prepare => "prepare",
        /// The plugin contributes doctor-time checks.
        Doctor => "doctor",
        /// The plugin contributes prompt assembly behavior.
        PromptAssembly => "prompt_assembly",
        /// The plugin contributes agent runtime bootstrap behavior.
        AgentBootstrap => "agent_bootstrap",
        /// The plugin contributes MCP server registration.
        McpRegistration => "mcp_registration",
        /// The plugin contributes data-source registration.
        DataSourceRegistration => "data_source_registration",
        /// The plugin contributes context-provider registration.
        ContextProviderRegistration => "context_provider_registration",
        /// The plugin contributes forge-provider registration.
        ForgeProviderRegistration => "forge_provider_registration",
        /// The plugin contributes remote-control bootstrap behavior.
        RemoteControlBootstrap => "remote_control_bootstrap",
        /// The plugin contributes policy enforcement behavior.
        PolicyEnforcement => "policy_enforcement",
        /// The plugin contributes work-item resolution for the `run` command.
        WorkItemResolution => "work_item_resolution",
        /// The plugin contributes agent launch behavior for the `run` command.
        AgentLaunch => "agent_launch",
        /// The plugin contributes TUI dashboard panels.
        TuiContribution => "tui_contribution",
        /// The plugin provides context management (export, import, compact).
        ContextManagement => "context_management",
        /// The plugin provides session persistence (save, load).
        SessionPersistence => "session_persistence",
        /// The plugin provides agent routing (task classification).
        AgentRouting => "agent_routing",
    }
}

/// Parses one stable runtime-hook identifier.
#[must_use]
pub fn parse_plugin_runtime_hook(value: &str) -> Option<PluginRuntimeHook> {
    ALL_PLUGIN_RUNTIME_HOOKS
        .iter()
        .find(|hook| hook.as_str() == value)
        .copied()
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
    /// Minimum plugin API version required by this plugin (semver major).
    pub plugin_api_version: u16,
    /// Declared plugin capabilities.
    pub capabilities: &'static [PluginCapability],
    /// Declared lifecycle stages supported by the plugin.
    pub lifecycle: &'static [PluginLifecycleStage],
    /// Declared runtime loading boundary for the plugin.
    pub load_boundary: PluginLoadBoundary,
    /// Declared runtime hooks contributed by the plugin.
    pub runtime_hooks: &'static [PluginRuntimeHook],
}

/// Current plugin API version supported by this runtime.
pub const CURRENT_PLUGIN_API_VERSION: u16 = 1;

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
        plugin_api_version: u16,
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
            plugin_api_version,
            capabilities,
            lifecycle,
            load_boundary,
            runtime_hooks,
        }
    }

    /// Returns whether this plugin is compatible with the current runtime API version.
    #[must_use]
    pub const fn is_api_compatible(&self) -> bool {
        self.plugin_api_version <= CURRENT_PLUGIN_API_VERSION
    }

    /// Validates all plugin descriptor invariants.
    ///
    /// Returns a list of human-readable validation errors. An empty list means
    /// the descriptor is valid. This is used for plugin isolation — invalid
    /// plugins are excluded from the runtime instead of crashing.
    #[must_use]
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.id.is_empty() {
            errors.push("plugin id must not be empty".to_owned());
        }
        if !self.id.contains('.') {
            errors.push(format!(
                "plugin id '{}' must use a namespace prefix (e.g., 'official.name')",
                self.id
            ));
        }
        if self.name.is_empty() {
            errors.push("plugin name must not be empty".to_owned());
        }
        if self.version.is_empty() {
            errors.push("plugin version must not be empty".to_owned());
        }
        if !self.is_api_compatible() {
            errors.push(format!(
                "plugin api version {} exceeds runtime version {}",
                self.plugin_api_version, CURRENT_PLUGIN_API_VERSION
            ));
        }
        if self.capabilities.is_empty() && self.runtime_hooks.is_empty() {
            errors.push("plugin must declare at least one capability or runtime hook".to_owned());
        }

        errors
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

// ---------------------------------------------------------------------------
// Plugin runtime execution trait
// ---------------------------------------------------------------------------

/// Error returned by plugin runtime operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluginRuntimeError {
    /// Short machine-readable error code.
    pub code: String,
    /// Human-readable error message.
    pub message: String,
}

impl PluginRuntimeError {
    /// Creates a new plugin runtime error.
    #[must_use]
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for PluginRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

/// Result of executing a prepare or doctor check.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckExecutionResult {
    /// Stable check identifier that was executed.
    pub check_id: String,
    /// Whether the check passed.
    pub passed: bool,
    /// Human-readable findings from the check.
    pub findings: Vec<String>,
}

/// Result of bootstrapping an agent runtime.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentBootstrapResult {
    /// Stable agent identifier that was bootstrapped.
    pub agent_id: String,
    /// Whether bootstrap succeeded.
    pub ready: bool,
    /// Human-readable status message.
    pub message: String,
}

/// A resolved work item from a workflow plugin.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkItemResolution {
    /// Original identifier as provided by the user.
    pub raw_id: String,
    /// Plugin-assigned canonical identifier (e.g., `"5.3"` for BMAD).
    pub canonical_id: String,
    /// Human-readable title of the work item.
    pub title: String,
    /// File path to the work item source (relative to project root).
    pub source_path: Option<String>,
    /// Plugin-specific key-value metadata (opaque to core).
    /// Keys are not guaranteed to be unique; order is preserved as
    /// provided by the plugin.
    pub metadata: Vec<(String, String)>,
}

/// Summary of one available work item for listing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkItemSummary {
    /// Canonical work item identifier.
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Current status (e.g., `"todo"`, `"in_progress"`, `"done"`).
    pub status: String,
    /// Whether this item is ready to be worked on.
    ///
    /// Set by the workflow plugin — core uses this to pick the next
    /// item in the autonomous loop without knowing status semantics.
    pub actionable: bool,
}

/// Assembled prompt context for an agent session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromptContext {
    /// The primary prompt/instruction text.
    pub prompt_text: String,
    /// Additional context files to pass to the agent.
    pub context_files: Vec<ContextFile>,
    /// Work item identifier this context was built for.
    pub work_item_id: String,
    /// Tools discovered from enabled plugins via `required_tools()`.
    /// The agent plugin merges these with base tools and config extras.
    /// Populated by the core `run` command, not by individual plugins.
    pub discovered_tools: Vec<String>,
}

/// One context file included in a prompt assembly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextFile {
    /// Label for the context file (e.g., `"story"`, `"project-rules"`).
    pub label: String,
    /// Content of the file.
    pub content: String,
}

/// Result of launching an agent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentLaunchResult {
    /// Agent identifier that was launched.
    pub agent_id: String,
    /// Whether the agent completed successfully.
    pub success: bool,
    /// Exit code from the agent process, if applicable.
    pub exit_code: Option<i32>,
    /// Human-readable summary of what happened.
    pub message: String,
}

/// A spawned agent process for TUI integration.
///
/// The TUI render loop reads `stdout` for stream-json events and
/// uses `pid` for pause/resume signal delivery.
pub struct SpawnedAgent {
    /// OS process ID for signal delivery (`SIGSTOP`/`SIGCONT`).
    pub pid: u32,
    /// Agent stdout for reading stream-json events.
    /// Use `take_stdout()` to move it to the reader thread.
    pub stdout: Option<std::process::ChildStdout>,
    /// The child process handle (for `wait()`).
    pub child: std::process::Child,
    /// Path to temporary context file (cleaned up on drop).
    pub context_file: Option<std::path::PathBuf>,
}

impl SpawnedAgent {
    /// Takes the stdout handle (moves ownership to caller).
    /// Returns `None` if already taken.
    pub fn take_stdout(&mut self) -> Option<std::process::ChildStdout> {
        self.stdout.take()
    }
}

impl Drop for SpawnedAgent {
    fn drop(&mut self) {
        if let Some(ref path) = self.context_file {
            let _ = std::fs::remove_file(path);
        }
    }
}

/// Result of registering an MCP server.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpRegistrationResult {
    /// Stable server identifier that was registered.
    pub server_id: String,
    /// Whether the server is ready to accept connections.
    pub ready: bool,
    /// Human-readable status message.
    pub message: String,
}

/// Plugin runtime execution contract.
///
/// Implementing this trait enables a plugin to execute real operations
/// beyond static metadata. The runtime calls these methods when
/// `checks run`, `agents launch`, or `mcp launch` request actual
/// execution instead of just topology inspection.
///
/// Each method receives an ID that matches a descriptor declared by the
/// plugin. The runtime guarantees it only calls methods for IDs the
/// plugin itself registered.
///
/// # Error Isolation
///
/// Errors from plugin execution are captured as `PluginRuntimeError`
/// and never propagate as panics to the core runtime. This ensures
/// one broken plugin cannot crash the entire system.
pub trait PluginRuntime: Send + Sync {
    /// Returns the plugin identifier this runtime belongs to.
    fn plugin_id(&self) -> &str;

    /// Executes a prepare or doctor check.
    ///
    /// Called when `checks run <check-id>` targets a check owned by this
    /// plugin. The `project_root` is the directory where the project
    /// files should be validated.
    fn run_check(
        &self,
        check_id: &str,
        kind: PluginCheckKind,
        project_root: &Path,
    ) -> Result<CheckExecutionResult, PluginRuntimeError>;

    /// Bootstraps an agent runtime session.
    ///
    /// Called when `agents launch <agent-id>` targets an agent owned by
    /// this plugin. Returns whether the agent is ready to operate.
    fn bootstrap_agent(&self, agent_id: &str) -> Result<AgentBootstrapResult, PluginRuntimeError>;

    /// Registers an MCP server and validates it can start.
    ///
    /// Called when `mcp launch <server-id>` targets a server owned by
    /// this plugin. Returns whether the server is ready to accept
    /// connections.
    fn register_mcp_server(
        &self,
        server_id: &str,
    ) -> Result<McpRegistrationResult, PluginRuntimeError>;

    /// Resolves a work-item identifier into structured metadata.
    ///
    /// Called when `run <work-item-id>` needs the workflow plugin to
    /// parse and locate a work item. The format of `work_item_id` is
    /// plugin-specific (e.g., BMAD uses `"5.3"` dot notation for epic.story).
    fn resolve_work_item(
        &self,
        _work_item_id: &str,
        _project_root: &Path,
    ) -> Result<WorkItemResolution, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_a_workflow_plugin",
            format!("Plugin '{}' does not resolve work items", self.plugin_id()),
        ))
    }

    /// Lists available work items from the project tracker.
    ///
    /// Called when `run --list` asks the workflow plugin to enumerate
    /// actionable items from the configured tracker.
    fn list_work_items(
        &self,
        _project_root: &Path,
    ) -> Result<Vec<WorkItemSummary>, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_a_workflow_plugin",
            format!("Plugin '{}' does not list work items", self.plugin_id()),
        ))
    }

    /// Builds the prompt context for an agent session targeting one work item.
    ///
    /// Called after `resolve_work_item` succeeds. The workflow plugin
    /// assembles story text, acceptance criteria, project rules, and any
    /// other context the agent needs.
    fn build_prompt_context(
        &self,
        _resolution: &WorkItemResolution,
        _project_root: &Path,
    ) -> Result<PromptContext, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_a_workflow_plugin",
            format!(
                "Plugin '{}' does not build prompt context",
                self.plugin_id()
            ),
        ))
    }

    /// Launches an agent with the given prompt context (blocking).
    ///
    /// Called after prompt context is assembled. The agent plugin spawns
    /// the actual process and waits for completion. Used in `--no-tui` mode.
    fn launch_agent(
        &self,
        _agent_id: &str,
        _context: &PromptContext,
        _project_root: &Path,
    ) -> Result<AgentLaunchResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_agent_plugin",
            format!("Plugin '{}' does not launch agents", self.plugin_id()),
        ))
    }

    /// Spawns an agent process and returns it for TUI integration.
    ///
    /// Unlike `launch_agent()` which blocks until completion, this method
    /// returns the child process immediately. The caller (TUI render loop)
    /// reads stdout events and manages the process lifecycle.
    fn spawn_agent(
        &self,
        _agent_id: &str,
        _context: &PromptContext,
        _project_root: &Path,
    ) -> Result<SpawnedAgent, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_agent_plugin",
            format!("Plugin '{}' does not spawn agents", self.plugin_id()),
        ))
    }

    /// Returns tool names/patterns this plugin requires for agent sessions.
    ///
    /// The core collects required tools from ALL enabled plugins,
    /// deduplicates, and passes the merged list to the agent plugin.
    /// This enables auto-discovery: each plugin declares its needs
    /// (e.g., MCP tool patterns) without the user listing them manually.
    ///
    /// Default: no required tools (most plugins don't need agent tools).
    fn required_tools(&self) -> &[&str] {
        &[]
    }

    /// Returns prompt sections this plugin contributes to agent sessions.
    ///
    /// Called by the core `run` command after `build_prompt_context`.
    /// Each contribution is appended to the prompt text and added
    /// to the context files. This enables plugins like `official.findings`
    /// to inject content without coupling to the workflow plugin.
    ///
    /// Default: no contributions (most plugins don't contribute prompts).
    fn prompt_contributions(&self, _project_root: &Path) -> Vec<PromptContribution> {
        Vec::new()
    }

    /// Returns init-time contributions (questions, files, config sections)
    /// that this plugin wants to add to `ralph-engine init`.
    ///
    /// Plugins can contribute additional setup questions, config entries,
    /// or files during interactive project initialization. The default
    /// implementation returns no contributions.
    fn init_contributions(&self) -> Vec<InitContribution> {
        Vec::new()
    }

    /// Returns files this plugin requires in the project directory.
    ///
    /// Used by `doctor` and `checks` to validate project health.
    /// Core collects required files from ALL enabled plugins and checks
    /// their existence — plugins own what they need, core just validates.
    fn required_files(&self) -> &[&str] {
        &[]
    }

    /// Validates plugin-specific config from the project config YAML.
    ///
    /// Called during `doctor` and config loading. Plugins check that
    /// their owned config sections are well-formed. The raw config
    /// content is passed as a string — plugins parse their own sections.
    /// Core never interprets plugin-owned config keys.
    fn validate_config(&self, _config_content: &str) -> Vec<ConfigIssue> {
        Vec::new()
    }

    /// Migrates plugin-owned config when plugin version changes.
    ///
    /// Called during `ralph-engine doctor --fix` or explicit upgrade.
    /// Returns the updated config content, or None if no migration needed.
    /// Plugins own their sections — they read, transform, and return
    /// without touching other plugins' config.
    fn migrate_config(
        &self,
        _config_content: &str,
        _from_version: &str,
        _to_version: &str,
    ) -> Option<String> {
        None
    }

    /// Returns CLI subcommands this plugin contributes.
    ///
    /// Plugins can extend the CLI with custom subcommands discoverable
    /// via `ralph-engine --help`. Core routes to the plugin when the
    /// subcommand is invoked. Third-party plugins use this to add
    /// domain-specific commands without modifying core.
    fn cli_contributions(&self) -> Vec<CliContribution> {
        Vec::new()
    }

    /// Handles a CLI subcommand contributed by this plugin.
    ///
    /// Called when the user invokes a plugin-contributed command
    /// (e.g. `ralph-engine my-command`). The plugin receives the
    /// command name and arguments to process.
    ///
    /// # Errors
    ///
    /// Returns an error if the command fails.
    fn handle_cli_command(
        &self,
        _command: &str,
        _args: &[String],
    ) -> Result<String, PluginRuntimeError> {
        Err(PluginRuntimeError {
            code: "not_implemented".to_owned(),
            message: "CLI command handling not implemented".to_owned(),
        })
    }

    /// Pauses a spawned agent. Called by TUI when user presses pause.
    ///
    /// Default implementation sends SIGSTOP via `kill` command.
    /// Plugins can override for agent-specific pause behavior
    /// (e.g., API-based pause, session checkpoint).
    fn pause_agent(&self, pid: u32) -> Result<(), PluginRuntimeError> {
        std::process::Command::new("kill")
            .args(["-STOP", &pid.to_string()])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map_err(|e| PluginRuntimeError::new("pause_failed", format!("SIGSTOP failed: {e}")))?;
        Ok(())
    }

    /// Resumes a paused agent. Called by TUI when user presses resume.
    ///
    /// Default implementation sends SIGCONT via `kill` command.
    /// Plugins can override for agent-specific resume behavior.
    fn resume_agent(&self, pid: u32) -> Result<(), PluginRuntimeError> {
        std::process::Command::new("kill")
            .args(["-CONT", &pid.to_string()])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map_err(|e| {
                PluginRuntimeError::new("resume_failed", format!("SIGCONT failed: {e}"))
            })?;
        Ok(())
    }

    /// Injects user feedback into the agent context for the next run.
    ///
    /// Called when the user provides feedback during a pause. The default
    /// implementation writes feedback to a `.ralph-engine/.feedback.md` file.
    /// Plugins can override to use agent-specific injection mechanisms
    /// (e.g., Claude `--resume`, Codex stdin pipe, API-based context update).
    ///
    /// Returns the path to the feedback file (for context merging on re-spawn).
    fn inject_feedback(
        &self,
        feedback: &str,
        project_root: &Path,
    ) -> Result<std::path::PathBuf, PluginRuntimeError> {
        let feedback_dir = project_root.join(".ralph-engine");
        if !feedback_dir.exists() {
            std::fs::create_dir_all(&feedback_dir).map_err(|e| {
                PluginRuntimeError::new(
                    "feedback_dir_failed",
                    format!("Failed to create .ralph-engine/: {e}"),
                )
            })?;
        }
        let feedback_path = feedback_dir.join(".feedback.md");
        std::fs::write(&feedback_path, format!("## User Feedback\n\n{feedback}\n")).map_err(
            |e| {
                PluginRuntimeError::new(
                    "feedback_write_failed",
                    format!("Failed to write feedback: {e}"),
                )
            },
        )?;
        Ok(feedback_path)
    }

    /// Returns TUI panel contributions for the dashboard sidebar.
    ///
    /// Plugins with the `tui_widgets` capability declare panels that
    /// appear in the TUI sidebar. Core auto-discovers panels from all
    /// enabled plugins and renders them in the sidebar zone.
    ///
    /// Panel content is a static snapshot — plugins return current state
    /// as lines of text. Core calls this on each render frame.
    fn tui_contributions(&self) -> Vec<TuiPanel> {
        Vec::new()
    }

    /// Returns keybinding contributions for the TUI.
    ///
    /// Plugins declare keybindings that the TUI dispatches when the user
    /// presses the matching key. Core renders these in the help bar and
    /// routes key events to `handle_tui_key()`.
    fn tui_keybindings(&self) -> Vec<TuiKeybinding> {
        Vec::new()
    }

    /// Handles a TUI key event dispatched by the core.
    ///
    /// Called when the user presses a key that matches a keybinding
    /// declared by this plugin via `tui_keybindings()`. The `tui_state`
    /// string represents the current TUI state (e.g. `"Running"`, `"Paused"`).
    ///
    /// Returns a `TuiKeyResult` telling the core what to do next.
    fn handle_tui_key(&self, _key: &str, _tui_state: &str) -> TuiKeyResult {
        TuiKeyResult::NotHandled
    }

    /// Handles text input submitted from the TUI.
    ///
    /// Called when the user submits text (Enter) in the chat input bar.
    /// The plugin processes the text (e.g., saves feedback, sends to agent)
    /// and returns a result.
    fn handle_tui_text_input(&self, _text: &str, _project_root: &Path) -> TuiKeyResult {
        TuiKeyResult::NotHandled
    }

    /// Returns the placeholder text for the TUI chat input bar.
    ///
    /// If any enabled plugin returns `Some`, the TUI shows a persistent
    /// input bar at the bottom. The plugin owns the decision to show it
    /// and the placeholder text. Core just renders the widget.
    ///
    /// Default: `None` (no input bar — read-only dashboard).
    fn tui_input_placeholder(&self) -> Option<String> {
        None
    }

    /// Discovers slash commands available from this agent.
    ///
    /// Agent plugins scan their command/skill directories and return
    /// the list. Core auto-discovers from all enabled agent plugins
    /// and shows autocomplete in the TUI when the user types the
    /// command prefix (e.g. `/`).
    ///
    /// Default: empty (most plugins don't provide agent commands).
    fn discover_agent_commands(&self, _project_root: &Path) -> Vec<AgentCommand> {
        Vec::new()
    }

    /// Returns the command prefix that triggers autocomplete (e.g. `"/"`).
    ///
    /// Default: `"/"` (standard for Claude Code, Codex, Gemini CLI).
    fn tui_command_prefix(&self) -> &str {
        "/"
    }

    /// Exports the current session context for transfer to another agent.
    ///
    /// Agent plugins serialize their native session state (Claude JSONL,
    /// Codex rollout, Gemini checkpoint) into the portable format.
    /// Core calls this when the user switches agents mid-session.
    ///
    /// Default: not supported (returns error).
    fn export_session_context(
        &self,
        _project_root: &Path,
    ) -> Result<PortableContext, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "context_export_not_supported",
            format!(
                "Plugin '{}' does not support context export",
                self.plugin_id()
            ),
        ))
    }

    /// Imports a portable context from another agent.
    ///
    /// Agent plugins convert the portable format into their native
    /// representation (e.g., prepend as system prompt, write to session
    /// file, pass via CLI flags).
    ///
    /// Default: not supported (returns error).
    fn import_session_context(
        &self,
        _context: &PortableContext,
        _project_root: &Path,
    ) -> Result<(), PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "context_import_not_supported",
            format!(
                "Plugin '{}' does not support context import",
                self.plugin_id()
            ),
        ))
    }

    /// Returns the maximum context window size (in tokens) for this agent.
    ///
    /// Used by the context plugin to compact/summarize before transfer.
    /// Default: 0 (unknown — context plugin uses a safe default).
    fn context_window_size(&self) -> usize {
        0
    }

    /// Compacts a portable context to fit within a token budget.
    ///
    /// Context management plugins implement this to summarize
    /// conversation history before transfer to a smaller context window.
    /// The strategy (truncate, summarize, recent-only) is plugin-owned.
    ///
    /// Default: returns the context unchanged.
    fn compact_context(
        &self,
        context: &PortableContext,
        _target_tokens: usize,
    ) -> Result<PortableContext, PluginRuntimeError> {
        Ok(context.clone())
    }

    /// Saves a portable context to persistent storage.
    ///
    /// Context management plugins implement this to write session
    /// snapshots to disk (e.g., `.ralph-engine/sessions/`).
    ///
    /// Default: not supported.
    fn save_session(
        &self,
        _context: &PortableContext,
        _path: &Path,
    ) -> Result<(), PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "session_save_not_supported",
            format!(
                "Plugin '{}' does not support session persistence",
                self.plugin_id()
            ),
        ))
    }

    /// Loads a portable context from persistent storage.
    ///
    /// Default: not supported.
    fn load_session(&self, _path: &Path) -> Result<PortableContext, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "session_load_not_supported",
            format!(
                "Plugin '{}' does not support session persistence",
                self.plugin_id()
            ),
        ))
    }
}

/// A slash command discovered from an agent CLI.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentCommand {
    /// Command name without prefix (e.g. `"compact"`, `"skill"`).
    pub name: String,
    /// Short description shown in autocomplete.
    pub description: String,
    /// Plugin that owns this command.
    pub plugin_id: String,
}

// ---------------------------------------------------------------------------
// Portable context types for cross-agent session transfer
// ---------------------------------------------------------------------------

/// Role of a message in a conversation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MessageRole {
    /// User message.
    User,
    /// Assistant (agent) response.
    Assistant,
    /// System prompt or instruction.
    System,
    /// Tool invocation result.
    Tool,
}

impl MessageRole {
    /// Returns the stable string identifier.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Assistant => "assistant",
            Self::System => "system",
            Self::Tool => "tool",
        }
    }
}

/// One block of content within a message.
///
/// Based on the Anthropic content-block model (the most expressive
/// superset). Convertible to/from `OpenAI` and Gemini formats.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContentBlock {
    /// Plain text content.
    Text {
        /// The text.
        text: String,
    },
    /// Tool invocation by the assistant.
    ToolUse {
        /// Tool call identifier.
        id: String,
        /// Tool name.
        name: String,
        /// Tool input as JSON string.
        input: String,
    },
    /// Result of a tool invocation.
    ToolResult {
        /// Matching tool call identifier.
        tool_use_id: String,
        /// Result content.
        content: String,
        /// Whether the tool call failed.
        is_error: bool,
    },
}

/// One message in a portable conversation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PortableMessage {
    /// Message role.
    pub role: MessageRole,
    /// Content blocks (text, tool use, tool result).
    pub content: Vec<ContentBlock>,
    /// Unix timestamp (seconds) when the message was created.
    pub timestamp: Option<u64>,
}

/// Metadata about a portable context snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextMetadata {
    /// Agent plugin that produced this context.
    pub source_agent: String,
    /// Model used in the session (e.g. `"claude-opus-4-6"`).
    pub source_model: String,
    /// Session identifier from the agent CLI.
    pub session_id: Option<String>,
    /// Unix timestamp when the context was exported.
    pub created_at: u64,
}

/// A portable conversation context that can be transferred between agents.
///
/// This is the canonical format for cross-agent context sharing.
/// Agent plugins convert their native format (Claude JSONL, Codex rollout,
/// Gemini checkpoint) to/from this struct. Core never inspects the content —
/// it only passes the struct between plugins (Model B).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PortableContext {
    /// System prompt / instructions.
    pub system_prompt: Option<String>,
    /// Conversation messages.
    pub messages: Vec<PortableMessage>,
    /// Files currently in scope.
    pub active_files: Vec<String>,
    /// Summarized history (from compaction).
    pub summary: Option<String>,
    /// Estimated token count.
    pub token_count: usize,
    /// Maximum token budget for the target agent.
    pub max_tokens: usize,
    /// Metadata about the source session.
    pub metadata: ContextMetadata,
}

/// An issue found during plugin config validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigIssue {
    /// Severity: "error" (blocks), "warning" (informational).
    pub severity: String,
    /// Human-readable description of the issue.
    pub message: String,
}

/// A CLI subcommand contributed by a plugin.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CliContribution {
    /// Subcommand name (e.g. "bmad-status" becomes `ralph-engine bmad-status`).
    pub name: String,
    /// Short description for --help output.
    pub description: String,
    /// Handler receives args after the subcommand name.
    /// Returns output text or error message.
    pub handler_id: String,
}

/// A contribution to the `ralph-engine init` interactive flow.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitContribution {
    /// Short label for the init step.
    pub label: String,
    /// Description shown to the user during init.
    pub description: String,
    /// Config YAML snippet to append when this option is selected.
    pub config_snippet: Option<String>,
    /// Additional files to create (path → contents).
    pub files: Vec<(String, String)>,
}

/// A TUI panel contributed by a plugin for the dashboard sidebar.
///
/// Plugins declare panels via `tui_contributions()`. Core renders
/// these in the sidebar zone, auto-discovered from all enabled plugins
/// with the `tui_widgets` capability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TuiPanel {
    /// Unique panel identifier (e.g. `"findings"`, `"sprint-status"`).
    pub id: String,
    /// Localized panel title shown in the sidebar header.
    pub title: String,
    /// Panel content as lines of text. Updated on each call to
    /// `tui_contributions()`. Core renders these in the sidebar.
    pub lines: Vec<String>,
    /// Position hint: `"sidebar"` (default), `"bottom"`, or `"main"`.
    /// Core decides final placement based on layout tier.
    pub zone_hint: String,
}

/// A keybinding contributed by a plugin for the TUI.
///
/// Plugins declare keybindings via `tui_keybindings()`. Core renders
/// these in the help bar and dispatches matching key events to the
/// owning plugin's `handle_tui_key()`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TuiKeybinding {
    /// Key identifier (e.g. `"p"`, `"f"`). Must be a single character.
    pub key: String,
    /// Short description shown in the help bar (e.g. `"Pause/Resume"`).
    pub description: String,
    /// Plugin that owns this keybinding.
    pub plugin_id: String,
    /// TUI states where this keybinding is active.
    /// Empty means active in all states.
    pub active_states: Vec<String>,
}

/// Result of a plugin handling a TUI key event or text input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TuiKeyResult {
    /// The key/text was handled. No further action needed.
    Handled,
    /// The key/text was not handled. Core may try other plugins.
    NotHandled,
    /// Enter text input mode with the given prompt.
    EnterTextInput {
        /// Prompt text shown to the user (e.g. "Type feedback:").
        prompt: String,
    },
    /// Request the core to change the TUI state.
    SetState(String),
    /// Request the core to show a message in the activity stream.
    ShowMessage(String),
}

/// A prompt section contributed by a plugin at runtime.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromptContribution {
    /// Short label for logging and context file tracking.
    pub label: String,
    /// The prompt text to inject (with XML tags if desired).
    pub content: String,
}

// ---------------------------------------------------------------------------
// Shared utilities for plugin runtime implementations
// ---------------------------------------------------------------------------

/// Probes whether a binary is available on the system PATH.
///
/// Returns the resolved absolute path if found, or `None` if the binary
/// is not installed. Works cross-platform (uses `which` on Unix,
/// `where` on Windows).
#[must_use]
pub fn probe_binary_on_path(program: &str) -> Option<String> {
    let which_cmd = if cfg!(windows) { "where" } else { "which" };
    match std::process::Command::new(which_cmd).arg(program).output() {
        Ok(output) if output.status.success() => String::from_utf8_lossy(&output.stdout)
            .trim()
            .lines()
            .next()
            .filter(|s| !s.is_empty())
            .map(std::borrow::ToOwned::to_owned),
        _ => None,
    }
}
