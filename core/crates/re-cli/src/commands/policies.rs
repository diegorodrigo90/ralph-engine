//! Policy command handlers.

use std::path::Path;

use re_core::{
    RuntimePolicyEnforcementPlan, build_runtime_policy_result, policy_runtime_hook,
    render_runtime_policy_result_for_locale,
};

use super::format;
use crate::commands::embedded_assets::{MaterializedAsset, materialize_assets};
use crate::commands::runtime_state::with_official_runtime_snapshot;
use crate::{CliError, catalog, i18n};

use catalog::OfficialPolicyContribution;

/// Executes the policies command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_policy_listing(
            &catalog::official_policy_contributions(),
            locale,
        )),
        Some("show") => show_policy(args.get(1).map(String::as_str), locale),
        Some("plan") => show_policy_plan(args.get(1).map(String::as_str), locale),
        Some("run") => run_policy(args.get(1).map(String::as_str), locale),
        Some("asset") => show_policy_asset(
            args.get(1).map(String::as_str),
            args.get(2).map(String::as_str),
            locale,
        ),
        Some("materialize") => materialize_policy(
            args.get(1).map(String::as_str),
            args.get(2).map(String::as_str),
            locale,
        ),
        Some(other) => Err(CliError::usage(i18n::unknown_subcommand(
            locale, "policies", other,
        ))),
    }
}

fn show_policy(policy_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let policy_id = policy_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "policies",
            i18n::policy_id_entity_label(locale),
        ))
    })?;
    let policy = catalog::find_official_policy_contribution(policy_id).ok_or_else(|| {
        CliError::usage(i18n::unknown_entity(
            locale,
            i18n::policy_entity_label(locale),
            policy_id,
        ))
    })?;

    Ok(render_policy_detail(policy, locale))
}

fn run_policy(policy_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let policy_id = policy_id.ok_or_else(|| {
        CliError::new(i18n::missing_argument(
            locale,
            "policies run",
            i18n::policy_id_entity_label(locale),
        ))
    })?;
    let policy = catalog::find_official_policy_contribution(policy_id).ok_or_else(|| {
        CliError::usage(i18n::unknown_entity(
            locale,
            i18n::policy_entity_label(locale),
            policy_id,
        ))
    })?;

    let topology_output = with_official_runtime_snapshot(|runtime| {
        let result = build_runtime_policy_result(policy.descriptor.id, &runtime.topology)
            .ok_or_else(|| {
                CliError::usage(i18n::unknown_entity(
                    locale,
                    i18n::policy_entity_label(locale),
                    policy.descriptor.id,
                ))
            })?;

        Ok(render_runtime_policy_result_for_locale(&result, locale))
    })?;

    let cwd = std::env::current_dir().unwrap_or_default();
    let filesystem_output = run_policy_filesystem_checks(&policy, &cwd, locale);

    if filesystem_output.is_empty() {
        Ok(topology_output)
    } else {
        Ok(format!("{topology_output}\n\n{filesystem_output}"))
    }
}

/// Runs filesystem validations for a policy against a project root.
///
/// Checks whether the policy asset files have been materialized in the
/// project directory, giving operators confidence that the policy
/// expectations are visible to contributors.
fn run_policy_filesystem_checks(
    policy: &OfficialPolicyContribution,
    project_root: &Path,
    locale: &str,
) -> String {
    let assets = policy.descriptor.assets;
    if assets.is_empty() {
        return String::new();
    }

    let mut missing: Vec<&str> = Vec::new();
    let mut found: Vec<&str> = Vec::new();

    for asset in assets {
        let expected = project_root
            .join(".ralph-engine")
            .join("policies")
            .join(asset.path);
        if expected.exists() {
            found.push(asset.path);
        } else {
            missing.push(asset.path);
        }
    }

    let mut lines = vec![format!(
        "--- {} ---",
        i18n::policies_file_validation(locale)
    )];

    for path in &found {
        lines.push(format!("  [OK] .ralph-engine/policies/{path}"));
    }
    for path in &missing {
        lines.push(format!(
            "  [{}] .ralph-engine/policies/{path}",
            i18n::policies_missing_label(locale)
        ));
    }

    if !missing.is_empty() {
        lines.push(String::new());
        lines.push(i18n::policies_materialize_hint(
            locale,
            policy.descriptor.id,
        ));
    }

    lines.join("\n")
}

