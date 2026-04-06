//! TUI shell — struct definition, state management, input handling.
//!
//! The TUI shell manages terminal lifecycle, user input, and delegates
//! rendering to the `render*` submodules. It is the central state
//! machine for the TUI.

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use tui_scrollview::ScrollViewState;

use super::autocomplete::AutocompleteState;
use super::types::{
    AgentPid, CommandEntry, PluginKeyAction, RegisteredKeybinding, SidebarPanel,
    TOAST_DEFAULT_TICKS, Toast, ToastLevel, TuiConfig, TuiError, TuiLabels, TuiState,
};

/// The TUI shell — manages terminal lifecycle and render loop.
///
/// Create via [`TuiShell::new`], then call [`TuiShell::run_demo`]
/// to start the render loop. The terminal is restored on drop.
pub struct TuiShell {
    pub(super) config: TuiConfig,
    pub(super) labels: TuiLabels,
    pub(super) theme: Box<dyn crate::theme::Theme>,
    pub(super) state: TuiState,
    pub(super) progress: u16,
    pub(super) activity_lines: Vec<String>,
    pub(super) feed: crate::feed::Feed,
    pub(super) feed_scroll: ScrollViewState,
    pub(super) follow_mode: bool,
    pub(super) focused_block: Option<usize>,
    pub(super) indicator_panel: crate::indicators::IndicatorPanel,
    pub(super) tool_count: usize,
    pub(super) token_count: usize,
    pub(super) cost_label: Option<String>,
    pub(super) extra_usage: bool,
    pub(super) thinking_message: Option<String>,
    pub(super) tick: usize,
    pub(super) should_quit: bool,
    pub(super) quit_pending: bool,
    pub(super) help_modal_visible: bool,
    pub(super) sidebar_panels: Vec<SidebarPanel>,
    pub(super) sidebar_visible: bool,
    pub(super) main_panels: Vec<SidebarPanel>,
    pub(super) available_agents: Vec<String>,
    pub(super) agent_switcher_visible: bool,
    pub(super) agent_switcher_selected: usize,
    pub(super) agent_pid: AgentPid,
    pub(super) plugin_keybindings: Vec<RegisteredKeybinding>,
    pub(super) input_enabled: bool,
    pub(super) text_input_buffer: String,
    pub(super) pending_text_input: Option<String>,
    pub(super) autocomplete: AutocompleteState,
    pub(super) toasts: Vec<Toast>,
    pub(super) pending_blocks: std::collections::VecDeque<crate::feed::FeedBlock>,
    pub(super) pending_total: usize,
    pub(super) drip_counter: usize,
}

impl TuiShell {
    /// Creates a new TUI shell with the given configuration.
    #[must_use]
    pub fn new(config: TuiConfig) -> Self {
        Self {
            config,
            labels: TuiLabels::default(),
            theme: Box::new(crate::theme::CatppuccinMocha),
            state: TuiState::Running,
            progress: 0,
            activity_lines: Vec::new(),
            feed: crate::feed::Feed::new(),
            feed_scroll: ScrollViewState::default(),
            follow_mode: true,
            focused_block: None,
            indicator_panel: crate::indicators::IndicatorPanel::new(),
            tool_count: 0,
            token_count: 0,
            cost_label: None,
            extra_usage: false,
            thinking_message: None,
            tick: 0,
            should_quit: false,
            quit_pending: false,
            help_modal_visible: false,
            sidebar_panels: Vec::new(),
            sidebar_visible: true,
            main_panels: Vec::new(),
            available_agents: Vec::new(),
            agent_switcher_visible: false,
            agent_switcher_selected: 0,
            agent_pid: None,
            plugin_keybindings: Vec::new(),
            input_enabled: false,
            text_input_buffer: String::new(),
            pending_text_input: None,
            autocomplete: AutocompleteState::new(Vec::new(), "/".to_owned()),
            toasts: Vec::new(),
            pending_blocks: std::collections::VecDeque::new(),
            pending_total: 0,
            drip_counter: 0,
        }
    }

    // ── Getters ─────────────────────────────────────────────────

    /// Returns the current TUI state.
    #[must_use]
    pub fn state(&self) -> TuiState {
        self.state
    }

