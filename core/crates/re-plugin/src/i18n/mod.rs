mod en;
mod pt_br;

fn is_pt_br(locale: &str) -> bool {
    locale.eq_ignore_ascii_case("pt-br")
}

pub(crate) fn official_plugins_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::OFFICIAL_PLUGINS
    } else {
        en::OFFICIAL_PLUGINS
    }
}

pub(crate) fn plugin_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::PLUGIN
    } else {
        en::PLUGIN
    }
}

pub(crate) fn kind_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::KIND
    } else {
        en::KIND
    }
}

pub(crate) fn trust_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::TRUST
    } else {
        en::TRUST
    }
}

pub(crate) fn name_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::NAME
    } else {
        en::NAME
    }
}

pub(crate) fn version_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::VERSION
    } else {
        en::VERSION
    }
}

pub(crate) fn summary_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::SUMMARY
    } else {
        en::SUMMARY
    }
}

pub(crate) fn capabilities_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::CAPABILITIES
    } else {
        en::CAPABILITIES
    }
}

pub(crate) fn lifecycle_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::LIFECYCLE
    } else {
        en::LIFECYCLE
    }
}

pub(crate) fn load_boundary_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::LOAD_BOUNDARY
    } else {
        en::LOAD_BOUNDARY
    }
}

pub(crate) fn runtime_hooks_label(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        pt_br::RUNTIME_HOOKS
    } else {
        en::RUNTIME_HOOKS
    }
}
