//! Tests for the TUI shell — unit + render snapshot tests.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use super::*;

fn empty_shell() -> TuiShell {
    TuiShell::new(TuiConfig {
        title: "Test".to_owned(),
        agent_id: "test.agent".to_owned(),
        locale: "en".to_owned(),
        project_name: String::new(),
    })
}

/// Shell with input enabled and focused (simulates guided plugin active).
fn interactive_shell() -> TuiShell {
    let mut shell = empty_shell();
    shell.enable_input();
    shell.focus = super::types::FocusTarget::Input;
    shell
}

fn test_shell() -> TuiShell {
    let mut shell = TuiShell::new(TuiConfig {
        title: "Test Task".to_owned(),
        agent_id: "test.agent".to_owned(),
        locale: "en".to_owned(),
        project_name: String::new(),
    });
    shell.push_activity(">> Tool Call: search".to_owned());
    shell.push_activity(">> Result: found 3 items".to_owned());
    shell.set_progress(42);
    shell.increment_tools();
    shell
}

/// Shell with feed content — triggers active layout (tabs, sidebar, metrics).
fn active_shell() -> TuiShell {
    let mut shell = test_shell();
    shell
        .feed_mut()
        .push_block(crate::feed::FeedBlock::completed(
            crate::feed::BlockKind::System,
            "test block".into(),
        ));
    shell
}

fn render_to_buffer(shell: &mut TuiShell, width: u16, height: u16) -> String {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| shell.render_frame(frame)).unwrap();
    let buf = terminal.backend().buffer();
    let mut output = String::new();
    for y in 0..height {
        for x in 0..width {
            let cell = &buf[(x, y)];
            output.push_str(cell.symbol());
        }
        output.push('\n');
    }
    output
}

// ── Shell defaults ──────────────────────────────────────────

#[test]
fn tui_shell_new_has_correct_defaults() {
    let shell = TuiShell::new(TuiConfig {
        title: "Test".to_owned(),
        agent_id: "test.agent".to_owned(),
        locale: "en".to_owned(),
        project_name: String::new(),
    });
    assert_eq!(shell.state(), TuiState::Running);
    assert_eq!(shell.progress, 0);
    assert!(shell.log_lines.is_empty());
    assert_eq!(shell.tool_count, 0);
    assert!(!shell.is_input_enabled());
    assert_eq!(shell.theme().id(), "catppuccin");
}

#[test]
fn set_theme_switches_active_theme() {
    let mut shell = empty_shell();
    assert_eq!(shell.theme().id(), "catppuccin");
    shell.set_theme("dracula");
    assert_eq!(shell.theme().id(), "dracula");
    shell.set_theme("nord");
    assert_eq!(shell.theme().id(), "nord");
}

#[test]
fn set_theme_unknown_falls_back_to_default() {
    let mut shell = empty_shell();
    shell.set_theme("nonexistent");
    assert_eq!(shell.theme().id(), "catppuccin");
}

#[test]
fn push_activity_appends_line() {
    let mut shell = empty_shell();
    shell.push_activity("hello".to_owned());
    shell.push_activity("world".to_owned());
    assert_eq!(shell.log_lines.len(), 2);
}

#[test]
fn push_activity_bounds_buffer() {
    let mut shell = empty_shell();
    for i in 0..10_001 {
        shell.push_activity(format!("line {i}"));
    }
    assert!(shell.log_lines.len() <= 10_000);
    assert_eq!(shell.log_lines.last().unwrap(), "line 10000");
}

#[test]
fn set_state_transitions() {
    let mut shell = empty_shell();
    assert_eq!(shell.state(), TuiState::Running);
    shell.set_state(TuiState::Paused);
    assert_eq!(shell.state(), TuiState::Paused);
}

#[test]
fn set_progress_clamps_to_100() {
    let mut shell = empty_shell();
    shell.set_progress(150);
    assert_eq!(shell.progress, 100);
}

#[test]
fn increment_tools_counts() {
    let mut shell = empty_shell();
    shell.increment_tools();
    shell.increment_tools();
    shell.increment_tools();
    assert_eq!(shell.tool_count, 3);
}

// ── Key handling ────────────────────────────────────────────

#[test]
fn handle_key_q_requires_confirmation() {
    let mut shell = empty_shell();
    assert!(!shell.should_quit());

    shell.handle_key(KeyCode::Char('q'));
    assert!(!shell.should_quit());
    assert!(shell.is_quit_pending());

    shell.handle_key(KeyCode::Char('n'));
    assert!(!shell.should_quit());
    assert!(!shell.is_quit_pending());

    shell.handle_key(KeyCode::Char('q'));
    shell.handle_key(KeyCode::Char('y'));
    assert!(shell.should_quit());
}