    /// Returns a reference to the active theme.
    #[must_use]
    pub fn theme(&self) -> &dyn crate::theme::Theme {
        self.theme.as_ref()
    }

    /// Returns a reference to the localized labels.
    #[must_use]
    pub fn labels(&self) -> &TuiLabels {
        &self.labels
    }

    pub(super) fn localized_state_label(&self) -> &str {
        match self.state {
            TuiState::Running => &self.labels.state_running,
            TuiState::Paused => &self.labels.state_paused,
            TuiState::Complete => &self.labels.state_complete,
            TuiState::Error => &self.labels.state_error,
        }
    }

    /// Whether the TUI is waiting for quit confirmation.
    #[must_use]
    pub fn is_quit_pending(&self) -> bool {
        self.quit_pending
    }

    /// Whether the TUI should exit.
    #[must_use]
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Whether the input bar is enabled (a plugin requested it).
    #[must_use]
    pub fn is_input_enabled(&self) -> bool {
        self.input_enabled
    }

    /// Returns the current text input buffer (for rendering).
    #[must_use]
    pub fn text_input_buffer(&self) -> &str {
        &self.text_input_buffer
    }

    /// Returns the agent PID if set.
    #[must_use]
    pub fn agent_pid(&self) -> AgentPid {
        self.agent_pid
    }

    /// Returns a reference to the block-based feed.
    #[must_use]
    pub fn feed(&self) -> &crate::feed::Feed {
        &self.feed
    }

    /// Returns a mutable reference to the block-based feed.
    pub fn feed_mut(&mut self) -> &mut crate::feed::Feed {
        &mut self.feed
    }

    /// Whether follow mode is active (auto-scroll on new content).
    #[must_use]
    pub fn is_follow_mode(&self) -> bool {
        self.follow_mode
    }

    /// Returns the index of the focused block, if any.
    #[must_use]
    pub fn focused_block(&self) -> Option<usize> {
        self.focused_block
    }

    /// Returns a reference to the gate pipeline.
    #[must_use]
    pub fn indicator_panel(&self) -> &crate::indicators::IndicatorPanel {
        &self.indicator_panel
    }

    /// Returns a mutable reference to the gate pipeline.
    pub fn indicator_panel_mut(&mut self) -> &mut crate::indicators::IndicatorPanel {
        &mut self.indicator_panel
    }

    // ── Setters ─────────────────────────────────────────────────

    /// Sets the TUI state.
    pub fn set_state(&mut self, state: TuiState) {
        tracing::debug!(old = ?self.state, new = ?state, "TUI state transition");
        self.state = state;
    }

    /// Switches the active theme by config ID.
    pub fn set_theme(&mut self, id: &str) {
        self.theme = crate::theme::resolve_theme(id);
    }

    /// Sets labels.
    pub fn set_labels(&mut self, labels: TuiLabels) {
        self.labels = labels;
    }

    /// Sets progress.
    pub fn set_progress(&mut self, pct: u16) {
        self.progress = pct.min(100);
    }

    /// Sets token count.
    pub fn set_token_count(&mut self, count: usize) {
        self.token_count = count;
    }

    /// Sets cost label.
    pub fn set_cost_label(&mut self, label: String) {
        self.cost_label = Some(label);
    }

    /// Sets extra usage.
    pub fn set_extra_usage(&mut self, extra: bool) {
        self.extra_usage = extra;
    }

    /// Sets thinking message.
    pub fn set_thinking_message(&mut self, msg: Option<String>) {
        self.thinking_message = msg;
    }

    /// Sets available agents.
    pub fn set_available_agents(&mut self, agents: Vec<String>) {
        self.available_agents = agents;
    }

    /// Sets sidebar panels (`zone_hint="sidebar"`).
    pub fn set_sidebar_panels(&mut self, panels: Vec<SidebarPanel>) {
        self.sidebar_panels = panels;
    }

    /// Sets main-zone panels (`zone_hint="main"`) — rendered in the feed area when idle.
    pub fn set_main_panels(&mut self, panels: Vec<SidebarPanel>) {
        self.main_panels = panels;
    }

    /// Sets plugin keybindings.
    pub fn set_plugin_keybindings(&mut self, bindings: Vec<RegisteredKeybinding>) {
        self.plugin_keybindings = bindings;
    }

