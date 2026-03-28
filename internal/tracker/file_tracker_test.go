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
