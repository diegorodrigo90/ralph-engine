//! Interactive project initialization command.
//!
//! Creates a `.ralph-engine/` directory with config, hooks, and prompt
//! files based on user-selected template and plugin preferences.
//! Templates and plugins are auto-discovered from the official catalog.

use std::io::Write as _;
use std::path::Path;

use crate::{CliError, catalog, i18n};

use super::embedded_assets::{MaterializedAsset, materialize_assets};

/// Executes the init command.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        Some("--help" | "-h") => Ok(locale_str!(
            locale,
            "Usage: ralph-engine init [directory]\n\n\
             Interactively initialize a new Ralph Engine project.\n\
             Creates .ralph-engine/ with config, hooks, and prompt files.\n\n\
             Templates and plugins are auto-discovered from the catalog.",
            "Uso: ralph-engine init [diretório]\n\n\
             Inicializa interativamente um novo projeto Ralph Engine.\n\
             Cria .ralph-engine/ com config, hooks e arquivos de prompt.\n\n\
             Templates e plugins são auto-descobertos do catálogo."
        )
        .to_owned()),
        _ => run_interactive_init(args.first().map(String::as_str).unwrap_or("."), locale),
    }
}

/// Reads a line from stdin, trimmed.
fn read_line() -> String {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().to_owned()
}

/// Prompts user and returns trimmed input.
fn prompt(question: &str) -> String {
    eprint!("{question} ");
    let _ = std::io::stderr().flush();
    read_line()
}

/// Prompts yes/no with default.
fn confirm(question: &str, default: bool) -> bool {
    let suffix = if default { "(Y/n)" } else { "(y/N)" };
    let answer = prompt(&format!("{question} {suffix}"));
    if answer.is_empty() {
        return default;
    }
    matches!(answer.to_lowercase().as_str(), "y" | "yes" | "s" | "sim")
}

