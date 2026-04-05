//! Keybinding system with core + plugin-contributed bindings.
//!
//! Core keybindings (q, p, ?, l) are always registered and cannot be
//! overridden by plugins. Plugins can contribute namespaced keybindings
//! via the `tui_contributions()` trait method.

use ratatui::crossterm::event::KeyCode;

/// A registered keybinding with its action.
#[derive(Debug, Clone)]
pub struct Keybinding {
    /// The key that triggers this binding.
    pub key: KeyCode,
    /// Short label for the help bar (e.g. `"pause"`).
    pub label: String,
    /// Longer description for the help modal.
    pub description: String,
    /// Source: `"core"` or plugin ID.
    pub source: String,
}

/// Core keybindings that cannot be overridden.
pub const CORE_KEYS: &[KeyCode] = &[
    KeyCode::Char('q'),
    KeyCode::Char('p'),
    KeyCode::Char('r'),
    KeyCode::Char('s'),
    KeyCode::Char('?'),
    KeyCode::Char('l'),
    KeyCode::Esc,
];

/// Returns `true` if the given key is reserved by core.
#[must_use]
pub fn is_core_key(key: KeyCode) -> bool {
    CORE_KEYS.contains(&key)
}

/// The keybinding registry — holds core + plugin bindings.
#[derive(Debug, Default)]
pub struct KeybindingRegistry {
    bindings: Vec<Keybinding>,
}

impl KeybindingRegistry {
    /// Creates a new registry with core keybindings pre-registered.
    #[must_use]
    pub fn new() -> Self {
        let mut registry = Self::default();
        registry.register_core_bindings();
        registry
    }

    /// Registers the default core keybindings.
    fn register_core_bindings(&mut self) {
        let core = [
            (KeyCode::Char('q'), "quit", "Quit the TUI"),
            (KeyCode::Char('p'), "pause", "Pause/resume agent"),
            (KeyCode::Char('?'), "help", "Show help"),
            (KeyCode::Char('l'), "logs", "Toggle log panel"),
            (KeyCode::Esc, "back", "Close modal/cancel"),
        ];

        for (key, label, description) in core {
            self.bindings.push(Keybinding {
                key,
                label: label.to_owned(),
                description: description.to_owned(),
                source: "core".to_owned(),
            });
        }
    }

    /// Registers a plugin-contributed keybinding.
    ///
    /// Returns `false` if the key is reserved by core (binding is
    /// rejected). Plugins cannot override core keybindings.
    pub fn register_plugin_binding(
        &mut self,
        key: KeyCode,
        label: &str,
        description: &str,
        plugin_id: &str,
    ) -> bool {
        if is_core_key(key) {
            tracing::warn!(
                plugin = plugin_id,
                key = ?key,
                "plugin tried to register reserved core keybinding — rejected"
            );
            return false;
        }

        // Check for duplicate key from another plugin
        if self.bindings.iter().any(|b| b.key == key) {
            tracing::warn!(
                plugin = plugin_id,
                key = ?key,
                "plugin keybinding conflicts with existing binding — rejected"
            );
            return false;
        }

        self.bindings.push(Keybinding {
            key,
            label: label.to_owned(),
            description: description.to_owned(),
            source: plugin_id.to_owned(),
        });

        true
    }

    /// Returns all registered bindings (core + plugins).
    #[must_use]
    pub fn all_bindings(&self) -> &[Keybinding] {
        &self.bindings
    }

    /// Returns only core bindings.
    #[must_use]
    pub fn core_bindings(&self) -> Vec<&Keybinding> {
        self.bindings
            .iter()
            .filter(|b| b.source == "core")
            .collect()
    }

    /// Returns only plugin-contributed bindings.
    #[must_use]
    pub fn plugin_bindings(&self) -> Vec<&Keybinding> {
        self.bindings
            .iter()
            .filter(|b| b.source != "core")
            .collect()
    }

    /// Finds the binding for a given key, if registered.
    #[must_use]
    pub fn find_binding(&self, key: KeyCode) -> Option<&Keybinding> {
        self.bindings.iter().find(|b| b.key == key)
    }

    /// Renders the help bar text (short labels for visible bindings).
    #[must_use]
    pub fn help_bar_text(&self) -> Vec<(String, String)> {
        self.bindings
            .iter()
            .filter(|b| b.source == "core") // Help bar shows core only
            .map(|b| {
                let key_str = format_key(b.key);
                (key_str, b.label.clone())
            })
            .collect()
    }
}

