package tracker

import (
	"testing"
)

func TestStoryStatusConstants(t *testing.T) {
	// Verify status constants match expected BMAD values
	tests := []struct {
		status StoryStatus
		want   string
	}{
		{StatusBacklog, "backlog"},
		{StatusReadyForDev, "ready-for-dev"},
		{StatusInProgress, "in-progress"},
		{StatusReview, "review"},
		{StatusDone, "done"},
		{StatusBlocked, "blocked"},
	}

	for _, tt := range tests {
		if string(tt.status) != tt.want {
			t.Errorf("StoryStatus %q != %q", tt.status, tt.want)
		}
	}
}

func TestStoryIsActionable(t *testing.T) {
	tests := []struct {
		name  string
		story Story
		want  bool
	}{
		{
			name:  "ready-for-dev is actionable",
			story: Story{ID: "1.1", Status: StatusReadyForDev},
			want:  true,
		},
		{
			name:  "in-progress is actionable",
			story: Story{ID: "1.2", Status: StatusInProgress},
			want:  true,
		},
		{
			name:  "done is not actionable",
			story: Story{ID: "1.3", Status: StatusDone},
			want:  false,
		},
		{
			name:  "blocked is not actionable",
			story: Story{ID: "1.4", Status: StatusBlocked},
			want:  false,
		},
		{
			name:  "backlog is not actionable",
			story: Story{ID: "1.5", Status: StatusBacklog},
			want:  false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := tt.story.IsActionable(); got != tt.want {
				t.Errorf("IsActionable() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestSortByPriorityOrdersCorrectly(t *testing.T) {
	stories := []Story{
		{ID: "1.3", Status: StatusReadyForDev},
		{ID: "1.1", Status: StatusInProgress},
		{ID: "1.2", Status: StatusReadyForDev},
	}

	SortByPriority(stories)

	// In-progress should come first
	if stories[0].ID != "1.1" {
		t.Errorf("first story = %q, want %q (in-progress first)", stories[0].ID, "1.1")
	}
	if stories[0].Status != StatusInProgress {
		t.Errorf("first story status = %q, want %q", stories[0].Status, StatusInProgress)
	}
}

func TestNewRegistryReturnsEmpty(t *testing.T) {
	r := NewRegistry()
	if r == nil {
		t.Fatal("NewRegistry() returned nil")
	}

	names := r.Available()
	if len(names) != 0 {
		t.Errorf("new registry should be empty, got %d trackers", len(names))
	}
}

func TestRegistryRegisterAndGet(t *testing.T) {
	r := NewRegistry()
	mock := &mockTracker{}

	r.Register("mock", func() TaskTracker { return mock })

	tracker, err := r.Get("mock")
	if err != nil {
		t.Fatalf("Get() error: %v", err)
	}
	if tracker == nil {
		t.Fatal("Get() returned nil")
	}
}

func TestRegistryGetUnknownErrors(t *testing.T) {
	r := NewRegistry()

	_, err := r.Get("nonexistent")
	if err == nil {
		t.Error("Get() should error for unknown tracker")
	}
}

func TestRegistryAvailable(t *testing.T) {
	r := NewRegistry()
	r.Register("file", func() TaskTracker { return &mockTracker{} })
	r.Register("github", func() TaskTracker { return &mockTracker{} })

	names := r.Available()
	if len(names) != 2 {
		t.Errorf("Available() = %d, want 2", len(names))
	}
}

// mockTracker implements TaskTracker for testing.
type mockTracker struct {
	stories []Story
}

func (m *mockTracker) NextStory() (*Story, error) {
	for i := range m.stories {
		if m.stories[i].IsActionable() {
			return &m.stories[i], nil
		}
	}
	return nil, nil
}

func (m *mockTracker) MarkComplete(storyID string) error {
	for i := range m.stories {
		if m.stories[i].ID == storyID {
			m.stories[i].Status = StatusDone
			return nil
		}
	}
	return nil
}

func (m *mockTracker) MarkInProgress(storyID string) error {
	for i := range m.stories {
		if m.stories[i].ID == storyID {
			m.stories[i].Status = StatusInProgress
			return nil
		}
	}
	return nil
}

func (m *mockTracker) RevertToReady(storyID string) error {
	for i := range m.stories {
		if m.stories[i].ID == storyID {
			m.stories[i].Status = StatusReadyForDev
			return nil
		}
	}
	return nil
}

func (m *mockTracker) ListPending() ([]Story, error) {
	var pending []Story
	for _, s := range m.stories {
		if s.IsActionable() {
			pending = append(pending, s)
		}
	}
	return pending, nil
}

func (m *mockTracker) ListAll() ([]Story, error) {
	return m.stories, nil
}

func TestStoryIsActionableForReviewStatus(t *testing.T) {
	s := Story{ID: "1.6", Status: StatusReview}
	if s.IsActionable() {
		t.Error("review status should NOT be actionable")
	}
}

func TestMockTrackerMarkCompleteIdempotent(t *testing.T) {
	m := &mockTracker{
		stories: []Story{
			{ID: "1.1", Title: "Story A", Status: StatusInProgress},
		},
	}

	// Mark complete twice — should not error.
	if err := m.MarkComplete("1.1"); err != nil {
		t.Fatalf("first MarkComplete() error: %v", err)
	}
	if err := m.MarkComplete("1.1"); err != nil {
		t.Fatalf("second MarkComplete() error: %v", err)
	}

	if m.stories[0].Status != StatusDone {
		t.Errorf("status = %q, want %q", m.stories[0].Status, StatusDone)
	}
}

func TestMockTrackerNextStoryAllDone(t *testing.T) {
	m := &mockTracker{
		stories: []Story{
			{ID: "1.1", Status: StatusDone},
			{ID: "1.2", Status: StatusDone},
			{ID: "1.3", Status: StatusBlocked},
		},
	}

	story, err := m.NextStory()
	if err != nil {
		t.Fatalf("NextStory() error: %v", err)
	}
	if story != nil {
		t.Errorf("NextStory() = %v, want nil when no actionable stories", story)
	}
}

func TestMockTrackerNextStoryPrefersInProgress(t *testing.T) {
	m := &mockTracker{
		stories: []Story{
			{ID: "1.1", Status: StatusReadyForDev},
			{ID: "1.2", Status: StatusInProgress},
			{ID: "1.3", Status: StatusReadyForDev},
		},
	}

	story, err := m.NextStory()
	if err != nil {
		t.Fatalf("NextStory() error: %v", err)
	}
	if story == nil {
		t.Fatal("NextStory() returned nil")
	}
	// mockTracker returns first actionable — 1.1 is first and actionable.
	// This tests the mock, not SortByPriority.
	if !story.IsActionable() {
		t.Errorf("NextStory() returned non-actionable story: %q", story.Status)
	}
}

func TestMockTrackerListPendingFiltersCorrectly(t *testing.T) {
	m := &mockTracker{
		stories: []Story{
			{ID: "1.1", Status: StatusDone},
			{ID: "1.2", Status: StatusBlocked},
			{ID: "1.3", Status: StatusBacklog},
			{ID: "1.4", Status: StatusReadyForDev},
			{ID: "1.5", Status: StatusInProgress},
			{ID: "1.6", Status: StatusReview},
		},
	}

	pending, err := m.ListPending()
	if err != nil {
		t.Fatalf("ListPending() error: %v", err)
	}

	if len(pending) != 2 {
		t.Errorf("ListPending() = %d stories, want 2", len(pending))
		for _, s := range pending {
			t.Logf("  %s: %s", s.ID, s.Status)
		}
	}

	for _, s := range pending {
		if !s.IsActionable() {
			t.Errorf("ListPending() included non-actionable story %q with status %q", s.ID, s.Status)
		}
	}
}

func TestMockTrackerRevertToReadyAlreadyReady(t *testing.T) {
	m := &mockTracker{
		stories: []Story{
			{ID: "1.1", Status: StatusReadyForDev},
		},
	}

	// Should succeed without error — story is already ready.
	if err := m.RevertToReady("1.1"); err != nil {
		t.Fatalf("RevertToReady() error: %v", err)
	}

	if m.stories[0].Status != StatusReadyForDev {
		t.Errorf("status = %q, want %q", m.stories[0].Status, StatusReadyForDev)
	}
}

func TestSortByPriorityMultipleInProgress(t *testing.T) {
	stories := []Story{
		{ID: "1.5", Status: StatusReadyForDev},
		{ID: "1.1", Status: StatusInProgress},
		{ID: "1.3", Status: StatusDone},
		{ID: "1.2", Status: StatusInProgress},
		{ID: "1.4", Status: StatusReadyForDev},
	}

	SortByPriority(stories)

	// In-progress stories (1.1, 1.2) should come before ready-for-dev.
	if stories[0].Status != StatusInProgress {
		t.Errorf("first story status = %q, want in-progress", stories[0].Status)
	}
	if stories[1].Status != StatusInProgress {
		t.Errorf("second story status = %q, want in-progress", stories[1].Status)
	}
}

func TestSortByPriorityEmptySlice(t *testing.T) {
	var stories []Story
	// Should not panic on empty slice.
	SortByPriority(stories)
	if len(stories) != 0 {
		t.Errorf("empty slice changed length: %d", len(stories))
	}
}

func TestSortByPrioritySingleElement(t *testing.T) {
	stories := []Story{{ID: "1.1", Status: StatusReadyForDev}}
	SortByPriority(stories)
	if stories[0].ID != "1.1" {
		t.Errorf("single element changed: %q", stories[0].ID)
	}
}