#[test]
fn handle_key_help_toggles_modal() {
    let mut shell = empty_shell();
    assert!(!shell.help_modal_visible);
    shell.handle_key(KeyCode::Char('?'));
    assert!(shell.help_modal_visible);
    shell.handle_key(KeyCode::Char('a'));
    assert!(!shell.help_modal_visible);
}

#[test]
fn handle_key_unknown_returns_not_handled() {
    let mut shell = empty_shell();
    let result = shell.handle_key(KeyCode::Char('x'));
    assert_eq!(result, PluginKeyAction::NotHandled);
}

#[test]
fn plugin_keybinding_dispatch() {
    let mut shell = empty_shell();
    shell.set_plugin_keybindings(vec![RegisteredKeybinding {
        key: 'p',
        description: "Pause".to_owned(),
        plugin_id: "test.guided".to_owned(),
        active_states: vec!["Running".to_owned()],
    }]);

    let binding = shell.find_active_binding('p', "Running");
    assert!(binding.is_some());
    assert_eq!(binding.unwrap().plugin_id, "test.guided");

    let binding = shell.find_active_binding('p', "Complete");
    assert!(binding.is_none());
}

#[test]
fn apply_plugin_action_set_state() {
    let mut shell = empty_shell();
    shell.apply_plugin_action(&PluginKeyAction::SetState(TuiState::Paused));
    assert_eq!(shell.state(), TuiState::Paused);
}

#[test]
fn apply_plugin_action_enter_text_input() {
    let mut shell = empty_shell();
    shell.apply_plugin_action(&PluginKeyAction::EnterTextInput {
        prompt: "Type feedback:".to_owned(),
    });
    assert!(shell.is_input_enabled());
    assert!(shell.log_lines.last().unwrap().contains("feedback"));
}

#[test]
fn apply_plugin_action_show_message() {
    let mut shell = empty_shell();
    shell.apply_plugin_action(&PluginKeyAction::ShowMessage("Agent paused.".to_owned()));
    assert!(shell.log_lines.last().unwrap().contains("paused"));
}

// ── Chat input ──────────────────────────────────────────────

#[test]
fn chat_input_type_and_send() {
    let mut shell = interactive_shell();
    shell.handle_key(KeyCode::Char('f'));
    shell.handle_key(KeyCode::Char('i'));
    shell.handle_key(KeyCode::Char('x'));
    assert_eq!(shell.text_input_buffer(), "fix");

    shell.handle_key(KeyCode::Enter);
    assert_eq!(shell.take_text_input(), Some("fix".to_owned()));
    assert!(shell.text_input_buffer().is_empty());
}

#[test]
fn chat_input_esc_exits_focus_preserves_buffer() {
    let mut shell = interactive_shell();
    shell.handle_key(KeyCode::Char('a'));
    shell.handle_key(KeyCode::Char('b'));
    assert_eq!(shell.text_input_buffer(), "ab");

    // Esc exits input focus but PRESERVES buffer text
    shell.handle_key(KeyCode::Esc);
    assert_eq!(shell.text_input_buffer(), "ab");
    assert_eq!(shell.focus, super::types::FocusTarget::Activity);
}

#[test]
fn chat_input_backspace_deletes() {
    let mut shell = interactive_shell();
    shell.handle_key(KeyCode::Char('a'));
    shell.handle_key(KeyCode::Char('b'));
    shell.handle_key(KeyCode::Backspace);
    assert_eq!(shell.text_input_buffer(), "a");
}

#[test]
fn chat_input_empty_enter_does_nothing() {
    let mut shell = interactive_shell();
    shell.handle_key(KeyCode::Enter);
    assert!(shell.take_text_input().is_none());
}

#[test]
fn chat_input_keybinding_while_typing_goes_to_buffer() {
    let mut shell = interactive_shell();
    shell.handle_key(KeyCode::Char('h'));
    shell.handle_key(KeyCode::Char('p'));
    assert_eq!(shell.text_input_buffer(), "hp");
}

#[test]
fn no_chat_input_when_input_disabled() {
    let mut shell = empty_shell();
    shell.handle_key(KeyCode::Char('a'));
    assert!(shell.text_input_buffer().is_empty());
}

