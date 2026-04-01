//! Policy command handlers.

use re_core::RuntimePolicyRegistration;

use crate::{CliError, catalog};

/// Executes the policies command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_policy_listing(&catalog::official_runtime_policies())),
        Some("show") => show_policy(args.get(1).map(String::as_str)),
        Some(other) => Err(CliError::new(format!("unknown policies command: {other}"))),
    }
}

fn show_policy(policy_id: Option<&str>) -> Result<String, CliError> {
    let policy_id = policy_id.ok_or_else(|| CliError::new("policies show requires a policy id"))?;
    let registration = catalog::official_runtime_policies()
        .into_iter()
        .find(|registration| registration.policy_id == policy_id)
        .ok_or_else(|| CliError::new(format!("unknown policy: {policy_id}")))?;

    Ok(render_policy_detail(&registration))
}

fn render_policy_listing(registrations: &[RuntimePolicyRegistration]) -> String {
    if registrations.is_empty() {
        return "Policies (0)".to_owned();
    }

    let lines = registrations
        .iter()
        .map(|registration| {
            format!(
                "- {} | plugin={} | activation={}",
                registration.policy_id,
                registration.plugin_id,
                registration.activation.as_str()
            )
        })
        .collect::<Vec<_>>();

    format!("Policies ({})\n{}", registrations.len(), lines.join("\n"))
}

fn render_policy_detail(registration: &RuntimePolicyRegistration) -> String {
    [
        format!("Policy: {}", registration.policy_id),
        format!("Provider: {}", registration.plugin_id),
        format!("Activation: {}", registration.activation.as_str()),
        format!("Load boundary: {}", registration.load_boundary.as_str()),
        format!(
            "Policy enforcement hook: {}",
            registration.enforcement_hook_registered
        ),
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_core::RuntimePolicyRegistration;
    use re_plugin::PluginLoadBoundary;

    use super::{render_policy_detail, render_policy_listing};

    #[test]
    fn render_policy_listing_handles_empty_sets() {
        // Arrange
        let registrations = [];

        // Act
        let rendered = render_policy_listing(&registrations);

        // Assert
        assert_eq!(rendered, "Policies (0)");
    }

    #[test]
    fn render_policy_detail_is_human_readable() {
        // Arrange
        let registration = RuntimePolicyRegistration::new(
            "official.tdd-strict",
            "official.tdd-strict",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        );

        // Act
        let rendered = render_policy_detail(&registration);

        // Assert
        assert!(rendered.contains("Policy: official.tdd-strict"));
        assert!(rendered.contains("Provider: official.tdd-strict"));
        assert!(rendered.contains("Activation: enabled"));
        assert!(rendered.contains("Load boundary: in_process"));
        assert!(rendered.contains("Policy enforcement hook: true"));
    }
}
