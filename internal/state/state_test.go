package state

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

func TestNewReturnsDefaultState(t *testing.T) {
	s := New()

	if s.EngineStatus != StatusRunning {
		t.Errorf("EngineStatus = %q, want %q", s.EngineStatus, StatusRunning)
	}
	if s.SessionNumber != 1 {
		t.Errorf("SessionNumber = %d, want 1", s.SessionNumber)
	}
	if len(s.StoriesCompletedThisSession) != 0 {
		t.Errorf("StoriesCompletedThisSession should be empty, got %d", len(s.StoriesCompletedThisSession))
	}
	if s.LastCheckpoint == "" {
		t.Error("LastCheckpoint should be set")
	}
}

func TestSaveAndLoad(t *testing.T) {
	dir := t.TempDir()
	original := New()
	original.CurrentStory = "65-8"
	original.CurrentTask = 3
	original.CurrentPhase = "implementation"
	original.SSHAvailable = true

	if err := original.Save(dir); err != nil {
		t.Fatalf("Save() error: %v", err)
	}

	loaded, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}

	if loaded.CurrentStory != "65-8" {
		t.Errorf("CurrentStory = %q, want %q", loaded.CurrentStory, "65-8")
	}
	if loaded.CurrentTask != 3 {
		t.Errorf("CurrentTask = %d, want 3", loaded.CurrentTask)
	}
	if loaded.CurrentPhase != "implementation" {
		t.Errorf("CurrentPhase = %q, want %q", loaded.CurrentPhase, "implementation")
	}
	if !loaded.SSHAvailable {
		t.Error("SSHAvailable should be true")
	}
}

func TestLoadReturnsFreshStateWhenFileMissing(t *testing.T) {
	dir := t.TempDir()

	s, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}
	if s.EngineStatus != StatusRunning {
		t.Errorf("EngineStatus = %q, want %q", s.EngineStatus, StatusRunning)
	}
}

func TestLoadReturnsFreshStateOnCorruptedFile(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "state.json")
	if err := os.WriteFile(path, []byte("{invalid json"), 0644); err != nil {
		t.Fatal(err)
	}

	s, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() should not error on corrupted file, got: %v", err)
	}
	if s.EngineStatus != StatusRunning {
		t.Errorf("Should return fresh state, got status %q", s.EngineStatus)
	}
}

func TestSaveWritesAtomically(t *testing.T) {
	dir := t.TempDir()
	s := New()
	s.CurrentStory = "test-story"

	if err := s.Save(dir); err != nil {
		t.Fatalf("Save() error: %v", err)
	}

	// Verify no .tmp file remains
	tmpPath := filepath.Join(dir, "state.json.tmp")
	if _, err := os.Stat(tmpPath); !os.IsNotExist(err) {
		t.Error("Temporary file should not exist after save")
	}

	// Verify main file is valid JSON
	data, err := os.ReadFile(filepath.Join(dir, "state.json"))
	if err != nil {
		t.Fatalf("Failed to read state file: %v", err)
	}
	var parsed Engine
	if err := json.Unmarshal(data, &parsed); err != nil {
		t.Fatalf("State file is not valid JSON: %v", err)
	}
	if parsed.CurrentStory != "test-story" {
		t.Errorf("CurrentStory = %q, want %q", parsed.CurrentStory, "test-story")
	}
}

func TestMarkStoryComplete(t *testing.T) {
	s := New()
	s.CurrentStory = "65-8"
	s.CurrentTask = 5
	s.CurrentPhase = "implementation"

	s.MarkStoryComplete("65-8")

	if s.CurrentStory != "" {
		t.Errorf("CurrentStory should be empty after completion, got %q", s.CurrentStory)
	}
	if s.CurrentTask != 0 {
		t.Errorf("CurrentTask should be 0 after completion, got %d", s.CurrentTask)
	}
	if s.StoriesCompletedTotal != 1 {
		t.Errorf("StoriesCompletedTotal = %d, want 1", s.StoriesCompletedTotal)
	}
	if len(s.StoriesCompletedThisSession) != 1 {
		t.Fatalf("StoriesCompletedThisSession length = %d, want 1", len(s.StoriesCompletedThisSession))
	}
	if s.StoriesCompletedThisSession[0] != "65-8" {
		t.Errorf("StoriesCompletedThisSession[0] = %q, want %q", s.StoriesCompletedThisSession[0], "65-8")
	}
}

func TestMarkBlocked(t *testing.T) {
	s := New()

	s.MarkBlocked("docker down")

	if !s.Blocked {
		t.Error("Blocked should be true")
	}
	if s.BlockedReason != "docker down" {
		t.Errorf("BlockedReason = %q, want %q", s.BlockedReason, "docker down")
	}
	if s.EngineStatus != StatusBlocked {
		t.Errorf("EngineStatus = %q, want %q", s.EngineStatus, StatusBlocked)
	}
}

func TestStartNewSession(t *testing.T) {
	s := New()
	s.StoriesCompletedThisSession = []string{"65-6", "65-7"}
	s.EngineStatus = StatusSessionComplete
	s.SessionCostUSD = 1.50

	s.StartNewSession()

	if s.SessionNumber != 2 {
		t.Errorf("SessionNumber = %d, want 2", s.SessionNumber)
	}
	if len(s.StoriesCompletedThisSession) != 0 {
		t.Error("StoriesCompletedThisSession should be reset")
	}
	if s.EngineStatus != StatusRunning {
		t.Errorf("EngineStatus = %q, want %q", s.EngineStatus, StatusRunning)
	}
	if s.SessionCostUSD != 0 {
		t.Errorf("SessionCostUSD = %f, want 0", s.SessionCostUSD)
	}
}

func TestIsResumable(t *testing.T) {
	s := New()
	if s.IsResumable() {
		t.Error("Fresh state should not be resumable")
	}

	s.EngineStatus = StatusSessionComplete
	s.NextStory = "65-9"
	if !s.IsResumable() {
		t.Error("SessionComplete with NextStory should be resumable")
	}
}

func TestAddCost(t *testing.T) {
	s := New()
	s.AddCost(0.50)
	s.AddCost(0.75)

	if s.SessionCostUSD != 1.25 {
		t.Errorf("SessionCostUSD = %f, want 1.25", s.SessionCostUSD)
	}
	if s.TotalCostUSD != 1.25 {
		t.Errorf("TotalCostUSD = %f, want 1.25", s.TotalCostUSD)
	}
}

func TestIsAllComplete(t *testing.T) {
	s := New()
	if s.IsAllComplete() {
		t.Error("Fresh state should not be all_complete")
	}

	s.EngineStatus = StatusAllComplete
	if !s.IsAllComplete() {
		t.Error("Should be all_complete")
	}
}

func TestIsBlocked(t *testing.T) {
	s := New()
	if s.IsBlocked() {
		t.Error("Fresh state should not be blocked")
	}

	s.Blocked = true
	if !s.IsBlocked() {
		t.Error("Should be blocked")
	}
}