#[test]
fn explicit_enter_text_input_activates() {
    let mut shell = interactive_shell();
    shell.apply_plugin_action(&PluginKeyAction::EnterTextInput {
        prompt: "Feedback:".to_owned(),
    });
    assert!(shell.is_input_enabled());
    shell.handle_key(KeyCode::Char('x'));
    assert_eq!(shell.text_input_buffer(), "x");
}

#[test]
fn chat_input_ctrl_c_clears_all_text() {
    let mut shell = interactive_shell();
    shell.handle_key(KeyCode::Char('h'));
    shell.handle_key(KeyCode::Char('e'));
    shell.handle_key(KeyCode::Char('l'));
    assert_eq!(shell.text_input_buffer(), "hel");

    shell.handle_key_with_modifiers(KeyCode::Char('c'), KeyModifiers::CONTROL);
    assert!(shell.text_input_buffer().is_empty());
    // Focus stays on Input
    assert_eq!(shell.focus, super::types::FocusTarget::Input);
}

#[test]
fn chat_input_ctrl_z_undoes_last_change() {
    let mut shell = interactive_shell();
    shell.handle_key(KeyCode::Char('a'));
    shell.handle_key(KeyCode::Char('b'));
    assert_eq!(shell.text_input_buffer(), "ab");

    shell.handle_key_with_modifiers(KeyCode::Char('z'), KeyModifiers::CONTROL);
    assert_eq!(shell.text_input_buffer(), "a");

    shell.handle_key_with_modifiers(KeyCode::Char('z'), KeyModifiers::CONTROL);
    assert!(shell.text_input_buffer().is_empty());
}

#[test]
fn mouse_click_on_input_area_sets_focus() {
    use ratatui::crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

    let mut shell = interactive_shell();
    // Set a known input area
    shell.input_area = ratatui::layout::Rect::new(0, 20, 80, 3);
    shell.focus = super::types::FocusTarget::Activity;

    // Click inside input area
    shell.handle_mouse(MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 21,
        modifiers: KeyModifiers::NONE,
    });
    assert_eq!(shell.focus, super::types::FocusTarget::Input);
}

#[test]
fn mouse_click_outside_input_area_sets_activity_focus() {
    use ratatui::crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

    let mut shell = interactive_shell();
    shell.input_area = ratatui::layout::Rect::new(0, 20, 80, 3);
    shell.focus = super::types::FocusTarget::Input;

    // Click outside input area (in feed area)
    shell.handle_mouse(MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 5,
        modifiers: KeyModifiers::NONE,
    });
    assert_eq!(shell.focus, super::types::FocusTarget::Activity);
}

#[test]
fn paste_small_appends_directly() {
    let mut shell = interactive_shell();
    shell.handle_paste("hello world");
    assert_eq!(shell.text_input_buffer(), "hello world");
}

#[test]
fn paste_large_stored_in_full_buffer() {
    let mut shell = interactive_shell();
    let large_text = "line1\nline2\nline3\nline4\nline5\nline6\nline7";
    shell.handle_paste(large_text);

    // Buffer stores the FULL text — render layer handles visual collapse
    assert_eq!(shell.text_input_buffer(), large_text);
}

#[test]
fn paste_respects_max_limit() {
    let mut shell = interactive_shell();
    // Pre-fill buffer close to limit
    let big = "x".repeat(49_990);
    shell.handle_paste(&big);
    assert_eq!(shell.text_input_buffer().len(), 49_990);

    // Paste more — should truncate to MAX_INPUT_CHARS
    shell.handle_paste("y".repeat(100).as_str());
    assert_eq!(shell.text_input_buffer().len(), 50_000);
}

#[test]
fn cursor_left_right_moves_position() {
    let mut shell = interactive_shell();
    shell.handle_key(KeyCode::Char('a'));
    shell.handle_key(KeyCode::Char('b'));
    shell.handle_key(KeyCode::Char('c'));
    assert_eq!(shell.cursor_pos, 3);

    shell.handle_key(KeyCode::Left);
    assert_eq!(shell.cursor_pos, 2);

    shell.handle_key(KeyCode::Left);
    assert_eq!(shell.cursor_pos, 1);

    // Type inserts at cursor
    shell.handle_key(KeyCode::Char('X'));
    assert_eq!(shell.text_input_buffer(), "aXbc");
    assert_eq!(shell.cursor_pos, 2);

    shell.handle_key(KeyCode::Right);
    assert_eq!(shell.cursor_pos, 3);
}

#[test]
fn cursor_home_end_line_boundaries() {
    let mut shell = interactive_shell();
    shell.handle_paste("hello");
    assert_eq!(shell.cursor_pos, 5);

    shell.handle_key(KeyCode::Home);
    assert_eq!(shell.cursor_pos, 0);

    shell.handle_key(KeyCode::End);
    assert_eq!(shell.cursor_pos, 5);
}