    /// Sets agent pid.
    pub fn set_agent_pid(&mut self, pid: u32) {
        self.agent_pid = Some(pid);
    }

    /// Sets agent commands.
    pub fn set_agent_commands(&mut self, commands: Vec<CommandEntry>, prefix: String) {
        self.autocomplete = AutocompleteState::new(commands, prefix);
    }

    /// Enables input.
    pub fn enable_input(&mut self) {
        self.input_enabled = true;
    }

    // ── Activity & toasts ───────────────────────────────────────

    /// Pushes activity.
    pub fn push_activity(&mut self, line: String) {
        if self.activity_lines.len() >= 10_000 {
            self.activity_lines.drain(..1_000);
        }
        self.activity_lines.push(line);
    }

    /// Shows toast.
    pub fn show_toast(&mut self, message: String, level: ToastLevel) {
        self.toasts.push(Toast {
            message,
            level,
            remaining_ticks: TOAST_DEFAULT_TICKS,
        });
    }

    /// Shows a info.
    pub fn toast_info(&mut self, message: String) {
        self.show_toast(message, ToastLevel::Info);
    }

    /// Shows a success.
    pub fn toast_success(&mut self, message: String) {
        self.show_toast(message, ToastLevel::Success);
    }

    /// Shows error modal.
    pub fn show_error_modal(&mut self, title: &str, message: &str) {
        self.push_activity(format!("  ✗ {title}: {message}"));
        self.show_toast(format!("✗ {title}"), ToastLevel::Error);
    }

    /// Increments tools.
    pub fn increment_tools(&mut self) {
        self.tool_count += 1;
    }

    /// Pushes startup banner.
    pub fn push_startup_banner(&mut self) {
        self.push_activity(String::new());
        self.push_activity(format!("  ◎ Ralph Engine v{}", env!("CARGO_PKG_VERSION")));
        self.push_activity(format!("  Agent:   {}", self.config.agent_id));
        self.push_activity(format!("  Work:    {}", self.config.title));
        let total_panels = self.sidebar_panels.len() + self.main_panels.len();
        self.push_activity(format!("  Plugins: {total_panels} panels"));
        self.push_activity(String::new());
        self.push_activity("  Initializing...".to_owned());
        self.push_activity(String::new());
    }

    // ── Agent selection ─────────────────────────────────────────

    /// Returns the agent ID selected in the switcher, if confirmed.
    #[must_use]
    pub fn take_selected_agent(&mut self) -> Option<String> {
        if !self.agent_switcher_visible {
            return None;
        }
        self.available_agents
            .get(self.agent_switcher_selected)
            .cloned()
    }

    /// Takes text input.
    pub fn take_text_input(&mut self) -> Option<String> {
        self.pending_text_input.take()
    }

    // ── Scroll & focus ──────────────────────────────────────────

    /// Scrolls feed up.
    pub fn scroll_feed_up(&mut self) {
        self.follow_mode = false;
        self.feed_scroll.scroll_up();
    }

    /// Scrolls feed down.
    pub fn scroll_feed_down(&mut self) {
        self.follow_mode = false;
        self.feed_scroll.scroll_down();
    }

    /// Scrolls feed page up.
    pub fn scroll_feed_page_up(&mut self) {
        self.follow_mode = false;
        self.feed_scroll.scroll_page_up();
    }

    /// Scrolls feed page down.
    pub fn scroll_feed_page_down(&mut self) {
        self.follow_mode = false;
        self.feed_scroll.scroll_page_down();
    }

    /// Scrolls feed to top.
    pub fn scroll_feed_to_top(&mut self) {
        self.follow_mode = false;
        self.feed_scroll.scroll_to_top();
    }

    /// Scrolls feed to bottom.
    pub fn scroll_feed_to_bottom(&mut self) {
        self.follow_mode = true;
        self.feed_scroll.scroll_to_bottom();
    }

    /// Focuses next block.
    pub fn focus_next_block(&mut self) {
        let count = self.feed.len();
        if count == 0 {
            return;
        }
        self.follow_mode = false;
        self.focused_block = Some(match self.focused_block {
            Some(i) if i + 1 < count => i + 1,
            Some(_) => count - 1,
            None => count - 1,
        });
    }

