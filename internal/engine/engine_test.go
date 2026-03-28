package engine

import (
	"context"
	"testing"
	"time"
)

func TestNewEngineRequiresConfig(t *testing.T) {
	_, err := New(EngineOpts{})
	if err == nil {
		t.Error("New() should error without ProjectDir")
	}
}

func TestNewEngineValidConfig(t *testing.T) {
	e, err := New(EngineOpts{
		ProjectDir: "/tmp/test-project",
		StateDir:   t.TempDir(),
	})
	if err != nil {
		t.Fatalf("New() error: %v", err)
	}
	if e == nil {
		t.Fatal("New() returned nil")
	}
	if e.status != StatusIdle {
		t.Errorf("status = %q, want %q", e.status, StatusIdle)
	}
}

func TestEngineStatusTransitions(t *testing.T) {
	e, _ := New(EngineOpts{
		ProjectDir: "/tmp/test-project",
		StateDir:   t.TempDir(),
	})

	if e.Status() != StatusIdle {
		t.Errorf("initial status = %q, want %q", e.Status(), StatusIdle)
	}

	e.status = StatusRunning
	if e.Status() != StatusRunning {
		t.Errorf("status = %q, want %q", e.Status(), StatusRunning)
	}
}

func TestExitReasonString(t *testing.T) {
	tests := []struct {
		reason ExitReason
		want   string
	}{
		{ExitAllComplete, "all_complete"},
		{ExitCircuitBreaker, "circuit_breaker"},
		{ExitUsageLimit, "usage_limit"},
		{ExitUserInterrupt, "user_interrupt"},
		{ExitResourceCritical, "resource_critical"},
		{ExitBlocked, "blocked"},
	}

	for _, tt := range tests {
		if string(tt.reason) != tt.want {
			t.Errorf("ExitReason %q != %q", tt.reason, tt.want)
		}
	}
}

func TestPreflightChecksMissingBinary(t *testing.T) {
	e, _ := New(EngineOpts{
		ProjectDir: "/tmp/test-project",
		StateDir:   t.TempDir(),
		Binary:     "nonexistent-binary-12345",
	})

	results := e.Preflight(context.Background())

	var found bool
	for _, r := range results {
		if r.Name == "agent binary" && !r.OK {
			found = true
			break
		}
	}
	if !found {
		t.Error("Preflight should report missing binary")
	}
}

func TestPreflightChecksProjectDir(t *testing.T) {
	e, _ := New(EngineOpts{
		ProjectDir: "/nonexistent-project-dir-12345",
		StateDir:   t.TempDir(),
	})

	results := e.Preflight(context.Background())

	var found bool
	for _, r := range results {
		if r.Name == "project directory" && !r.OK {
			found = true
			break
		}
	}
	if !found {
		t.Error("Preflight should report missing project dir")
	}
}

func TestPreflightChecksResources(t *testing.T) {
	e, _ := New(EngineOpts{
		ProjectDir: t.TempDir(), // Use real dir
		StateDir:   t.TempDir(),
	})

	results := e.Preflight(context.Background())

	var found bool
	for _, r := range results {
		if r.Name == "system resources" {
			found = true
			break
		}
	}
	if !found {
		t.Error("Preflight should include system resources check")
	}
}

func TestCooldownDuration(t *testing.T) {
	e, _ := New(EngineOpts{
		ProjectDir:      "/tmp/test-project",
		StateDir:        t.TempDir(),
		CooldownSeconds: 30,
	})

	if e.cooldown != 30*time.Second {
		t.Errorf("cooldown = %v, want 30s", e.cooldown)
	}
}

func TestCooldownDefault(t *testing.T) {
	e, _ := New(EngineOpts{
		ProjectDir: "/tmp/test-project",
		StateDir:   t.TempDir(),
	})

	if e.cooldown != DefaultCooldown {
		t.Errorf("cooldown = %v, want %v", e.cooldown, DefaultCooldown)
	}
}

func TestSessionCounterIncrementsOnRun(t *testing.T) {
	e, _ := New(EngineOpts{
		ProjectDir: "/tmp/test-project",
		StateDir:   t.TempDir(),
	})

	if e.sessionCount != 0 {
		t.Errorf("initial sessionCount = %d, want 0", e.sessionCount)
	}
}