#[test]
fn backspace_at_cursor_removes_correct_char() {
    let mut shell = interactive_shell();
    shell.handle_key(KeyCode::Char('a'));
    shell.handle_key(KeyCode::Char('b'));
    shell.handle_key(KeyCode::Char('c'));
    shell.handle_key(KeyCode::Left); // cursor at 2 (before 'c')
    shell.handle_key(KeyCode::Backspace); // removes 'b'
    assert_eq!(shell.text_input_buffer(), "ac");
    assert_eq!(shell.cursor_pos, 1);
}

#[test]
fn delete_at_cursor_removes_char_ahead() {
    let mut shell = interactive_shell();
    shell.handle_key(KeyCode::Char('a'));
    shell.handle_key(KeyCode::Char('b'));
    shell.handle_key(KeyCode::Char('c'));
    shell.handle_key(KeyCode::Home); // cursor at 0
    shell.handle_key(KeyCode::Delete); // removes 'a'
    assert_eq!(shell.text_input_buffer(), "bc");
    assert_eq!(shell.cursor_pos, 0);
}

// ── Rendering snapshot tests ────────────────────────────────

#[test]
fn render_compact_shows_header_with_agent_id() {
    let mut shell = active_shell();
    let output = render_to_buffer(&mut shell, 120, 24);
    assert!(output.contains("test.agent"));
    assert!(output.contains("RUNNING"));
}

#[test]
fn render_compact_idle_shows_dashboard() {
    let mut shell = empty_shell();
    let output = render_to_buffer(&mut shell, 80, 24);
    // Idle mode: shows logo and commands, not activity lines
    assert!(output.contains("Ralph"));
}

#[test]
fn render_compact_shows_metrics() {
    let mut shell = active_shell();
    let output = render_to_buffer(&mut shell, 80, 24);
    assert!(output.contains("Tools: 1"));
}

#[test]
fn render_compact_shows_help_bar() {
    let mut shell = test_shell();
    let output = render_to_buffer(&mut shell, 80, 24);
    assert!(output.contains("[q]"));
    assert!(output.contains("compact"));
}

#[test]
fn render_compact_no_sidebar() {
    let mut shell = test_shell();
    let output = render_to_buffer(&mut shell, 80, 24);
    assert!(!output.contains("Plugins"));
}

#[test]
fn render_standard_shows_sidebar() {
    let mut shell = test_shell();
    // Use a known agent plugin so it appears in sidebar (not hidden)
    shell.set_sidebar_panels(vec![SidebarPanel {
        title: "Claude".to_owned(),
        items: vec![super::types::PanelItem {
            label: Some("Binary".to_owned()),
            value: Some("Available".to_owned()),
            hint: super::types::PanelHint::Indicator,
            severity: super::types::PanelSeverity::Success,
            ..super::types::PanelItem::default()
        }],
        plugin_id: "official.claude".to_owned(),
        is_agent: true,
    }]);
    // Need feed blocks to trigger active layout (sidebar only shows in active mode)
    shell
        .feed_mut()
        .push_block(crate::feed::FeedBlock::completed(
            crate::feed::BlockKind::System,
            "test".into(),
        ));
    let output = render_to_buffer(&mut shell, 140, 40);
    assert!(output.contains("Agents"));
}

#[test]
fn render_wide_shows_control_panel_when_active() {
    let mut shell = TuiShell::new(TuiConfig {
        title: "Fix Bug".to_owned(),
        agent_id: "test.agent".to_owned(),
        locale: "en".to_owned(),
        project_name: String::new(),
    });
    // Control panel only shows when feed has content (active mode)
    shell
        .feed_mut()
        .push_block(crate::feed::FeedBlock::completed(
            crate::feed::BlockKind::System,
            "test".into(),
        ));
    let output = render_to_buffer(&mut shell, 200, 60);
    assert!(output.contains("State"));
    assert!(output.contains("wide"));
}

#[test]
fn render_too_small_shows_error() {
    let mut shell = test_shell();
    let output = render_to_buffer(&mut shell, 60, 20);
    assert!(output.contains("too small"));
}

#[test]
fn render_paused_state_shows_in_header() {
    let mut shell = test_shell();
    // Need feed content to trigger active layout where header shows state badge
    shell
        .feed_mut()
        .push_block(crate::feed::FeedBlock::completed(
            crate::feed::BlockKind::System,
            "test".into(),
        ));
    shell.set_state(TuiState::Paused);
    let output = render_to_buffer(&mut shell, 120, 24);
    assert!(output.contains("PAUSED"));
}

