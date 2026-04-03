//! Build script for bmad plugin locale generation.
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
                toml_section: "template",
                const_prefix: "TEMPLATE",
                fn_prefix: "template",
                fields: &["name", "summary"],
                localized_text_type: "re_plugin::PluginLocalizedText",
            },
            PluginLocaleSection {
                toml_section: "prompt",
                const_prefix: "PROMPT",
                fn_prefix: "prompt",
                fields: &["name", "summary"],
                localized_text_type: "re_plugin::PluginLocalizedText",
            },
            PluginLocaleSection {
                toml_section: "prepare_check",
                const_prefix: "PREPARE_CHECK",
                fn_prefix: "prepare_check",
                fields: &["name", "summary"],
                localized_text_type: "re_plugin::PluginLocalizedText",
            },
            PluginLocaleSection {
                toml_section: "doctor_check",
                const_prefix: "DOCTOR_CHECK",
                fn_prefix: "doctor_check",
                fields: &["name", "summary"],
                localized_text_type: "re_plugin::PluginLocalizedText",
            },
        ],
    );

    let out = re_build_utils::out_dir().join("i18n_generated.rs");
    fs::write(&out, code).unwrap_or_else(|e| panic!("bmad build.rs: {e}"));
}
