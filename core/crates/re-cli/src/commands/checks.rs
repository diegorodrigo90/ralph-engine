//! Runtime check command handlers.

use std::path::Path;

use re_core::{
    RuntimeCheckExecutionPlan, RuntimeCheckKind, RuntimeCheckRegistration,
    build_runtime_check_result, parse_runtime_check_kind, render_runtime_check_result_for_locale,
    runtime_hook_for_check,
};

use super::format;
use crate::{
    CliError, catalog,
    commands::embedded_assets::{MaterializedAsset, materialize_assets},
    commands::runtime_state::with_official_runtime_snapshot,
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
        Some("asset") => show_check_asset(
            args.get(1).map(String::as_str),
            args.get(2).map(String::as_str),
            locale,
        ),
        Some("materialize") => materialize_check(
            args.get(1).map(String::as_str),
            args.get(2).map(String::as_str),
            locale,
        ),
        Some("plan") => show_check_plan(args.get(1).map(String::as_str), locale),
        Some("run") => run_check(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "checks", other,
        ))),
    }
}

fn show_check_asset(
    check_id: Option<&str>,
    asset_path: Option<&str>,
    locale: &str,
) -> Result<String, CliError> {
    let check_id = check_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "checks asset",
            i18n::check_id_entity_label(locale),
        ))
    })?;
    let asset_path = asset_path
        .ok_or_else(|| CliError::new(i18n::missing_asset_path(locale, "checks asset")))?;
    let surface = catalog::find_official_check_surface(check_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::check_entity_label(locale),
            check_id,
        ))
    })?;
    let asset = surface
        .contribution
        .descriptor
        .assets
        .iter()
        .find(|asset| asset.path == asset_path)
        .ok_or_else(|| CliError::new(i18n::unknown_check_asset(locale, asset_path)))?;

    Ok(asset.contents.to_owned())
}

fn materialize_check(
    check_id: Option<&str>,
    output_dir: Option<&str>,
    locale: &str,
) -> Result<String, CliError> {
    let check_id = check_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "checks materialize",
            i18n::check_id_entity_label(locale),
        ))
    })?;
    let output_dir = output_dir.ok_or_else(|| {
        CliError::new(i18n::missing_output_directory(locale, "checks materialize"))
    })?;
    let surface = catalog::find_official_check_surface(check_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::check_entity_label(locale),
            check_id,
        ))
    })?;
    let assets = surface
        .contribution
        .descriptor
        .assets
        .iter()
        .map(|asset| MaterializedAsset {
            path: asset.path,
            contents: asset.contents,
        })
        .collect::<Vec<_>>();

    materialize_assets(&assets, Path::new(output_dir), locale)
}

fn run_check(check_kind: Option<&str>, locale: &str) -> Result<String, CliError> {
    let check_id = check_kind.ok_or_else(|| {
        CliError::new(i18n::missing_argument(
            locale,
            "checks run",
            i18n::check_id_entity_label(locale),
        ))
    })?;

    let kind = if let Some(surface) = catalog::find_official_check_surface(check_id) {
        surface.registration.kind
    } else {
        parse_runtime_check_kind(check_id).ok_or_else(|| {
            CliError::new(i18n::unknown_entity(
                locale,
                i18n::check_entity_label(locale),
                check_id,
            ))
        })?
    };

    let topology_output = with_official_runtime_snapshot(|runtime| {
        render_runtime_check_result_for_locale(
            &build_runtime_check_result(kind, &runtime.topology),
            locale,
        )
    });

    let cwd = std::env::current_dir().unwrap_or_default();
    let filesystem_output = run_filesystem_checks(kind, &cwd, locale);
    let plugin_output = run_plugin_checks(check_id, kind, &cwd, locale);

    let mut parts = vec![topology_output];
    if !filesystem_output.is_empty() {
        parts.push(filesystem_output);
    }
    if !plugin_output.is_empty() {
        parts.push(plugin_output);
    }

    Ok(parts.join("\n\n"))
}