#[test]
fn render_progress_gauge_shows_in_wide_header() {
    let mut shell = active_shell();
    shell.set_progress(75);
    let output = render_to_buffer(&mut shell, 100, 24);
    assert!(output.contains("test.agent"));
}

// ── process_event tests ─────────────────────────────────────

use crate::events::AgentEvent;

#[test]
fn process_event_text_delta_appends() {
    let mut shell = empty_shell();
    shell.process_event(&AgentEvent::TextDelta("Hello".to_owned()));
    assert_eq!(shell.log_lines.len(), 1);
}

#[test]
fn process_event_tool_use_increments_count() {
    let mut shell = empty_shell();
    shell.process_event(&AgentEvent::ToolUse {
        name: "Read".to_owned(),
    });
    assert_eq!(shell.tool_count, 1);
    assert!(shell.log_lines[0].contains("Tool: Read"));
}

#[test]
fn process_event_complete_sets_state() {
    let mut shell = empty_shell();
    shell.process_event(&AgentEvent::Complete { is_error: false });
    assert_eq!(shell.state(), TuiState::Complete);

    let mut shell2 = empty_shell();
    shell2.process_event(&AgentEvent::Complete { is_error: true });
    assert_eq!(shell2.state(), TuiState::Error);
}

#[test]
fn process_event_tool_result_appends() {
    let mut shell = empty_shell();
    shell.process_event(&AgentEvent::ToolResult {
        name: "Bash".to_owned(),
        success: true,
    });
    assert!(shell.log_lines[0].contains("Bash [OK]"));
}

#[test]
fn process_event_system_appends() {
    let mut shell = empty_shell();
    shell.process_event(&AgentEvent::System("Starting session".to_owned()));
    assert!(shell.log_lines[0].contains("Starting session"));
}

#[test]
fn process_event_unknown_skips_empty() {
    let mut shell = empty_shell();
    shell.process_event(&AgentEvent::Unknown(String::new()));
    assert!(shell.log_lines.is_empty());
}

#[test]
fn process_event_sequence() {
    let mut shell = empty_shell();
    shell.process_event(&AgentEvent::System("Init".to_owned()));
    shell.process_event(&AgentEvent::ToolUse {
        name: "Read".to_owned(),
    });
    shell.process_event(&AgentEvent::ToolResult {
        name: "Read".to_owned(),
        success: true,
    });
    shell.process_event(&AgentEvent::TextDelta("Processing...".to_owned()));
    shell.process_event(&AgentEvent::Complete { is_error: false });

    assert_eq!(shell.log_lines.len(), 5);
    assert_eq!(shell.tool_count, 1);
    assert_eq!(shell.state(), TuiState::Complete);
}

// ── Sidebar panel rendering ─────────────────────────────────

#[test]
fn render_standard_with_plugin_panels() {
    let mut shell = test_shell();
    shell.set_sidebar_panels(vec![
        SidebarPanel {
            title: "Claude".to_owned(),
            items: Vec::new(),
            plugin_id: "official.claude".to_owned(),
            is_agent: true,
        },
        SidebarPanel {
            title: "Sprint Status".to_owned(),
            items: Vec::new(),
            plugin_id: "official.bmad".to_owned(),
            is_agent: false,
        },
        SidebarPanel {
            title: "Findings".to_owned(),
            items: Vec::new(),
            plugin_id: "official.findings".to_owned(),
            is_agent: false,
        },
    ]);
    let output = render_to_buffer(&mut shell, 140, 40);
    // Grouped sidebar: Agents, Sprint, Findings
    assert!(output.contains("Agents"));
    assert!(output.contains("Sprint"));
    assert!(output.contains("Findings"));
    // Findings group heading is visible (items are empty in this test)
}

#[test]
fn render_standard_no_sidebar_when_no_panels() {
    let mut shell = active_shell();
    let output = render_to_buffer(&mut shell, 140, 40);
    // No sidebar panels with actionable data → sidebar hidden
    assert!(!output.contains("Agents"), "no Agents group when no panels");
}

#[test]
fn set_sidebar_panels_replaces() {
    let mut shell = empty_shell();
    shell.set_sidebar_panels(vec![SidebarPanel {
        title: "A".to_owned(),
        items: Vec::new(),
        plugin_id: "test".to_owned(),
        is_agent: false,
    }]);
    assert_eq!(shell.sidebar_panels.len(), 1);
    shell.set_sidebar_panels(vec![]);
    assert!(shell.sidebar_panels.is_empty());
}

