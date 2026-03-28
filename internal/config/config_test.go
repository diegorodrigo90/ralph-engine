package config

import (
	"os"
	"path/filepath"
	"testing"
)

func TestLoadReturnsDefaults(t *testing.T) {
	cfg, err := Load("")
	if err != nil {
		t.Fatalf("Load() returned error: %v", err)
	}

	if cfg.Agent.Type != "claude-code" {
		t.Errorf("Agent.Type = %q, want %q", cfg.Agent.Type, "claude-code")
	}
	if cfg.Agent.MaxStoriesPerSession != 5 {
		t.Errorf("Agent.MaxStoriesPerSession = %d, want 5", cfg.Agent.MaxStoriesPerSession)
	}
	if cfg.Agent.CooldownSeconds != 30 {
		t.Errorf("Agent.CooldownSeconds = %d, want 30", cfg.Agent.CooldownSeconds)
	}
	if cfg.Workflow.Type != "basic" {
		t.Errorf("Workflow.Type = %q, want %q", cfg.Workflow.Type, "basic")
	}
	if cfg.Quality.Type != "standard" {
		t.Errorf("Quality.Type = %q, want %q", cfg.Quality.Type, "standard")
	}
	if !cfg.Quality.Gates.Tests {
		t.Error("Quality.Gates.Tests should be true by default")
	}
	if cfg.Quality.Gates.E2E {
		t.Error("Quality.Gates.E2E should be false by default")
	}
	if cfg.CircuitBreaker.MaxFailures != 3 {
		t.Errorf("CircuitBreaker.MaxFailures = %d, want 3", cfg.CircuitBreaker.MaxFailures)
	}
	if cfg.Resources.MinFreeRAMMB != 2048 {
		t.Errorf("Resources.MinFreeRAMMB = %d, want 2048", cfg.Resources.MinFreeRAMMB)
	}
	if cfg.Tracker.Type != "file" {
		t.Errorf("Tracker.Type = %q, want %q", cfg.Tracker.Type, "file")
	}
}

func TestLoadReadsProjectConfig(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ProjectConfigDir)
	if err := os.MkdirAll(configDir, 0755); err != nil {
		t.Fatal(err)
	}

	configContent := `
agent:
  type: claudebox
  max_stories_per_session: 3
workflow:
  type: bmad-v6
quality:
  type: full
  gates:
    e2e: true
    browser: true
`
	if err := os.WriteFile(filepath.Join(configDir, "config.yaml"), []byte(configContent), 0644); err != nil {
		t.Fatal(err)
	}

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() returned error: %v", err)
	}

	if cfg.Agent.Type != "claudebox" {
		t.Errorf("Agent.Type = %q, want %q", cfg.Agent.Type, "claudebox")
	}
	if cfg.Agent.MaxStoriesPerSession != 3 {
		t.Errorf("Agent.MaxStoriesPerSession = %d, want 3", cfg.Agent.MaxStoriesPerSession)
	}
	if cfg.Workflow.Type != "bmad-v6" {
		t.Errorf("Workflow.Type = %q, want %q", cfg.Workflow.Type, "bmad-v6")
	}
	if cfg.Quality.Type != "full" {
		t.Errorf("Quality.Type = %q, want %q", cfg.Quality.Type, "full")
	}
	if !cfg.Quality.Gates.E2E {
		t.Error("Quality.Gates.E2E should be true from project config")
	}
	if !cfg.Quality.Gates.Browser {
		t.Error("Quality.Gates.Browser should be true from project config")
	}
	// Default values should still apply for unset fields
	if !cfg.Quality.Gates.Tests {
		t.Error("Quality.Gates.Tests should still be true (default)")
	}
}

func TestLoadEnvOverridesProjectConfig(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ProjectConfigDir)
	if err := os.MkdirAll(configDir, 0755); err != nil {
		t.Fatal(err)
	}

	configContent := `
agent:
  type: claudebox
`
	if err := os.WriteFile(filepath.Join(configDir, "config.yaml"), []byte(configContent), 0644); err != nil {
		t.Fatal(err)
	}

	// ENV should override project config
	t.Setenv("RALPH_AGENT_TYPE", "cursor")

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() returned error: %v", err)
	}

	if cfg.Agent.Type != "cursor" {
		t.Errorf("Agent.Type = %q, want %q (env should override project config)", cfg.Agent.Type, "cursor")
	}
}

func TestInitProjectCreatesConfigFile(t *testing.T) {
	dir := t.TempDir()

	if err := InitProject(dir, "basic"); err != nil {
		t.Fatalf("InitProject() returned error: %v", err)
	}

	configPath := filepath.Join(dir, ProjectConfigDir, ConfigFileName+".yaml")
	if _, err := os.Stat(configPath); os.IsNotExist(err) {
		t.Error("InitProject() did not create config file")
	}
}

func TestInitProjectBMADPreset(t *testing.T) {
	dir := t.TempDir()

	if err := InitProject(dir, "bmad-v6"); err != nil {
		t.Fatalf("InitProject() returned error: %v", err)
	}

	// Load the created config and verify preset values
	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() returned error: %v", err)
	}

	if cfg.Agent.Type != "claude-code" {
		t.Errorf("BMAD preset: Agent.Type = %q, want %q", cfg.Agent.Type, "claude-code")
	}
	if cfg.Workflow.Type != "bmad-v6" {
		t.Errorf("BMAD preset: Workflow.Type = %q, want %q", cfg.Workflow.Type, "bmad-v6")
	}
	if cfg.Quality.Type != "full" {
		t.Errorf("BMAD preset: Quality.Type = %q, want %q", cfg.Quality.Type, "full")
	}
	if !cfg.Quality.Gates.E2E {
		t.Error("BMAD preset: Quality.Gates.E2E should be true")
	}
	if !cfg.Quality.Gates.Browser {
		t.Error("BMAD preset: Quality.Gates.Browser should be true")
	}
	if !cfg.SSH.Enabled {
		t.Error("BMAD preset: SSH.Enabled should be true")
	}
	if cfg.Tracker.StatusFile != "sprint-status.yaml" {
		t.Errorf("BMAD preset: Tracker.StatusFile = %q, want %q", cfg.Tracker.StatusFile, "sprint-status.yaml")
	}
}

func TestSaveCreatesUserConfig(t *testing.T) {
	dir := t.TempDir()
	t.Setenv("XDG_CONFIG_HOME", dir)

	if err := Save("agent.type", "claudebox"); err != nil {
		t.Fatalf("Save() returned error: %v", err)
	}

	configPath := filepath.Join(dir, AppName, ConfigFileName+".yaml")
	if _, err := os.Stat(configPath); os.IsNotExist(err) {
		t.Error("Save() did not create user config file")
	}
}

func TestUserConfigDirRespectsXDG(t *testing.T) {
	t.Setenv("XDG_CONFIG_HOME", "/custom/config")
	got := userConfigDir()
	want := filepath.Join("/custom/config", AppName)
	if got != want {
		t.Errorf("userConfigDir() = %q, want %q", got, want)
	}
}

func TestProjectConfigDirConstant(t *testing.T) {
	if ProjectConfigDir != ".ralph-engine" {
		t.Errorf("ProjectConfigDir = %q, want %q", ProjectConfigDir, ".ralph-engine")
	}
}
