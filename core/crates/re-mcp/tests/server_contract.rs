//! Integration tests for the shared Ralph Engine MCP contract.

use re_mcp::{
    McpAvailability, McpCommandDescriptor, McpEnvironmentPolicy, McpLaunchPolicy, McpProcessModel,
    McpServerDescriptor, McpTransport, McpWorkingDirectoryPolicy, render_mcp_server_detail,
    render_mcp_server_detail_for_locale, render_mcp_server_listing,
    render_mcp_server_listing_for_locale,
};

fn claude_server() -> McpServerDescriptor {
    McpServerDescriptor::new(
        "official.claude.session",
        "official.claude",
        "Claude Session",
        McpTransport::Stdio,
        McpLaunchPolicy::PluginRuntime,
        McpAvailability::OnDemand,
    )
}

fn invalid_server() -> McpServerDescriptor {
    McpServerDescriptor::new(
        "claude",
        "claude",
        "Broken",
        McpTransport::Stdio,
        McpLaunchPolicy::SpawnProcess(McpCommandDescriptor::new(
            "broken-mcp",
            &["serve"],
            McpWorkingDirectoryPolicy::ProjectRoot,
            McpEnvironmentPolicy::PluginScoped,
        )),
        McpAvailability::ExplicitOptIn,
    )
}

#[test]
fn server_descriptor_requires_namespaced_identifier() {
    // Arrange
    let server = claude_server();

    // Act
    let namespaced = server.is_namespaced();

    // Assert
    assert!(namespaced);
}

#[test]
fn server_descriptor_rejects_non_namespaced_identifier() {
    // Arrange
    let server = invalid_server();

    // Act
    let namespaced = server.is_namespaced();

    // Assert
    assert!(!namespaced);
}

#[test]
fn server_descriptor_requires_plugin_namespace() {
    // Arrange
    let server = claude_server();

    // Act
    let namespaced = server.has_plugin_namespace();

    // Assert
    assert!(namespaced);
}

#[test]
fn server_descriptor_rejects_non_namespaced_plugin_identifier() {
    // Arrange
    let server = invalid_server();

    // Act
    let namespaced = server.has_plugin_namespace();

    // Assert
    assert!(!namespaced);
}

#[test]
fn transport_display_is_stable() {
    // Arrange
    let transport = McpTransport::Stdio;

    // Act
    let rendered = transport.to_string();

    // Assert
    assert_eq!(rendered, "stdio");
}

#[test]
fn process_model_display_is_stable() {
    // Arrange
    let process_models = [
        McpProcessModel::PluginManaged,
        McpProcessModel::ExternalBinary,
    ];

    // Act
    let rendered = process_models
        .into_iter()
        .map(|process_model| process_model.to_string())
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(rendered, vec!["plugin_managed", "external_binary"]);
}

#[test]
fn working_directory_policy_display_is_stable() {
    // Arrange
    let policies = [
        McpWorkingDirectoryPolicy::RuntimeManaged,
        McpWorkingDirectoryPolicy::ProjectRoot,
        McpWorkingDirectoryPolicy::PluginWorkspace,
    ];

    // Act
    let rendered = policies
        .into_iter()
        .map(|policy| policy.to_string())
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(
        rendered,
        vec!["runtime_managed", "project_root", "plugin_workspace"]
    );
}

#[test]
fn environment_policy_display_is_stable() {
    // Arrange
    let policies = [
        McpEnvironmentPolicy::MinimalRuntime,
        McpEnvironmentPolicy::PluginScoped,
    ];

    // Act
    let rendered = policies
        .into_iter()
        .map(|policy| policy.to_string())
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(rendered, vec!["minimal_runtime", "plugin_scoped"]);
}

#[test]
fn launch_policy_display_is_stable() {
    // Arrange
    let policies = [
        McpLaunchPolicy::PluginRuntime,
        McpLaunchPolicy::SpawnProcess(McpCommandDescriptor::new(
            "codex-mcp",
            &["serve"],
            McpWorkingDirectoryPolicy::ProjectRoot,
            McpEnvironmentPolicy::PluginScoped,
        )),
    ];

    // Act
    let rendered = policies
        .into_iter()
        .map(|policy| policy.to_string())
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(rendered, vec!["plugin_runtime", "spawn_process"]);
}

#[test]
fn command_descriptor_reports_arguments_when_present() {
    // Arrange
    let command = McpCommandDescriptor::new(
        "codex-mcp",
        &["serve"],
        McpWorkingDirectoryPolicy::ProjectRoot,
        McpEnvironmentPolicy::PluginScoped,
    );

    // Act
    let has_args = command.has_args();
    let invocation = command.render_invocation();

    // Assert
    assert!(has_args);
    assert_eq!(invocation, "codex-mcp serve");
}

#[test]
fn command_descriptor_handles_argument_free_invocation() {
    // Arrange
    let command = McpCommandDescriptor::new(
        "claude-mcp",
        &[],
        McpWorkingDirectoryPolicy::RuntimeManaged,
        McpEnvironmentPolicy::MinimalRuntime,
    );

    // Act
    let has_args = command.has_args();
    let invocation = command.render_invocation();

    // Assert
    assert!(!has_args);
    assert_eq!(invocation, "claude-mcp");
}

