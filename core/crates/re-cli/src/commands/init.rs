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
        Some("--help" | "-h") => Ok(i18n::init_help(locale).to_owned()),
        Some("--auto") => run_auto_init(".", locale),
        _ => run_interactive_init(args.first().map(String::as_str).unwrap_or("."), locale),
    }
}

/// Reads a line from stdin, trimmed.
///
/// # Panics
///
/// Returns empty string if stdin is not a terminal (CI, pipes).
/// Callers should check `is_interactive()` before starting flows
/// that depend on user input.
fn read_line() -> String {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().to_owned()
}

/// Returns true if stdin is a terminal (safe to read interactively).
fn is_interactive() -> bool {
    use std::io::IsTerminal as _;
    std::io::stdin().is_terminal()
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

/// Runs a non-interactive init with sensible defaults.
///
/// Used by the TUI dashboard (`/init`) where stdin is unavailable.
/// Uses the first template and enables all official plugins.
fn run_auto_init(target_dir: &str, locale: &str) -> Result<String, CliError> {
    let target = Path::new(target_dir);
    let re_dir = target.join(".ralph-engine");
    let mut output = Vec::new();

    if re_dir.exists() {
        return Ok(i18n::tui_already_initialized(locale).to_owned());
    }

    // Use first available template (usually "basic")
    let templates = catalog::official_template_contributions();
    if templates.is_empty() {
        return Err(CliError::new(i18n::init_no_templates(locale).to_owned()));
    }
    let selected = &templates[0];

    // Materialize template
    let materialize_result = materialize_template(selected, target, locale)?;
    output.push(materialize_result);

    // Enable ALL optional plugins automatically
    let config_path = target.join(".ralph-engine/config.yaml");
    let config_content = std::fs::read_to_string(&config_path).unwrap_or_default();
    let all_plugins = catalog::official_runtime_plugins();
    let optional: Vec<_> = all_plugins
        .iter()
        .filter(|p| !config_content.contains(p.descriptor.id))
        .collect();

    if !optional.is_empty() {
        enable_plugins_in_config(&config_path, &optional, "# Plugins enabled automatically");
        output.push(format!("  Enabled {} additional plugins", optional.len()));
    }

    // Accept all plugin init contributions
    apply_init_contributions(target, &config_path, locale, &mut output);

    output.push(format!("  {}", i18n::tui_project_initialized(locale)));
    Ok(output.join("\n"))
}

/// Runs the interactive init flow with auto-discovered templates and plugins.
fn run_interactive_init(target_dir: &str, locale: &str) -> Result<String, CliError> {
    // Rule 62: stdin must be a terminal for interactive init
    if !is_interactive() {
        return Err(CliError::new(
            "Interactive init requires a terminal. Use --preset <name> for non-interactive setup."
                .to_owned(),
        ));
    }

    let target = Path::new(target_dir);
    let re_dir = target.join(".ralph-engine");
    let mut output = Vec::new();

    output.push(format!("\n  {}\n", re_core::banner()));

    // ── Check existing directory ─────────────────────────────────────
    if re_dir.exists() {
        eprintln!("  {}", i18n::init_exists_warning(locale));
        if !confirm(&format!("  {}", i18n::init_overwrite_prompt(locale)), false) {
            return Ok(format!("  {}", i18n::init_cancelled(locale)));
        }
        std::fs::remove_dir_all(&re_dir)
            .map_err(|err| CliError::new(i18n::init_remove_failed(locale, &err.to_string())))?;
    }

    // ── Auto-discover templates from catalog ─────────────────────────
    let templates = catalog::official_template_contributions();
    if templates.is_empty() {
        return Err(CliError::new(i18n::init_no_templates(locale).to_owned()));
    }

    let selected = prompt_template_selection(&templates, locale);

    // ── Materialize template assets ──────────────────────────────────
    let materialize_result = materialize_template(selected, target, locale)?;
    output.push(materialize_result);

    // ── Auto-discover optional plugins ───────────────────────────────
    let config_path = target.join(".ralph-engine/config.yaml");
    prompt_optional_plugins(&config_path, locale, &mut output);

    // ── Plugin init contributions (auto-discovery) ────────────────────
    prompt_init_contributions(target, &config_path, locale, &mut output);

    // ── Done ─────────────────────────────────────────────────────────
    output.push(format!(
        "\n  {} 'ralph-engine doctor' {}",
        i18n::init_done_prefix(locale),
        i18n::init_done_suffix(locale)
    ));
    output.push("  Then: 'ralph-engine run' to start orchestration".to_owned());

    Ok(output.join("\n"))
}

/// Prompts the user to select a template and returns the chosen one.
fn prompt_template_selection<'a>(
    templates: &'a [re_official::OfficialTemplateContribution],
    locale: &str,
) -> &'a re_official::OfficialTemplateContribution {
    eprintln!("  {}", i18n::init_select_template(locale));
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
    &templates[template_idx]
}

