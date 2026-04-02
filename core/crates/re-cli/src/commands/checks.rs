//! Runtime check command handlers.

use re_core::{RuntimeCheckKind, RuntimeCheckRegistration, parse_runtime_check_kind};

use crate::{
    CliError, catalog,
    commands::grouped_surfaces::{render_grouped_surface_detail, render_grouped_surface_listing},
    i18n,
};

/// Executes the checks command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_check_listing(
            &catalog::official_runtime_checks(),
            locale,
        )),
        Some("show") => show_check(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "checks", other,
        ))),
    }
}

fn show_check(check_kind: Option<&str>, locale: &str) -> Result<String, CliError> {
    let check_kind = check_kind.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "checks",
            i18n::check_id_entity_label(locale),
        ))
    })?;
    let kind = parse_runtime_check_kind(check_kind).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::check_entity_label(locale),
            check_kind,
        ))
    })?;
    let checks = catalog::find_official_runtime_checks(kind);

    Ok(render_check_detail(kind, &checks, locale))
}

fn render_check_listing(registrations: &[RuntimeCheckRegistration], locale: &str) -> String {
    render_grouped_surface_listing(
        registrations,
        locale,
        i18n::checks_label,
        |registration| registration.kind.as_str(),
        |registration| registration.is_enabled(),
    )
}

fn render_check_detail(
    kind: RuntimeCheckKind,
    checks: &[RuntimeCheckRegistration],
    locale: &str,
) -> String {
    render_grouped_surface_detail(kind.as_str(), checks, locale, i18n::check_label, |check| {
        format!(
            "- {} | activation={} | boundary={} | runtime_hook={}",
            check.plugin_id,
            check.activation.as_str(),
            check.load_boundary.as_str(),
            check.runtime_hook_registered
        )
    })
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_core::{RuntimeCheckKind, RuntimeCheckRegistration};
    use re_plugin::PluginLoadBoundary;

    use super::{render_check_detail, render_check_listing};
    use re_core::parse_runtime_check_kind;

    #[test]
    fn parse_check_kind_supports_stable_identifiers() {
        // Arrange
        let values = ["prepare", "doctor"];

        // Act
        let parsed = values
            .into_iter()
            .map(parse_runtime_check_kind)
            .collect::<Vec<_>>();

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
        let parsed = parse_runtime_check_kind(value);

        // Assert
        assert_eq!(parsed, None);
    }

    #[test]
    fn render_check_listing_handles_empty_sets() {
        // Arrange
        let registrations = [];

        // Act
        let rendered = render_check_listing(&registrations, "en");

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
        let rendered = render_check_listing(&registrations, "en");

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
        let rendered = render_check_detail(RuntimeCheckKind::Prepare, &checks, "en");

        // Assert
        assert!(rendered.contains("Check: prepare"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains(
            "- official.bmad | activation=enabled | boundary=in_process | runtime_hook=true"
        ));
    }

    #[test]
    fn render_check_detail_supports_pt_br() {
        let checks = [RuntimeCheckRegistration::new(
            RuntimeCheckKind::Prepare,
            "official.bmad",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )];

        let rendered = render_check_detail(RuntimeCheckKind::Prepare, &checks, "pt-br");

        assert!(rendered.contains("Verificação: prepare"));
        assert!(rendered.contains("Provedores (1)"));
    }
}
