//! Prompt provider command handlers.

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
    let asset_path = asset_path.ok_or_else(|| {
        CliError::new("subcommand `prompts asset` requires an asset path".to_owned())
    })?;
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
        .ok_or_else(|| {
            CliError::new(if i18n::is_pt_br(locale) {
                format!("asset de prompt desconhecido: {asset_path}")
            } else {
                format!("unknown prompt asset: {asset_path}")
            })
        })?;

    Ok(asset.contents.to_owned())
}

fn render_prompt_listing(registrations: &[OfficialPromptContribution], locale: &str) -> String {
    let mut lines = Vec::with_capacity(registrations.len() + 1);
    lines.push(i18n::list_heading(
        locale,
        "Prompts",
        "Prompts",
        registrations.len(),
    ));

    for registration in registrations {
        lines.push(format!(
            "- {} | {} | plugin={} | activation={}",
            registration.descriptor.id,
            registration.descriptor.display_name_for_locale(locale),
            registration.descriptor.plugin_id,
            registration.activation.as_str(),
        ));
    }

    lines.join("\n")
}

fn render_prompt_detail(prompt: OfficialPromptContribution, locale: &str) -> String {
    let name_label = if i18n::is_pt_br(locale) {
        "Nome"
    } else {
        "Name"
    };
    let summary_label = if i18n::is_pt_br(locale) {
        "Resumo"
    } else {
        "Summary"
    };
    let hook_label = if i18n::is_pt_br(locale) {
        "Hook de runtime"
    } else {
        "Runtime hook"
    };
    let assets_label = "Assets";
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

    format!(
        "Prompt: {}\n{name_label}: {}\n{summary_label}: {}\nPlugin: {}\n{}: {}\n{}: {}\n{hook_label}: {}\n{assets_label}: {}",
        prompt.descriptor.id,
        prompt.descriptor.display_name_for_locale(locale),
        prompt.descriptor.summary_for_locale(locale),
        prompt.descriptor.plugin_id,
        i18n::activation_label(locale),
        prompt.activation.as_str(),
        i18n::load_boundary_label(locale),
        prompt.load_boundary.as_str(),
        if prompt.prompt_hook_registered {
            "prompt_assembly"
        } else {
            "missing"
        },
        asset_paths,
    )
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
    const PROMPT_ASSETS: &[PluginPromptAsset] = &[PluginPromptAsset::new(
        "prompts/workflow.md",
        "# workflow\n",
    )];

    fn prompt_descriptor() -> PluginPromptDescriptor {
        PluginPromptDescriptor::new(
            "official.bmad.workflow",
            "official.bmad",
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

        assert_eq!(rendered, "Prompts (0)");
    }

    #[test]
    fn render_prompt_listing_handles_empty_sets_in_pt_br() {
        let registrations = [];

        let rendered = render_prompt_listing(&registrations, "pt-br");

        assert_eq!(rendered, "Prompts (0)");
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

        assert!(rendered.contains("Prompt: official.bmad.workflow"));
        assert!(rendered.contains("Name: BMAD workflow prompt"));
        assert!(rendered.contains("Plugin: official.bmad"));
        assert!(rendered.contains("Activation: disabled"));
        assert!(rendered.contains("Runtime hook: prompt_assembly"));
        assert!(rendered.contains("Assets: prompts/workflow.md"));
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

        assert!(rendered.contains("Prompt: official.bmad.workflow"));
        assert!(rendered.contains("Nome: Prompt de workflow BMAD"));
        assert!(rendered.contains("Resumo: Pacote de prompts para montar workflows BMAD."));
        assert!(rendered.contains("Plugin: official.bmad"));
        assert!(rendered.contains("Assets: prompts/workflow.md"));
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
}
