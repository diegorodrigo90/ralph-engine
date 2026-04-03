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

// Locale modules and dispatch function generated from locales/*.toml
include!(concat!(env!("OUT_DIR"), "/i18n_generated.rs"));

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
