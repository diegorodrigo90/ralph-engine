// Package state manages the persistent engine state (state.json).
// State survives session boundaries and enables resume from exact checkpoint.
package state

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"time"
)

// Status represents the engine execution status.
type Status string

const (
	StatusRunning         Status = "running"
	StatusSessionComplete Status = "session_complete"
	StatusAllComplete     Status = "all_complete"
	StatusBlocked         Status = "blocked"
)

// Engine holds the persistent state written to state.json.
type Engine struct {
	// EngineStatus tracks the current execution state.
	EngineStatus Status `json:"engine_status"`
	// SessionNumber increments each time the runner starts a new Claude session.
	SessionNumber int `json:"session_number"`
	// CurrentStory is the story being worked on (e.g., "65-8").
	CurrentStory string `json:"current_story"`
	// CurrentTask is the task index within the story (0-based).
	CurrentTask int `json:"current_task"`
	// CurrentPhase is the engine phase (e.g., "implementation").
	CurrentPhase string `json:"current_phase"`
	// StoriesCompletedThisSession lists stories done in current session.
	StoriesCompletedThisSession []string `json:"stories_completed_this_session"`
	// StoriesCompletedTotal is the cumulative count across all sessions.
	StoriesCompletedTotal int `json:"stories_completed_total"`
	// NextStory is the next story to pick up.
	NextStory string `json:"next_story"`
	// Blocked indicates if the engine is blocked and needs user intervention.
	Blocked bool `json:"blocked"`
	// BlockedReason describes why the engine is blocked.
	BlockedReason string `json:"blocked_reason"`
	// SSHAvailable tracks SSH connectivity.
	SSHAvailable bool `json:"ssh_available"`
	// FindingsCount tracks accumulated findings in this session.
	FindingsCount int `json:"findings_count"`
	// LastCheckpoint is the ISO timestamp of the last state save.
	LastCheckpoint string `json:"last_checkpoint"`
	// TotalCostUSD tracks cumulative API cost.
	TotalCostUSD float64 `json:"total_cost_usd"`
	// SessionCostUSD tracks cost for current session.
	SessionCostUSD float64 `json:"session_cost_usd"`
}

// New creates a fresh engine state.
func New() *Engine {
	return &Engine{
		EngineStatus:                StatusRunning,
		SessionNumber:               1,
		StoriesCompletedThisSession: []string{},
		LastCheckpoint:              time.Now().UTC().Format(time.RFC3339),
	}
}

// Load reads state from a JSON file. Returns a new state if file doesn't exist.
func Load(stateDir string) (*Engine, error) {
	path := filepath.Join(stateDir, "state.json")

	data, err := os.ReadFile(path) // #nosec G304 -- path is stateDir + "state.json"
	if os.IsNotExist(err) {
		return New(), nil
	}
	if err != nil {
		return nil, fmt.Errorf("failed to read state file: %w", err)
	}

	var state Engine
	if err := json.Unmarshal(data, &state); err != nil {
		// Corrupted state — start fresh rather than crash
		return New(), nil
	}

	return &state, nil
}

// Save writes the state to a JSON file.
func (e *Engine) Save(stateDir string) error {
	if err := os.MkdirAll(stateDir, 0750); err != nil {
		return fmt.Errorf("failed to create state directory: %w", err)
	}

	e.LastCheckpoint = time.Now().UTC().Format(time.RFC3339)

	data, err := json.MarshalIndent(e, "", "  ")
	if err != nil {
		return fmt.Errorf("failed to marshal state: %w", err)
	}

	path := filepath.Join(stateDir, "state.json")

	// Write atomically: write to temp file, then rename
	tmpPath := path + ".tmp"
	if err := os.WriteFile(tmpPath, data, 0600); err != nil {
		return fmt.Errorf("failed to write state file: %w", err)
	}
	if err := os.Rename(tmpPath, path); err != nil {
		_ = os.Remove(tmpPath) //nolint:errcheck // best-effort cleanup of temp file after rename failure
		return fmt.Errorf("failed to rename state file: %w", err)
	}

	return nil
}

// IsAllComplete returns true if all stories are done.
func (e *Engine) IsAllComplete() bool {
	return e.EngineStatus == StatusAllComplete
}

// IsBlocked returns true if the engine needs user intervention.
func (e *Engine) IsBlocked() bool {
	return e.Blocked
}

// IsResumable returns true if there's an interrupted session to continue.
func (e *Engine) IsResumable() bool {
	return e.EngineStatus == StatusSessionComplete && e.NextStory != ""
}

// MarkStoryComplete records a completed story.
func (e *Engine) MarkStoryComplete(storyKey string) {
	e.StoriesCompletedThisSession = append(e.StoriesCompletedThisSession, storyKey)
	e.StoriesCompletedTotal++
	e.CurrentStory = ""
	e.CurrentTask = 0
	e.CurrentPhase = ""
}

// MarkBlocked marks the engine as blocked with a reason.
func (e *Engine) MarkBlocked(reason string) {
	e.Blocked = true
	e.BlockedReason = reason
	e.EngineStatus = StatusBlocked
}

// StartNewSession prepares state for a new runner iteration.
func (e *Engine) StartNewSession() {
	e.SessionNumber++
	e.StoriesCompletedThisSession = []string{}
	e.EngineStatus = StatusRunning
	e.Blocked = false
	e.BlockedReason = ""
	e.SessionCostUSD = 0
}

// StoriesThisSession returns the count of stories completed in this session.
func (e *Engine) StoriesThisSession() int {
	return len(e.StoriesCompletedThisSession)
}

// AddCost records API cost from a session.
func (e *Engine) AddCost(costUSD float64) {
	e.SessionCostUSD += costUSD
	e.TotalCostUSD += costUSD
}
