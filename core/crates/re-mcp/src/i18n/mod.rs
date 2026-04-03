pub(super) struct McpLocaleCatalog {
    pub official_servers: &'static str,
    pub launch_plan: &'static str,
    pub launch_step: &'static str,
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

// Locale modules and dispatch function generated from locales/*.toml
include!(concat!(env!("OUT_DIR"), "/i18n_generated.rs"));

macro_rules! locale_label {
    ($fn_name:ident, $field:ident) => {
        pub(crate) fn $fn_name(locale: &str) -> &'static str {
            locale_catalog(locale).$field
        }
    };
}

locale_label!(official_servers_label, official_servers);
locale_label!(launch_plan_label, launch_plan);
locale_label!(launch_step_label, launch_step);
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