// ── Help bar with plugin keybindings ────────────────────────

#[test]
fn help_bar_shows_plugin_keybindings() {
    let mut shell = test_shell();
    shell.set_plugin_keybindings(vec![
        RegisteredKeybinding {
            key: 'p',
            description: "Pause".to_owned(),
            plugin_id: "test".to_owned(),
            active_states: vec![],
        },
        RegisteredKeybinding {
            key: 'f',
            description: "Feedback".to_owned(),
            plugin_id: "test".to_owned(),
            active_states: vec!["Paused".to_owned()],
        },
    ]);
    let output = render_to_buffer(&mut shell, 80, 24);
    assert!(
        output.contains("[p]"),
        "help should show [p], got:\n{output}"
    );
    assert!(
        !output.contains("[f]"),
        "help should NOT show [f] in Running, got:\n{output}"
    );
}

#[test]
fn help_bar_shows_state_specific_bindings() {
    let mut shell = test_shell();
    shell.set_plugin_keybindings(vec![RegisteredKeybinding {
        key: 'f',
        description: "Feedback".to_owned(),
        plugin_id: "test".to_owned(),
        active_states: vec!["Paused".to_owned()],
    }]);
    shell.set_state(TuiState::Paused);
    let output = render_to_buffer(&mut shell, 80, 24);
    assert!(
        output.contains("[f]"),
        "help should show [f] when Paused, got:\n{output}"
    );
}

// ── Scroll tests ────────────────────────────────────────────

#[test]
fn follow_mode_enabled_by_default() {
    let shell = test_shell();
    assert!(shell.is_follow_mode());
}

#[test]
fn scroll_up_disables_follow_mode() {
    let mut shell = test_shell();
    shell.scroll_feed_up();
    assert!(!shell.is_follow_mode());
}

#[test]
fn scroll_to_bottom_re_enables_follow() {
    let mut shell = test_shell();
    shell.scroll_feed_up();
    assert!(!shell.is_follow_mode());
    shell.scroll_feed_to_bottom();
    assert!(shell.is_follow_mode());
}

#[test]
fn page_up_disables_follow() {
    let mut shell = test_shell();
    shell.scroll_feed_page_up();
    assert!(!shell.is_follow_mode());
}

#[test]
fn scroll_to_top_disables_follow() {
    let mut shell = test_shell();
    shell.scroll_feed_to_top();
    assert!(!shell.is_follow_mode());
}

#[test]
fn focus_keys_handled_in_key_handler() {
    let mut shell = test_shell();
    shell
        .feed_mut()
        .push_block(crate::feed::FeedBlock::completed(
            crate::feed::BlockKind::System,
            "block-a".into(),
        ));
    shell
        .feed_mut()
        .push_block(crate::feed::FeedBlock::completed(
            crate::feed::BlockKind::System,
            "block-b".into(),
        ));

    let result = shell.handle_key(KeyCode::Char('j'));
    assert_eq!(result, PluginKeyAction::Handled);
    assert!(!shell.is_follow_mode());
    assert!(shell.focused_block().is_some());

    let result = shell.handle_key(KeyCode::Char('G'));
    assert_eq!(result, PluginKeyAction::Handled);
    assert!(shell.is_follow_mode());

    shell.handle_key(KeyCode::Char('j'));
    let result = shell.handle_key(KeyCode::Enter);
    assert_eq!(result, PluginKeyAction::Handled);

    let result = shell.handle_key(KeyCode::Esc);
    assert_eq!(result, PluginKeyAction::Handled);
    assert!(shell.focused_block().is_none());
    assert!(shell.is_follow_mode());
}

#[test]
fn mouse_scroll_disables_follow() {
    use ratatui::crossterm::event::{MouseEvent, MouseEventKind};
    let mut shell = test_shell();
    shell.handle_mouse(MouseEvent {
        kind: MouseEventKind::ScrollUp,
        column: 0,
        row: 0,
        modifiers: ratatui::crossterm::event::KeyModifiers::NONE,
    });
    assert!(!shell.is_follow_mode());
}

// ── Toast tests ─────────────────────────────────────────────

#[test]
fn toast_info_creates_info_toast() {
    let mut shell = empty_shell();
    shell.toast_info("Test message".to_owned());
    assert_eq!(shell.toasts.len(), 1);
    assert_eq!(shell.toasts[0].level, ToastLevel::Info);
    assert_eq!(shell.toasts[0].message, "Test message");
}