/// Runs plugin-specific checks via the `PluginRuntime` trait.
fn run_plugin_checks(
    check_id: &str,
    kind: RuntimeCheckKind,
    project_root: &std::path::Path,
    locale: &str,
) -> String {
    let plugin_check_kind = match kind {
        RuntimeCheckKind::Prepare => re_plugin::PluginCheckKind::Prepare,
        RuntimeCheckKind::Doctor => re_plugin::PluginCheckKind::Doctor,
    };

    // Find which plugin owns this check
    let surface = catalog::find_official_check_surface(check_id);
    let plugin_id = surface.map(|s| s.registration.plugin_id);

    let Some(plugin_id) = plugin_id else {
        return String::new();
    };
    let Some(runtime) = catalog::official_plugin_runtime(plugin_id) else {
        return String::new();
    };

    match runtime.run_check(check_id, plugin_check_kind, project_root) {
        Ok(result) => {
            let heading = i18n::checks_plugin_execution(locale);
            let status = if result.passed { "PASSED" } else { "FAILED" };
            let mut lines = vec![format!("--- {heading}: {check_id} [{status}] ---")];
            for finding in &result.findings {
                lines.push(format!("  {finding}"));
            }
            lines.join("\n")
        }
        Err(_) => String::new(),
    }
}

/// Core-required files (always checked, regardless of plugins).
const CORE_REQUIRED_FILES: &[&str] = &[".ralph-engine/config.yaml"];

/// Runs filesystem validations combining core requirements with
/// plugin-declared required files via auto-discovery.
fn run_filesystem_checks(_kind: RuntimeCheckKind, project_root: &Path, locale: &str) -> String {
    // Core always requires config.yaml. Plugins declare additional
    // files via required_files() — auto-discovered at runtime.
    let plugin_files = catalog::collect_required_files_from_plugins();
    let mut all_required: Vec<&str> = CORE_REQUIRED_FILES.to_vec();
    for pf in &plugin_files {
        if !all_required.contains(&pf.as_str()) {
            all_required.push(pf.as_str());
        }
    }
    let required_files = &all_required;

    let mut missing: Vec<&str> = Vec::new();
    let mut found: Vec<&str> = Vec::new();

    for path in required_files {
        if project_root.join(path).exists() {
            found.push(path);
        } else {
            missing.push(path);
        }
    }

    if found.is_empty() && missing.is_empty() {
        return String::new();
    }

    let mut lines = vec![format!("--- {} ---", i18n::checks_file_validation(locale))];

    for path in &found {
        lines.push(format!("  [OK] {path}"));
    }
    for path in &missing {
        lines.push(format!("  [{}] {path}", i18n::checks_missing_label(locale)));
    }

    if !missing.is_empty() {
        lines.push(String::new());
        lines.push(i18n::checks_scaffold_hint(locale).to_owned());
    }

    lines.join("\n")
}

fn show_check_plan(check_kind: Option<&str>, locale: &str) -> Result<String, CliError> {
    let check_id = check_kind.ok_or_else(|| {
        CliError::new(i18n::missing_argument(
            locale,
            "checks plan",
            i18n::check_id_entity_label(locale),
        ))
    })?;

    let surface = catalog::find_official_check_surface(check_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::check_entity_label(locale),
            check_id,
        ))
    })?;

    let plan = RuntimeCheckExecutionPlan::new(
        surface.registration.kind,
        surface.registration.plugin_id,
        surface.registration.load_boundary,
        runtime_hook_for_check(surface.registration.kind),
        surface.registration.runtime_hook_registered,
    );

    Ok(render_check_plan(surface.contribution, plan, locale))
}

fn show_check(check_kind: Option<&str>, locale: &str) -> Result<String, CliError> {
    let check_id = check_kind.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "checks",
            i18n::check_id_entity_label(locale),
        ))
    })?;

    if let Some(surface) = catalog::find_official_check_surface(check_id) {
        return Ok(render_check_contribution_detail(
            surface.contribution,
            surface.registration,
            locale,
        ));
    }

    let kind = parse_runtime_check_kind(check_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::check_entity_label(locale),
            check_id,
        ))
    })?;
    let checks = catalog::find_official_runtime_checks(kind);
    let contributions = catalog::find_official_check_contributions(kind);

    Ok(render_check_detail(kind, &checks, &contributions, locale))
}

