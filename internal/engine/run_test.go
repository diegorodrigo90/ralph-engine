package engine

import (
	"context"
	"encoding/json"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/diegorodrigo90/ralph-engine/internal/hooks"
	"github.com/diegorodrigo90/ralph-engine/internal/state"
	"github.com/diegorodrigo90/ralph-engine/internal/tracker"
)

// mockTracker implements tracker.TaskTracker for testing.
type mockTracker struct {
	stories []tracker.Story
	calls   map[string]int
}

func newMockTracker(stories []tracker.Story) *mockTracker {
	return &mockTracker{
		stories: stories,
		calls:   make(map[string]int),
	}
}

func (m *mockTracker) NextStory() (*tracker.Story, error) {
	m.calls["NextStory"]++
	for i := range m.stories {
		if m.stories[i].IsActionable() {
			return &m.stories[i], nil
		}
	}
	return nil, nil
}

func (m *mockTracker) MarkComplete(storyID string) error {
	m.calls["MarkComplete"]++
	for i := range m.stories {
		if m.stories[i].ID == storyID {
			m.stories[i].Status = tracker.StatusDone
		}
	}
	return nil
}

func (m *mockTracker) MarkInProgress(storyID string) error {
	m.calls["MarkInProgress"]++
	for i := range m.stories {
		if m.stories[i].ID == storyID {
			m.stories[i].Status = tracker.StatusInProgress
		}
	}
	return nil
}

func (m *mockTracker) RevertToReady(storyID string) error {
	m.calls["RevertToReady"]++
	for i := range m.stories {
		if m.stories[i].ID == storyID {
			m.stories[i].Status = tracker.StatusReadyForDev
		}
	}
	return nil
}

func (m *mockTracker) ListPending() ([]tracker.Story, error) {
	var pending []tracker.Story
	for _, s := range m.stories {
		if s.IsActionable() {
			pending = append(pending, s)
		}
	}
	return pending, nil
}

func (m *mockTracker) ListAll() ([]tracker.Story, error) {
	return m.stories, nil
}

func TestRunExitsAllCompleteWhenNoStories(t *testing.T) {
	e, _ := New(EngineOpts{
		ProjectDir: t.TempDir(),
		StateDir:   t.TempDir(),
	})

	mt := newMockTracker([]tracker.Story{
		{ID: "1.1", Status: tracker.StatusDone},
	})

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	result := e.Run(ctx, mt, nil)

	if result.ExitReason != ExitAllComplete {
		t.Errorf("ExitReason = %q, want %q", result.ExitReason, ExitAllComplete)
	}
}

func TestRunExitsOnUserInterrupt(t *testing.T) {
	e, _ := New(EngineOpts{
		ProjectDir: t.TempDir(),
		StateDir:   t.TempDir(),
		Binary:     "nonexistent-binary-12345", // Will fail immediately
	})

	mt := newMockTracker([]tracker.Story{
		{ID: "1.1", Status: tracker.StatusReadyForDev, Title: "Test Story"},
	})

	// Cancel immediately.
	ctx, cancel := context.WithCancel(context.Background())
	cancel()

	result := e.Run(ctx, mt, nil)

	if result.ExitReason != ExitUserInterrupt {
		t.Errorf("ExitReason = %q, want %q", result.ExitReason, ExitUserInterrupt)
	}
}

