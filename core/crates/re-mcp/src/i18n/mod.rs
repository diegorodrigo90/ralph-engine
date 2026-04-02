pub(super) struct McpLocaleCatalog {
    pub official_servers: &'static str,
    pub server: &'static str,
    pub name: &'static str,
    pub plugin: &'static str,
    pub transport: &'static str,
    pub process_model: &'static str,
    pub launch_policy: &'static str,
    pub availability: &'static str,
    pub command: &'static str,
    pub working_directory: &'static str,
    pub environment: &'static str,
    pub runtime_managed_command: &'static str,
}

mod en;
mod pt_br;

fn locale_catalog(locale: &str) -> &'static McpLocaleCatalog {
    match re_config::resolve_supported_locale_or_default(locale) {
        re_config::SupportedLocale::En => &en::LOCALE,
        re_config::SupportedLocale::PtBr => &pt_br::LOCALE,
    }
}

macro_rules! locale_label {
    ($fn_name:ident, $field:ident) => {
        pub(crate) fn $fn_name(locale: &str) -> &'static str {
            locale_catalog(locale).$field
        }
    };
}

locale_label!(official_servers_label, official_servers);
locale_label!(server_label, server);
locale_label!(name_label, name);
locale_label!(plugin_label, plugin);
locale_label!(transport_label, transport);
locale_label!(process_model_label, process_model);
locale_label!(launch_policy_label, launch_policy);
locale_label!(availability_label, availability);
locale_label!(command_label, command);
locale_label!(working_directory_label, working_directory);
locale_label!(environment_label, environment);
locale_label!(runtime_managed_command_label, runtime_managed_command);