fn render_check_listing(registrations: &[RuntimeCheckRegistration], locale: &str) -> String {
    let mut seen = Vec::new();
    let mut grouped_rows: Vec<Vec<String>> = Vec::new();

    for reg in registrations {
        let key = reg.kind.as_str();
        if seen.contains(&key) {
            continue;
        }
        seen.push(key);

        let all = registrations
            .iter()
            .filter(|r| r.kind.as_str() == key)
            .collect::<Vec<_>>();
        let enabled = all.iter().filter(|r| r.is_enabled()).count();

        grouped_rows.push(vec![
            key.to_owned(),
            all.len().to_string(),
            enabled.to_string(),
        ]);
    }

    let label = i18n::checks_label(locale);
    let heading = i18n::list_heading(locale, label, label, grouped_rows.len());

    if grouped_rows.is_empty() {
        return heading;
    }

    let headers = &["CHECK", "PROVIDERS", "ENABLED"];
    format!(
        "{heading}\n\n{}",
        format::render_table(headers, &grouped_rows)
    )
}

fn render_check_detail(
    kind: RuntimeCheckKind,
    checks: &[RuntimeCheckRegistration],
    contributions: &[catalog::OfficialCheckContribution],
    locale: &str,
) -> String {
    let label = i18n::check_label(locale);
    let heading = i18n::detail_heading(locale, label, label, kind.as_str());
    let providers_heading = i18n::providers_heading(locale, checks.len());

    let headers = &["ID", "PLUGIN", "NAME", "STATUS"];
    let rows: Vec<Vec<String>> = checks
        .iter()
        .map(|check| {
            let contribution = contributions
                .iter()
                .find(|c| c.descriptor.plugin_id == check.plugin_id);

            vec![
                contribution
                    .map_or(check.plugin_id, |e| e.descriptor.id)
                    .to_owned(),
                check.plugin_id.to_owned(),
                contribution
                    .map_or(check.plugin_id, |e| {
                        e.descriptor.display_name_for_locale(locale)
                    })
                    .to_owned(),
                check.activation.as_str().to_owned(),
            ]
        })
        .collect();

    format!(
        "{heading}\n{providers_heading}\n\n{}",
        format::render_table(headers, &rows)
    )
}

fn render_check_contribution_detail(
    contribution: catalog::OfficialCheckContribution,
    registration: RuntimeCheckRegistration,
    locale: &str,
) -> String {
    let asset_paths = if contribution.descriptor.has_assets() {
        contribution
            .descriptor
            .assets
            .iter()
            .map(|asset| asset.path)
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        "none".to_owned()
    };

    let heading = format!("{}:", i18n::check_label(locale));
    let pairs = vec![
        (heading.as_str(), contribution.descriptor.id.to_owned()),
        (
            i18n::name_label(locale),
            contribution
                .descriptor
                .display_name_for_locale(locale)
                .to_owned(),
        ),
        (
            i18n::summary_label(locale),
            contribution
                .descriptor
                .summary_for_locale(locale)
                .to_owned(),
        ),
        ("Plugin:", contribution.descriptor.plugin_id.to_owned()),
        ("", String::new()),
        (
            i18n::kind_label(locale),
            contribution.descriptor.kind.as_str().to_owned(),
        ),
        (
            i18n::activation_label(locale),
            registration.activation.as_str().to_owned(),
        ),
        (
            i18n::load_boundary_label(locale),
            registration.load_boundary.as_str().to_owned(),
        ),
        (
            i18n::hook_label(locale),
            registration.runtime_hook_registered.to_string(),
        ),
        (i18n::assets_label(locale), asset_paths),
    ];

    format::render_detail(&pairs)
}