func TestRunEmitsEvents(t *testing.T) {
	e, _ := New(EngineOpts{
		ProjectDir: t.TempDir(),
		StateDir:   t.TempDir(),
	})

	mt := newMockTracker([]tracker.Story{}) // Empty = all complete

	var events []EngineEvent
	handler := func(event EngineEvent) {
		events = append(events, event)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	e.Run(ctx, mt, handler)

	if len(events) == 0 {
		t.Error("Run() should emit at least one event")
	}

	// First event should be info about engine starting.
	if events[0].Type != "info" {
		t.Errorf("first event type = %q, want %q", events[0].Type, "info")
	}
}

func TestRunResultTracksSessionCount(t *testing.T) {
	e, _ := New(EngineOpts{
		ProjectDir: t.TempDir(),
		StateDir:   t.TempDir(),
	})

	mt := newMockTracker(nil) // nil stories = all complete

	ctx := context.Background()
	result := e.Run(ctx, mt, nil)

	if result.SessionsRun != 0 {
		t.Errorf("SessionsRun = %d, want 0 (no stories to run)", result.SessionsRun)
	}
}

// ── saveHandoff tests ──

func TestSaveHandoff_WritesFile(t *testing.T) {
	dir := t.TempDir()
	e, _ := New(EngineOpts{
		ProjectDir: dir,
		StateDir:   dir,
	})

	s := &state.Engine{
		SessionNumber:       3,
		StoriesCompletedThisRun: 2,
		StoriesCompletedTotal:   5,
		SessionCostUSD:      1.25,
	}
	story := &tracker.Story{
		ID:      "65-8",
		Title:   "Test Story",
		EpicID:  "65",
	}

	var events []EngineEvent
	emit := func(eventType, msg string) {
		events = append(events, EngineEvent{Type: eventType, Message: msg})
	}

	e.saveHandoff(s, story, 42, 5*time.Minute, emit)

	// Verify file was written.
	handoffPath := dir + "/handoff-65-8.json"
	data, err := os.ReadFile(handoffPath)
	if err != nil {
		t.Fatalf("handoff file should exist: %v", err)
	}

	var handoff map[string]interface{}
	if err := json.Unmarshal(data, &handoff); err != nil {
		t.Fatalf("handoff should be valid JSON: %v", err)
	}

	if handoff["story_id"] != "65-8" {
		t.Errorf("story_id = %v, want 65-8", handoff["story_id"])
	}
	if handoff["story_title"] != "Test Story" {
		t.Errorf("story_title = %v, want Test Story", handoff["story_title"])
	}
	if handoff["epic_id"] != "65" {
		t.Errorf("epic_id = %v, want 65", handoff["epic_id"])
	}
	if handoff["exit_reason"] != "usage_limit" {
		t.Errorf("exit_reason = %v, want usage_limit", handoff["exit_reason"])
	}
	// JSON unmarshals numbers as float64.
	if handoff["tool_calls"] != float64(42) {
		t.Errorf("tool_calls = %v, want 42", handoff["tool_calls"])
	}
	if handoff["stories_done_this_run"] != float64(2) {
		t.Errorf("stories_done_this_run = %v, want 2", handoff["stories_done_this_run"])
	}
	if handoff["stories_done_total"] != float64(5) {
		t.Errorf("stories_done_total = %v, want 5", handoff["stories_done_total"])
	}

	// Verify emit was called with info about handoff path.
	foundInfo := false
	for _, ev := range events {
		if ev.Type == "info" && strings.Contains(ev.Message, "Handoff saved") {
			foundInfo = true
		}
	}
	if !foundInfo {
		t.Error("should emit info about handoff save")
	}
}

func TestSaveHandoff_InvalidDir_EmitsWarning(t *testing.T) {
	e, _ := New(EngineOpts{
		ProjectDir: t.TempDir(),
		StateDir:   "/nonexistent/path/that/does/not/exist",
	})

	s := &state.Engine{}
	story := &tracker.Story{ID: "1.1", Title: "Test"}

	var events []EngineEvent
	emit := func(eventType, msg string) {
		events = append(events, EngineEvent{Type: eventType, Message: msg})
	}

	e.saveHandoff(s, story, 0, 0, emit)

	foundWarn := false
	for _, ev := range events {
		if ev.Type == "warn" && strings.Contains(ev.Message, "Could not write handoff") {
			foundWarn = true
		}
	}
	if !foundWarn {
		t.Error("should emit warning when handoff write fails")
	}
}

// ── allGatesWouldBeSkipped tests ──

func TestAllGatesSkipped_NilHooks(t *testing.T) {
	e := &Engine{opts: EngineOpts{Hooks: nil}}
	if e.allGatesWouldBeSkipped([]string{"apps/api/src/main.ts"}) {
		t.Error("nil hooks should not report all skipped")
	}
}

func TestAllGatesSkipped_EmptySteps(t *testing.T) {
	e := &Engine{opts: EngineOpts{
		Hooks: &hooks.HooksConfig{
			QualityGates: hooks.HookPhase{Steps: []hooks.HookStep{}},
		},
	}}
	if e.allGatesWouldBeSkipped([]string{"file.ts"}) {
		t.Error("empty steps should not report all skipped")
	}
}

func TestAllGatesSkipped_AllPathsMatch(t *testing.T) {
	e := &Engine{opts: EngineOpts{
		Hooks: &hooks.HooksConfig{
			QualityGates: hooks.HookPhase{Steps: []hooks.HookStep{
				{Name: "tests", Run: "pnpm test", Required: true, Paths: []string{"apps/**"}},
				{Name: "build", Run: "pnpm build", Required: true, Paths: []string{"apps/**"}},
			}},
		},
	}}
	if e.allGatesWouldBeSkipped([]string{"apps/api/src/main.ts"}) {
		t.Error("should NOT be all-skipped when files match gate paths")
	}
}

func TestAllGatesSkipped_NoPathsMatch(t *testing.T) {
	e := &Engine{opts: EngineOpts{
		Hooks: &hooks.HooksConfig{
			QualityGates: hooks.HookPhase{Steps: []hooks.HookStep{
				{Name: "tests", Run: "pnpm test", Required: true, Paths: []string{"apps/**"}},
				{Name: "build", Run: "pnpm build", Required: true, Paths: []string{"packages/**"}},
			}},
		},
	}}
	if !e.allGatesWouldBeSkipped([]string{".ralph-engine/state.json", "_bmad-output/sprint-status.yaml"}) {
		t.Error("should be all-skipped when NO files match any gate path")
	}
}

func TestAllGatesSkipped_GateWithoutPaths_NeverSkipped(t *testing.T) {
	e := &Engine{opts: EngineOpts{
		Hooks: &hooks.HooksConfig{
			QualityGates: hooks.HookPhase{Steps: []hooks.HookStep{
				{Name: "tests", Run: "pnpm test", Required: true, Paths: []string{"apps/**"}},
				{Name: "lint", Run: "eslint .", Required: true}, // No paths = always runs
			}},
		},
	}}
	if e.allGatesWouldBeSkipped([]string{".ralph-engine/state.json"}) {
		t.Error("gate without paths filter should never be skipped")
	}
}

func TestAllGatesSkipped_EmptyRunCommand(t *testing.T) {
	e := &Engine{opts: EngineOpts{
		Hooks: &hooks.HooksConfig{
			QualityGates: hooks.HookPhase{Steps: []hooks.HookStep{
				{Name: "disabled", Run: "", Paths: []string{"apps/**"}},
				{Name: "whitespace", Run: "  ", Paths: []string{"apps/**"}},
			}},
		},
	}}
	if !e.allGatesWouldBeSkipped([]string{"apps/web/page.tsx"}) {
		t.Error("steps with empty run commands should be treated as non-existent")
	}
}

func TestAllGatesSkipped_NilChangedFiles(t *testing.T) {
	e := &Engine{opts: EngineOpts{
		Hooks: &hooks.HooksConfig{
			QualityGates: hooks.HookPhase{Steps: []hooks.HookStep{
				{Name: "tests", Run: "pnpm test", Required: true, Paths: []string{"apps/**"}},
			}},
		},
	}}
	if e.allGatesWouldBeSkipped(nil) {
		t.Error("nil changedFiles should not report all skipped")
	}
}

func TestAllGatesSkipped_MixedPathsOneMatches(t *testing.T) {
	e := &Engine{opts: EngineOpts{
		Hooks: &hooks.HooksConfig{
			QualityGates: hooks.HookPhase{Steps: []hooks.HookStep{
				{Name: "ts-tests", Run: "pnpm test", Required: true, Paths: []string{"apps/**"}},
				{Name: "py-tests", Run: "pytest", Required: true, Paths: []string{"workers/**"}},
			}},
		},
	}}
	if e.allGatesWouldBeSkipped([]string{"workers/scrapper-py/main.py"}) {
		t.Error("at least one gate matches workers/** — should not be all skipped")
	}
}

func TestAllGatesSkipped_EmptyChangedFiles(t *testing.T) {
	e := &Engine{opts: EngineOpts{
		Hooks: &hooks.HooksConfig{
			QualityGates: hooks.HookPhase{Steps: []hooks.HookStep{
				{Name: "tests", Run: "pnpm test", Required: true, Paths: []string{"apps/**"}},
			}},
		},
	}}
	// Empty list (not nil) — no files changed at all.
	if !e.allGatesWouldBeSkipped([]string{}) {
		t.Error("empty changedFiles list should report all skipped")
	}
}

func TestAllGatesSkipped_DoubleStarGlobEdgeCases(t *testing.T) {
	e := &Engine{opts: EngineOpts{
		Hooks: &hooks.HooksConfig{
			QualityGates: hooks.HookPhase{Steps: []hooks.HookStep{
				{Name: "graphql", Run: "codegen", Required: true, Paths: []string{"**/*.graphql"}},
			}},
		},
	}}

	tests := []struct {
		name    string
		files   []string
		skipped bool
	}{
		{"graphql file matches", []string{"apps/api/schema.graphql"}, false},
		{"non-graphql file skips", []string{"apps/api/service.ts"}, true},
		{"mixed files, one matches", []string{"README.md", "schema.graphql"}, false},
		{"deeply nested graphql matches", []string{"a/b/c/d/schema.graphql"}, false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := e.allGatesWouldBeSkipped(tt.files)
			if got != tt.skipped {
				t.Errorf("allGatesWouldBeSkipped(%v) = %v, want %v", tt.files, got, tt.skipped)
			}
		})
	}
}

