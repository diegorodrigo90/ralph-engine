//! Integration tests for the shared Ralph Engine plugin contract.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use re_plugin::{
    AGENT_RUNTIME, ALL_PLUGIN_CAPABILITIES, ALL_PLUGIN_KINDS, ALL_PLUGIN_RUNTIME_HOOKS,
    ALL_PLUGIN_RUNTIME_SURFACES, ALL_PLUGIN_TRUST_LEVELS, CONTEXT_PROVIDER, DATA_SOURCE,
    DOCTOR_CHECKS, FORGE_PROVIDER, MCP_CONTRIBUTION, POLICY, PREPARE_CHECKS, PROMPT_FRAGMENTS,
    PluginAgentDescriptor, PluginCapability, PluginCheckAsset, PluginCheckDescriptor,
    PluginCheckKind, PluginDescriptor, PluginKind, PluginLifecycleStage, PluginLoadBoundary,
    PluginLocalizedText, PluginPolicyAsset, PluginPolicyDescriptor, PluginPromptAsset,
    PluginPromptDescriptor, PluginProviderDescriptor, PluginProviderKind, PluginRuntimeHook,
    PluginRuntimeSurface, PluginTemplateAsset, PluginTemplateDescriptor, PluginTrustLevel,
    REMOTE_CONTROL, TEMPLATE, WORKFLOW, parse_plugin_runtime_hook,
    parse_reviewed_plugin_capability, render_plugin_detail, render_plugin_detail_for_locale,
    render_plugin_listing, render_plugin_listing_for_locale, runtime_surface_for_capability,
};

const BASIC_CAPABILITIES: &[PluginCapability] = &[PluginCapability::new("template")];
const GITHUB_CAPABILITIES: &[PluginCapability] = &[
    PluginCapability::new("data_source"),
    PluginCapability::new("forge_provider"),
];
const BASIC_LIFECYCLE: &[PluginLifecycleStage] = &[
    PluginLifecycleStage::Discover,
    PluginLifecycleStage::Configure,
    PluginLifecycleStage::Load,
];
const GITHUB_LIFECYCLE: &[PluginLifecycleStage] = &[
    PluginLifecycleStage::Discover,
    PluginLifecycleStage::Configure,
    PluginLifecycleStage::Load,
];
const BASIC_RUNTIME_HOOKS: &[PluginRuntimeHook] = &[PluginRuntimeHook::Scaffold];
const BASIC_LOCALIZED_NAMES: &[PluginLocalizedText] =
    &[PluginLocalizedText::new("pt-br", "Base de teste")];
const BASIC_SUMMARY: &str = "Shared fixture plugin for starter templates.";
const BASIC_LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    "Plugin de fixture para templates iniciais.",
)];
const BASIC_TEMPLATE_LOCALIZED_NAMES: &[PluginLocalizedText] =
    &[PluginLocalizedText::new("pt-br", "Starter de teste")];
const BASIC_TEMPLATE_LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    "Template inicial de fixture para novos projetos Ralph Engine.",
)];
const BMAD_PROMPT_LOCALIZED_NAMES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    "Prompt de workflow de teste",
)];
const BMAD_PROMPT_LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    "Pacote de prompts para montar workflows de teste.",
)];
const CODEX_AGENT_LOCALIZED_NAMES: &[PluginLocalizedText] =
    &[PluginLocalizedText::new("pt-br", "Sessão de teste")];
const CODEX_AGENT_LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    "Sessão de runtime de teste para o Ralph Engine.",
)];
const TDD_POLICY_LOCALIZED_NAMES: &[PluginLocalizedText] =
    &[PluginLocalizedText::new("pt-br", "Guardrails de teste")];
const TDD_POLICY_LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    "Política de fixture com guardrails estritos de teste.",
)];
const BMAD_CHECK_LOCALIZED_NAMES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    "Verificação de preparo de teste",
)];
const BMAD_CHECK_LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    "Executa validação tipada de preparo para o workflow de teste.",
)];
const GITHUB_PROVIDER_LOCALIZED_NAMES: &[PluginLocalizedText] =
    &[PluginLocalizedText::new("pt-br", "Fonte de dados de teste")];