#[test]
fn availability_display_is_stable() {
    // Arrange
    let availability = [McpAvailability::OnDemand, McpAvailability::ExplicitOptIn];

    // Act
    let rendered = availability
        .into_iter()
        .map(|policy| policy.to_string())
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(rendered, vec!["on_demand", "explicit_opt_in"]);
}

#[test]
fn server_descriptor_reports_plugin_managed_execution() {
    // Arrange
    let server = claude_server();

    // Act
    let plugin_managed = server.is_plugin_managed();

    // Assert
    assert!(plugin_managed);
}

#[test]
fn server_descriptor_reports_external_execution() {
    // Arrange
    let server = invalid_server();

    // Act
    let plugin_managed = server.is_plugin_managed();

    // Assert
    assert!(!plugin_managed);
}

#[test]
fn launch_policy_returns_spawn_command_for_external_binary() {
    // Arrange
    let server = invalid_server();

    // Act
    let command = server.command();
    let invocation = command.map(|spawn_command| spawn_command.render_invocation());

    // Assert
    assert!(command.is_some());
    assert_eq!(invocation, Some("broken-mcp serve".to_owned()));
}

#[test]
fn launch_policy_reports_no_spawn_command_for_plugin_runtime() {
    // Arrange
    let server = claude_server();

    // Act
    let command = server.command();

    // Assert
    assert!(command.is_none());
}

#[test]
fn server_descriptor_reports_on_demand_availability() {
    // Arrange
    let server = claude_server();

    // Act
    let on_demand = server.is_on_demand();

    // Assert
    assert!(on_demand);
}

#[test]
fn server_descriptor_reports_explicit_opt_in_availability() {
    // Arrange
    let server = invalid_server();

    // Act
    let on_demand = server.is_on_demand();

    // Assert
    assert!(!on_demand);
}

#[test]
fn render_mcp_server_listing_includes_human_readable_lines() {
    // Arrange
    let servers = [claude_server()];

    // Act
    let listing = render_mcp_server_listing(&servers);

    // Assert
    assert!(listing.contains("Official MCP servers (1)"));
    assert!(
        listing.contains("- official.claude.session | Claude Session | official.claude | stdio")
    );
}

#[test]
fn render_mcp_server_listing_handles_empty_sets() {
    // Arrange
    let servers = [];

    // Act
    let listing = render_mcp_server_listing(&servers);

    // Assert
    assert_eq!(listing, "Official MCP servers (0)");
}

#[test]
fn render_mcp_server_detail_includes_process_model_and_policy() {
    // Arrange
    let server = claude_server();

    // Act
    let detail = render_mcp_server_detail(&server);

    // Assert
    assert!(detail.contains("MCP server: official.claude.session"));
    assert!(detail.contains("Process model: plugin_managed"));
    assert!(detail.contains("Launch policy: plugin_runtime"));
    assert!(detail.contains("Availability: on_demand"));
    assert!(detail.contains("Command: managed by plugin runtime"));
}

#[test]
fn render_mcp_server_detail_includes_spawn_contract() {
    // Arrange
    let server = invalid_server();

    // Act
    let detail = render_mcp_server_detail(&server);

    // Assert
    assert!(detail.contains("Process model: external_binary"));
    assert!(detail.contains("Launch policy: spawn_process"));
    assert!(detail.contains("Command: broken-mcp serve"));
    assert!(detail.contains("Working directory: project_root"));
    assert!(detail.contains("Environment: plugin_scoped"));
}

#[test]
fn render_mcp_server_listing_supports_pt_br() {
    let rendered = render_mcp_server_listing_for_locale(&[claude_server()], "pt-br");

    assert!(rendered.contains("Servidores MCP oficiais (1)"));
    assert!(
        rendered.contains("official.claude.session | Claude Session | official.claude | stdio")
    );
}

#[test]
fn render_mcp_server_detail_supports_pt_br_and_runtime_fallback_text() {
    let rendered = render_mcp_server_detail_for_locale(&claude_server(), "pt-br");

    assert!(rendered.contains("Servidor MCP: official.claude.session"));
    assert!(rendered.contains("Nome: Claude Session"));
    assert!(rendered.contains("Transporte: stdio"));
    assert!(rendered.contains("Política de launch: plugin_runtime"));
    assert!(rendered.contains("Comando: gerenciado pelo runtime do plugin"));
    assert!(rendered.contains("Diretório de trabalho: runtime_managed"));
    assert!(rendered.contains("Ambiente: minimal_runtime"));
}

#[test]
fn render_mcp_server_detail_supports_pt_br_for_spawn_process() {
    let rendered = render_mcp_server_detail_for_locale(&invalid_server(), "pt-br");

    assert!(rendered.contains("Comando: broken-mcp serve"));
    assert!(rendered.contains("Diretório de trabalho: project_root"));
    assert!(rendered.contains("Ambiente: plugin_scoped"));
}
