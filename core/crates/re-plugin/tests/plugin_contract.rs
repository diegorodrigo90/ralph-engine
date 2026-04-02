//! Integration tests for the shared Ralph Engine plugin contract.

use re_plugin::{
    AGENT_RUNTIME, ALL_PLUGIN_CAPABILITIES, ALL_PLUGIN_KINDS, ALL_PLUGIN_RUNTIME_HOOKS,
    ALL_PLUGIN_RUNTIME_SURFACES, ALL_PLUGIN_TRUST_LEVELS, CONTEXT_PROVIDER, DATA_SOURCE,
    DOCTOR_CHECKS, FORGE_PROVIDER, MCP_CONTRIBUTION, POLICY, PREPARE_CHECKS, PROMPT_FRAGMENTS,
    PluginAgentDescriptor, PluginCapability, PluginDescriptor, PluginKind, PluginLifecycleStage,
    PluginLoadBoundary, PluginLocalizedText, PluginPolicyDescriptor, PluginPromptAsset,
    PluginPromptDescriptor, PluginRuntimeHook, PluginRuntimeSurface, PluginTemplateAsset,
    PluginTemplateDescriptor, PluginTrustLevel, REMOTE_CONTROL, TEMPLATE,
    parse_plugin_runtime_hook, parse_reviewed_plugin_capability, render_plugin_detail,
    render_plugin_detail_for_locale, render_plugin_listing, render_plugin_listing_for_locale,
    runtime_surface_for_capability,
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

#[test]
fn runtime_surface_mapping_covers_all_reviewed_capabilities() {
    let uncovered = ALL_PLUGIN_CAPABILITIES
        .iter()
        .copied()
        .filter(|capability| runtime_surface_for_capability(*capability).is_none())
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