const GITHUB_PROVIDER_LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    "Expõe contexto e dados tipados para o workflow de teste.",
)];
const GITHUB_RUNTIME_HOOKS: &[PluginRuntimeHook] = &[
    PluginRuntimeHook::McpRegistration,
    PluginRuntimeHook::DataSourceRegistration,
    PluginRuntimeHook::ForgeProviderRegistration,
];
const TEMPLATE_ASSETS: &[PluginTemplateAsset] = &[PluginTemplateAsset::new(
    ".ralph-engine/config.yaml",
    "schema_version: 1\n",
)];
const PROMPT_ASSETS: &[PluginPromptAsset] = &[PluginPromptAsset::new(
    "prompts/workflow.md",
    "# workflow\n",
)];
const POLICY_ASSETS: &[PluginPolicyAsset] = &[PluginPolicyAsset::new(
    "policies/guardrails.md",
    "# guardrails\n",
)];
const CHECK_ASSETS: &[PluginCheckAsset] =
    &[PluginCheckAsset::new("checks/prepare.md", "# prepare\n")];

fn basic_plugin() -> PluginDescriptor {
    PluginDescriptor::new(
        "test.basic",
        PluginKind::Template,
        PluginTrustLevel::Community,
        "Test Basic",
        BASIC_LOCALIZED_NAMES,
        BASIC_SUMMARY,
        BASIC_LOCALIZED_SUMMARIES,
        "0.2.0-alpha.1",
        1,
        BASIC_CAPABILITIES,
        BASIC_LIFECYCLE,
        PluginLoadBoundary::InProcess,
        BASIC_RUNTIME_HOOKS,
    )
}

fn github_plugin() -> PluginDescriptor {
    PluginDescriptor::new(
        "test.github",
        PluginKind::DataSource,
        PluginTrustLevel::Community,
        "Test GitHub",
        &[],
        "Fixture integration for data and forge workflows.",
        &[],
        "0.2.0-alpha.1",
        1,
        GITHUB_CAPABILITIES,
        GITHUB_LIFECYCLE,
        PluginLoadBoundary::InProcess,
        GITHUB_RUNTIME_HOOKS,
    )
}

fn invalid_plugin() -> PluginDescriptor {
    PluginDescriptor::new(
        "basic",
        PluginKind::Template,
        PluginTrustLevel::Community,
        "Broken",
        &[],
        "Broken plugin fixture.",
        &[],
        "0.2.0-alpha.1",
        1,
        &[],
        &[],
        PluginLoadBoundary::Remote,
        &[],
    )
}

#[test]
fn capability_display_is_stable() {
    // Arrange
    let capability = PluginCapability::new("template");

    // Act
    let rendered = capability.to_string();

    // Assert
    assert_eq!(rendered, "template");
}

#[test]
fn localized_text_constructor_is_stable() {
    let entry = PluginLocalizedText::new("pt-br", "Base de teste");

    assert_eq!(entry.locale, "pt-br");
    assert_eq!(entry.value, "Base de teste");
}

#[test]
fn template_descriptor_resolves_locales_with_english_fallback() {
    let descriptor = PluginTemplateDescriptor::new(
        "test.basic.starter",
        "test.basic",
        "Test starter",
        BASIC_TEMPLATE_LOCALIZED_NAMES,
        "Starter fixture template for new Ralph Engine projects.",
        BASIC_TEMPLATE_LOCALIZED_SUMMARIES,
        TEMPLATE_ASSETS,
    );

    assert_eq!(
        descriptor.display_name_for_locale("pt-br"),
        "Starter de teste"
    );
    assert_eq!(
        descriptor.summary_for_locale("pt-br"),
        "Template inicial de fixture para novos projetos Ralph Engine."
    );
    assert_eq!(descriptor.display_name_for_locale("es"), "Test starter");
    assert_eq!(
        descriptor.summary_for_locale("es"),
        "Starter fixture template for new Ralph Engine projects."
    );
    assert!(descriptor.has_assets());
    assert_eq!(descriptor.assets[0].path, ".ralph-engine/config.yaml");
}

#[test]
fn prompt_descriptor_resolves_locales_with_english_fallback() {
    let descriptor = PluginPromptDescriptor::new(
        "test.prompts.workflow",
        "test.prompts",
        "Fixture workflow prompt",
        BMAD_PROMPT_LOCALIZED_NAMES,
        "Prompt bundle for fixture workflow assembly.",
        BMAD_PROMPT_LOCALIZED_SUMMARIES,
        PROMPT_ASSETS,
    );

    assert_eq!(
        descriptor.display_name_for_locale("pt-br"),
        "Prompt de workflow de teste"
    );
    assert_eq!(
        descriptor.summary_for_locale("pt-br"),
        "Pacote de prompts para montar workflows de teste."
    );
    assert_eq!(
        descriptor.display_name_for_locale("fr"),
        "Fixture workflow prompt"
    );
    assert_eq!(
        descriptor.summary_for_locale("fr"),
        "Prompt bundle for fixture workflow assembly."
    );
    assert!(descriptor.has_assets());
    assert_eq!(descriptor.assets[0].path, "prompts/workflow.md");
}