/// Formats a key code for display (e.g. `"q"`, `"Esc"`, `"Alt+1"`).
#[must_use]
pub fn format_key(key: KeyCode) -> String {
    match key {
        KeyCode::Char(c) => c.to_string(),
        KeyCode::Esc => "⎋".to_owned(),
        KeyCode::Enter => "⏎".to_owned(),
        KeyCode::Tab => "⇥".to_owned(),
        KeyCode::Up => "↑".to_owned(),
        KeyCode::Down => "↓".to_owned(),
        KeyCode::Left => "←".to_owned(),
        KeyCode::Right => "→".to_owned(),
        KeyCode::PageUp => "PgUp".to_owned(),
        KeyCode::PageDown => "PgDn".to_owned(),
        KeyCode::Home => "Home".to_owned(),
        KeyCode::End => "End".to_owned(),
        KeyCode::F(n) => format!("F{n}"),
        _ => format!("{key:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_keys_are_reserved() {
        assert!(is_core_key(KeyCode::Char('q')));
        assert!(is_core_key(KeyCode::Char('p')));
        assert!(is_core_key(KeyCode::Char('?')));
        assert!(!is_core_key(KeyCode::Char('x')));
        assert!(!is_core_key(KeyCode::Char('d')));
    }

    #[test]
    fn new_registry_has_core_bindings() {
        let reg = KeybindingRegistry::new();
        assert!(reg.core_bindings().len() >= 4);
        assert!(reg.find_binding(KeyCode::Char('q')).is_some());
        assert!(reg.find_binding(KeyCode::Char('p')).is_some());
    }

    #[test]
    fn plugin_can_register_non_core_key() {
        let mut reg = KeybindingRegistry::new();
        let ok = reg.register_plugin_binding(
            KeyCode::Char('d'),
            "detail",
            "Show finding detail",
            "test.plugin",
        );
        assert!(ok);
        let binding = reg.find_binding(KeyCode::Char('d'));
        assert!(binding.is_some());
        assert_eq!(binding.map(|b| b.source.as_str()), Some("test.plugin"));
    }

    #[test]
    fn plugin_cannot_override_core_key() {
        let mut reg = KeybindingRegistry::new();
        let ok = reg.register_plugin_binding(
            KeyCode::Char('q'),
            "my-quit",
            "Custom quit",
            "evil.plugin",
        );
        assert!(!ok);
        // Core binding still there
        let binding = reg.find_binding(KeyCode::Char('q'));
        assert_eq!(binding.map(|b| b.source.as_str()), Some("core"));
    }

    #[test]
    fn duplicate_plugin_key_rejected() {
        let mut reg = KeybindingRegistry::new();
        assert!(reg.register_plugin_binding(
            KeyCode::Char('d'),
            "detail",
            "Show detail",
            "plugin.a",
        ));
        assert!(!reg.register_plugin_binding(
            KeyCode::Char('d'),
            "delete",
            "Delete item",
            "plugin.b",
        ));
    }

    #[test]
    fn plugin_bindings_filtered_correctly() {
        let mut reg = KeybindingRegistry::new();
        reg.register_plugin_binding(KeyCode::Char('d'), "detail", "Detail", "findings");
        reg.register_plugin_binding(KeyCode::Char('n'), "next", "Next", "bmad");

        let plugin = reg.plugin_bindings();
        assert_eq!(plugin.len(), 2);
        assert!(plugin.iter().all(|b| b.source != "core"));
    }

    #[test]
    fn help_bar_shows_core_only() {
        let mut reg = KeybindingRegistry::new();
        reg.register_plugin_binding(KeyCode::Char('d'), "detail", "Detail", "findings");

        let bar = reg.help_bar_text();
        assert!(bar.iter().all(|(_, label)| label != "detail"));
        assert!(bar.iter().any(|(_, label)| label == "quit"));
    }

    #[test]
    fn format_key_renders_chars_and_special() {
        assert_eq!(format_key(KeyCode::Char('q')), "q");
        assert_eq!(format_key(KeyCode::Esc), "⎋");
        assert_eq!(format_key(KeyCode::Enter), "⏎");
        assert_eq!(format_key(KeyCode::Up), "↑");
        assert_eq!(format_key(KeyCode::Down), "↓");
        assert_eq!(format_key(KeyCode::F(1)), "F1");
    }
}
