package engine

import (
	"context"
	"testing"
	"time"

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
