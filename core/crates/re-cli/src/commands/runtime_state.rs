//! Shared runtime snapshot and remediation helpers for CLI commands.

use re_config::{
    OwnedProjectConfig, apply_project_config_patch, default_project_config,
    render_owned_project_config_yaml,
};
use re_core::{RuntimeSnapshot, build_runtime_snapshot};

use crate::catalog;

/// Runs one operation against the canonical runtime snapshot used by public CLI
/// commands.
pub fn with_official_runtime_snapshot<T>(callback: impl FnOnce(&RuntimeSnapshot<'_>) -> T) -> T {
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();
    let runtime = build_runtime_snapshot(&topology);

    callback(&runtime)
}

/// Renders the fully materialized project configuration after applying the
/// canonical runtime remediation patch.
#[must_use]
pub fn render_official_runtime_patched_config() -> String {
    render_owned_project_config_yaml(&official_runtime_patched_config())
}

/// Builds the fully materialized project configuration after applying the
/// canonical runtime remediation patch.
#[must_use]
pub fn official_runtime_patched_config() -> OwnedProjectConfig {
    with_official_runtime_snapshot(|runtime| {
        apply_project_config_patch(
            &default_project_config(),
            &runtime.config_patch.plugins,
            &runtime.config_patch.mcp_servers,
        )
    })
}
