//! Integration tests for the shared Ralph Engine plugin contract.

use re_plugin::{PluginDescriptor, render_plugin_listing};

fn basic_plugin() -> PluginDescriptor {
    PluginDescriptor::new("official.basic", "Basic", "0.2.0-alpha.1", &["template"])
}

fn github_plugin() -> PluginDescriptor {
    PluginDescriptor::new(
        "official.github",
        "GitHub",
        "0.2.0-alpha.1",
        &["data_source", "forge_provider"],
    )
}

fn invalid_plugin() -> PluginDescriptor {
    PluginDescriptor::new("basic", "Broken", "0.2.0-alpha.1", &[])
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
