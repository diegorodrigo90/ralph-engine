mod en;
mod pt_br;

#[derive(Clone, Copy)]
enum PluginLocale {
    En,
    PtBr,
}

impl PluginLocale {
    fn resolve(locale: &str) -> Self {
        if locale.eq_ignore_ascii_case("pt-br") {
            Self::PtBr
        } else {
            Self::En
        }
    }
}

pub(crate) fn official_plugins_label(locale: &str) -> &'static str {
    match PluginLocale::resolve(locale) {
        PluginLocale::En => en::OFFICIAL_PLUGINS,
        PluginLocale::PtBr => pt_br::OFFICIAL_PLUGINS,
    }
}

pub(crate) fn plugin_label(locale: &str) -> &'static str {
    match PluginLocale::resolve(locale) {
        PluginLocale::En => en::PLUGIN,
        PluginLocale::PtBr => pt_br::PLUGIN,
    }
}

pub(crate) fn kind_label(locale: &str) -> &'static str {
    match PluginLocale::resolve(locale) {
        PluginLocale::En => en::KIND,
        PluginLocale::PtBr => pt_br::KIND,
    }
}

pub(crate) fn trust_label(locale: &str) -> &'static str {
    match PluginLocale::resolve(locale) {
        PluginLocale::En => en::TRUST,
        PluginLocale::PtBr => pt_br::TRUST,
    }
}

pub(crate) fn name_label(locale: &str) -> &'static str {
    match PluginLocale::resolve(locale) {
        PluginLocale::En => en::NAME,
        PluginLocale::PtBr => pt_br::NAME,
    }
}

pub(crate) fn version_label(locale: &str) -> &'static str {
    match PluginLocale::resolve(locale) {
        PluginLocale::En => en::VERSION,
        PluginLocale::PtBr => pt_br::VERSION,
    }
}

pub(crate) fn summary_label(locale: &str) -> &'static str {
    match PluginLocale::resolve(locale) {
        PluginLocale::En => en::SUMMARY,
        PluginLocale::PtBr => pt_br::SUMMARY,
    }
}

pub(crate) fn capabilities_label(locale: &str) -> &'static str {
    match PluginLocale::resolve(locale) {
        PluginLocale::En => en::CAPABILITIES,
        PluginLocale::PtBr => pt_br::CAPABILITIES,
    }
}

pub(crate) fn lifecycle_label(locale: &str) -> &'static str {
    match PluginLocale::resolve(locale) {
        PluginLocale::En => en::LIFECYCLE,
        PluginLocale::PtBr => pt_br::LIFECYCLE,
    }
}

pub(crate) fn load_boundary_label(locale: &str) -> &'static str {
    match PluginLocale::resolve(locale) {
        PluginLocale::En => en::LOAD_BOUNDARY,
        PluginLocale::PtBr => pt_br::LOAD_BOUNDARY,
    }
}

pub(crate) fn runtime_hooks_label(locale: &str) -> &'static str {
    match PluginLocale::resolve(locale) {
        PluginLocale::En => en::RUNTIME_HOOKS,
        PluginLocale::PtBr => pt_br::RUNTIME_HOOKS,
    }
}
