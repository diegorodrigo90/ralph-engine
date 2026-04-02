mod en;
mod pt_br;

fn is_pt_br(locale: &str) -> bool {
    locale.eq_ignore_ascii_case("pt-br")
}

pub(crate) fn official_servers_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::OFFICIAL_SERVERS
    } else {
        en::OFFICIAL_SERVERS
    }
}

pub(crate) fn server_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::SERVER
    } else {
        en::SERVER
    }
}

pub(crate) fn name_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::NAME
    } else {
        en::NAME
    }
}

pub(crate) fn plugin_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::PLUGIN
    } else {
        en::PLUGIN
    }
}

pub(crate) fn transport_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::TRANSPORT
    } else {
        en::TRANSPORT
    }
}

pub(crate) fn process_model_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::PROCESS_MODEL
    } else {
        en::PROCESS_MODEL
    }
}

pub(crate) fn launch_policy_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::LAUNCH_POLICY
    } else {
        en::LAUNCH_POLICY
    }
}

pub(crate) fn availability_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::AVAILABILITY
    } else {
        en::AVAILABILITY
    }
}

pub(crate) fn command_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::COMMAND
    } else {
        en::COMMAND
    }
}

pub(crate) fn working_directory_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::WORKING_DIRECTORY
    } else {
        en::WORKING_DIRECTORY
    }
}

pub(crate) fn environment_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::ENVIRONMENT
    } else {
        en::ENVIRONMENT
    }
}

pub(crate) fn runtime_managed_command_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::RUNTIME_MANAGED_COMMAND
    } else {
        en::RUNTIME_MANAGED_COMMAND
    }
}