#[test]
fn toast_success_creates_success_toast() {
    let mut shell = empty_shell();
    shell.toast_success("Done!".to_owned());
    assert_eq!(shell.toasts.len(), 1);
    assert_eq!(shell.toasts[0].level, ToastLevel::Success);
}

#[test]
fn show_error_modal_creates_error_toast_and_activity() {
    let mut shell = empty_shell();
    shell.show_error_modal("Title", "Details");
    assert_eq!(shell.toasts.len(), 1);
    assert_eq!(shell.toasts[0].level, ToastLevel::Error);
    assert!(shell.log_lines.iter().any(|l| l.contains("Title")));
}

#[test]
fn toasts_expire_after_ticks() {
    let mut shell = empty_shell();
    shell.show_toast("Temp".to_owned(), ToastLevel::Info);
    assert_eq!(shell.toasts.len(), 1);
    shell.toasts[0].remaining_ticks = 1;
    shell.toasts.retain_mut(|t| {
        t.remaining_ticks = t.remaining_ticks.saturating_sub(1);
        t.remaining_ticks > 0
    });
    assert!(shell.toasts.is_empty());
}

// ── Idle dashboard ──────────────────────────────────────────

#[test]
fn render_idle_shows_dashboard() {
    let mut shell = empty_shell();
    let output = render_to_buffer(&mut shell, 120, 30);
    assert!(output.contains("Ralph"), "idle should show logo");
}

#[test]
fn header_shows_project_name() {
    let mut shell = TuiShell::new(TuiConfig {
        title: "Test".to_owned(),
        agent_id: "test.agent".to_owned(),
        locale: "en".to_owned(),
        project_name: "my-cool-project".to_owned(),
    });
    shell.set_state(TuiState::Complete);
    let output = render_to_buffer(&mut shell, 120, 30);
    assert!(
        output.contains("my-cool-project"),
        "header should show project name, got:\n{output}"
    );
}

#[test]
fn idle_hints_from_plugins_replace_hardcoded() {
    use super::types::IdleHint;
    let mut shell = empty_shell();
    shell.set_idle_hints(vec![
        IdleHint {
            command: "/run".to_owned(),
            description: "start autonomous loop".to_owned(),
        },
        IdleHint {
            command: "/run 5.3".to_owned(),
            description: "execute story 5.3".to_owned(),
        },
    ]);
    let output = render_to_buffer(&mut shell, 120, 30);
    assert!(
        output.contains("/run"),
        "idle should show plugin hint /run, got:\n{output}"
    );
    assert!(
        output.contains("5.3"),
        "idle should show plugin hint 5.3, got:\n{output}"
    );
    // Should NOT show "No project found" when hints are present
    assert!(
        !output.contains("No project found"),
        "should not show no-project when hints exist"
    );
}

#[test]
fn idle_no_hints_shows_no_project() {
    let mut shell = empty_shell();
    // No idle hints set → shows no-project message
    let output = render_to_buffer(&mut shell, 120, 30);
    assert!(
        output.contains("No project found"),
        "idle with no hints should show no-project, got:\n{output}"
    );
}

#[test]
fn startup_banner_creates_feed_block() {
    let mut shell = empty_shell();
    shell.set_sidebar_panels(vec![SidebarPanel {
        title: "Side".to_owned(),
        items: Vec::new(),
        plugin_id: "a".to_owned(),
        is_agent: false,
    }]);
    shell.push_startup_banner();
    assert!(
        !shell.feed().is_empty(),
        "startup banner should create a feed block"
    );
    assert_eq!(
        shell.feed().blocks()[0].kind,
        crate::feed::BlockKind::System
    );
}

#[test]
fn config_tab_shows_all_plugin_details() {
    use super::types::{PanelHint, PanelItem, PanelSeverity};
    let mut shell = empty_shell();
    // Need feed to trigger active layout where Config tab is accessible
    shell
        .feed_mut()
        .push_block(crate::feed::FeedBlock::completed(
            crate::feed::BlockKind::System,
            "test".into(),
        ));
    shell.set_sidebar_panels(vec![SidebarPanel {
        title: "Claude".to_owned(),
        items: vec![
            PanelItem {
                label: Some("Binary".to_owned()),
                value: Some("Available".to_owned()),
                hint: PanelHint::Indicator,
                severity: PanelSeverity::Success,
                ..PanelItem::default()
            },
            PanelItem {
                hint: PanelHint::Pairs,
                pairs: vec![("Mode".to_owned(), "-p (prompt)".to_owned())],
                ..PanelItem::default()
            },
        ],
        plugin_id: "official.claude".to_owned(),
        is_agent: true,
    }]);
    shell.active_tab = TuiTab::Config;
    let output = render_to_buffer(&mut shell, 140, 40);
    // Config tab shows ALL details including Pairs
    assert!(output.contains("official.claude"));
    assert!(output.contains("Mode"));
}

