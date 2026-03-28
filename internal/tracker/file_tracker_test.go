package tracker

import (
	"os"
	"path/filepath"
	"testing"
)

const sampleSprintStatus = `epics:
  - id: "65"
    title: "Permission System"
    status: "in-progress"
    stories:
      - id: "65.1"
        title: "Custom Roles CRUD"
        status: "done"
      - id: "65.2"
        title: "User Permission Grant/Deny"
        status: "done"
      - id: "65.3"
        title: "Permission UI Components"
        status: "ready-for-dev"
      - id: "65.4"
        title: "Impersonation Flow"
        status: "ready-for-dev"
      - id: "65.5"
        title: "API Key Permissions"
        status: "backlog"
  - id: "63"
    title: "Event Broker"
    status: "in-progress"
    stories:
      - id: "63.4"
        title: "Consumer Error Handling"
        status: "in-progress"
      - id: "63.5"
        title: "Dead Letter Queue"
        status: "ready-for-dev"
`

func writeTestSprintStatus(t *testing.T, dir, content string) string {
	t.Helper()
	path := filepath.Join(dir, "sprint-status.yaml")
	if err := os.WriteFile(path, []byte(content), 0644); err != nil {
		t.Fatalf("writing test sprint-status.yaml: %v", err)
	}
	return path
}

func TestFileTrackerNextStoryPicksInProgressFirst(t *testing.T) {
	dir := t.TempDir()
	writeTestSprintStatus(t, dir, sampleSprintStatus)

	ft := NewFileTracker(dir, "sprint-status.yaml")
	story, err := ft.NextStory()
	if err != nil {
		t.Fatalf("NextStory() error: %v", err)
	}
	if story == nil {
		t.Fatal("NextStory() returned nil, expected in-progress story")
	}
	// Should pick 63.4 (in-progress) before 65.3 (ready-for-dev)
	if story.ID != "63.4" {
		t.Errorf("NextStory() = %q, want %q (in-progress first)", story.ID, "63.4")
	}
	if story.Status != StatusInProgress {
		t.Errorf("NextStory() status = %q, want %q", story.Status, StatusInProgress)
	}
}

func TestFileTrackerListPending(t *testing.T) {
	dir := t.TempDir()
	writeTestSprintStatus(t, dir, sampleSprintStatus)

	ft := NewFileTracker(dir, "sprint-status.yaml")
	pending, err := ft.ListPending()
	if err != nil {
		t.Fatalf("ListPending() error: %v", err)
	}

	// Expected: 63.4 (in-progress), 65.3, 65.4, 63.5 (ready-for-dev)
	if len(pending) != 4 {
		t.Errorf("ListPending() = %d stories, want 4", len(pending))
	}

	// First should be in-progress
	if len(pending) > 0 && pending[0].Status != StatusInProgress {
		t.Errorf("first pending story status = %q, want %q", pending[0].Status, StatusInProgress)
	}
}

func TestFileTrackerListAll(t *testing.T) {
	dir := t.TempDir()
	writeTestSprintStatus(t, dir, sampleSprintStatus)

	ft := NewFileTracker(dir, "sprint-status.yaml")
	all, err := ft.ListAll()
	if err != nil {
		t.Fatalf("ListAll() error: %v", err)
	}

	// 5 stories in epic 65 + 2 in epic 63 = 7
	if len(all) != 7 {
		t.Errorf("ListAll() = %d stories, want 7", len(all))
	}
}

func TestFileTrackerMarkComplete(t *testing.T) {
	dir := t.TempDir()
	writeTestSprintStatus(t, dir, sampleSprintStatus)

	ft := NewFileTracker(dir, "sprint-status.yaml")

	err := ft.MarkComplete("63.4")
	if err != nil {
		t.Fatalf("MarkComplete() error: %v", err)
	}

	// Re-read and verify
	all, _ := ft.ListAll()
	for _, s := range all {
		if s.ID == "63.4" && s.Status != StatusDone {
			t.Errorf("story 63.4 status = %q, want %q after MarkComplete", s.Status, StatusDone)
		}
	}
}

func TestFileTrackerMarkInProgress(t *testing.T) {
	dir := t.TempDir()
	writeTestSprintStatus(t, dir, sampleSprintStatus)

	ft := NewFileTracker(dir, "sprint-status.yaml")

	err := ft.MarkInProgress("65.3")
	if err != nil {
		t.Fatalf("MarkInProgress() error: %v", err)
	}

	all, _ := ft.ListAll()
	for _, s := range all {
		if s.ID == "65.3" && s.Status != StatusInProgress {
			t.Errorf("story 65.3 status = %q, want %q after MarkInProgress", s.Status, StatusInProgress)
		}
	}
}

func TestFileTrackerMissingFileReturnsEmpty(t *testing.T) {
	dir := t.TempDir()
	ft := NewFileTracker(dir, "sprint-status.yaml")

	stories, err := ft.ListAll()
	if err != nil {
		t.Fatalf("ListAll() error on missing file: %v", err)
	}
	if len(stories) != 0 {
		t.Errorf("ListAll() = %d stories, want 0 for missing file", len(stories))
	}
}