#[test]
fn agent_descriptor_resolves_locales_with_english_fallback() {
    let descriptor = PluginAgentDescriptor::new(
        "test.agents.session",
        "test.agents",
        "Fixture session",
        CODEX_AGENT_LOCALIZED_NAMES,
        "Fixture runtime session for Ralph Engine.",
        CODEX_AGENT_LOCALIZED_SUMMARIES,
    );

    assert_eq!(
        descriptor.display_name_for_locale("pt-br"),
        "Sessão de teste"
    );
    assert_eq!(
        descriptor.summary_for_locale("pt-br"),
        "Sessão de runtime de teste para o Ralph Engine."
    );
    assert_eq!(descriptor.display_name_for_locale("fr"), "Fixture session");
}

#[test]
fn policy_descriptor_resolves_locales_with_english_fallback() {
    let descriptor = PluginPolicyDescriptor::new(
        "test.policies.guardrails",
        "test.policies",
        "Fixture guardrails",
        TDD_POLICY_LOCALIZED_NAMES,
        "Fixture policy with strict testing guardrails.",
        TDD_POLICY_LOCALIZED_SUMMARIES,
        POLICY_ASSETS,
    );

    assert_eq!(
        descriptor.display_name_for_locale("pt-br"),
        "Guardrails de teste"
    );
    assert_eq!(
        descriptor.summary_for_locale("pt-br"),
        "Política de fixture com guardrails estritos de teste."
    );
    assert_eq!(
        descriptor.summary_for_locale("fr"),
        "Fixture policy with strict testing guardrails."
    );
}

#[test]
fn check_descriptor_resolves_locales_with_english_fallback() {
    let descriptor = PluginCheckDescriptor::new(
        "test.bmad.prepare",
        "test.bmad",
        PluginCheckKind::Prepare,
        "Fixture prepare check",
        BMAD_CHECK_LOCALIZED_NAMES,
        "Runs typed prepare-time validation for the fixture workflow.",
        BMAD_CHECK_LOCALIZED_SUMMARIES,
        CHECK_ASSETS,
    );

    assert_eq!(
        descriptor.display_name_for_locale("pt-br"),
        "Verificação de preparo de teste"
    );
    assert_eq!(
        descriptor.summary_for_locale("pt-br"),
        "Executa validação tipada de preparo para o workflow de teste."
    );
    assert_eq!(
        descriptor.summary_for_locale("fr"),
        "Runs typed prepare-time validation for the fixture workflow."
    );
    assert_eq!(descriptor.kind.as_str(), "prepare");
}

#[test]
fn provider_descriptor_resolves_locales_with_english_fallback() {
    let descriptor = PluginProviderDescriptor::new(
        "test.github.data",
        "test.github",
        PluginProviderKind::DataSource,
        "Fixture data source",
        GITHUB_PROVIDER_LOCALIZED_NAMES,
        "Exposes typed data and context for the fixture workflow.",
        GITHUB_PROVIDER_LOCALIZED_SUMMARIES,
    );

    assert_eq!(
        descriptor.display_name_for_locale("pt-br"),
        "Fonte de dados de teste"
    );
    assert_eq!(
        descriptor.summary_for_locale("pt-br"),
        "Expõe contexto e dados tipados para o workflow de teste."
    );
    assert_eq!(
        descriptor.summary_for_locale("fr"),
        "Exposes typed data and context for the fixture workflow."
    );
    assert_eq!(descriptor.kind.as_str(), "data_source");
}

