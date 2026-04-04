//! Build script for re-cli locale generation.
#![allow(missing_docs, clippy::panic)]

use std::fs;
use std::path::Path;

fn main() {
    let locales_dir = Path::new("locales");
    re_build_utils::rerun_if_locales_changed(locales_dir);

    let locales = re_build_utils::read_locale_dir(locales_dir);

    let fn_fields: &[&str] = &[
        "unknown_command",
        "unknown_subcommand",
        "missing_id",
        "missing_argument",
        "unknown_entity",
        "missing_asset_path",
        "missing_output_directory",
        "missing_output_path",
        "invalid_embedded_asset_path",
        "failed_to_write_output",
        "wrote_output",
        "unknown_template_asset",
        "unknown_prompt_asset",
        "unknown_check_asset",
        "unknown_policy_asset",
        // Install/uninstall parameterized messages
        "install_already_installed",
        "install_create_dir_failed",
        "install_clone_exec_failed",
        "install_clone_repo_failed",
        "install_not_installed",
        "install_remove_dir_failed",
        "install_uninstalled",
        // Init parameterized messages
        "init_remove_failed",
        // MCP parameterized messages
        "mcp_install_hint",
        // Policies parameterized messages
        "policies_materialize_hint",
    ];

    let mut code =
        re_build_utils::generate_locale_modules_with_fns("CliLocaleCatalog", &locales, fn_fields);
    code.push_str(&re_build_utils::generate_dispatch_function(
        "CliLocaleCatalog",
        &locales,
    ));

    let out = re_build_utils::out_dir().join("i18n_generated.rs");
    fs::write(&out, code).unwrap_or_else(|e| {
        panic!("re-cli build.rs: cannot write {}: {e}", out.display());
    });
}
