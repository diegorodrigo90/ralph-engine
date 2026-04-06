//! Tests for the TUI shell — unit + render snapshot tests.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::crossterm::event::KeyCode;

use super::*;

fn empty_shell() -> TuiShell {
    TuiShell::new(TuiConfig {
        title: "Test".to_owned(),
        agent_id: "test.agent".to_owned(),
        locale: "en".to_owned(),
    })
}

/// Shell with input enabled (simulates guided plugin active).
fn interactive_shell() -> TuiShell {
    let mut shell = empty_shell();
    shell.enable_input();
    shell
}

fn test_shell() -> TuiShell {
    let mut shell = TuiShell::new(TuiConfig {
        title: "Test Task".to_owned(),
        agent_id: "test.agent".to_owned(),
        locale: "en".to_owned(),
    });
    shell.push_activity(">> Tool Call: search".to_owned());
    shell.push_activity(">> Result: found 3 items".to_owned());
    shell.set_progress(42);
    shell.increment_tools();
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
    });
    assert_eq!(shell.state(), TuiState::Running);
    assert_eq!(shell.progress, 0);
    assert!(shell.activity_lines.is_empty());
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
    assert_eq!(shell.activity_lines.len(), 2);
}

#[test]
fn push_activity_bounds_buffer() {
    let mut shell = empty_shell();
    for i in 0..10_001 {
        shell.push_activity(format!("line {i}"));
    }
    assert!(shell.activity_lines.len() <= 10_000);
    assert_eq!(shell.activity_lines.last().unwrap(), "line 10000");
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
    assert!(shell.activity_lines.last().unwrap().contains("feedback"));
}

#[test]
fn apply_plugin_action_show_message() {
    let mut shell = empty_shell();
    shell.apply_plugin_action(&PluginKeyAction::ShowMessage("Agent paused.".to_owned()));
    assert!(shell.activity_lines.last().unwrap().contains("paused"));
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
fn chat_input_esc_clears() {
    let mut shell = interactive_shell();
    shell.handle_key(KeyCode::Char('a'));
    shell.handle_key(KeyCode::Char('b'));
    assert_eq!(shell.text_input_buffer(), "ab");

    shell.handle_key(KeyCode::Esc);
    assert!(shell.text_input_buffer().is_empty());
    assert!(shell.take_text_input().is_none());
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

// ── Rendering snapshot tests ────────────────────────────────

#[test]
fn render_compact_shows_header_with_agent_id() {
    let mut shell = test_shell();
    let output = render_to_buffer(&mut shell, 120, 24);
    assert!(output.contains("test.agent"));
    assert!(output.contains("RUNNING"));
}

#[test]
fn render_compact_shows_activity_lines() {
    let mut shell = test_shell();
    let output = render_to_buffer(&mut shell, 80, 24);
    assert!(output.contains("Tool Call: search"));
    assert!(output.contains("found 3 items"));
}

#[test]
fn render_compact_shows_metrics() {
    let mut shell = test_shell();
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
    shell.set_sidebar_panels(vec![SidebarPanel {
        title: "TestPanel".to_owned(),
        lines: vec!["line1".to_owned()],
        items: Vec::new(),
        plugin_id: "test.plugin".to_owned(),
    }]);
    let output = render_to_buffer(&mut shell, 140, 40);
    // Grouped sidebar: "test.plugin" → Tools group
    assert!(output.contains("Tools"));
    assert!(output.contains("standard"));
}

#[test]
fn render_wide_shows_control_panel() {
    let mut shell = TuiShell::new(TuiConfig {
        title: "Fix Bug".to_owned(),
        agent_id: "test.agent".to_owned(),
        locale: "en".to_owned(),
    });
    let output = render_to_buffer(&mut shell, 200, 60);
    assert!(output.contains("State"));
    assert!(output.contains("RUNNING"));
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
    shell.set_state(TuiState::Paused);
    let output = render_to_buffer(&mut shell, 120, 24);
    assert!(output.contains("PAUSED"));
}

#[test]
fn render_progress_gauge_shows_in_wide_header() {
    let mut shell = test_shell();
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
    assert_eq!(shell.activity_lines.len(), 1);
}

#[test]
fn process_event_tool_use_increments_count() {
    let mut shell = empty_shell();
    shell.process_event(&AgentEvent::ToolUse {
        name: "Read".to_owned(),
    });
    assert_eq!(shell.tool_count, 1);
    assert!(shell.activity_lines[0].contains("Tool: Read"));
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
    assert!(shell.activity_lines[0].contains("Bash [OK]"));
}

#[test]
fn process_event_system_appends() {
    let mut shell = empty_shell();
    shell.process_event(&AgentEvent::System("Starting session".to_owned()));
    assert!(shell.activity_lines[0].contains("Starting session"));
}

#[test]
fn process_event_unknown_skips_empty() {
    let mut shell = empty_shell();
    shell.process_event(&AgentEvent::Unknown(String::new()));
    assert!(shell.activity_lines.is_empty());
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

    assert_eq!(shell.activity_lines.len(), 5);
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
            lines: vec!["Available".to_owned()],
            items: Vec::new(),
            plugin_id: "official.claude".to_owned(),
        },
        SidebarPanel {
            title: "Sprint Status".to_owned(),
            lines: vec!["Story 5.3: in-progress".to_owned()],
            items: Vec::new(),
            plugin_id: "official.bmad".to_owned(),
        },
        SidebarPanel {
            title: "Findings".to_owned(),
            lines: vec!["3 issues found".to_owned()],
            items: Vec::new(),
            plugin_id: "official.findings".to_owned(),
        },
    ]);
    let output = render_to_buffer(&mut shell, 140, 40);
    // Grouped sidebar: Agents, Sprint, Findings
    assert!(output.contains("Agents"));
    assert!(output.contains("Sprint"));
    assert!(output.contains("Findings"));
    assert!(output.contains("3 issues"));
}

