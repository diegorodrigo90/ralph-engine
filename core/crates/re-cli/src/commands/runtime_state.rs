//! Shared runtime snapshot helpers for CLI commands.

use std::path::Path;

use re_config::{OwnedProjectConfig, render_owned_project_config_yaml};
use re_core::{RuntimeSnapshot, build_runtime_snapshot};

use super::embedded_assets::write_text_output;
use crate::catalog;
use crate::{CliError, i18n};

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
    #[allow(clippy::redundant_closure_for_method_calls)]
    with_official_runtime_snapshot(|runtime| runtime.patched_config())
}

/// Writes the fully materialized project configuration after applying the
/// canonical runtime remediation patch to one output path.
pub fn apply_official_runtime_patched_config(
    output_path: &Path,
    locale: &str,
    command_path: &str,
) -> Result<String, CliError> {
    if output_path.as_os_str().is_empty() {
        return Err(CliError::new(i18n::missing_output_path(
            locale,
            command_path,
        )));
    }

    write_text_output(
        output_path,
        &render_official_runtime_patched_config(),
        locale,
    )
}
