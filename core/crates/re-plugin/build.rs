//! Build script for re-plugin locale generation.
#![allow(missing_docs, clippy::panic)]

use std::fs;
use std::path::Path;

fn main() {
    let locales_dir = Path::new("locales");
    re_build_utils::rerun_if_locales_changed(locales_dir);

    let locales = re_build_utils::read_locale_dir(locales_dir);

    let mut code = re_build_utils::generate_flat_locale_modules("PluginLocaleCatalog", &locales);
    code.push_str(&re_build_utils::generate_dispatch_function(
        "PluginLocaleCatalog",
        &locales,
    ));

    let out = re_build_utils::out_dir().join("i18n_generated.rs");
    fs::write(&out, code).unwrap_or_else(|e| {
        panic!("re-plugin build.rs: cannot write {}: {e}", out.display());
    });
}
