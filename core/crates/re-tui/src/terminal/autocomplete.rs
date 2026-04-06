//! Autocomplete popup state for agent slash commands.

use super::types::CommandEntry;

/// Autocomplete popup state for agent slash commands.
#[derive(Debug)]
pub(super) struct AutocompleteState {
    /// All available commands (discovered from agent plugin).
    pub commands: Vec<CommandEntry>,
    /// Indices into `commands` matching the current filter.
    pub filtered: Vec<usize>,
    /// Selected index within `filtered`.
    pub selected: usize,
    /// Whether the popup is visible.
    pub visible: bool,
    /// The command prefix character (e.g. `/`).
    pub prefix: String,
}

impl AutocompleteState {
    /// Creates a new autocomplete state with the given commands and prefix.
    pub fn new(commands: Vec<CommandEntry>, prefix: String) -> Self {
        Self {
            commands,
            filtered: Vec::new(),
            selected: 0,
            visible: false,
            prefix,
        }
    }

    /// Updates the filter based on current input text.
    pub fn update_filter(&mut self, input: &str) {
        if input.starts_with(self.prefix.as_str()) && !self.commands.is_empty() {
            let query = &input[self.prefix.len()..];
            self.filtered = self
                .commands
                .iter()
                .enumerate()
                .filter(|(_, cmd)| {
                    query.is_empty()
                        || cmd.name.contains(query)
                        || cmd
                            .description
                            .to_lowercase()
                            .contains(&query.to_lowercase())
                })
                .map(|(i, _)| i)
                .collect();
            self.selected = self.selected.min(self.filtered.len().saturating_sub(1));
            self.visible = !self.filtered.is_empty();
        } else {
            self.visible = false;
        }
    }

    /// Returns the currently selected command name (with prefix), if any.
    pub fn selected_command(&self) -> Option<String> {
        if !self.visible || self.filtered.is_empty() {
            return None;
        }
        let idx = self.filtered[self.selected];
        Some(format!("{}{}", self.prefix, self.commands[idx].name))
    }
}
