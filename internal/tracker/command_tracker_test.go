package tracker

import (
	"os"
	"path/filepath"
	"testing"
)

func TestCommandTrackerNextStory(t *testing.T) {
	dir := t.TempDir()

	// Create a script that outputs a Story JSON.
	script := `#!/bin/sh
echo '{"id":"42.1","title":"My Custom Story","status":"ready-for-dev","epic_id":"42","epic_title":"Custom Epic"}'`
	scriptPath := filepath.Join(dir, "next.sh")
	os.WriteFile(scriptPath, []byte(script), 0755)

	ct := NewCommandTracker(dir, CommandConfig{
		Next: scriptPath,
	})

	story, err := ct.NextStory()
	if err != nil {
		t.Fatalf("NextStory() error: %v", err)
	}
	if story == nil {
		t.Fatal("NextStory() returned nil")
	}
	if story.ID != "42.1" {
		t.Errorf("ID = %q, want %q", story.ID, "42.1")
	}
	if story.Title != "My Custom Story" {
		t.Errorf("Title = %q, want %q", story.Title, "My Custom Story")
	}
}

func TestCommandTrackerNextStoryEmpty(t *testing.T) {
	dir := t.TempDir()

	script := `#!/bin/sh
echo ''`
	scriptPath := filepath.Join(dir, "next.sh")
	os.WriteFile(scriptPath, []byte(script), 0755)

	ct := NewCommandTracker(dir, CommandConfig{
		Next: scriptPath,
	})

	story, err := ct.NextStory()
	if err != nil {
		t.Fatalf("NextStory() error: %v", err)
	}
	if story != nil {
		t.Errorf("NextStory() should return nil for empty output, got %v", story)
	}
}

func TestCommandTrackerListPending(t *testing.T) {
	dir := t.TempDir()

	script := `#!/bin/sh
echo '[{"id":"1.1","title":"Story A","status":"ready-for-dev"},{"id":"1.2","title":"Story B","status":"ready-for-dev"}]'`
	scriptPath := filepath.Join(dir, "pending.sh")
	os.WriteFile(scriptPath, []byte(script), 0755)

	ct := NewCommandTracker(dir, CommandConfig{
		Pending: scriptPath,
	})

	stories, err := ct.ListPending()
	if err != nil {
		t.Fatalf("ListPending() error: %v", err)
	}
	if len(stories) != 2 {
		t.Errorf("ListPending() = %d, want 2", len(stories))
	}
}

func TestCommandTrackerMarkComplete(t *testing.T) {
	dir := t.TempDir()

	// Script that writes the story ID to a file so we can verify it was called.
	script := `#!/bin/sh
echo "$1" > ` + filepath.Join(dir, "completed.txt")
	scriptPath := filepath.Join(dir, "complete.sh")
	os.WriteFile(scriptPath, []byte(script), 0755)

	ct := NewCommandTracker(dir, CommandConfig{
		Complete: scriptPath,
	})

	err := ct.MarkComplete("42.1")
	if err != nil {
		t.Fatalf("MarkComplete() error: %v", err)
	}
}

func TestCommandTrackerMissingCommandErrors(t *testing.T) {
	ct := NewCommandTracker(".", CommandConfig{})

	_, err := ct.NextStory()
	if err == nil {
		t.Error("NextStory() should error when command not configured")
	}
}

func TestCommandTrackerDefaultTimeout(t *testing.T) {
	ct := NewCommandTracker(".", CommandConfig{})
	if ct.config.Timeout != 30 {
		t.Errorf("Timeout = %d, want 30", ct.config.Timeout)
	}
}

func TestCommandTrackerCustomTimeout(t *testing.T) {
	ct := NewCommandTracker(".", CommandConfig{Timeout: 60})
	if ct.config.Timeout != 60 {
		t.Errorf("Timeout = %d, want 60", ct.config.Timeout)
	}
}
