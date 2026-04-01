//! Integration tests for the shared Ralph Engine MCP contract.

use re_mcp::{McpServerDescriptor, McpTransport, render_mcp_server_listing};

fn claude_server() -> McpServerDescriptor {
    McpServerDescriptor::new(
        "official.claude.session",
        "official.claude",
        "Claude Session",
        McpTransport::Stdio,
    )
}

fn invalid_server() -> McpServerDescriptor {
    McpServerDescriptor::new("claude", "claude", "Broken", McpTransport::Stdio)
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
