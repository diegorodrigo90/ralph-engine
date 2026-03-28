package dashboard

import (
	"strings"
	"testing"
	"time"
)

func TestNewModelDefaults(t *testing.T) {
	m := NewModel()

	if m.State.EngineStatus != "idle" {
		t.Errorf("EngineStatus = %q, want %q", m.State.EngineStatus, "idle")
	}
	if m.State.BreakerState != "closed" {
		t.Errorf("BreakerState = %q, want %q", m.State.BreakerState, "closed")
	}
}

func TestViewContainsTitle(t *testing.T) {
	m := NewModel()
	view := m.View()

	if !strings.Contains(view, "ralph-engine") {
		t.Error("View should contain title")
	}
}

func TestViewShowsStoryProgress(t *testing.T) {
	m := NewModel()
	m.State.CurrentStory = "65.3"
	m.State.StoriesDone = 5
	m.State.StoriesTotal = 20

	view := m.View()

	if !strings.Contains(view, "65.3") {
		t.Error("View should show current story")
	}
	if !strings.Contains(view, "5/20") {
		t.Error("View should show progress")
	}
}

func TestViewShowsSystemResources(t *testing.T) {
	m := NewModel()
	m.State.RAMMB = 4096
	m.State.RAMTotalMB = 16384
	m.State.CPUPercent = 45.0
	m.State.DiskGB = 50
	m.State.DiskTotalGB = 100

	view := m.View()

	if !strings.Contains(view, "SYSTEM") {
		t.Error("View should show system section")
	}
}

func TestViewShowsQualityGates(t *testing.T) {
	m := NewModel()
	m.State.GatesCR = "pass"
	m.State.GatesTests = "running"
	m.State.GatesBuild = "pending"

	view := m.View()

	if !strings.Contains(view, "GATES") {
		t.Error("View should show gates section")
	}
}

func TestRenderProgressBar(t *testing.T) {
	bar := renderProgress(5, 10, 20)
	// Should have some filled and some empty chars
	if bar == "" {
		t.Error("progress bar should not be empty")
	}
}

func TestRenderProgressBarZeroTotal(t *testing.T) {
	bar := renderProgress(0, 0, 20)
	if bar == "" {
		t.Error("progress bar should render even with zero total")
	}
}

func TestFormatDuration(t *testing.T) {
	tests := []struct {
		d    time.Duration
		want string
	}{
		{30 * time.Second, "0m30s"},
		{5 * time.Minute, "5m00s"},
		{90 * time.Minute, "1h30m"},
		{2*time.Hour + 15*time.Minute, "2h15m"},
	}

	for _, tt := range tests {
		got := formatDuration(tt.d)
		if got != tt.want {
			t.Errorf("formatDuration(%v) = %q, want %q", tt.d, got, tt.want)
		}
	}
}

func TestViewQuitting(t *testing.T) {
	m := NewModel()
	m.quitting = true

	view := m.View()
	if !strings.Contains(view, "Shutting down") {
		t.Error("View should show shutdown message when quitting")
	}
}

func TestUpdateWithStateMsg(t *testing.T) {
	m := NewModel()

	newState := DashboardState{
		EngineStatus: "running",
		CurrentStory: "65.5",
		StoriesDone:  10,
	}

	updated, _ := m.Update(UpdateMsg{State: newState})
	model := updated.(Model)

	if model.State.EngineStatus != "running" {
		t.Errorf("EngineStatus = %q, want %q", model.State.EngineStatus, "running")
	}
	if model.State.CurrentStory != "65.5" {
		t.Errorf("CurrentStory = %q, want %q", model.State.CurrentStory, "65.5")
	}
}

func TestViewShowsFindings(t *testing.T) {
	m := NewModel()
	m.State.Findings = 3

	view := m.View()
	if !strings.Contains(view, "3") {
		t.Error("View should show findings count")
	}
}

func TestViewShowsCost(t *testing.T) {
	m := NewModel()
	m.State.SessionCost = 1.50
	m.State.TotalCost = 15.00

	view := m.View()
	if !strings.Contains(view, "$1.50") {
		t.Error("View should show session cost")
	}
	if !strings.Contains(view, "$15.00") {
		t.Error("View should show total cost")
	}
}
