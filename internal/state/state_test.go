package state

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
	"time"
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

func TestStartNewRun(t *testing.T) {
	s := New()
	// Simulate state from a previous run.
	s.StoriesCompletedTotal = 5
	s.StoriesCompletedThisRun = 3
	s.StoriesCompletedThisSession = []string{"65-6", "65-7", "65-8"}
	s.EngineStatus = StatusSessionComplete
	s.Blocked = true
	s.BlockedReason = "something"
	s.SessionCostUSD = 2.50
	s.TotalCostUSD = 10.00

	s.StartNewRun()

	if s.StoriesCompletedThisRun != 0 {
		t.Errorf("StoriesCompletedThisRun = %d, want 0", s.StoriesCompletedThisRun)
	}
	if s.StoriesCompletedTotal != 5 {
		t.Errorf("StoriesCompletedTotal = %d, want 5 (should be preserved)", s.StoriesCompletedTotal)
	}
	if s.TotalCostUSD != 10.00 {
		t.Errorf("TotalCostUSD = %f, want 10.00 (should be preserved)", s.TotalCostUSD)
	}
	if len(s.StoriesCompletedThisSession) != 0 {
		t.Errorf("StoriesCompletedThisSession should be empty, got %d", len(s.StoriesCompletedThisSession))
	}
	if s.EngineStatus != StatusRunning {
		t.Errorf("EngineStatus = %q, want %q", s.EngineStatus, StatusRunning)
	}
	if s.Blocked {
		t.Error("Blocked should be false")
	}
	if s.BlockedReason != "" {
		t.Errorf("BlockedReason should be empty, got %q", s.BlockedReason)
	}
	if s.SessionCostUSD != 0 {
		t.Errorf("SessionCostUSD = %f, want 0", s.SessionCostUSD)
	}
	if s.RunID == "" {
		t.Error("RunID should be set")
	}
	if s.RunStartedAt == "" {
		t.Error("RunStartedAt should be set")
	}
}

func TestStartNewRunGeneratesUniqueRunID(t *testing.T) {
	s := New()
	s.StartNewRun()
	id1 := s.RunID

	// Small delay to ensure different millisecond.
	time.Sleep(2 * time.Millisecond)
	s.StartNewRun()
	id2 := s.RunID

	if id1 == id2 {
		t.Errorf("RunIDs should be unique, got %q both times", id1)
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

// --- Tests for StartNewRun preserving/resetting counters ---

func TestStartNewRunPreservesLifetimeTotals(t *testing.T) {
	s := New()
	s.StoriesCompletedTotal = 15
	s.TotalCostUSD = 25.50
	s.StoriesCompletedThisRun = 5
	s.SessionCostUSD = 3.00

	s.StartNewRun()

	// Lifetime totals MUST be preserved.
	if s.StoriesCompletedTotal != 15 {
		t.Errorf("StoriesCompletedTotal = %d, want 15 (should NOT reset)", s.StoriesCompletedTotal)
	}
	if s.TotalCostUSD != 25.50 {
		t.Errorf("TotalCostUSD = %f, want 25.50 (should NOT reset)", s.TotalCostUSD)
	}
}

func TestStartNewRunResetsRunCounters(t *testing.T) {
	s := New()
	s.StoriesCompletedThisRun = 7
	s.SessionCostUSD = 5.00
	s.StoriesCompletedThisSession = []string{"1-1", "1-2", "1-3"}
	s.CurrentStory = "2-1"
	s.CurrentTask = 3
	s.CurrentPhase = "implementation"
	s.Blocked = true
	s.BlockedReason = "docker down"
	s.EngineStatus = StatusBlocked

	s.StartNewRun()

	// Per-run counters MUST be reset.
	if s.StoriesCompletedThisRun != 0 {
		t.Errorf("StoriesCompletedThisRun = %d, want 0", s.StoriesCompletedThisRun)
	}
	if s.SessionCostUSD != 0 {
		t.Errorf("SessionCostUSD = %f, want 0", s.SessionCostUSD)
	}
	if len(s.StoriesCompletedThisSession) != 0 {
		t.Errorf("StoriesCompletedThisSession = %v, want empty", s.StoriesCompletedThisSession)
	}
	// Blocked state MUST be cleared.
	if s.Blocked {
		t.Error("Blocked should be false after StartNewRun")
	}
	if s.BlockedReason != "" {
		t.Errorf("BlockedReason = %q, want empty", s.BlockedReason)
	}
	if s.EngineStatus != StatusRunning {
		t.Errorf("EngineStatus = %q, want %q", s.EngineStatus, StatusRunning)
	}
	// RunID and RunStartedAt MUST be set.
	if s.RunID == "" {
		t.Error("RunID should be set after StartNewRun")
	}
	if s.RunStartedAt == "" {
		t.Error("RunStartedAt should be set after StartNewRun")
	}
}

func TestStartNewRunAfterMultipleRuns(t *testing.T) {
	s := New()

	// Simulate run 1: complete 3 stories, cost $2.
	s.StartNewRun()
	run1ID := s.RunID
	s.MarkStoryComplete("1-1")
	s.MarkStoryComplete("1-2")
	s.MarkStoryComplete("1-3")
	s.StoriesCompletedThisRun = 3
	s.AddCost(2.00)

	// Verify run 1 state.
	if s.StoriesCompletedTotal != 3 {
		t.Fatalf("after run 1: StoriesCompletedTotal = %d, want 3", s.StoriesCompletedTotal)
	}
	if s.TotalCostUSD != 2.00 {
		t.Fatalf("after run 1: TotalCostUSD = %f, want 2.00", s.TotalCostUSD)
	}

	// Simulate run 2: complete 2 stories, cost $1.50.
	time.Sleep(2 * time.Millisecond) // Ensure different RunID.
	s.StartNewRun()
	run2ID := s.RunID
	if run2ID == run1ID {
		t.Error("run 2 should have different RunID from run 1")
	}
	s.MarkStoryComplete("2-1")
	s.MarkStoryComplete("2-2")
	s.StoriesCompletedThisRun = 2
	s.AddCost(1.50)

	// Verify run 2 state — totals accumulate.
	if s.StoriesCompletedTotal != 5 {
		t.Errorf("after run 2: StoriesCompletedTotal = %d, want 5", s.StoriesCompletedTotal)
	}
	if s.TotalCostUSD != 3.50 {
		t.Errorf("after run 2: TotalCostUSD = %f, want 3.50", s.TotalCostUSD)
	}

	// Simulate run 3: no stories yet, just started.
	time.Sleep(2 * time.Millisecond)
	s.StartNewRun()
	run3ID := s.RunID
	if run3ID == run2ID {
		t.Error("run 3 should have different RunID from run 2")
	}

	// Run counters should be reset, totals preserved.
	if s.StoriesCompletedThisRun != 0 {
		t.Errorf("after run 3 start: StoriesCompletedThisRun = %d, want 0", s.StoriesCompletedThisRun)
	}
	if s.SessionCostUSD != 0 {
		t.Errorf("after run 3 start: SessionCostUSD = %f, want 0", s.SessionCostUSD)
	}
	if s.StoriesCompletedTotal != 5 {
		t.Errorf("after run 3 start: StoriesCompletedTotal = %d, want 5 (preserved)", s.StoriesCompletedTotal)
	}
	if s.TotalCostUSD != 3.50 {
		t.Errorf("after run 3 start: TotalCostUSD = %f, want 3.50 (preserved)", s.TotalCostUSD)
	}
}
