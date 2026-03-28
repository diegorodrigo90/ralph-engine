package tracker

import (
	"os"
	"path/filepath"
	"testing"
)

const sampleFlatStatus = `generated: 2026-03-15
last_updated: 2026-03-27
project: myproject

development_status:
  epic-1: done
  1-1-setup-project: done
  1-2-add-auth: done
  epic-2: in-progress
  2-1-create-api: ready-for-dev
  2-2-add-tests: ready-for-dev
  2-3-deploy: backlog
  epic-3: in-progress
  3-1-fix-bug: in-progress
`

func writeFlatFile(t *testing.T, dir, content string) {
	t.Helper()
	path := filepath.Join(dir, "sprint-status.yaml")
	if err := os.WriteFile(path, []byte(content), 0644); err != nil {
		t.Fatalf("writing test file: %v", err)
	}
}

func TestFlatTrackerListAll(t *testing.T) {
	dir := t.TempDir()
	writeFlatFile(t, dir, sampleFlatStatus)

	ft := NewFlatFileTracker(dir, "sprint-status.yaml")
	all, err := ft.ListAll()
	if err != nil {
		t.Fatalf("ListAll() error: %v", err)
	}

	// 6 stories (3 epic entries excluded): 1-1, 1-2, 2-1, 2-2, 2-3, 3-1.
	if len(all) != 6 {
		t.Errorf("ListAll() = %d stories, want 6", len(all))
	}
}

func TestFlatTrackerListPending(t *testing.T) {
	dir := t.TempDir()
	writeFlatFile(t, dir, sampleFlatStatus)

	ft := NewFlatFileTracker(dir, "sprint-status.yaml")
	pending, err := ft.ListPending()
	if err != nil {
		t.Fatalf("ListPending() error: %v", err)
	}

	// 3 actionable: 3-1 (in-progress), 2-1, 2-2 (ready-for-dev).
	if len(pending) != 3 {
		t.Errorf("ListPending() = %d, want 3", len(pending))
	}

	// In-progress first.
	if len(pending) > 0 && pending[0].Status != StatusInProgress {
		t.Errorf("first pending status = %q, want %q", pending[0].Status, StatusInProgress)
	}
}

func TestFlatTrackerNextStory(t *testing.T) {
	dir := t.TempDir()
	writeFlatFile(t, dir, sampleFlatStatus)

	ft := NewFlatFileTracker(dir, "sprint-status.yaml")
	story, err := ft.NextStory()
	if err != nil {
		t.Fatalf("NextStory() error: %v", err)
	}
	if story == nil {
		t.Fatal("NextStory() returned nil")
	}
	// Should pick in-progress first.
	if story.ID != "3.1" {
		t.Errorf("NextStory() = %q, want %q (in-progress first)", story.ID, "3.1")
	}
}

func TestFlatTrackerMarkComplete(t *testing.T) {
	dir := t.TempDir()
	writeFlatFile(t, dir, sampleFlatStatus)

	ft := NewFlatFileTracker(dir, "sprint-status.yaml")
	err := ft.MarkComplete("3.1")
	if err != nil {
		t.Fatalf("MarkComplete() error: %v", err)
	}

	// Verify.
	all, _ := ft.ListAll()
	for _, s := range all {
		if s.ID == "3.1" && s.Status != StatusDone {
			t.Errorf("story 3.1 status = %q, want %q", s.Status, StatusDone)
		}
	}
}

func TestFlatTrackerStoryIDFormat(t *testing.T) {
	dir := t.TempDir()
	writeFlatFile(t, dir, sampleFlatStatus)

	ft := NewFlatFileTracker(dir, "sprint-status.yaml")
	all, _ := ft.ListAll()

	for _, s := range all {
		if s.EpicID == "" {
			t.Errorf("story %q has empty EpicID", s.ID)
		}
		if s.Title == "" {
			t.Errorf("story %q has empty Title", s.ID)
		}
	}
}

func TestFlatTrackerMissingFile(t *testing.T) {
	dir := t.TempDir()
	ft := NewFlatFileTracker(dir, "sprint-status.yaml")

	all, err := ft.ListAll()
	if err != nil {
		t.Fatalf("ListAll() error on missing file: %v", err)
	}
	if len(all) != 0 {
		t.Errorf("ListAll() = %d, want 0 for missing file", len(all))
	}
}

func TestFlatTrackerPreservesComments(t *testing.T) {
	content := `development_status:
  epic-1: in-progress
  1-1-my-story: ready-for-dev  # This is a comment
`
	dir := t.TempDir()
	writeFlatFile(t, dir, content)

	ft := NewFlatFileTracker(dir, "sprint-status.yaml")
	ft.MarkInProgress("1.1")

	// Read raw file and check comment preserved.
	data, _ := os.ReadFile(filepath.Join(dir, "sprint-status.yaml"))
	raw := string(data)
	if !containsSubstr(raw, "# This is a comment") {
		t.Error("MarkInProgress should preserve trailing comments")
	}
}

func TestAutoDetectFlat(t *testing.T) {
	dir := t.TempDir()
	writeFlatFile(t, dir, sampleFlatStatus)

	tk := AutoDetect(dir, "sprint-status.yaml")
	// Should return FlatFileTracker.
	if _, ok := tk.(*FlatFileTracker); !ok {
		t.Errorf("AutoDetect should return FlatFileTracker for flat format, got %T", tk)
	}
}

func TestAutoDetectStructured(t *testing.T) {
	dir := t.TempDir()
	writeTestSprintStatus(t, dir, sampleSprintStatus)

	tk := AutoDetect(dir, "sprint-status.yaml")
	if _, ok := tk.(*FileTracker); !ok {
		t.Errorf("AutoDetect should return FileTracker for structured format, got %T", tk)
	}
}

func containsSubstr(s, substr string) bool {
	for i := 0; i <= len(s)-len(substr); i++ {
		if s[i:i+len(substr)] == substr {
			return true
		}
	}
	return false
}