#[test]
fn render_standard_empty_panels_shows_placeholder() {
    let mut shell = test_shell();
    let output = render_to_buffer(&mut shell, 140, 40);
    assert!(output.contains("no panels"));
}

#[test]
fn set_sidebar_panels_replaces() {
    let mut shell = empty_shell();
    shell.set_sidebar_panels(vec![SidebarPanel {
        title: "A".to_owned(),
        lines: vec![],
        items: Vec::new(),
        plugin_id: "test".to_owned(),
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
    assert!(shell.activity_lines.iter().any(|l| l.contains("Title")));
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

// ── Main panels (zone_hint="main") ──────────────────────────

#[test]
fn set_main_panels_stores_panels() {
    let mut shell = empty_shell();
    assert!(shell.main_panels.is_empty());
    shell.set_main_panels(vec![SidebarPanel {
        title: "Sprint".to_owned(),
        lines: vec!["Story 5.3: in-progress".to_owned()],
        items: Vec::new(),
        plugin_id: "test.bmad".to_owned(),
    }]);
    assert_eq!(shell.main_panels.len(), 1);
    assert_eq!(shell.main_panels[0].title, "Sprint");
}

#[test]
fn set_main_panels_replaces() {
    let mut shell = empty_shell();
    shell.set_main_panels(vec![SidebarPanel {
        title: "A".to_owned(),
        lines: vec![],
        items: Vec::new(),
        plugin_id: "test".to_owned(),
    }]);
    shell.set_main_panels(vec![]);
    assert!(shell.main_panels.is_empty());
}

#[test]
fn render_main_panels_when_idle() {
    let mut shell = empty_shell();
    shell.set_main_panels(vec![SidebarPanel {
        title: "Dashboard".to_owned(),
        lines: vec!["Status: healthy".to_owned()],
        items: Vec::new(),
        plugin_id: "test.dashboard".to_owned(),
    }]);
    // No activity + main_panels → should render main panels, not idle dashboard
    let output = render_to_buffer(&mut shell, 120, 30);
    assert!(
        output.contains("Dashboard"),
        "main panels should render when idle, got:\n{output}"
    );
    assert!(output.contains("Status"));
}

#[test]
fn render_idle_dashboard_when_no_main_panels() {
    let mut shell = empty_shell();
    // No activity + no main_panels → idle dashboard with logo
    let output = render_to_buffer(&mut shell, 120, 30);
    assert!(
        output.contains("Ralph"),
        "idle dashboard should show logo when no main panels"
    );
}

#[test]
fn startup_banner_counts_both_panel_types() {
    let mut shell = empty_shell();
    shell.set_sidebar_panels(vec![SidebarPanel {
        title: "Side".to_owned(),
        lines: vec![],
        items: Vec::new(),
        plugin_id: "a".to_owned(),
    }]);
    shell.set_main_panels(vec![SidebarPanel {
        title: "Main".to_owned(),
        lines: vec![],
        items: Vec::new(),
        plugin_id: "b".to_owned(),
    }]);
    shell.push_startup_banner();
    let banner = shell.activity_lines.join(" ");
    assert!(
        banner.contains("2 panels"),
        "banner should count sidebar + main panels"
    );
}

#[test]
fn render_main_panels_with_typed_blocks() {
    use super::types::{PanelHint, PanelItem, PanelSeverity};
    let mut shell = empty_shell();
    shell.set_main_panels(vec![SidebarPanel {
        title: "Sprint".to_owned(),
        lines: vec![],
        items: vec![
            PanelItem {
                label: Some("Progress".to_owned()),
                hint: PanelHint::Bar,
                numeric: Some(75),
                severity: PanelSeverity::Success,
                ..PanelItem::default()
            },
            PanelItem {
                label: Some("Stories".to_owned()),
                value: Some("3/5".to_owned()),
                hint: PanelHint::Inline,
                ..PanelItem::default()
            },
        ],
        plugin_id: "test.bmad".to_owned(),
    }]);
    let output = render_to_buffer(&mut shell, 120, 30);
    assert!(output.contains("Sprint"));
    assert!(output.contains("75%"));
}

// ── Tab rendering ──────────────────────────────────────────

#[test]
fn tab_bar_shows_counts() {
    let mut shell = test_shell();
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
    let mut shell = empty_shell();
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
    let mut shell = empty_shell();
    shell.push_log(">> Tool: Read".to_owned());
    shell.push_log("some output".to_owned());
    shell.active_tab = TuiTab::Log;
    let output = render_to_buffer(&mut shell, 140, 40);
    assert!(output.contains("2 lines"), "Log tab should show line count");
}

#[test]
fn config_tab_shows_grouped_sections() {
    let mut shell = test_shell();
    shell.set_sidebar_panels(vec![SidebarPanel {
        title: "Test".to_owned(),
        lines: vec![],
        items: Vec::new(),
        plugin_id: "official.claude".to_owned(),
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
    shell.set_sidebar_panels(vec![
        SidebarPanel {
            title: "Claude".to_owned(),
            lines: vec!["Available".to_owned()],
            items: Vec::new(),
            plugin_id: "official.claude".to_owned(),
        },
        SidebarPanel {
            title: "Codex".to_owned(),
            lines: vec!["Not found".to_owned()],
            items: Vec::new(),
            plugin_id: "official.codex".to_owned(),
        },
    ]);
    let output = render_to_buffer(&mut shell, 140, 40);
    assert!(
        output.contains("Agents"),
        "sidebar should have Agents group"
    );
    // Both agent lines should be under the same group
    assert!(output.contains("Available"));
    assert!(output.contains("Not found"));
}

#[test]
fn files_tab_empty_shows_placeholder() {
    let mut shell = empty_shell();
    shell.active_tab = TuiTab::Files;
    let output = render_to_buffer(&mut shell, 140, 40);
    assert!(output.contains("No files touched"));
}

#[test]
fn log_tab_empty_shows_placeholder() {
    let mut shell = empty_shell();
    shell.active_tab = TuiTab::Log;
    let output = render_to_buffer(&mut shell, 140, 40);
    assert!(output.contains("No log output"));
}
