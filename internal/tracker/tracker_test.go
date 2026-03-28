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
		name   string
		story  Story
		want   bool
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