// ── formatLimit tests ──

func TestFormatLimit(t *testing.T) {
	tests := []struct {
		n    int
		want string
	}{
		{0, "unlimited"},
		{-1, "unlimited"},
		{1, "1"},
		{100, "100"},
	}
	for _, tt := range tests {
		got := formatLimit(tt.n)
		if got != tt.want {
			t.Errorf("formatLimit(%d) = %q, want %q", tt.n, got, tt.want)
		}
	}
}

// ── splitLines tests ──

func TestSplitLines(t *testing.T) {
	tests := []struct {
		input string
		want  int
	}{
		{"", 0},
		{"one line", 1},
		{"line1\nline2", 2},
		{"line1\nline2\n", 2},
		{"a\nb\nc\nd\ne", 5},
		{"\n\n\n", 3},                         // 3 empty lines
		{"content\n\ncontent", 3},             // empty line in middle
		{strings.Repeat("x", 10000), 1},       // very long single line
	}
	for _, tt := range tests {
		got := splitLines(tt.input)
		if len(got) != tt.want {
			t.Errorf("splitLines(%q) = %d lines, want %d", tt.input[:min(len(tt.input), 20)], len(got), tt.want)
		}
	}
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

// ── Engine construction tests ──

func TestNewEngine_Defaults(t *testing.T) {
	eng, err := New(EngineOpts{ProjectDir: t.TempDir()})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if eng.opts.Binary != "claude" {
		t.Errorf("default binary = %q, want 'claude'", eng.opts.Binary)
	}
	if eng.opts.MaxFailures != 3 {
		t.Errorf("default max failures = %d, want 3", eng.opts.MaxFailures)
	}
	if eng.opts.StoriesPerSession != 4 {
		t.Errorf("default stories/session = %d, want 4", eng.opts.StoriesPerSession)
	}
	if eng.Status() != StatusIdle {
		t.Errorf("initial status = %v, want idle", eng.Status())
	}
}

func TestNewEngine_MissingProjectDir(t *testing.T) {
	_, err := New(EngineOpts{})
	if err == nil {
		t.Error("expected error for empty ProjectDir")
	}
}

func TestNewEngine_CustomConfig(t *testing.T) {
	eng, err := New(EngineOpts{
		ProjectDir:        t.TempDir(),
		Binary:            "codex",
		MaxFailures:       10,
		CooldownSeconds:   60,
		StoriesPerSession: 2,
		Model:             "sonnet",
		AllowedTools:      "Read,Write",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if eng.opts.Binary != "codex" {
		t.Errorf("binary = %q, want 'codex'", eng.opts.Binary)
	}
	if eng.opts.Model != "sonnet" {
		t.Errorf("model = %q, want 'sonnet'", eng.opts.Model)
	}
}

func TestNewEngine_StateDirDefaultsToProjectDir(t *testing.T) {
	dir := t.TempDir()
	eng, _ := New(EngineOpts{ProjectDir: dir})
	if eng.opts.StateDir != dir {
		t.Errorf("StateDir should default to ProjectDir, got %q", eng.opts.StateDir)
	}
}

func TestEngineStatusThreadSafe(t *testing.T) {
	eng, _ := New(EngineOpts{ProjectDir: t.TempDir()})

	// Simulate concurrent access.
	done := make(chan bool)
	go func() {
		for i := 0; i < 100; i++ {
			eng.setStatus(StatusRunning, "")
		}
		done <- true
	}()
	go func() {
		for i := 0; i < 100; i++ {
			_ = eng.Status()
			_ = eng.ExitInfo()
		}
		done <- true
	}()
	<-done
	<-done
}

// ── Preflight tests ──

func TestPreflightChecks_AllPass(t *testing.T) {
	dir := t.TempDir()
	eng, _ := New(EngineOpts{
		ProjectDir: dir,
		StateDir:   dir,
		Binary:     "echo",
	})

	results := eng.Preflight(nil)
	for _, r := range results {
		if !r.OK {
			t.Errorf("preflight check %q should pass: %s", r.Name, r.Message)
		}
	}
}

func TestPreflightBinaryNotFound(t *testing.T) {
	dir := t.TempDir()
	eng, _ := New(EngineOpts{
		ProjectDir: dir,
		StateDir:   dir,
		Binary:     "definitely-not-a-real-binary-xyz",
	})

	results := eng.Preflight(nil)
	found := false
	for _, r := range results {
		if r.Name == "agent binary" && !r.OK {
			found = true
		}
	}
	if !found {
		t.Error("binary check should fail for nonexistent binary")
	}
}

func TestPreflightNonexistentProjectDir(t *testing.T) {
	eng, _ := New(EngineOpts{
		ProjectDir: "/tmp/definitely-not-exists-xyz-" + t.Name(),
		StateDir:   t.TempDir(),
	})
	results := eng.Preflight(nil)
	if results[0].OK {
		t.Error("project dir check should fail for nonexistent dir")
	}
}

// ── DryRun tests ──

func TestDryRunShowsPendingStories(t *testing.T) {
	dir := t.TempDir()
	e, _ := New(EngineOpts{
		ProjectDir: dir,
		StateDir:   dir,
		DryRun:     true,
	})

	stories := []tracker.Story{
		{ID: "1.1", Title: "First", Status: tracker.StatusReadyForDev, EpicID: "1", EpicTitle: "Epic 1"},
		{ID: "1.2", Title: "Second", Status: tracker.StatusReadyForDev, EpicID: "1", EpicTitle: "Epic 1"},
		{ID: "1.3", Title: "Done", Status: tracker.StatusDone, EpicID: "1", EpicTitle: "Epic 1"},
	}
	mt := newMockTracker(stories)

	var events []EngineEvent
	ctx := context.Background()
	result := e.Run(ctx, mt, func(event EngineEvent) {
		events = append(events, event)
	})

	if result.ExitReason != ExitAllComplete {
		t.Errorf("dry run should return all_complete, got %v", result.ExitReason)
	}

	// Should show 2 pending, 1 done.
	foundPending := false
	for _, ev := range events {
		if strings.Contains(ev.Message, "2 pending") {
			foundPending = true
		}
	}
	if !foundPending {
		t.Error("dry run should report 2 pending stories")
	}
}

func TestDryRunMaxIterationsLimitsList(t *testing.T) {
	dir := t.TempDir()
	e, _ := New(EngineOpts{
		ProjectDir:    dir,
		StateDir:      dir,
		DryRun:        true,
		MaxIterations: 1,
	})

	stories := []tracker.Story{
		{ID: "1.1", Title: "First", Status: tracker.StatusReadyForDev},
		{ID: "1.2", Title: "Second", Status: tracker.StatusReadyForDev},
		{ID: "1.3", Title: "Third", Status: tracker.StatusReadyForDev},
	}
	mt := newMockTracker(stories)

	var events []EngineEvent
	e.Run(context.Background(), mt, func(event EngineEvent) {
		events = append(events, event)
	})

	// Should show "... and N more".
	foundMore := false
	for _, ev := range events {
		if strings.Contains(ev.Message, "more") {
			foundMore = true
		}
	}
	if !foundMore {
		t.Error("dry run with max-iterations=1 should show 'and N more'")
	}
}

func TestDryRunNoStories(t *testing.T) {
	dir := t.TempDir()
	e, _ := New(EngineOpts{
		ProjectDir: dir,
		StateDir:   dir,
		DryRun:     true,
	})

	mt := newMockTracker([]tracker.Story{})

	var events []EngineEvent
	e.Run(context.Background(), mt, func(event EngineEvent) {
		events = append(events, event)
	})

	foundEmpty := false
	for _, ev := range events {
		if strings.Contains(ev.Message, "No pending stories") {
			foundEmpty = true
		}
	}
	if !foundEmpty {
		t.Error("dry run with no stories should say 'No pending stories'")
	}
}

// ── pickStory tests ──

func TestPickStorySingleStoryMode(t *testing.T) {
	e, _ := New(EngineOpts{
		ProjectDir:  t.TempDir(),
		SingleStory: "2.3",
	})

	stories := []tracker.Story{
		{ID: "2.1", Status: tracker.StatusDone},
		{ID: "2.2", Status: tracker.StatusReadyForDev},
		{ID: "2.3", Status: tracker.StatusReadyForDev},
	}
	mt := newMockTracker(stories)

	story, err := e.pickStory(mt)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if story == nil || story.ID != "2.3" {
		t.Errorf("pickStory should return story 2.3, got %v", story)
	}
}

func TestPickStorySingleStoryAlreadyDone(t *testing.T) {
	e, _ := New(EngineOpts{
		ProjectDir:  t.TempDir(),
		SingleStory: "2.1",
	})

	stories := []tracker.Story{
		{ID: "2.1", Status: tracker.StatusDone},
	}
	mt := newMockTracker(stories)

	story, err := e.pickStory(mt)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if story != nil {
		t.Error("pickStory should return nil for already-done single story")
	}
}

func TestPickStorySingleStoryNotFound(t *testing.T) {
	e, _ := New(EngineOpts{
		ProjectDir:  t.TempDir(),
		SingleStory: "99.99",
	})

	mt := newMockTracker([]tracker.Story{
		{ID: "1.1", Status: tracker.StatusReadyForDev},
	})

	story, err := e.pickStory(mt)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if story != nil {
		t.Error("pickStory should return nil for nonexistent single story")
	}
}

func TestPickStoryNormalMode(t *testing.T) {
	e, _ := New(EngineOpts{ProjectDir: t.TempDir()})

	stories := []tracker.Story{
		{ID: "1.1", Status: tracker.StatusDone},
		{ID: "1.2", Status: tracker.StatusReadyForDev, Title: "Next"},
	}
	mt := newMockTracker(stories)

	story, err := e.pickStory(mt)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if story == nil || story.ID != "1.2" {
		t.Errorf("should pick first actionable story, got %v", story)
	}
}

// ── MaxIterations tests ──

func TestRunRespectsMaxIterations(t *testing.T) {
	dir := t.TempDir()
	e, _ := New(EngineOpts{
		ProjectDir:      dir,
		StateDir:        dir,
		MaxIterations:   1,
		CooldownSeconds: 1,
		Binary:          "false",
	})

	stories := []tracker.Story{
		{ID: "1.1", Status: tracker.StatusReadyForDev, Title: "Story 1"},
		{ID: "1.2", Status: tracker.StatusReadyForDev, Title: "Story 2"},
	}
	mt := newMockTracker(stories)

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	result := e.Run(ctx, mt, nil)
	// Should stop after 1 iteration (even if it failed).
	if result.SessionsRun > 1 {
		t.Errorf("should stop after 1 iteration, ran %d sessions", result.SessionsRun)
	}
}

// ── Circuit breaker tests ──

func TestRunTripsCircuitBreaker(t *testing.T) {
	dir := t.TempDir()
	e, _ := New(EngineOpts{
		ProjectDir:      dir,
		StateDir:        dir,
		MaxFailures:     1,  // Trip after 1 failure
		CooldownSeconds: 1,  // Minimal cooldown
		Binary:          "false",
	})

	stories := []tracker.Story{
		{ID: "1.1", Status: tracker.StatusReadyForDev, Title: "Story"},
		{ID: "1.2", Status: tracker.StatusReadyForDev, Title: "Story 2"},
	}
	mt := newMockTracker(stories)

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	result := e.Run(ctx, mt, nil)
	if result.ExitReason != ExitCircuitBreaker {
		t.Errorf("should trip circuit breaker, got %v", result.ExitReason)
	}
}