func TestFileTrackerNextStoryReturnsNilWhenAllDone(t *testing.T) {
	allDone := `epics:
  - id: "1"
    title: "Done Epic"
    status: "done"
    stories:
      - id: "1.1"
        title: "Done Story"
        status: "done"
`
	dir := t.TempDir()
	writeTestSprintStatus(t, dir, allDone)

	ft := NewFileTracker(dir, "sprint-status.yaml")
	story, err := ft.NextStory()
	if err != nil {
		t.Fatalf("NextStory() error: %v", err)
	}
	if story != nil {
		t.Errorf("NextStory() = %v, want nil when all done", story)
	}
}

func TestFileTrackerEpicInfoPropagated(t *testing.T) {
	dir := t.TempDir()
	writeTestSprintStatus(t, dir, sampleSprintStatus)

	ft := NewFileTracker(dir, "sprint-status.yaml")
	all, _ := ft.ListAll()

	for _, s := range all {
		if s.EpicID == "" {
			t.Errorf("story %q has empty EpicID", s.ID)
		}
		if s.EpicTitle == "" {
			t.Errorf("story %q has empty EpicTitle", s.ID)
		}
	}
}

func TestFileTrackerMarkCompleteIdempotent(t *testing.T) {
	dir := t.TempDir()
	writeTestSprintStatus(t, dir, sampleSprintStatus)

	ft := NewFileTracker(dir, "sprint-status.yaml")

	// Mark 63.4 complete twice — second call should be a no-op without error.
	if err := ft.MarkComplete("63.4"); err != nil {
		t.Fatalf("first MarkComplete() error: %v", err)
	}
	if err := ft.MarkComplete("63.4"); err != nil {
		t.Fatalf("second MarkComplete() error: %v", err)
	}

	all, _ := ft.ListAll()
	for _, s := range all {
		if s.ID == "63.4" && s.Status != StatusDone {
			t.Errorf("story 63.4 status = %q, want %q after double MarkComplete", s.Status, StatusDone)
		}
	}
}

func TestFileTrackerMarkInProgressNonExistentStory(t *testing.T) {
	dir := t.TempDir()
	writeTestSprintStatus(t, dir, sampleSprintStatus)

	ft := NewFileTracker(dir, "sprint-status.yaml")

	err := ft.MarkInProgress("999.99")
	if err == nil {
		t.Error("MarkInProgress() on non-existent story should return error")
	}
}

func TestFileTrackerRevertToReadyAlreadyReady(t *testing.T) {
	dir := t.TempDir()
	writeTestSprintStatus(t, dir, sampleSprintStatus)

	ft := NewFileTracker(dir, "sprint-status.yaml")

	// 65.3 is already ready-for-dev — reverting should succeed without changing anything.
	if err := ft.RevertToReady("65.3"); err != nil {
		t.Fatalf("RevertToReady() on already-ready story error: %v", err)
	}

	all, _ := ft.ListAll()
	for _, s := range all {
		if s.ID == "65.3" && s.Status != StatusReadyForDev {
			t.Errorf("story 65.3 status = %q, want %q after RevertToReady no-op", s.Status, StatusReadyForDev)
		}
	}
}

func TestFileTrackerNextStoryAllDoneReturnsNil(t *testing.T) {
	allDone := `epics:
  - id: "1"
    title: "Epic One"
    status: "done"
    stories:
      - id: "1.1"
        title: "Story A"
        status: "done"
      - id: "1.2"
        title: "Story B"
        status: "done"
  - id: "2"
    title: "Epic Two"
    status: "done"
    stories:
      - id: "2.1"
        title: "Story C"
        status: "done"
`
	dir := t.TempDir()
	writeTestSprintStatus(t, dir, allDone)

	ft := NewFileTracker(dir, "sprint-status.yaml")
	story, err := ft.NextStory()
	if err != nil {
		t.Fatalf("NextStory() error: %v", err)
	}
	if story != nil {
		t.Errorf("NextStory() = %v, want nil when all stories done", story)
	}
}

func TestFileTrackerNextStoryPrefersInProgressOverReady(t *testing.T) {
	mixed := `epics:
  - id: "10"
    title: "Mixed Epic"
    status: "in-progress"
    stories:
      - id: "10.1"
        title: "Ready Story"
        status: "ready-for-dev"
      - id: "10.2"
        title: "In Progress Story"
        status: "in-progress"
      - id: "10.3"
        title: "Another Ready"
        status: "ready-for-dev"
`
	dir := t.TempDir()
	writeTestSprintStatus(t, dir, mixed)

	ft := NewFileTracker(dir, "sprint-status.yaml")
	story, err := ft.NextStory()
	if err != nil {
		t.Fatalf("NextStory() error: %v", err)
	}
	if story == nil {
		t.Fatal("NextStory() returned nil, expected in-progress story")
	}
	if story.ID != "10.2" {
		t.Errorf("NextStory() = %q, want %q (in-progress preferred)", story.ID, "10.2")
	}
	if story.Status != StatusInProgress {
		t.Errorf("NextStory() status = %q, want %q", story.Status, StatusInProgress)
	}
}

