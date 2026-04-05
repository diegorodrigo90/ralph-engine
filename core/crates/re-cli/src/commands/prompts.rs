//! Prompt provider command handlers.

use std::path::Path;

use super::embedded_assets::{MaterializedAsset, materialize_assets};
use super::format;
use crate::{CliError, catalog, i18n};

use catalog::OfficialPromptContribution;

/// Executes the prompts command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_prompt_listing(
            &catalog::official_prompt_contributions(),
            locale,
        )),
        Some("show") => show_prompt(args.get(1).map(String::as_str), locale),
        Some("asset") => show_prompt_asset(
            args.get(1).map(String::as_str),
            args.get(2).map(String::as_str),
            locale,
        ),
        Some("materialize") => materialize_prompt(
            args.get(1).map(String::as_str),
            args.get(2).map(String::as_str),
            locale,
        ),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "prompts", other,
        ))),
    }
}

fn show_prompt(prompt_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let prompt_id = prompt_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "prompts",
            i18n::prompt_id_entity_label(locale),
        ))
    })?;
    let prompt = catalog::find_official_prompt_contribution(prompt_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::prompt_entity_label(locale),
            prompt_id,
        ))
    })?;

    Ok(render_prompt_detail(prompt, locale))
}

fn show_prompt_asset(
    prompt_id: Option<&str>,
    asset_path: Option<&str>,
    locale: &str,
) -> Result<String, CliError> {
    let prompt_id = prompt_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "prompts asset",
            i18n::prompt_id_entity_label(locale),
        ))
    })?;
    let asset_path = asset_path
        .ok_or_else(|| CliError::new(i18n::missing_asset_path(locale, "prompts asset")))?;
    let prompt = catalog::find_official_prompt_contribution(prompt_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::prompt_entity_label(locale),
            prompt_id,
        ))
    })?;
    let asset = prompt
        .descriptor
        .assets
        .iter()
        .find(|asset| asset.path == asset_path)
        .ok_or_else(|| CliError::new(i18n::unknown_prompt_asset(locale, asset_path)))?;

    Ok(asset.contents.to_owned())
}

fn materialize_prompt(
    prompt_id: Option<&str>,
    output_dir: Option<&str>,
    locale: &str,
) -> Result<String, CliError> {
    let prompt_id = prompt_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "prompts materialize",
            i18n::prompt_id_entity_label(locale),
        ))
    })?;
    let output_dir = output_dir.ok_or_else(|| {
        CliError::new(i18n::missing_output_directory(
            locale,
            "prompts materialize",
        ))
    })?;
    let prompt = catalog::find_official_prompt_contribution(prompt_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::prompt_entity_label(locale),
            prompt_id,
        ))
    })?;
    let assets = prompt
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

