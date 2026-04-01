//! Runtime check command handlers.

use re_core::{RuntimeCheckKind, RuntimeCheckRegistration};

use crate::{CliError, catalog};

/// Executes the checks command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_check_listing(&catalog::official_runtime_checks())),
        Some("show") => show_check(args.get(1).map(String::as_str)),
        Some(other) => Err(CliError::new(format!("unknown checks command: {other}"))),
    }
}

fn show_check(check_kind: Option<&str>) -> Result<String, CliError> {
    let check_kind = check_kind.ok_or_else(|| CliError::new("checks show requires a check id"))?;
    let kind = parse_check_kind(check_kind)
        .ok_or_else(|| CliError::new(format!("unknown check: {check_kind}")))?;
    let checks = catalog::official_runtime_checks()
        .into_iter()
        .filter(|registration| registration.kind == kind)
        .collect::<Vec<_>>();

    Ok(render_check_detail(kind, &checks))
}

fn parse_check_kind(value: &str) -> Option<RuntimeCheckKind> {
    match value {
        "prepare" => Some(RuntimeCheckKind::Prepare),
        "doctor" => Some(RuntimeCheckKind::Doctor),
        _ => None,
    }
}

fn render_check_listing(registrations: &[RuntimeCheckRegistration]) -> String {
    let mut seen = Vec::new();
    let mut lines = Vec::new();

    for registration in registrations {
        let check = registration.kind.as_str();

        if seen.contains(&check) {
            continue;
        }

        seen.push(check);

        let checks = registrations
            .iter()
            .filter(|candidate| candidate.kind == registration.kind)
            .collect::<Vec<_>>();
        let enabled_checks = checks.iter().filter(|check| check.is_enabled()).count();

        lines.push(format!(
            "- {} | providers={} | enabled={}",
            check,
            checks.len(),
            enabled_checks
        ));
    }

    if lines.is_empty() {
        "Checks (0)".to_owned()
    } else {
        format!("Checks ({})\n{}", lines.len(), lines.join("\n"))
    }
}

fn render_check_detail(kind: RuntimeCheckKind, checks: &[RuntimeCheckRegistration]) -> String {
    let mut lines = vec![
        format!("Check: {}", kind.as_str()),
        format!("Providers ({})", checks.len()),
    ];

    for check in checks {
        lines.push(format!(
            "- {} | activation={} | boundary={} | runtime_hook={}",
            check.plugin_id,
            check.activation.as_str(),
            check.load_boundary.as_str(),
            check.runtime_hook_registered
        ));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_core::{RuntimeCheckKind, RuntimeCheckRegistration};
    use re_plugin::PluginLoadBoundary;

    use super::{parse_check_kind, render_check_detail, render_check_listing};

    #[test]
    fn parse_check_kind_supports_stable_identifiers() {
        // Arrange
        let values = ["prepare", "doctor"];

        // Act
        let parsed = values.into_iter().map(parse_check_kind).collect::<Vec<_>>();

        // Assert
        assert_eq!(
            parsed,
            vec![
                Some(RuntimeCheckKind::Prepare),
                Some(RuntimeCheckKind::Doctor)
            ]
        );
    }

    #[test]
    fn parse_check_kind_rejects_unknown_identifier() {
        // Arrange
        let value = "unknown";

        // Act
        let parsed = parse_check_kind(value);

        // Assert
        assert_eq!(parsed, None);
    }

    #[test]
    fn render_check_listing_handles_empty_sets() {
        // Arrange
        let registrations = [];

        // Act
        let rendered = render_check_listing(&registrations);

        // Assert
        assert_eq!(rendered, "Checks (0)");
    }

    #[test]
    fn render_check_listing_deduplicates_check_kinds() {
        // Arrange
        let registrations = [
            RuntimeCheckRegistration::new(
                RuntimeCheckKind::Prepare,
                "official.bmad",
                PluginActivation::Enabled,
                PluginLoadBoundary::InProcess,
                true,
            ),
            RuntimeCheckRegistration::new(
                RuntimeCheckKind::Prepare,
                "official.other",
                PluginActivation::Disabled,
                PluginLoadBoundary::InProcess,
                false,
            ),
        ];

        // Act
        let rendered = render_check_listing(&registrations);

        // Assert
        assert_eq!(rendered, "Checks (1)\n- prepare | providers=2 | enabled=1");
    }

    #[test]
    fn render_check_detail_is_human_readable() {
        // Arrange
        let checks = [RuntimeCheckRegistration::new(
            RuntimeCheckKind::Prepare,
            "official.bmad",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )];

        // Act
        let rendered = render_check_detail(RuntimeCheckKind::Prepare, &checks);

        // Assert
        assert!(rendered.contains("Check: prepare"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains(
            "- official.bmad | activation=enabled | boundary=in_process | runtime_hook=true"
        ));
    }
}