#[test]
fn exported_capability_constants_are_stable() {
    // Arrange
    let capabilities = [
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

    // Act
    let rendered = capabilities
        .into_iter()
        .map(PluginCapability::as_str)
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(
        rendered,
        vec![
            "template",
            "prompt_fragments",
            "prepare_checks",
            "doctor_checks",
            "agent_runtime",
            "mcp_contribution",
            "data_source",
            "context_provider",
            "forge_provider",
            "remote_control",
            "policy",
            "workflow",
        ]
    );
}

#[test]
fn parse_reviewed_plugin_capability_supports_stable_identifiers() {
    let parsed = ALL_PLUGIN_CAPABILITIES
        .iter()
        .copied()
        .map(|capability| parse_reviewed_plugin_capability(capability.as_str()))
        .collect::<Vec<_>>();

    assert_eq!(
        parsed,
        ALL_PLUGIN_CAPABILITIES
            .iter()
            .copied()
            .map(Some)
            .collect::<Vec<_>>()
    );
    assert_eq!(parse_reviewed_plugin_capability("unknown"), None);
}

/// Capabilities that operate via hooks rather than dedicated surfaces.
const HOOK_ONLY_CAPABILITIES: &[&str] = &[
    "workflow",
    "tui_widgets",
    "context_management",
    "session_persistence",
    "agent_routing",
    "preset",
];

#[test]
fn runtime_surface_mapping_covers_all_reviewed_capabilities() {
    let uncovered = ALL_PLUGIN_CAPABILITIES
        .iter()
        .copied()
        .filter(|capability| {
            !HOOK_ONLY_CAPABILITIES.contains(&capability.as_str())
                && runtime_surface_for_capability(*capability).is_none()
        })
        .map(PluginCapability::as_str)
        .collect::<Vec<_>>();

    assert_eq!(uncovered, Vec::<&'static str>::new());
}

#[test]
fn exported_runtime_surfaces_are_stable() {
    let rendered = ALL_PLUGIN_RUNTIME_SURFACES
        .iter()
        .copied()
        .map(PluginRuntimeSurface::as_str)
        .collect::<Vec<_>>();

    assert_eq!(
        rendered,
        vec![
            "templates",
            "prompts",
            "checks",
            "agents",
            "mcp",
            "providers",
            "policies",
        ]
    );
}

#[test]
fn runtime_surface_mapping_groups_reviewed_capabilities() {
    assert_eq!(
        runtime_surface_for_capability(TEMPLATE),
        Some(PluginRuntimeSurface::Templates)
    );
    assert_eq!(
        runtime_surface_for_capability(PROMPT_FRAGMENTS),
        Some(PluginRuntimeSurface::Prompts)
    );
    assert_eq!(
        runtime_surface_for_capability(PREPARE_CHECKS),
        Some(PluginRuntimeSurface::Checks)
    );
    assert_eq!(
        runtime_surface_for_capability(DOCTOR_CHECKS),
        Some(PluginRuntimeSurface::Checks)
    );
    assert_eq!(
        runtime_surface_for_capability(AGENT_RUNTIME),
        Some(PluginRuntimeSurface::Agents)
    );
    assert_eq!(
        runtime_surface_for_capability(MCP_CONTRIBUTION),
        Some(PluginRuntimeSurface::Mcp)
    );
    assert_eq!(
        runtime_surface_for_capability(DATA_SOURCE),
        Some(PluginRuntimeSurface::Providers)
    );
    assert_eq!(
        runtime_surface_for_capability(CONTEXT_PROVIDER),
        Some(PluginRuntimeSurface::Providers)
    );
    assert_eq!(
        runtime_surface_for_capability(FORGE_PROVIDER),
        Some(PluginRuntimeSurface::Providers)
    );
    assert_eq!(
        runtime_surface_for_capability(REMOTE_CONTROL),
        Some(PluginRuntimeSurface::Providers)
    );
    assert_eq!(
        runtime_surface_for_capability(POLICY),
        Some(PluginRuntimeSurface::Policies)
    );
    assert_eq!(
        runtime_surface_for_capability(PluginCapability::new("unknown_surface")),
        None
    );
}

#[test]
fn lifecycle_display_is_stable() {
    // Arrange
    let stage = PluginLifecycleStage::Validate;

    // Act
    let rendered = stage.to_string();

    // Assert
    assert_eq!(rendered, "validate");
}

#[test]
fn lifecycle_as_str_is_stable() {
    // Arrange
    let stages = [
        PluginLifecycleStage::Discover,
        PluginLifecycleStage::Configure,
        PluginLifecycleStage::Validate,
        PluginLifecycleStage::Load,
    ];

    // Act
    let rendered = stages
        .into_iter()
        .map(PluginLifecycleStage::as_str)
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(rendered, vec!["discover", "configure", "validate", "load"]);
}

#[test]
fn kind_as_str_is_stable() {
    // Arrange
    let rendered = ALL_PLUGIN_KINDS
        .iter()
        .copied()
        .map(PluginKind::as_str)
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(
        rendered,
        vec![
            "template",
            "agent_runtime",
            "forge_provider",
            "context_provider",
            "data_source",
            "remote_control",
            "mcp_contribution",
            "policy",
            "workflow",
            "tui_extension",
            "context_manager",
            "agent_router",
            "preset",
        ]
    );
}

#[test]
fn trust_level_as_str_is_stable() {
    // Arrange
    let rendered = ALL_PLUGIN_TRUST_LEVELS
        .iter()
        .copied()
        .map(PluginTrustLevel::as_str)
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(rendered, vec!["official", "community"]);
}

#[test]
fn load_boundary_display_is_stable() {
    // Arrange
    let boundaries = [
        PluginLoadBoundary::InProcess,
        PluginLoadBoundary::Subprocess,
        PluginLoadBoundary::Remote,
    ];

    // Act
    let rendered = boundaries
        .into_iter()
        .map(|boundary| boundary.to_string())
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(rendered, vec!["in_process", "subprocess", "remote"]);
}

#[test]
fn runtime_hook_display_is_stable() {
    // Arrange
    let hooks = ALL_PLUGIN_RUNTIME_HOOKS;

    // Act
    let rendered = hooks.iter().map(|hook| hook.as_str()).collect::<Vec<_>>();

    // Assert
    assert_eq!(
        rendered,
        vec![
            "scaffold",
            "prepare",
            "doctor",
            "prompt_assembly",
            "agent_bootstrap",
            "mcp_registration",
            "data_source_registration",
            "context_provider_registration",
            "forge_provider_registration",
            "remote_control_bootstrap",
            "policy_enforcement",
            "work_item_resolution",
            "agent_launch",
            "tui_contribution",
            "context_management",
            "session_persistence",
            "agent_routing",
            "preset_application",
        ]
    );
}

#[test]
fn parse_runtime_hook_supports_stable_identifiers() {
    let parsed = ALL_PLUGIN_RUNTIME_HOOKS
        .iter()
        .copied()
        .map(|hook| parse_plugin_runtime_hook(hook.as_str()))
        .collect::<Vec<_>>();

    assert_eq!(
        parsed,
        ALL_PLUGIN_RUNTIME_HOOKS
            .iter()
            .copied()
            .map(Some)
            .collect::<Vec<_>>()
    );
    assert_eq!(parse_plugin_runtime_hook("unknown"), None);
}

#[test]
fn descriptor_requires_namespaced_identifier() {
    // Arrange
    let descriptor = basic_plugin();

    // Act
    let namespaced = descriptor.is_namespaced();

    // Assert
    assert!(namespaced);
    assert_eq!(descriptor.kind, PluginKind::Template);
    assert_eq!(descriptor.trust_level, PluginTrustLevel::Community);
}

#[test]
fn descriptor_rejects_non_namespaced_identifier() {
    // Arrange
    let descriptor = invalid_plugin();

    // Act
    let namespaced = descriptor.is_namespaced();

    // Assert
    assert!(!namespaced);
}

#[test]
fn descriptor_requires_capabilities() {
    // Arrange
    let descriptor = github_plugin();

    // Act
    let has_capabilities = descriptor.has_capabilities();

    // Assert
    assert!(has_capabilities);
}

#[test]
fn descriptor_rejects_missing_capabilities() {
    // Arrange
    let descriptor = invalid_plugin();

    // Act
    let has_capabilities = descriptor.has_capabilities();

    // Assert
    assert!(!has_capabilities);
}

#[test]
fn descriptor_requires_lifecycle_stages() {
    // Arrange
    let descriptor = github_plugin();

    // Act
    let has_lifecycle = descriptor.has_lifecycle();

    // Assert
    assert!(has_lifecycle);
}

#[test]
fn descriptor_rejects_missing_lifecycle_stages() {
    // Arrange
    let descriptor = invalid_plugin();

    // Act
    let has_lifecycle = descriptor.has_lifecycle();

    // Assert
    assert!(!has_lifecycle);
}

#[test]
fn descriptor_requires_runtime_hooks() {
    // Arrange
    let descriptor = github_plugin();

    // Act
    let has_runtime_hooks = descriptor.has_runtime_hooks();

    // Assert
    assert!(has_runtime_hooks);
}

#[test]
fn descriptor_rejects_missing_runtime_hooks() {
    // Arrange
    let descriptor = invalid_plugin();

    // Act
    let has_runtime_hooks = descriptor.has_runtime_hooks();

    // Assert
    assert!(!has_runtime_hooks);
}

#[test]
fn render_plugin_listing_includes_human_readable_lines() {
    // Arrange
    let plugins = [basic_plugin(), github_plugin()];

    // Act
    let listing = render_plugin_listing(&plugins);

    // Assert
    assert!(listing.contains("Official plugins (2)"));
    assert!(
        listing.contains(
            "- test.basic | template | community | Test Basic | v0.2.0-alpha.1 | template"
        )
    );
    assert!(listing.contains(
        "- test.github | data_source | community | Test GitHub | v0.2.0-alpha.1 | data_source, forge_provider"
    ));
}

#[test]
fn render_plugin_listing_handles_empty_sets() {
    // Arrange
    let plugins = [];

    // Act
    let listing = render_plugin_listing(&plugins);

    // Assert
    assert_eq!(listing, "Official plugins (0)");
}

#[test]
fn render_plugin_detail_includes_runtime_hooks() {
    // Arrange
    let plugin = github_plugin();

    // Act
    let detail = render_plugin_detail(&plugin);

    // Assert
    assert!(detail.contains(
        "Runtime hooks: mcp_registration, data_source_registration, forge_provider_registration"
    ));
}

#[test]
fn render_plugin_listing_supports_pt_br_and_falls_back_to_english() {
    // Arrange
    let plugins = [basic_plugin(), github_plugin()];

    // Act
    let rendered = render_plugin_listing_for_locale(&plugins, "pt-br");

    // Assert
    assert!(rendered.contains("Plugins oficiais (2)"));
    assert!(
        rendered.contains("test.basic | template | community | Base de teste | v0.2.0-alpha.1")
    );
    assert!(rendered.contains("Plugin de fixture para templates iniciais."));
    assert!(
        rendered.contains("test.github | data_source | community | Test GitHub | v0.2.0-alpha.1")
    );
    assert!(rendered.contains("Fixture integration for data and forge workflows."));
}

#[test]
fn render_plugin_detail_supports_pt_br_and_falls_back_to_english() {
    // Arrange
    let plugin = basic_plugin();

    // Act
    let rendered = render_plugin_detail_for_locale(&plugin, "pt-br");

    // Assert
    assert!(rendered.contains("Plugin: test.basic"));
    assert!(rendered.contains("Tipo: template"));
    assert!(rendered.contains("Confiança: community"));
    assert!(rendered.contains("Nome: Base de teste"));
    assert!(rendered.contains("Versão: v0.2.0-alpha.1"));
    assert!(rendered.contains("Resumo: Plugin de fixture para templates iniciais."));
    assert!(rendered.contains("Fronteira de carregamento: in_process"));
    assert!(rendered.contains("Hooks de runtime: scaffold"));
}

#[test]
fn plugin_display_name_falls_back_to_english_for_unknown_locale() {
    let plugin = basic_plugin();

    assert_eq!(plugin.display_name_for_locale("es"), "Test Basic");
}

#[test]
fn plugin_display_name_supports_canonical_locale_resolution() {
    let plugin = basic_plugin();

    assert_eq!(plugin.display_name_for_locale("pt-BR"), "Base de teste");
}

#[test]
fn plugin_summary_falls_back_to_english_for_unknown_locale() {
    let plugin = basic_plugin();

    assert_eq!(
        plugin.summary_for_locale("es"),
        "Shared fixture plugin for starter templates."
    );
}

#[test]
fn plugin_summary_supports_canonical_locale_resolution() {
    let plugin = basic_plugin();

    assert_eq!(
        plugin.summary_for_locale(" pt-BR "),
        "Plugin de fixture para templates iniciais."
    );
}

#[test]
fn render_plugin_detail_includes_capabilities_and_lifecycle() {
    // Arrange
    let plugin = github_plugin();

    // Act
    let detail = render_plugin_detail(&plugin);

    // Assert
    assert!(detail.contains("Plugin: test.github"));
    assert!(detail.contains("Kind: data_source"));
    assert!(detail.contains("Trust: community"));
    assert!(detail.contains("Capabilities: data_source, forge_provider"));
    assert!(detail.contains("Lifecycle: discover -> configure -> load"));
    assert!(detail.contains("Load boundary: in_process"));
}

#[test]
fn plugin_api_version_is_compatible_with_current_runtime() {
    let plugin = basic_plugin();
    assert!(plugin.is_api_compatible());
    assert_eq!(plugin.plugin_api_version, 1);
}

#[test]
fn plugin_api_version_rejects_future_versions() {
    let future_plugin = PluginDescriptor::new(
        "test.future",
        PluginKind::Template,
        PluginTrustLevel::Community,
        "Future Plugin",
        &[],
        "A plugin from the future.",
        &[],
        "9.0.0",
        99,
        &[],
        &[],
        PluginLoadBoundary::InProcess,
        &[],
    );
    assert!(!future_plugin.is_api_compatible());
}

#[test]
fn validate_reports_no_errors_for_well_formed_plugin() {
    let plugin = basic_plugin();
    assert!(plugin.validate().is_empty());
}

#[test]
fn validate_catches_empty_id() {
    let plugin = PluginDescriptor::new(
        "",
        PluginKind::Template,
        PluginTrustLevel::Community,
        "Test",
        &[],
        "Summary.",
        &[],
        "1.0.0",
        1,
        BASIC_CAPABILITIES,
        BASIC_LIFECYCLE,
        PluginLoadBoundary::InProcess,
        BASIC_RUNTIME_HOOKS,
    );
    let errors = plugin.validate();
    assert!(errors.iter().any(|e| e.contains("must not be empty")));
    assert!(errors.iter().any(|e| e.contains("namespace")));
}

#[test]
fn validate_catches_incompatible_api_version() {
    let plugin = PluginDescriptor::new(
        "test.future",
        PluginKind::Template,
        PluginTrustLevel::Community,
        "Future",
        &[],
        "Summary.",
        &[],
        "1.0.0",
        99,
        BASIC_CAPABILITIES,
        BASIC_LIFECYCLE,
        PluginLoadBoundary::InProcess,
        BASIC_RUNTIME_HOOKS,
    );
    let errors = plugin.validate();
    assert!(errors.iter().any(|e| e.contains("api version")));
}

#[test]
fn validate_catches_no_capabilities_or_hooks() {
    let plugin = PluginDescriptor::new(
        "test.empty",
        PluginKind::Template,
        PluginTrustLevel::Community,
        "Empty",
        &[],
        "Summary.",
        &[],
        "1.0.0",
        1,
        &[],
        &[],
        PluginLoadBoundary::InProcess,
        &[],
    );
    let errors = plugin.validate();
    assert!(
        errors
            .iter()
            .any(|e| e.contains("capability or runtime hook"))
    );
}

// ---------------------------------------------------------------------------
// Community plugin end-to-end test
// ---------------------------------------------------------------------------

/// Simulates the full lifecycle of a community plugin:
/// descriptor creation → validation → runtime execution.
mod community_plugin_e2e {
    use std::path::Path;

    use re_plugin::{
        AgentBootstrapResult, CURRENT_PLUGIN_API_VERSION, CheckExecutionResult,
        McpRegistrationResult, PREPARE_CHECKS, PluginCheckKind, PluginDescriptor, PluginKind,
        PluginLifecycleStage, PluginLoadBoundary, PluginRuntime, PluginRuntimeError,
        PluginRuntimeHook, PluginTrustLevel,
    };

    const COMMUNITY_PLUGIN_ID: &str = "community.acme.linter";

    fn community_descriptor() -> PluginDescriptor {
        PluginDescriptor::new(
            COMMUNITY_PLUGIN_ID,
            PluginKind::Template,
            PluginTrustLevel::Community,
            "ACME Linter",
            &[],
            "Community linter plugin for ACME projects.",
            &[],
            "1.0.0",
            CURRENT_PLUGIN_API_VERSION,
            &[PREPARE_CHECKS],
            &[PluginLifecycleStage::Discover, PluginLifecycleStage::Load],
            PluginLoadBoundary::InProcess,
            &[PluginRuntimeHook::Prepare],
        )
    }

    struct AcmeLinterRuntime;

    impl PluginRuntime for AcmeLinterRuntime {
        fn plugin_id(&self) -> &str {
            COMMUNITY_PLUGIN_ID
        }

        fn run_check(
            &self,
            check_id: &str,
            _kind: PluginCheckKind,
            project_root: &Path,
        ) -> Result<CheckExecutionResult, PluginRuntimeError> {
            let config = project_root.join(".acme-linter.toml");
            let mut findings = Vec::new();
            if !config.exists() {
                findings.push("missing: .acme-linter.toml".to_owned());
            }
            Ok(CheckExecutionResult {
                check_id: check_id.to_owned(),
                passed: findings.is_empty(),
                findings,
            })
        }

        fn bootstrap_agent(
            &self,
            agent_id: &str,
        ) -> Result<AgentBootstrapResult, PluginRuntimeError> {
            Err(PluginRuntimeError::new(
                "unsupported",
                format!("no agent: {agent_id}"),
            ))
        }

        fn register_mcp_server(
            &self,
            server_id: &str,
        ) -> Result<McpRegistrationResult, PluginRuntimeError> {
            Err(PluginRuntimeError::new(
                "unsupported",
                format!("no mcp: {server_id}"),
            ))
        }
    }

    #[test]
    fn community_plugin_descriptor_passes_validation() {
        let descriptor = community_descriptor();
        let errors = descriptor.validate();
        assert!(errors.is_empty(), "validation errors: {errors:?}");
        assert!(descriptor.is_api_compatible());
        assert!(descriptor.is_namespaced());
        assert!(descriptor.has_capabilities());
        assert!(descriptor.has_runtime_hooks());
    }

    #[test]
    fn community_plugin_runtime_executes_check() {
        let tmp = std::env::temp_dir().join("re-community-e2e");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).ok();

        let rt = AcmeLinterRuntime;
        assert_eq!(rt.plugin_id(), COMMUNITY_PLUGIN_ID);

        // Check fails when config missing
        let result = rt.run_check("acme.lint", PluginCheckKind::Prepare, &tmp);
        assert!(result.is_ok());
        let Ok(output) = result else { return };
        assert!(!output.passed);
        assert!(output.findings[0].contains(".acme-linter.toml"));

        // Check passes when config exists
        std::fs::write(tmp.join(".acme-linter.toml"), "# config").ok();
        let result = rt.run_check("acme.lint", PluginCheckKind::Prepare, &tmp);
        assert!(result.is_ok());
        let Ok(output) = result else { return };
        assert!(output.passed);
        assert!(output.findings.is_empty());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn community_plugin_runtime_rejects_unsupported_operations() {
        let rt = AcmeLinterRuntime;
        assert!(rt.bootstrap_agent("acme.agent").is_err());
        assert!(rt.register_mcp_server("acme.mcp").is_err());
    }

    #[test]
    fn default_workflow_methods_return_typed_errors() {
        let rt = AcmeLinterRuntime;
        let root = Path::new(".");

        let resolve_err = rt.resolve_work_item("5.3", root).unwrap_err();
        assert_eq!(resolve_err.code, "not_a_workflow_plugin");
        assert!(resolve_err.message.contains(COMMUNITY_PLUGIN_ID));

        let list_err = rt.list_work_items(root).unwrap_err();
        assert_eq!(list_err.code, "not_a_workflow_plugin");

        let prompt_err = rt
            .build_prompt_context(
                &re_plugin::WorkItemResolution {
                    raw_id: "5.3".to_owned(),
                    canonical_id: "5.3".to_owned(),
                    title: "Test".to_owned(),
                    source_path: None,
                    metadata: Vec::new(),
                },
                root,
            )
            .unwrap_err();
        assert_eq!(prompt_err.code, "not_a_workflow_plugin");

        let launch_err = rt
            .launch_agent(
                "acme.agent",
                &re_plugin::PromptContext {
                    prompt_text: String::new(),
                    context_files: Vec::new(),
                    work_item_id: "5.3".to_owned(),
                    discovered_tools: Vec::new(),
                },
                root,
            )
            .unwrap_err();
        assert_eq!(launch_err.code, "not_an_agent_plugin");

        // required_tools default returns empty
        assert!(rt.required_tools().is_empty());
    }

    #[test]
    fn community_plugin_error_is_human_readable() {
        let error = PluginRuntimeError::new("test_code", "something went wrong");
        assert_eq!(error.to_string(), "[test_code] something went wrong");
    }

    // ── FeedContribution ────────────────────────────────────────

    #[test]
    fn feed_contribution_fields_accessible() {
        let fc = re_plugin::FeedContribution {
            title: "Story 5.3 started".to_owned(),
            content: vec!["Resolving work item...".to_owned()],
            kind: "system".to_owned(),
            phase_marker: Some("start:resolve".to_owned()),
            success: None,
        };
        assert_eq!(fc.title, "Story 5.3 started");
        assert_eq!(fc.kind, "system");
        assert!(fc.phase_marker.is_some());
        assert!(fc.success.is_none());
    }

    #[test]
    fn feed_contribution_gate_result() {
        let pass = re_plugin::FeedContribution {
            title: "lint".to_owned(),
            content: vec!["0 errors".to_owned()],
            kind: "gate-pass".to_owned(),
            phase_marker: Some("pass:lint".to_owned()),
            success: Some(true),
        };
        let fail = re_plugin::FeedContribution {
            title: "test".to_owned(),
            content: vec!["3 failures".to_owned()],
            kind: "gate-fail".to_owned(),
            phase_marker: Some("fail:test".to_owned()),
            success: Some(false),
        };
        assert_eq!(pass.success, Some(true));
        assert_eq!(fail.success, Some(false));
    }

    #[test]
    fn feed_contribution_empty_content() {
        let fc = re_plugin::FeedContribution {
            title: "System init".to_owned(),
            content: vec![],
            kind: "system".to_owned(),
            phase_marker: None,
            success: None,
        };
        assert!(fc.content.is_empty());
        assert!(fc.phase_marker.is_none());
    }
}
