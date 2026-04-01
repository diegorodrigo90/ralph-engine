//! Integration tests for the shared Ralph Engine plugin contract.

use re_plugin::{
    AGENT_RUNTIME, CONTEXT_PROVIDER, DATA_SOURCE, DOCTOR_CHECKS, FORGE_PROVIDER, MCP_CONTRIBUTION,
    POLICY, PREPARE_CHECKS, PROMPT_FRAGMENTS, PluginCapability, PluginDescriptor,
    PluginLifecycleStage, PluginLoadBoundary, PluginRuntimeHook, REMOTE_CONTROL, TEMPLATE,
    render_plugin_detail, render_plugin_listing,
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
const GITHUB_RUNTIME_HOOKS: &[PluginRuntimeHook] = &[
    PluginRuntimeHook::McpRegistration,
    PluginRuntimeHook::DataSourceRegistration,
    PluginRuntimeHook::ForgeProviderRegistration,
];

fn basic_plugin() -> PluginDescriptor {
    PluginDescriptor::new(
        "official.basic",
        "Basic",
        "0.2.0-alpha.1",
        BASIC_CAPABILITIES,
        BASIC_LIFECYCLE,
        PluginLoadBoundary::InProcess,
        BASIC_RUNTIME_HOOKS,
    )
}

fn github_plugin() -> PluginDescriptor {
    PluginDescriptor::new(
        "official.github",
        "GitHub",
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
        "Broken",
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
    let hooks = [
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

    // Act
    let rendered = hooks
        .into_iter()
        .map(PluginRuntimeHook::as_str)
        .collect::<Vec<_>>();

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
fn descriptor_requires_namespaced_identifier() {
    // Arrange
    let descriptor = basic_plugin();

    // Act
    let namespaced = descriptor.is_namespaced();

    // Assert
    assert!(namespaced);
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
    assert!(listing.contains("- official.basic | Basic | v0.2.0-alpha.1 | template"));
    assert!(
        listing
            .contains("- official.github | GitHub | v0.2.0-alpha.1 | data_source, forge_provider")
    );
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
fn render_plugin_detail_includes_capabilities_and_lifecycle() {
    // Arrange
    let plugin = github_plugin();

    // Act
    let detail = render_plugin_detail(&plugin);

    // Assert
    assert!(detail.contains("Plugin: official.github"));
    assert!(detail.contains("Capabilities: data_source, forge_provider"));
    assert!(detail.contains("Lifecycle: discover -> configure -> load"));
    assert!(detail.contains("Load boundary: in_process"));
}