    /// Focuses prev block.
    pub fn focus_prev_block(&mut self) {
        let count = self.feed.len();
        if count == 0 {
            return;
        }
        self.follow_mode = false;
        self.focused_block = Some(match self.focused_block {
            Some(i) if i > 0 => i - 1,
            Some(_) => 0,
            None => 0,
        });
    }

    /// Toggles focused block.
    pub fn toggle_focused_block(&mut self) {
        if let Some(idx) = self.focused_block {
            self.feed.toggle_block(idx);
        }
    }

    /// Clears focus.
    pub fn clear_focus(&mut self) {
        self.focused_block = None;
        self.follow_mode = true;
        self.feed_scroll.scroll_to_bottom();
    }

    /// Copies focused block.
    pub fn copy_focused_block(&mut self) -> bool {
        let Some(idx) = self.focused_block else {
            return false;
        };
        let Some(block) = self.feed.blocks().get(idx) else {
            return false;
        };

        let text = crate::clipboard::block_to_copyable_text(&block.title, &block.content);
        if text.is_empty() {
            return false;
        }

        if crate::clipboard::copy_to_clipboard(&text) {
            self.toast_success(format!("✓ Copied {} chars", text.len()));
            true
        } else {
            self.show_toast("Copy failed (no clipboard)".to_owned(), ToastLevel::Warning);
            false
        }
    }

    // ── Event processing ────────────────────────────────────────

    /// Processes event.
    pub fn process_event(&mut self, event: &crate::events::AgentEvent) {
        use crate::events::AgentEvent;

        crate::feed::process_agent_event(&mut self.feed, event);

        match event {
            AgentEvent::TextDelta(_) => {
                self.push_activity(event.activity_line());
            }
            AgentEvent::ToolUse { .. } => {
                self.increment_tools();
                self.push_activity(event.activity_line());
            }
            AgentEvent::ToolResult { .. } => {
                self.push_activity(event.activity_line());
            }
            AgentEvent::Complete { is_error } => {
                self.push_activity(event.activity_line());
                if *is_error {
                    self.set_state(TuiState::Error);
                } else {
                    self.set_state(TuiState::Complete);
                }
            }
            AgentEvent::System(_) | AgentEvent::Unknown(_) => {
                let line = event.activity_line();
                if !line.is_empty() {
                    self.push_activity(line);
                }
            }
        }
    }

    // ── Drip feed ───────────────────────────────────────────────

    /// Enqueues blocks.
    pub fn enqueue_blocks(&mut self, blocks: Vec<crate::feed::FeedBlock>) {
        self.pending_total = blocks.len();
        self.pending_blocks = blocks.into();
        self.drip_counter = 0;
        self.follow_mode = true;
    }