fn show_policy_plan(policy_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let policy_id = policy_id.ok_or_else(|| {
        CliError::new(i18n::missing_argument(
            locale,
            "policies plan",
            i18n::policy_id_entity_label(locale),
        ))
    })?;
    let policy = catalog::find_official_policy_contribution(policy_id).ok_or_else(|| {
        CliError::usage(i18n::unknown_entity(
            locale,
            i18n::policy_entity_label(locale),
            policy_id,
        ))
    })?;

    let plan = RuntimePolicyEnforcementPlan::new(
        policy.descriptor.id,
        policy.descriptor.plugin_id,
        policy.load_boundary,
        policy_runtime_hook(),
        policy.enforcement_hook_registered,
    );

    Ok(render_policy_plan(policy, plan, locale))
}

fn show_policy_asset(
    policy_id: Option<&str>,
    asset_path: Option<&str>,
    locale: &str,
) -> Result<String, CliError> {
    let policy_id = policy_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "policies asset",
            i18n::policy_id_entity_label(locale),
        ))
    })?;
    let asset_path = asset_path
        .ok_or_else(|| CliError::new(i18n::missing_asset_path(locale, "policies asset")))?;
    let policy = catalog::find_official_policy_contribution(policy_id).ok_or_else(|| {
        CliError::usage(i18n::unknown_entity(
            locale,
            i18n::policy_entity_label(locale),
            policy_id,
        ))
    })?;
    let asset = policy
        .descriptor
        .assets
        .iter()
        .find(|asset| asset.path == asset_path)
        .ok_or_else(|| CliError::usage(i18n::unknown_policy_asset(locale, asset_path)))?;

    Ok(asset.contents.to_owned())
}

fn materialize_policy(
    policy_id: Option<&str>,
    output_dir: Option<&str>,
    locale: &str,
) -> Result<String, CliError> {
    let policy_id = policy_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "policies materialize",
            i18n::policy_id_entity_label(locale),
        ))
    })?;
    let output_dir = output_dir.ok_or_else(|| {
        CliError::new(i18n::missing_output_directory(
            locale,
            "policies materialize",
        ))
    })?;
    let policy = catalog::find_official_policy_contribution(policy_id).ok_or_else(|| {
        CliError::usage(i18n::unknown_entity(
            locale,
            i18n::policy_entity_label(locale),
            policy_id,
        ))
    })?;
    let assets = policy
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

fn render_policy_listing(registrations: &[OfficialPolicyContribution], locale: &str) -> String {
    let heading = i18n::list_heading(
        locale,
        i18n::policies_label(locale),
        i18n::policies_label(locale),
        registrations.len(),
    );

    let headers = &["ID", "NAME", "PLUGIN", "STATUS"];
    let rows: Vec<Vec<String>> = registrations
        .iter()
        .map(|r| {
            vec![
                r.descriptor.id.to_owned(),
                r.descriptor.display_name_for_locale(locale).to_owned(),
                r.descriptor.plugin_id.to_owned(),
                r.activation.as_str().to_owned(),
            ]
        })
        .collect();

    if rows.is_empty() {
        return heading;
    }

    format!("{heading}\n\n{}", format::render_table(headers, &rows))
}