fn render_check_plan(
    contribution: catalog::OfficialCheckContribution,
    plan: RuntimeCheckExecutionPlan,
    locale: &str,
) -> String {
    let pairs = vec![
        ("Runtime check plan:", contribution.descriptor.id.to_owned()),
        (
            i18n::name_label(locale),
            contribution
                .descriptor
                .display_name_for_locale(locale)
                .to_owned(),
        ),
        ("Plugin:", contribution.descriptor.plugin_id.to_owned()),
        (
            i18n::kind_label(locale),
            contribution.descriptor.kind.as_str().to_owned(),
        ),
        (
            i18n::activation_label(locale),
            contribution.activation.as_str().to_owned(),
        ),
        (
            i18n::load_boundary_label(locale),
            plan.load_boundary.as_str().to_owned(),
        ),
        (
            i18n::hook_label(locale),
            plan.runtime_hook.as_str().to_owned(),
        ),
        ("registered:", plan.runtime_hook_registered.to_string()),
    ];

    format::render_detail(&pairs)
}

#[cfg(test)]
mod tests {
    use crate::catalog::OfficialCheckContribution;
    use re_config::PluginActivation;
    use re_core::{RuntimeCheckKind, RuntimeCheckRegistration};
    use re_plugin::{
        PluginCheckAsset, PluginCheckDescriptor, PluginCheckKind, PluginLoadBoundary,
        PluginLocalizedText,
    };

    use super::{render_check_contribution_detail, render_check_detail, render_check_listing};
    use re_core::parse_runtime_check_kind;