// ── Tab rendering ──────────────────────────────────────────

#[test]
fn tab_bar_shows_counts() {
    let mut shell = active_shell();
    shell.touch_file("Read".to_owned());
    shell.touch_file("Edit".to_owned());
    shell.push_log("some log".to_owned());
    let output = render_to_buffer(&mut shell, 140, 40);
    // Tab bar should show file count
    assert!(
        output.contains("Files (2)"),
        "tab bar should show Files (2), got:\n{output}"
    );
}

#[test]
fn files_tab_shows_tool_icons() {
    let mut shell = active_shell();
    shell.touch_file("Edit".to_owned());
    shell.touch_file("Read".to_owned());
    shell.touch_file("Bash".to_owned());
    shell.active_tab = TuiTab::Files;
    let output = render_to_buffer(&mut shell, 140, 40);
    assert!(
        output.contains("3 tools used"),
        "Files tab should show tool count"
    );
}

#[test]
fn log_tab_shows_line_numbers() {
    let mut shell = active_shell();
    shell.push_log(">> Tool: Read".to_owned());
    shell.push_log("some output".to_owned());
    shell.active_tab = TuiTab::Log;
    let output = render_to_buffer(&mut shell, 140, 40);
    // active_shell has 2 log lines + 2 we added = 4
    assert!(output.contains("4 lines"), "Log tab should show line count");
}

#[test]
fn config_tab_shows_grouped_sections() {
    let mut shell = active_shell();
    shell.set_sidebar_panels(vec![SidebarPanel {
        title: "Test".to_owned(),
        items: Vec::new(),
        plugin_id: "official.claude".to_owned(),
        is_agent: true,
    }]);
    shell.active_tab = TuiTab::Config;
    let output = render_to_buffer(&mut shell, 140, 40);
    assert!(
        output.contains("Session"),
        "Config tab should have Session heading"
    );
    assert!(
        output.contains("Plugins"),
        "Config tab should have Plugins heading"
    );
    assert!(
        output.contains("official.claude"),
        "Config tab should list plugin IDs"
    );
}

#[test]
fn sidebar_groups_all_agents_in_one_section() {
    let mut shell = test_shell();
    // Need feed blocks to enter active layout
    shell
        .feed_mut()
        .push_block(crate::feed::FeedBlock::completed(
            crate::feed::BlockKind::System,
            "test".into(),
        ));
    shell.set_sidebar_panels(vec![
        SidebarPanel {
            title: "Claude".to_owned(),
            items: vec![super::types::PanelItem {
                label: Some("Binary".to_owned()),
                value: Some("Available".to_owned()),
                hint: super::types::PanelHint::Indicator,
                severity: super::types::PanelSeverity::Success,
                ..super::types::PanelItem::default()
            }],
            plugin_id: "official.claude".to_owned(),
            is_agent: true,
        },
        SidebarPanel {
            title: "Codex".to_owned(),
            items: vec![super::types::PanelItem {
                label: Some("Binary".to_owned()),
                value: Some("Not found".to_owned()),
                hint: super::types::PanelHint::Indicator,
                severity: super::types::PanelSeverity::Error,
                ..super::types::PanelItem::default()
            }],
            plugin_id: "official.codex".to_owned(),
            is_agent: true,
        },
    ]);
    let output = render_to_buffer(&mut shell, 140, 40);
    assert!(
        output.contains("Agents"),
        "sidebar should have Agents group"
    );
    assert!(output.contains("available"));
    assert!(output.contains("not found"));
}

#[test]
fn files_tab_empty_shows_placeholder() {
    let mut shell = active_shell();
    shell.active_tab = TuiTab::Files;
    let output = render_to_buffer(&mut shell, 140, 40);
    assert!(output.contains("No files touched"));
}

#[test]
fn log_tab_empty_shows_placeholder() {
    let mut shell = empty_shell();
    // Feed block triggers active layout but log_lines is empty
    shell
        .feed_mut()
        .push_block(crate::feed::FeedBlock::completed(
            crate::feed::BlockKind::System,
            "x".into(),
        ));
    shell.active_tab = TuiTab::Log;
    let output = render_to_buffer(&mut shell, 140, 40);
    assert!(output.contains("No log output"));
}