fn render_policy_detail(policy: OfficialPolicyContribution, locale: &str) -> String {
    let asset_paths = if policy.descriptor.has_assets() {
        policy
            .descriptor
            .assets
            .iter()
            .map(|asset| asset.path)
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        "none".to_owned()
    };

    let heading = format!("{}:", i18n::policy_label(locale));
    let pairs = vec![
        (heading.as_str(), policy.descriptor.id.to_owned()),
        (
            i18n::name_label(locale),
            policy.descriptor.display_name_for_locale(locale).to_owned(),
        ),
        (
            i18n::summary_label(locale),
            policy.descriptor.summary_for_locale(locale).to_owned(),
        ),
        (
            i18n::provider_label(locale),
            policy.descriptor.plugin_id.to_owned(),
        ),
        ("", String::new()),
        (
            i18n::activation_label(locale),
            policy.activation.as_str().to_owned(),
        ),
        (
            i18n::load_boundary_label(locale),
            policy.load_boundary.as_str().to_owned(),
        ),
        (
            i18n::policy_enforcement_hook_label(locale),
            if policy.enforcement_hook_registered {
                "policy_enforcement"
            } else {
                "missing"
            }
            .to_owned(),
        ),
        (i18n::assets_label(locale), asset_paths),
    ];

    format::render_detail(&pairs)
}