    /// Drains one pending block into the feed if the cadence allows.
    pub(super) fn drain_pending_block(&mut self) -> bool {
        if self.pending_blocks.is_empty() {
            return false;
        }
        self.drip_counter += 1;

        let last_kind = self.feed.blocks().last().map(|b| (b.kind, b.active));

        let hold = match last_kind {
            Some((crate::feed::BlockKind::Thinking, true)) => 80,
            Some((crate::feed::BlockKind::Command, true)) => 100,
            Some((crate::feed::BlockKind::System, true)) => 40,
            _ => 0,
        };

        let next_kind = self.pending_blocks[0].kind;
        let appear_delay = match next_kind {
            crate::feed::BlockKind::FileEdit => 16,
            crate::feed::BlockKind::FileRead => 10,
            crate::feed::BlockKind::AgentText => 20,
            crate::feed::BlockKind::GatePass | crate::feed::BlockKind::GateFail => 6,
            _ => 14,
        };

        let total_interval = hold + appear_delay;

        if hold > 0 && self.drip_counter < hold && self.drip_counter.is_multiple_of(40) {
            const DEMO_MESSAGES: &[&str] = &[
                "Thinking...",
                "Reasoning deeply...",
                "Analyzing the codebase...",
                "Considering approaches...",
                "Crafting a solution...",
                "Planning the implementation...",
            ];
            let msg_idx = (self.tick / 80) % DEMO_MESSAGES.len();
            self.thinking_message = Some(DEMO_MESSAGES[msg_idx].to_owned());
        }

        if self.drip_counter >= total_interval {
            self.drip_counter = 0;

            if let Some(last) = self.feed.blocks_mut().last_mut()
                && last.active
            {
                last.finalize(last.success.unwrap_or(true));
            }

            let Some(mut block) = self.pending_blocks.pop_front() else {
                return false;
            };

            if !self.pending_blocks.is_empty() {
                block.active = true;
            } else if block.success.is_some() {
                block.active = false;
            }

            let is_tool = matches!(
                block.kind,
                crate::feed::BlockKind::FileRead
                    | crate::feed::BlockKind::FileEdit
                    | crate::feed::BlockKind::Command
            );

            if let Some(ref marker) = block.phase_marker {
                if let Some(id) = marker.strip_prefix("start:") {
                    self.indicator_panel.start(id);
                } else if let Some(id) = marker.strip_prefix("pass:") {
                    self.indicator_panel.pass(id);
                } else if let Some(id) = marker.strip_prefix("fail:") {
                    self.indicator_panel.fail(id, "");
                }
            }

            self.feed.push_block(block);

            if is_tool {
                self.tool_count += 1;
            }
            let tokens_delta = match next_kind {
                crate::feed::BlockKind::Thinking => 1200,
                crate::feed::BlockKind::FileEdit => 800,
                crate::feed::BlockKind::FileRead => 300,
                crate::feed::BlockKind::Command => 500,
                crate::feed::BlockKind::AgentText => 600,
                _ => 100,
            };
            self.token_count += tokens_delta;

            self.thinking_message = None;

            let completed = self.pending_total - self.pending_blocks.len();
            let pct = (completed * 100 / self.pending_total.max(1)) as u16;
            self.set_progress(pct);

            true
        } else if self.drip_counter == hold && hold > 0 {
            if let Some(last) = self.feed.blocks_mut().last_mut()
                && last.active
            {
                last.finalize(last.success.unwrap_or(true));
            }
            false
        } else {
            false
        }
    }

    // ── Input handling ──────────────────────────────────────────

    /// Handles paste.
    pub fn handle_paste(&mut self, text: &str) {
        if !self.input_enabled {
            return;
        }
        self.text_input_buffer.push_str(text);
        self.autocomplete.update_filter(&self.text_input_buffer);
    }

    /// Handles mouse.
    pub fn handle_mouse(&mut self, kind: ratatui::crossterm::event::MouseEventKind) {
        use ratatui::crossterm::event::MouseEventKind;
        match kind {
            MouseEventKind::ScrollUp => self.scroll_feed_up(),
            MouseEventKind::ScrollDown => self.scroll_feed_down(),
            MouseEventKind::Down(_) => {
                self.focus_next_block();
            }
            _ => {}
        }
    }

    /// Handles key.
    pub fn handle_key(&mut self, code: KeyCode) -> PluginKeyAction {
        self.handle_key_with_modifiers(code, KeyModifiers::NONE)
    }

    /// Handles key with modifiers.
    pub fn handle_key_with_modifiers(
        &mut self,
        code: KeyCode,
        modifiers: KeyModifiers,
    ) -> PluginKeyAction {
        if self.help_modal_visible {
            self.help_modal_visible = false;
            return PluginKeyAction::Handled;
        }

        if self.quit_pending {
            match code {
                KeyCode::Char('y' | 'Y') => {
                    tracing::info!("user confirmed quit");
                    self.should_quit = true;
                }
                _ => {
                    self.quit_pending = false;
                }
            }
            return PluginKeyAction::Handled;
        }

        let typing = self.input_enabled && !self.text_input_buffer.is_empty();

        if typing {
            return self.handle_typing_key(code, modifiers);
        }

        // Core keys (always available when not typing)
        if let Some(action) = self.handle_core_key(code, modifiers) {
            return action;
        }

        // Plugin keybinding dispatch
        if let KeyCode::Char(c) = code {
            let state_label = format!("{:?}", self.state);
            if self.find_active_binding(c, &state_label).is_some() {
                tracing::debug!(key = %c, "dispatching key to plugin");
                return PluginKeyAction::NotHandled;
            }

            if self.input_enabled {
                self.text_input_buffer.push(c);
                self.autocomplete.update_filter(&self.text_input_buffer);
                return PluginKeyAction::Handled;
            }
        }

        PluginKeyAction::NotHandled
    }