    const CHECK_LOCALIZED_NAMES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
        "pt-br",
        "Verificação de preparo BMAD",
    )];
    const CHECK_LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
        "pt-br",
        "Executa validação tipada de preparo para workflows BMAD.",
    )];
    const CHECK_ID: &str = "fixture.bmad.prepare";
    const PRIMARY_PLUGIN_ID: &str = "fixture.bmad";
    const SECONDARY_PLUGIN_ID: &str = "fixture.other";
    const CHECK_ASSETS: &[PluginCheckAsset] = &[PluginCheckAsset::new(
        "checks/prepare.md",
        "# BMAD Prepare Check\n",
    )];

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
        let registrations = [];

        let rendered = render_check_listing(&registrations, "en");

        assert!(rendered.contains("Checks (0)"));
    }

    #[test]
    fn render_check_listing_deduplicates_check_kinds() {
        let registrations = [
            RuntimeCheckRegistration::new(
                RuntimeCheckKind::Prepare,
                PRIMARY_PLUGIN_ID,
                PluginActivation::Enabled,
                PluginLoadBoundary::InProcess,
                true,
            ),
            RuntimeCheckRegistration::new(
                RuntimeCheckKind::Prepare,
                SECONDARY_PLUGIN_ID,
                PluginActivation::Disabled,
                PluginLoadBoundary::InProcess,
                false,
            ),
        ];

        let rendered = render_check_listing(&registrations, "en");

        assert!(rendered.contains("Checks (1)"));
        assert!(rendered.contains("prepare"));
        assert!(rendered.contains("2"));
        assert!(rendered.contains("1"));
    }

    #[test]
    fn render_check_detail_is_human_readable() {
        let checks = [RuntimeCheckRegistration::new(
            RuntimeCheckKind::Prepare,
            PRIMARY_PLUGIN_ID,
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )];

        let contributions = [OfficialCheckContribution {
            descriptor: PluginCheckDescriptor::new(
                CHECK_ID,
                PRIMARY_PLUGIN_ID,
                PluginCheckKind::Prepare,
                "BMAD prepare check",
                CHECK_LOCALIZED_NAMES,
                "Runs typed prepare-time validation for BMAD workflows.",
                CHECK_LOCALIZED_SUMMARIES,
                CHECK_ASSETS,
            ),
            activation: PluginActivation::Enabled,
            load_boundary: PluginLoadBoundary::InProcess,
            runtime_hook_registered: true,
        }];

        let rendered =
            render_check_detail(RuntimeCheckKind::Prepare, &checks, &contributions, "en");

        assert!(rendered.contains("Check: prepare"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains("fixture.bmad.prepare"));
        assert!(rendered.contains("fixture.bmad"));
        assert!(rendered.contains("BMAD prepare check"));
        assert!(rendered.contains("enabled"));
    }

    #[test]
    fn render_check_detail_supports_pt_br() {
        let checks = [RuntimeCheckRegistration::new(
            RuntimeCheckKind::Prepare,
            PRIMARY_PLUGIN_ID,
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )];

        let contributions = [OfficialCheckContribution {
            descriptor: PluginCheckDescriptor::new(
                CHECK_ID,
                PRIMARY_PLUGIN_ID,
                PluginCheckKind::Prepare,
                "BMAD prepare check",
                CHECK_LOCALIZED_NAMES,
                "Runs typed prepare-time validation for BMAD workflows.",
                CHECK_LOCALIZED_SUMMARIES,
                CHECK_ASSETS,
            ),
            activation: PluginActivation::Enabled,
            load_boundary: PluginLoadBoundary::InProcess,
            runtime_hook_registered: true,
        }];

        let rendered =
            render_check_detail(RuntimeCheckKind::Prepare, &checks, &contributions, "pt-br");

        assert!(rendered.contains("Verificação: prepare"));
        assert!(rendered.contains("Provedores (1)"));
        assert!(rendered.contains("Verificação de preparo BMAD"));
    }

    #[test]
    fn render_check_contribution_detail_lists_assets() {
        let rendered = render_check_contribution_detail(
            OfficialCheckContribution {
                descriptor: PluginCheckDescriptor::new(
                    CHECK_ID,
                    PRIMARY_PLUGIN_ID,
                    PluginCheckKind::Prepare,
                    "BMAD prepare check",
                    CHECK_LOCALIZED_NAMES,
                    "Runs typed prepare-time validation for BMAD workflows.",
                    CHECK_LOCALIZED_SUMMARIES,
                    CHECK_ASSETS,
                ),
                activation: PluginActivation::Enabled,
                load_boundary: PluginLoadBoundary::InProcess,
                runtime_hook_registered: true,
            },
            RuntimeCheckRegistration::new(
                RuntimeCheckKind::Prepare,
                PRIMARY_PLUGIN_ID,
                PluginActivation::Enabled,
                PluginLoadBoundary::InProcess,
                true,
            ),
            "en",
        );

        assert!(rendered.contains("checks/prepare.md"));
    }

    #[test]
    fn filesystem_checks_reports_missing_files_for_prepare() {
        let tmp = std::env::temp_dir().join("re-fs-check-test-prepare");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).ok();

        let output = super::run_filesystem_checks(RuntimeCheckKind::Prepare, &tmp, "en");
        assert!(output.contains("[MISSING]"));
        assert!(output.contains(".ralph-engine/config.yaml"));
        assert!(output.contains("Hint:"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn filesystem_checks_reports_ok_when_files_exist() {
        let tmp = std::env::temp_dir().join("re-fs-check-test-ok");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::write(tmp.join(".ralph-engine/config.yaml"), "# test").ok();
        // Plugins may declare additional required files via auto-discovery.
        // Create all plugin-declared files to avoid MISSING results.
        for file in crate::catalog::collect_required_files_from_plugins() {
            let path = tmp.join(&file);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).ok();
            }
            std::fs::write(&path, "# test").ok();
        }

        let output = super::run_filesystem_checks(RuntimeCheckKind::Prepare, &tmp, "en");
        assert!(output.contains("[OK]"));
        assert!(!output.contains("[MISSING]"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn filesystem_checks_supports_pt_br_locale() {
        let tmp = std::env::temp_dir().join("re-fs-check-test-ptbr");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).ok();

        let output = super::run_filesystem_checks(RuntimeCheckKind::Prepare, &tmp, "pt-br");
        assert!(output.contains("[FALTANDO]"));
        assert!(output.contains("Dica:"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn filesystem_checks_doctor_requires_both_config_and_prompt() {
        let tmp = std::env::temp_dir().join("re-fs-check-test-doctor");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::write(tmp.join(".ralph-engine/config.yaml"), "# test").ok();

        let output = super::run_filesystem_checks(RuntimeCheckKind::Doctor, &tmp, "en");
        assert!(output.contains("[OK] .ralph-engine/config.yaml"));
        assert!(output.contains("[MISSING] .ralph-engine/prompt.md"));

        let _ = std::fs::remove_dir_all(&tmp);
    }
}
