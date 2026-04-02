pub(super) struct PluginLocaleCatalog {
    pub official_plugins: &'static str,
    pub plugin: &'static str,
    pub kind: &'static str,
    pub trust: &'static str,
    pub name: &'static str,
    pub version: &'static str,
    pub summary: &'static str,
    pub capabilities: &'static str,
    pub lifecycle: &'static str,
    pub load_boundary: &'static str,
    pub runtime_hooks: &'static str,
}

mod en;
mod pt_br;

#[derive(Clone, Copy)]
enum PluginLocale {
    En,
    PtBr,
}

impl PluginLocale {
    fn resolve(locale: &str) -> Self {
        if re_config::resolve_locale_or_default(locale) == "pt-br" {
            Self::PtBr
        } else {
            Self::En
        }
    }
}

fn locale_catalog(locale: &str) -> &'static PluginLocaleCatalog {
    match PluginLocale::resolve(locale) {
        PluginLocale::En => &en::LOCALE,
        PluginLocale::PtBr => &pt_br::LOCALE,
    }
}

macro_rules! locale_label {
    ($fn_name:ident, $field:ident) => {
        pub(crate) fn $fn_name(locale: &str) -> &'static str {
            locale_catalog(locale).$field
        }
    };
}

locale_label!(official_plugins_label, official_plugins);
locale_label!(plugin_label, plugin);
locale_label!(kind_label, kind);
locale_label!(trust_label, trust);
locale_label!(name_label, name);
locale_label!(version_label, version);
locale_label!(summary_label, summary);
locale_label!(capabilities_label, capabilities);
locale_label!(lifecycle_label, lifecycle);
locale_label!(load_boundary_label, load_boundary);
locale_label!(runtime_hooks_label, runtime_hooks);