    fn handle_typing_key(&mut self, code: KeyCode, modifiers: KeyModifiers) -> PluginKeyAction {
        if self.autocomplete.visible {
            match code {
                KeyCode::Up => {
                    self.autocomplete.selected = self.autocomplete.selected.saturating_sub(1);
                    return PluginKeyAction::Handled;
                }
                KeyCode::Down => {
                    self.autocomplete.selected = (self.autocomplete.selected + 1)
                        .min(self.autocomplete.filtered.len().saturating_sub(1));
                    return PluginKeyAction::Handled;
                }
                KeyCode::Tab => {
                    if let Some(cmd) = self.autocomplete.selected_command() {
                        self.text_input_buffer = cmd;
                        self.autocomplete.visible = false;
                    }
                    return PluginKeyAction::Handled;
                }
                KeyCode::Enter => {
                    if let Some(cmd) = self.autocomplete.selected_command() {
                        self.text_input_buffer = cmd;
                    }
                    // Fall through to normal Enter handling
                }
                KeyCode::Esc => {
                    self.autocomplete.visible = false;
                    return PluginKeyAction::Handled;
                }
                _ => {}
            }
        }

        match code {
            KeyCode::Enter if modifiers.contains(KeyModifiers::ALT) => {
                self.text_input_buffer.push('\n');
            }
            KeyCode::Enter => {
                if !self.text_input_buffer.trim().is_empty() {
                    let text = self.text_input_buffer.trim().to_owned();
                    self.push_activity(format!(">> You: {text}"));
                    self.pending_text_input = Some(text);
                }
                self.text_input_buffer.clear();
                self.autocomplete.visible = false;
            }
            KeyCode::Esc => {
                self.text_input_buffer.clear();
                self.autocomplete.visible = false;
            }
            KeyCode::Backspace => {
                self.text_input_buffer.pop();
                self.autocomplete.update_filter(&self.text_input_buffer);
            }
            KeyCode::Char(c) => {
                self.text_input_buffer.push(c);
                self.autocomplete.update_filter(&self.text_input_buffer);
            }
            _ => {}
        }
        PluginKeyAction::Handled
    }

    fn handle_core_key(
        &mut self,
        code: KeyCode,
        modifiers: KeyModifiers,
    ) -> Option<PluginKeyAction> {
        match code {
            KeyCode::Char('q') => {
                self.quit_pending = true;
                Some(PluginKeyAction::Handled)
            }
            KeyCode::Char('p') => {
                if self.state == TuiState::Running {
                    self.set_state(TuiState::Paused);
                    self.push_activity(">> PAUSED — press [p] to resume".to_owned());
                } else if self.state == TuiState::Paused {
                    self.set_state(TuiState::Running);
                    self.push_activity(">> RUNNING".to_owned());
                }
                Some(PluginKeyAction::Handled)
            }
            KeyCode::Char('?') => {
                self.help_modal_visible = !self.help_modal_visible;
                Some(PluginKeyAction::Handled)
            }
            KeyCode::Char('j') => {
                self.focus_next_block();
                Some(PluginKeyAction::Handled)
            }
            KeyCode::Char('k') => {
                self.focus_prev_block();
                Some(PluginKeyAction::Handled)
            }
            KeyCode::Enter => {
                self.toggle_focused_block();
                Some(PluginKeyAction::Handled)
            }
            KeyCode::Char('y') => {
                self.copy_focused_block();
                Some(PluginKeyAction::Handled)
            }
            KeyCode::Esc => {
                self.clear_focus();
                Some(PluginKeyAction::Handled)
            }
            KeyCode::Up => {
                self.scroll_feed_up();
                Some(PluginKeyAction::Handled)
            }
            KeyCode::Down => {
                self.scroll_feed_down();
                Some(PluginKeyAction::Handled)
            }
            KeyCode::PageUp => {
                self.scroll_feed_page_up();
                Some(PluginKeyAction::Handled)
            }
            KeyCode::PageDown => {
                self.scroll_feed_page_down();
                Some(PluginKeyAction::Handled)
            }
            KeyCode::Home => {
                self.scroll_feed_to_top();
                Some(PluginKeyAction::Handled)
            }
            KeyCode::End | KeyCode::Char('G') => {
                self.scroll_feed_to_bottom();
                Some(PluginKeyAction::Handled)
            }
            KeyCode::F(2) => {
                self.sidebar_visible = !self.sidebar_visible;
                Some(PluginKeyAction::Handled)
            }
            KeyCode::Char('a') if modifiers.contains(KeyModifiers::CONTROL) => {
                if !self.available_agents.is_empty() {
                    self.agent_switcher_visible = !self.agent_switcher_visible;
                    self.agent_switcher_selected = 0;
                }
                Some(PluginKeyAction::Handled)
            }
            _ => None,
        }
    }

