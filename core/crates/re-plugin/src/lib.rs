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

    /// Launches an agent with the given prompt context.
    ///
    /// Called after prompt context is assembled. The agent plugin spawns
    /// the actual process (e.g., `claude` CLI) with the prompt.
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