/// Runs the interactive init flow with auto-discovered templates and plugins.
fn run_interactive_init(target_dir: &str, locale: &str) -> Result<String, CliError> {
    let target = Path::new(target_dir);
    let re_dir = target.join(".ralph-engine");
    let mut output = Vec::new();

    output.push(format!("\n  {}\n", re_core::banner()));

    // ── Check existing directory ─────────────────────────────────────
    if re_dir.exists() {
        eprintln!(
            "{}",
            locale_str!(
                locale,
                "  .ralph-engine/ already exists.\n  \
                 WARNING: Overwriting will DELETE all current configuration.",
                "  .ralph-engine/ já existe.\n  \
                 AVISO: Sobrescrever vai APAGAR toda configuração atual."
            )
        );
        if !confirm(
            locale_str!(locale, "  Overwrite?", "  Sobrescrever?"),
            false,
        ) {
            return Ok(locale_str!(locale, "  Cancelled.", "  Cancelado.").to_owned());
        }
        std::fs::remove_dir_all(&re_dir)
            .map_err(|err| CliError::new(format!("Failed to remove .ralph-engine/: {err}")))?;
    }

    // ── Auto-discover templates from catalog ─────────────────────────
    let templates = catalog::official_template_contributions();
    if templates.is_empty() {
        return Err(CliError::new(
            "No templates found in catalog. Cannot initialize.".to_owned(),
        ));
    }

    eprintln!(
        "{}",
        locale_str!(locale, "  Select a template:", "  Selecione um template:")
    );
    for (i, t) in templates.iter().enumerate() {
        let name = t.descriptor.display_name_for_locale(locale);
        let summary = t.descriptor.summary_for_locale(locale);
        eprintln!("    {}) {name} — {summary}", i + 1);
    }

    let template_input = prompt(&format!("  Template (1-{}):", templates.len()));
    let template_idx: usize = template_input
        .parse::<usize>()
        .unwrap_or(1)
        .saturating_sub(1)
        .min(templates.len() - 1);
    let selected = &templates[template_idx];

    // ── Materialize template assets ──────────────────────────────────
    let assets: Vec<MaterializedAsset<'_>> = selected
        .descriptor
        .assets
        .iter()
        .map(|a| MaterializedAsset {
            path: a.path,
            contents: a.contents,
        })
        .collect();

    let materialize_result = materialize_assets(&assets, target, locale)?;
    output.push(materialize_result);

    // ── Auto-discover optional plugins ───────────────────────────────
    // Show plugins that are NOT already enabled by the template config.
    let all_plugins = catalog::official_runtime_plugins();
    // Template config already enables some plugins (basic, bmad, etc).
    // Read the generated config to find which are already there.
    let config_path = target.join(".ralph-engine/config.yaml");
    let config_content = std::fs::read_to_string(&config_path).unwrap_or_default();
    let optional: Vec<_> = all_plugins
        .iter()
        .filter(|p| !config_content.contains(p.descriptor.id))
        .collect();

    if !optional.is_empty() {
        eprintln!(
            "\n{}",
            locale_str!(
                locale,
                "  Enable additional plugins? (comma-separated numbers, or Enter to skip)",
                "  Ativar plugins adicionais? (números separados por vírgula, ou Enter para pular)"
            )
        );
        for (i, p) in optional.iter().enumerate() {
            let name = p.descriptor.display_name_for_locale(locale);
            let summary = p.descriptor.summary_for_locale(locale);
            eprintln!("    {}) {name} — {summary}", i + 1);
        }

        let plugins_input = prompt("  Plugins:");
        let selected_ids: Vec<&str> = if plugins_input.is_empty() {
            Vec::new()
        } else {
            plugins_input
                .split(',')
                .filter_map(|s| {
                    let idx: usize = s.trim().parse().ok()?;
                    optional.get(idx.checked_sub(1)?).map(|p| p.descriptor.id)
                })
                .collect()
        };

        if !selected_ids.is_empty() {
            if let Ok(mut config) = std::fs::read_to_string(&config_path) {
                config.push_str("\n# Plugins enabled during init\n");
                for id in &selected_ids {
                    config.push_str(&format!("  - id: {id}\n    activation: enabled\n"));
                }
                let _ = std::fs::write(&config_path, config);
            }
            output.push(format!(
                "  {} {} {}",
                locale_str!(locale, "Enabled", "Ativou"),
                selected_ids.len(),
                locale_str!(locale, "additional plugins", "plugins adicionais")
            ));
        }
    }

    // ── Plugin init contributions (auto-discovery) ────────────────────
    // Plugins can contribute additional init steps via init_contributions().
    // This enables third-party plugins to add questions, config, or files.
    let init_contributions = catalog::collect_init_contributions_from_plugins();
    if !init_contributions.is_empty() {
        let config_path_for_contrib = target.join(".ralph-engine/config.yaml");
        for contrib in &init_contributions {
            eprintln!("  {} — {}", contrib.label, contrib.description);
            if confirm(
                &format!("  {}?", locale_str!(locale, "Enable", "Ativar")),
                true,
            ) {
                if let Some(snippet) = &contrib.config_snippet
                    && let Ok(mut config) = std::fs::read_to_string(&config_path_for_contrib)
                {
                    config.push_str(&format!("\n{snippet}\n"));
                    let _ = std::fs::write(&config_path_for_contrib, config);
                }
                for (file_path, file_contents) in &contrib.files {
                    let full_path = target.join(file_path);
                    if let Some(parent) = full_path.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    let _ = std::fs::write(&full_path, file_contents);
                    output.push(format!(
                        "  {} {file_path}",
                        locale_str!(locale, "Created", "Criado"),
                    ));
                }
            }
        }
    }

    // ── Done ─────────────────────────────────────────────────────────
    output.push(format!(
        "\n  {} 'ralph-engine doctor' {}",
        locale_str!(locale, "Done! Run", "Pronto! Execute"),
        locale_str!(locale, "to verify your setup.", "para verificar.")
    ));

    Ok(output.join("\n"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn help_flag_returns_usage_en() {
        let result = execute(&["--help".to_owned()], "en");
        assert!(result.is_ok());
        let text = result.unwrap();
        assert!(text.contains("ralph-engine init"));
        assert!(text.contains("auto-discovered"));
    }

    #[test]
    fn help_flag_returns_usage_pt_br() {
        let result = execute(&["-h".to_owned()], "pt-br");
        assert!(result.is_ok());
        let text = result.unwrap();
        assert!(text.contains("ralph-engine init"));
        assert!(text.contains("auto-descobertos"));
    }

    #[test]
    fn catalog_has_templates_for_init() {
        let templates = catalog::official_template_contributions();
        assert!(
            templates.len() >= 3,
            "Expected at least 3 templates, found {}",
            templates.len()
        );
    }

    #[test]
    fn catalog_has_plugins_for_init() {
        let plugins = catalog::official_runtime_plugins();
        assert!(
            plugins.len() >= 10,
            "Expected at least 10 plugins, found {}",
            plugins.len()
        );
    }

    // Note: run_interactive_init() reads from stdin — cannot be tested
    // in unit tests or git hooks (stdin is closed, read_line blocks).
    // Interactive flow is validated by manual `ralph-engine init` runs.
}