    /// Applies plugin action.
    pub fn apply_plugin_action(&mut self, action: &PluginKeyAction) {
        match action {
            PluginKeyAction::Handled | PluginKeyAction::NotHandled => {}
            PluginKeyAction::EnterTextInput { prompt } => {
                self.input_enabled = true;
                self.text_input_buffer.clear();
                self.push_activity(format!(">> {prompt}"));
            }
            PluginKeyAction::SetState(new_state) => {
                self.set_state(*new_state);
                let new_label = format!("{new_state:?}");
                let available: Vec<String> = self
                    .plugin_keybindings
                    .iter()
                    .filter(|b| {
                        b.active_states.is_empty()
                            || b.active_states.iter().any(|s| s == &new_label)
                    })
                    .map(|b| format!("[{}] {}", b.key, b.description))
                    .collect();
                if available.is_empty() {
                    self.push_activity(format!(">> {}", new_state.label()));
                } else {
                    self.push_activity(format!(
                        ">> {} — {}",
                        new_state.label(),
                        available.join("  ")
                    ));
                }
            }
            PluginKeyAction::ShowMessage(msg) => {
                self.push_activity(format!(">> {msg}"));
            }
        }
    }

    /// Finds an active plugin keybinding for the given key and state.
    #[must_use]
    pub fn find_active_binding(
        &self,
        key: char,
        state_label: &str,
    ) -> Option<&RegisteredKeybinding> {
        self.plugin_keybindings.iter().find(|b| {
            b.key == key
                && (b.active_states.is_empty() || b.active_states.iter().any(|s| s == state_label))
        })
    }

    // ── Demo loop ───────────────────────────────────────────────

    /// Runs the TUI demo loop.
    ///
    /// # Errors
    ///
    /// Returns an error if terminal initialization fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn run_demo(&mut self) -> Result<(), TuiError> {
        self.push_activity(">> Ralph Engine TUI initialized".to_owned());
        self.push_activity(format!(">> Agent: {}", self.config.agent_id));
        self.push_activity(">> Waiting for agent stream...".to_owned());

        let mut terminal = init_terminal();

        let result = (|| -> Result<(), TuiError> {
            loop {
                terminal
                    .draw(|frame| self.render_frame(frame))
                    .map_err(|e| TuiError::new(format!("render failed: {e}")))?;

                if event::poll(std::time::Duration::from_millis(100))
                    .map_err(|e| TuiError::new(format!("event poll failed: {e}")))?
                {
                    match event::read()
                        .map_err(|e| TuiError::new(format!("event read failed: {e}")))?
                    {
                        Event::Key(key) if key.kind == KeyEventKind::Press => {
                            self.handle_key(key.code);
                        }
                        Event::Mouse(mouse) => {
                            self.handle_mouse(mouse.kind);
                        }
                        _ => {}
                    }
                }

                if self.should_quit {
                    break;
                }
            }
            Ok(())
        })();

        restore_terminal();
        result
    }
}

/// Initializes the terminal with ratatui defaults.
#[cfg_attr(coverage_nightly, coverage(off))]
fn init_terminal() -> ratatui::DefaultTerminal {
    ratatui::init()
}

/// Restores terminal to normal mode.
#[cfg_attr(coverage_nightly, coverage(off))]
fn restore_terminal() {
    ratatui::restore();
}
