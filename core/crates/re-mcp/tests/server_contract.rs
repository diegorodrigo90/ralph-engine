//! Integration tests for the shared Ralph Engine MCP contract.

use re_mcp::{
    McpAvailability, McpProcessModel, McpServerDescriptor, McpTransport, render_mcp_server_detail,
    render_mcp_server_listing,
};

fn claude_server() -> McpServerDescriptor {
    McpServerDescriptor::new(
        "official.claude.session",
        "official.claude",
        "Claude Session",
        McpTransport::Stdio,
        McpProcessModel::PluginManaged,
        McpAvailability::OnDemand,
    )
}

fn invalid_server() -> McpServerDescriptor {
    McpServerDescriptor::new(
        "claude",
        "claude",
        "Broken",
        McpTransport::Stdio,
        McpProcessModel::ExternalBinary,
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
    assert!(detail.contains("Availability: on_demand"));
}
