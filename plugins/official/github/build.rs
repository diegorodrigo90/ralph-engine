//! Build script for github plugin locale generation.
#![allow(missing_docs, clippy::panic)]

use std::fs;
use std::path::Path;

use re_build_utils::PluginLocaleSection;

fn main() {
    let locales_dir = Path::new("locales");
    re_build_utils::rerun_if_locales_changed(locales_dir);
    let locales = re_build_utils::read_locale_dir(locales_dir);

    let code = re_build_utils::generate_plugin_locale_module(
        &locales,
        &[
            PluginLocaleSection {
                toml_section: "plugin",
                const_prefix: "PLUGIN",
                fn_prefix: "plugin",
                fields: &["name", "summary"],
                localized_text_type: "re_plugin::PluginLocalizedText",
            },
            PluginLocaleSection {
                toml_section: "mcp_server",
                const_prefix: "MCP_SERVER",
                fn_prefix: "mcp_server",
                fields: &["name"],
                localized_text_type: "re_mcp::McpLocalizedText",
            },
            PluginLocaleSection {
                toml_section: "data_source",
                const_prefix: "DATA_SOURCE",
                fn_prefix: "data_source",
                fields: &["name", "summary"],
                localized_text_type: "re_plugin::PluginLocalizedText",
            },
            PluginLocaleSection {
                toml_section: "context_provider",
                const_prefix: "CONTEXT_PROVIDER",
                fn_prefix: "context_provider",
                fields: &["name", "summary"],
                localized_text_type: "re_plugin::PluginLocalizedText",
            },
            PluginLocaleSection {
                toml_section: "forge_provider",
                const_prefix: "FORGE_PROVIDER",
                fn_prefix: "forge_provider",
                fields: &["name", "summary"],
                localized_text_type: "re_plugin::PluginLocalizedText",
            },
        ],
    );

    let out = re_build_utils::out_dir().join("i18n_generated.rs");
    fs::write(&out, code).unwrap_or_else(|e| panic!("github build.rs: {e}"));
}