fn render_prompt_listing(registrations: &[OfficialPromptContribution], locale: &str) -> String {
    let heading = i18n::list_heading(locale, "Prompts", "Prompts", registrations.len());

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

fn render_prompt_detail(prompt: OfficialPromptContribution, locale: &str) -> String {
    let asset_paths = if prompt.descriptor.has_assets() {
        prompt
            .descriptor
            .assets
            .iter()
            .map(|asset| asset.path)
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        "none".to_owned()
    };

    let pairs = vec![
        ("Prompt:", prompt.descriptor.id.to_owned()),
        (
            i18n::name_label(locale),
            prompt.descriptor.display_name_for_locale(locale).to_owned(),
        ),
        (
            i18n::summary_label(locale),
            prompt.descriptor.summary_for_locale(locale).to_owned(),
        ),
        ("Plugin:", prompt.descriptor.plugin_id.to_owned()),
        ("", String::new()),
        (
            i18n::activation_label(locale),
            prompt.activation.as_str().to_owned(),
        ),
        (
            i18n::load_boundary_label(locale),
            prompt.load_boundary.as_str().to_owned(),
        ),
        (
            i18n::hook_label(locale),
            if prompt.prompt_hook_registered {
                "prompt_assembly"
            } else {
                "missing"
            }
            .to_owned(),
        ),
        (i18n::assets_label(locale), asset_paths),
    ];

    format::render_detail(&pairs)
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_plugin::{
        PluginLoadBoundary, PluginLocalizedText, PluginPromptAsset, PluginPromptDescriptor,
    };

    use super::{OfficialPromptContribution, execute, render_prompt_detail, render_prompt_listing};

    const LOCALIZED_NAMES: &[PluginLocalizedText] =
        &[PluginLocalizedText::new("pt-br", "Prompt de workflow BMAD")];
    const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
        "pt-br",
        "Pacote de prompts para montar workflows BMAD.",
    )];
    const PROMPT_ID: &str = "fixture.prompts.workflow";
    const PLUGIN_ID: &str = "fixture.prompts";
    const PROMPT_ASSETS: &[PluginPromptAsset] = &[PluginPromptAsset::new(
        "prompts/workflow.md",
        "# workflow\n",
    )];

    fn prompt_descriptor() -> PluginPromptDescriptor {
        PluginPromptDescriptor::new(
            PROMPT_ID,
            PLUGIN_ID,
            "BMAD workflow prompt",
            LOCALIZED_NAMES,
            "Prompt bundle for BMAD workflow assembly.",
            LOCALIZED_SUMMARIES,
            PROMPT_ASSETS,
        )
    }

    #[test]
    fn render_prompt_listing_handles_empty_sets() {
        let registrations = [];

        let rendered = render_prompt_listing(&registrations, "en");

        assert!(rendered.contains("Prompts (0)"));
    }

    #[test]
    fn render_prompt_listing_handles_empty_sets_in_pt_br() {
        let registrations = [];

        let rendered = render_prompt_listing(&registrations, "pt-br");

        assert!(rendered.contains("Prompts (0)"));
    }

    #[test]
    fn render_prompt_detail_is_human_readable() {
        let rendered = render_prompt_detail(
            OfficialPromptContribution {
                descriptor: prompt_descriptor(),
                activation: PluginActivation::Disabled,
                load_boundary: PluginLoadBoundary::InProcess,
                prompt_hook_registered: true,
            },
            "en",
        );

        assert!(rendered.contains("fixture.prompts.workflow"));
        assert!(rendered.contains("BMAD workflow prompt"));
        assert!(rendered.contains("fixture.prompts"));
        assert!(rendered.contains("disabled"));
        assert!(rendered.contains("prompt_assembly"));
        assert!(rendered.contains("prompts/workflow.md"));
    }

    #[test]
    fn render_prompt_detail_supports_pt_br() {
        let rendered = render_prompt_detail(
            OfficialPromptContribution {
                descriptor: prompt_descriptor(),
                activation: PluginActivation::Disabled,
                load_boundary: PluginLoadBoundary::InProcess,
                prompt_hook_registered: true,
            },
            "pt-br",
        );

        assert!(rendered.contains("fixture.prompts.workflow"));
        assert!(rendered.contains("Prompt de workflow BMAD"));
        assert!(rendered.contains("Pacote de prompts para montar workflows BMAD."));
        assert!(rendered.contains("fixture.prompts"));
        assert!(rendered.contains("prompts/workflow.md"));
    }

    #[test]
    fn execute_prompt_asset_returns_embedded_contents() {
        let output = execute(
            &[
                "asset".to_owned(),
                "official.bmad.workflow".to_owned(),
                "prompts/workflow.md".to_owned(),
            ],
            "en",
        );

        assert!(output.is_ok());
        assert!(
            output
                .unwrap_or_default()
                .contains("# Ralph Engine — BMAD Template")
        );
    }

    #[test]
    fn execute_prompt_materialize_writes_embedded_assets() {
        let base = std::env::temp_dir().join(format!(
            "ralph-engine-prompt-materialize-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&base);

        let output = execute(
            &[
                "materialize".to_owned(),
                "official.bmad.workflow".to_owned(),
                base.display().to_string(),
            ],
            "en",
        );

        assert!(output.is_ok());
        let rendered = output.unwrap_or_default();
        assert!(rendered.contains("Materialized assets (1)"));
        assert!(base.join("prompts/workflow.md").exists());

        let _ = std::fs::remove_dir_all(base);
    }
}