/// Materializes template assets to disk.
fn materialize_template(
    selected: &re_official::OfficialTemplateContribution,
    target: &Path,
    locale: &str,
) -> Result<String, CliError> {
    let assets: Vec<MaterializedAsset<'_>> = selected
        .descriptor
        .assets
        .iter()
        .map(|a| MaterializedAsset {
            path: a.path,
            contents: a.contents,
        })
        .collect();
    materialize_assets(&assets, target, locale)
}

/// Prompts user to select optional plugins and enables them in config.
fn prompt_optional_plugins(config_path: &Path, locale: &str, output: &mut Vec<String>) {
    let config_content = std::fs::read_to_string(config_path).unwrap_or_default();
    let all_plugins = catalog::official_runtime_plugins();
    let optional: Vec<_> = all_plugins
        .iter()
        .filter(|p| !config_content.contains(p.descriptor.id))
        .collect();

    if optional.is_empty() {
        return;
    }

    eprintln!("\n  {}", i18n::init_enable_additional(locale));
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
        if let Ok(mut config) = std::fs::read_to_string(config_path) {
            config.push_str("\n# Plugins enabled during init\n");
            for id in &selected_ids {
                config.push_str(&format!("  - id: {id}\n    activation: enabled\n"));
            }
            let _ = std::fs::write(config_path, config);
        }
        output.push(format!(
            "  {} {} {}",
            i18n::init_enabled_label(locale),
            selected_ids.len(),
            i18n::init_additional_plugins(locale)
        ));
    }
}

/// Interactively applies plugin init contributions (asks user for each).
fn prompt_init_contributions(
    target: &Path,
    config_path: &Path,
    locale: &str,
    output: &mut Vec<String>,
) {
    let init_contributions = catalog::collect_init_contributions_from_plugins();
    if init_contributions.is_empty() {
        return;
    }

    for contrib in &init_contributions {
        eprintln!("  {} — {}", contrib.label, contrib.description);
        if confirm(&format!("  {}?", i18n::init_enable_label(locale)), true) {
            apply_single_contribution(target, config_path, contrib, locale, output);
        }
    }
}

/// Applies all plugin init contributions without prompting (auto mode).
fn apply_init_contributions(
    target: &Path,
    config_path: &Path,
    locale: &str,
    output: &mut Vec<String>,
) {
    let init_contributions = catalog::collect_init_contributions_from_plugins();
    for contrib in &init_contributions {
        apply_single_contribution(target, config_path, contrib, locale, output);
    }
}

/// Applies a single init contribution: config snippet + files.
fn apply_single_contribution(
    target: &Path,
    config_path: &Path,
    contrib: &re_plugin::InitContribution,
    locale: &str,
    output: &mut Vec<String>,
) {
    if let Some(snippet) = &contrib.config_snippet
        && let Ok(mut config) = std::fs::read_to_string(config_path)
    {
        config.push_str(&format!("\n{snippet}\n"));
        let _ = std::fs::write(config_path, config);
    }
    for (file_path, file_contents) in &contrib.files {
        let full_path = target.join(file_path);
        if let Some(parent) = full_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&full_path, file_contents);
        output.push(format!(
            "  {} {file_path}",
            i18n::init_created_label(locale),
        ));
    }
}

/// Enables a list of plugins in the config file.
fn enable_plugins_in_config(
    config_path: &Path,
    plugins: &[&re_core::RuntimePluginRegistration],
    header_comment: &str,
) {
    if let Ok(mut config) = std::fs::read_to_string(config_path) {
        config.push_str(&format!("\n{header_comment}\n"));
        for p in plugins {
            config.push_str(&format!(
                "  - id: {}\n    activation: enabled\n",
                p.descriptor.id
            ));
        }
        let _ = std::fs::write(config_path, config);
    }
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