func TestFileTrackerListPendingReturnsOnlyActionable(t *testing.T) {
	mixed := `epics:
  - id: "5"
    title: "Test Epic"
    status: "in-progress"
    stories:
      - id: "5.1"
        title: "Done"
        status: "done"
      - id: "5.2"
        title: "Blocked"
        status: "blocked"
      - id: "5.3"
        title: "Backlog"
        status: "backlog"
      - id: "5.4"
        title: "Ready"
        status: "ready-for-dev"
      - id: "5.5"
        title: "In Progress"
        status: "in-progress"
      - id: "5.6"
        title: "Review"
        status: "review"
`
	dir := t.TempDir()
	writeTestSprintStatus(t, dir, mixed)

	ft := NewFileTracker(dir, "sprint-status.yaml")
	pending, err := ft.ListPending()
	if err != nil {
		t.Fatalf("ListPending() error: %v", err)
	}

	// Only ready-for-dev (5.4) and in-progress (5.5) are actionable.
	if len(pending) != 2 {
		t.Errorf("ListPending() = %d stories, want 2 (only actionable)", len(pending))
		for _, s := range pending {
			t.Logf("  %s: %s", s.ID, s.Status)
		}
	}

	// In-progress should come first.
	if len(pending) >= 1 && pending[0].ID != "5.5" {
		t.Errorf("first pending = %q, want %q (in-progress first)", pending[0].ID, "5.5")
	}
}

func TestFileTrackerMalformedYAML(t *testing.T) {
	dir := t.TempDir()
	malformed := `epics: [{{invalid yaml content!!!`
	writeTestSprintStatus(t, dir, malformed)

	ft := NewFileTracker(dir, "sprint-status.yaml")
	_, err := ft.ListAll()
	if err == nil {
		t.Error("ListAll() should return error for malformed YAML")
	}
}

func TestFileTrackerEmptyStatusFile(t *testing.T) {
	dir := t.TempDir()
	writeTestSprintStatus(t, dir, "")

	ft := NewFileTracker(dir, "sprint-status.yaml")
	stories, err := ft.ListAll()
	if err != nil {
		t.Fatalf("ListAll() error on empty file: %v", err)
	}
	if len(stories) != 0 {
		t.Errorf("ListAll() = %d stories, want 0 for empty file", len(stories))
	}
}

func TestFileTrackerMissingStatusFile(t *testing.T) {
	dir := t.TempDir()
	// No file written — file does not exist.

	ft := NewFileTracker(dir, "sprint-status.yaml")

	// ListAll should return empty, no error.
	stories, err := ft.ListAll()
	if err != nil {
		t.Fatalf("ListAll() error: %v", err)
	}
	if len(stories) != 0 {
		t.Errorf("ListAll() = %d, want 0", len(stories))
	}

	// NextStory should return nil, no error.
	story, err := ft.NextStory()
	if err != nil {
		t.Fatalf("NextStory() error: %v", err)
	}
	if story != nil {
		t.Errorf("NextStory() = %v, want nil for missing file", story)
	}

	// MarkComplete should return error (file not found).
	err = ft.MarkComplete("1.1")
	if err == nil {
		t.Error("MarkComplete() on missing file should return error")
	}
}

func TestStoryIDsWithSpecialCharacters(t *testing.T) {
	specialIDs := `epics:
  - id: "special-epic"
    title: "Special ID Epic"
    status: "in-progress"
    stories:
      - id: "1.2.3"
        title: "Dotted ID"
        status: "ready-for-dev"
      - id: "story-with-dashes"
        title: "Dashed ID"
        status: "ready-for-dev"
      - id: "story_with_underscores"
        title: "Underscored ID"
        status: "in-progress"
      - id: "26.3"
        title: "Standard Dot ID"
        status: "ready-for-dev"
`
	dir := t.TempDir()
	writeTestSprintStatus(t, dir, specialIDs)

	ft := NewFileTracker(dir, "sprint-status.yaml")

	tests := []struct {
		name     string
		storyID  string
		action   string
		wantErr  bool
	}{
		{"dotted ID mark complete", "1.2.3", "complete", false},
		{"dashed ID mark in-progress", "story-with-dashes", "in-progress", false},
		{"underscored ID revert", "story_with_underscores", "revert", false},
		{"standard dot ID mark complete", "26.3", "complete", false},
		{"non-existent special ID", "no.such.id", "complete", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Re-create the file for each subtest to avoid state leakage.
			writeTestSprintStatus(t, dir, specialIDs)

			var err error
			switch tt.action {
			case "complete":
				err = ft.MarkComplete(tt.storyID)
			case "in-progress":
				err = ft.MarkInProgress(tt.storyID)
			case "revert":
				err = ft.RevertToReady(tt.storyID)
			}

			if tt.wantErr && err == nil {
				t.Errorf("expected error for story ID %q, got nil", tt.storyID)
			}
			if !tt.wantErr && err != nil {
				t.Errorf("unexpected error for story ID %q: %v", tt.storyID, err)
			}
		})
	}
}
