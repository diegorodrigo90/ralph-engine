mod en;
mod pt_br;

#[derive(Clone, Copy)]
enum McpLocale {
    En,
    PtBr,
}

impl McpLocale {
    fn resolve(locale: &str) -> Self {
        if locale.eq_ignore_ascii_case("pt-br") {
            Self::PtBr
        } else {
            Self::En
        }
    }
}

pub(crate) fn official_servers_label(locale: &str) -> &'static str {
    match McpLocale::resolve(locale) {
        McpLocale::En => en::OFFICIAL_SERVERS,
        McpLocale::PtBr => pt_br::OFFICIAL_SERVERS,
    }
}

pub(crate) fn server_label(locale: &str) -> &'static str {
    match McpLocale::resolve(locale) {
        McpLocale::En => en::SERVER,
        McpLocale::PtBr => pt_br::SERVER,
    }
}

pub(crate) fn name_label(locale: &str) -> &'static str {
    match McpLocale::resolve(locale) {
        McpLocale::En => en::NAME,
        McpLocale::PtBr => pt_br::NAME,
    }
}

pub(crate) fn plugin_label(locale: &str) -> &'static str {
    match McpLocale::resolve(locale) {
        McpLocale::En => en::PLUGIN,
        McpLocale::PtBr => pt_br::PLUGIN,
    }
}

pub(crate) fn transport_label(locale: &str) -> &'static str {
    match McpLocale::resolve(locale) {
        McpLocale::En => en::TRANSPORT,
        McpLocale::PtBr => pt_br::TRANSPORT,
    }
}

pub(crate) fn process_model_label(locale: &str) -> &'static str {
    match McpLocale::resolve(locale) {
        McpLocale::En => en::PROCESS_MODEL,
        McpLocale::PtBr => pt_br::PROCESS_MODEL,
    }
}

pub(crate) fn launch_policy_label(locale: &str) -> &'static str {
    match McpLocale::resolve(locale) {
        McpLocale::En => en::LAUNCH_POLICY,
        McpLocale::PtBr => pt_br::LAUNCH_POLICY,
    }
}

pub(crate) fn availability_label(locale: &str) -> &'static str {
    match McpLocale::resolve(locale) {
        McpLocale::En => en::AVAILABILITY,
        McpLocale::PtBr => pt_br::AVAILABILITY,
    }
}

pub(crate) fn command_label(locale: &str) -> &'static str {
    match McpLocale::resolve(locale) {
        McpLocale::En => en::COMMAND,
        McpLocale::PtBr => pt_br::COMMAND,
    }
}

pub(crate) fn working_directory_label(locale: &str) -> &'static str {
    match McpLocale::resolve(locale) {
        McpLocale::En => en::WORKING_DIRECTORY,
        McpLocale::PtBr => pt_br::WORKING_DIRECTORY,
    }
}

pub(crate) fn environment_label(locale: &str) -> &'static str {
    match McpLocale::resolve(locale) {
        McpLocale::En => en::ENVIRONMENT,
        McpLocale::PtBr => pt_br::ENVIRONMENT,
    }
}

pub(crate) fn runtime_managed_command_label(locale: &str) -> &'static str {
    match McpLocale::resolve(locale) {
        McpLocale::En => en::RUNTIME_MANAGED_COMMAND,
        McpLocale::PtBr => pt_br::RUNTIME_MANAGED_COMMAND,
    }
}