fn render_policy_plan(
    policy: OfficialPolicyContribution,
    plan: RuntimePolicyEnforcementPlan,
    locale: &str,
) -> String {
    let pairs = vec![
        ("Policy enforcement plan:", policy.descriptor.id.to_owned()),
        (
            i18n::name_label(locale),
            policy.descriptor.display_name_for_locale(locale).to_owned(),
        ),
        ("Plugin:", policy.descriptor.plugin_id.to_owned()),
        (
            i18n::activation_label(locale),
            policy.activation.as_str().to_owned(),
        ),
        (
            i18n::load_boundary_label(locale),
            plan.load_boundary.as_str().to_owned(),
        ),
        (
            i18n::policy_enforcement_hook_label(locale),
            plan.enforcement_hook.as_str().to_owned(),
        ),
        ("registered:", plan.enforcement_hook_registered.to_string()),
    ];

    format::render_detail(&pairs)
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_core::{RuntimePolicyEnforcementPlan, policy_runtime_hook};
    use re_plugin::{
        PluginLoadBoundary, PluginLocalizedText, PluginPolicyAsset, PluginPolicyDescriptor,
    };

    use super::{
        OfficialPolicyContribution, render_policy_detail, render_policy_listing, render_policy_plan,
    };

    const LOCALIZED_NAMES: &[PluginLocalizedText] =
        &[PluginLocalizedText::new("pt-br", "Guardrails TDD estrito")];
    const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
        "pt-br",
        "Política oficial com guardrails estritos de TDD.",
    )];
    const POLICY_ID: &str = "fixture.strict.guardrails";
    const PLUGIN_ID: &str = "fixture.strict";
    const POLICY_ASSETS: &[PluginPolicyAsset] = &[PluginPolicyAsset::new(
        "policies/guardrails.md",
        "# guardrails\n",
    )];

    fn policy_descriptor() -> PluginPolicyDescriptor {
        PluginPolicyDescriptor::new(
            POLICY_ID,
            PLUGIN_ID,
            "TDD strict guardrails",
            LOCALIZED_NAMES,
            "Official policy with strict TDD guardrails.",
            LOCALIZED_SUMMARIES,
            POLICY_ASSETS,
        )
    }

    #[test]
    fn render_policy_listing_handles_empty_sets() {
        let registrations = [];

        let rendered = render_policy_listing(&registrations, "en");

        assert!(rendered.contains("Policies (0)"));
    }

    #[test]
    fn render_policy_detail_is_human_readable() {
        let rendered = render_policy_detail(
            OfficialPolicyContribution {
                descriptor: policy_descriptor(),
                activation: PluginActivation::Enabled,
                load_boundary: PluginLoadBoundary::InProcess,
                enforcement_hook_registered: true,
            },
            "en",
        );

        assert!(rendered.contains("fixture.strict.guardrails"));
        assert!(rendered.contains("TDD strict guardrails"));
        assert!(rendered.contains("fixture.strict"));
        assert!(rendered.contains("enabled"));
        assert!(rendered.contains("policy_enforcement"));
        assert!(rendered.contains("policies/guardrails.md"));
    }

    #[test]
    fn render_policy_detail_supports_pt_br() {
        let rendered = render_policy_detail(
            OfficialPolicyContribution {
                descriptor: policy_descriptor(),
                activation: PluginActivation::Enabled,
                load_boundary: PluginLoadBoundary::InProcess,
                enforcement_hook_registered: true,
            },
            "pt-br",
        );

        assert!(rendered.contains("fixture.strict.guardrails"));
        assert!(rendered.contains("Guardrails TDD estrito"));
        assert!(rendered.contains("fixture.strict"));
        assert!(rendered.contains("policy_enforcement"));
    }

    #[test]
    fn render_policy_plan_is_human_readable() {
        let rendered = render_policy_plan(
            OfficialPolicyContribution {
                descriptor: policy_descriptor(),
                activation: PluginActivation::Enabled,
                load_boundary: PluginLoadBoundary::InProcess,
                enforcement_hook_registered: true,
            },
            RuntimePolicyEnforcementPlan::new(
                POLICY_ID,
                PLUGIN_ID,
                PluginLoadBoundary::InProcess,
                policy_runtime_hook(),
                true,
            ),
            "en",
        );

        assert!(rendered.contains("Policy enforcement plan:"));
        assert!(rendered.contains("fixture.strict.guardrails"));
        assert!(rendered.contains("fixture.strict"));
        assert!(rendered.contains("policy_enforcement"));
    }

    #[test]
    fn policy_filesystem_checks_reports_missing_when_not_materialized() {
        let tmp = std::env::temp_dir().join("re-policy-fs-test-missing");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).ok();

        let policy = OfficialPolicyContribution {
            descriptor: policy_descriptor(),
            activation: PluginActivation::Enabled,
            load_boundary: PluginLoadBoundary::InProcess,
            enforcement_hook_registered: true,
        };

        let output = super::run_policy_filesystem_checks(&policy, &tmp, "en");
        assert!(output.contains("[MISSING]"));
        assert!(output.contains("policies/guardrails.md"));
        assert!(output.contains("Hint:"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn policy_filesystem_checks_reports_ok_when_materialized() {
        let tmp = std::env::temp_dir().join("re-policy-fs-test-ok");
        let _ = std::fs::remove_dir_all(&tmp);
        let policy_dir = tmp.join(".ralph-engine/policies/policies");
        std::fs::create_dir_all(&policy_dir).ok();
        std::fs::write(policy_dir.join("guardrails.md"), "# test").ok();

        let policy = OfficialPolicyContribution {
            descriptor: policy_descriptor(),
            activation: PluginActivation::Enabled,
            load_boundary: PluginLoadBoundary::InProcess,
            enforcement_hook_registered: true,
        };

        let output = super::run_policy_filesystem_checks(&policy, &tmp, "en");
        assert!(output.contains("[OK]"));
        assert!(!output.contains("[MISSING]"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn policy_filesystem_checks_supports_pt_br() {
        let tmp = std::env::temp_dir().join("re-policy-fs-test-ptbr");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).ok();

        let policy = OfficialPolicyContribution {
            descriptor: policy_descriptor(),
            activation: PluginActivation::Enabled,
            load_boundary: PluginLoadBoundary::InProcess,
            enforcement_hook_registered: true,
        };

        let output = super::run_policy_filesystem_checks(&policy, &tmp, "pt-br");
        assert!(output.contains("[FALTANDO]"));
        assert!(output.contains("Dica:"));

        let _ = std::fs::remove_dir_all(&tmp);
    }
}
