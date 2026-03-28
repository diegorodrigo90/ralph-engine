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

	if cfg.Agent.Type != "claude" {
		t.Errorf("Agent.Type = %q, want %q", cfg.Agent.Type, "claude")
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

	if cfg.Agent.Type != "claude" {
		t.Errorf("BMAD preset: Agent.Type = %q, want %q", cfg.Agent.Type, "claude")
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

func TestLoadWithNoConfigFiles(t *testing.T) {
	// Load from a temp dir with no config files — should return only defaults.
	dir := t.TempDir()

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() returned error: %v", err)
	}

	// Verify all default values are set correctly.
	tests := []struct {
		name string
		got  interface{}
		want interface{}
	}{
		// Engine
		{"Engine.Version", cfg.Engine.Version, "1.0.0"},
		{"Engine.Name", cfg.Engine.Name, ""},
		// Agent
		{"Agent.Type", cfg.Agent.Type, "claude"},
		{"Agent.Model", cfg.Agent.Model, ""},
		{"Agent.MaxStoriesPerSession", cfg.Agent.MaxStoriesPerSession, 5},
		{"Agent.CooldownSeconds", cfg.Agent.CooldownSeconds, 30},
		{"Agent.AllowedTools", cfg.Agent.AllowedTools, "Write,Read,Edit,Bash,Glob,Grep,Skill,Agent,WebSearch,WebFetch,ToolSearch"},
		// Workflow
		{"Workflow.Type", cfg.Workflow.Type, "basic"},
		// Quality
		{"Quality.Type", cfg.Quality.Type, "standard"},
		{"Quality.MaxRetries", cfg.Quality.MaxRetries, 0},
		{"Quality.Gates.CR", cfg.Quality.Gates.CR, true},
		{"Quality.Gates.Tests", cfg.Quality.Gates.Tests, true},
		{"Quality.Gates.Build", cfg.Quality.Gates.Build, true},
		{"Quality.Gates.TypeCheck", cfg.Quality.Gates.TypeCheck, true},
		{"Quality.Gates.Storybook", cfg.Quality.Gates.Storybook, false},
		{"Quality.Gates.E2E", cfg.Quality.Gates.E2E, false},
		{"Quality.Gates.Browser", cfg.Quality.Gates.Browser, false},
		{"Quality.Gates.DevLogs", cfg.Quality.Gates.DevLogs, false},
		// Tracker
		{"Tracker.Type", cfg.Tracker.Type, "file"},
		{"Tracker.StatusFile", cfg.Tracker.StatusFile, "sprint-status.yaml"},
		// Resources
		{"Resources.MinFreeRAMMB", cfg.Resources.MinFreeRAMMB, 2048},
		{"Resources.MaxCPULoadPercent", cfg.Resources.MaxCPULoadPercent, 80},
		{"Resources.MinFreeDiskGB", cfg.Resources.MinFreeDiskGB, 5},
		{"Resources.MaxLogSizeMB", cfg.Resources.MaxLogSizeMB, 50},
		{"Resources.MaxLogFiles", cfg.Resources.MaxLogFiles, 10},
		// CircuitBreaker
		{"CircuitBreaker.MaxFailures", cfg.CircuitBreaker.MaxFailures, 3},
		{"CircuitBreaker.CooldownMinutes", cfg.CircuitBreaker.CooldownMinutes, 5},
		// SSH
		{"SSH.Enabled", cfg.SSH.Enabled, false},
		{"SSH.ReconnectScript", cfg.SSH.ReconnectScript, ""},
		{"SSH.DevExecScript", cfg.SSH.DevExecScript, ""},
		// Security
		{"Security.NoticeAccepted", cfg.Security.NoticeAccepted, false},
		{"Security.RequireContainer", cfg.Security.RequireContainer, true},
		{"Security.DailyBudgetUSD", cfg.Security.DailyBudgetUSD, float64(0)},
		{"Security.MaxCostPerSessionUSD", cfg.Security.MaxCostPerSessionUSD, float64(0)},
		// Paths
		{"Paths.Stories", cfg.Paths.Stories, ""},
		{"Paths.Architecture", cfg.Paths.Architecture, ""},
		{"Paths.PRD", cfg.Paths.PRD, ""},
		{"Paths.UX", cfg.Paths.UX, ""},
		{"Paths.Decisions", cfg.Paths.Decisions, ""},
		{"Paths.Status", cfg.Paths.Status, ""},
		{"Paths.Rules", cfg.Paths.Rules, ""},
		// Research
		{"Research.Enabled", cfg.Research.Enabled, false},
		{"Research.Strategy", cfg.Research.Strategy, "always"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.got != tt.want {
				t.Errorf("%s = %v, want %v", tt.name, tt.got, tt.want)
			}
		})
	}

	// Check slice/map defaults separately.
	if len(cfg.Agent.Flags) != 0 {
		t.Errorf("Agent.Flags = %v, want empty", cfg.Agent.Flags)
	}
	if len(cfg.Workflow.CustomPhases) != 0 {
		t.Errorf("Workflow.CustomPhases = %v, want empty", cfg.Workflow.CustomPhases)
	}
	if len(cfg.Prompt.Sections) != 0 {
		t.Errorf("Prompt.Sections = %v, want empty", cfg.Prompt.Sections)
	}
	if len(cfg.Research.Tools) != 0 {
		t.Errorf("Research.Tools = %v, want empty", cfg.Research.Tools)
	}
}

func TestLoadWithInvalidYAML(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ProjectConfigDir)
	if err := os.MkdirAll(configDir, 0755); err != nil {
		t.Fatal(err)
	}

	invalidYAML := `
agent:
  type: "claude"
  max_stories_per_session: [invalid yaml
  broken: {{{
`
	if err := os.WriteFile(filepath.Join(configDir, "config.yaml"), []byte(invalidYAML), 0644); err != nil {
		t.Fatal(err)
	}

	// Viper's MergeInConfig silently ignores parse errors, so Load should
	// still succeed with defaults. Verify it doesn't panic.
	cfg, err := Load(dir)
	if err != nil {
		// If viper returns an error for invalid YAML, that's also acceptable.
		t.Logf("Load() returned error for invalid YAML (acceptable): %v", err)
		return
	}
	// If no error, defaults should be in place.
	if cfg.Agent.Type != "claude" {
		t.Errorf("Agent.Type = %q, want default %q after invalid YAML", cfg.Agent.Type, "claude")
	}
}

func TestInitProjectDoesNotOverwriteExistingFiles(t *testing.T) {
	dir := t.TempDir()

	// First init creates files.
	if err := InitProject(dir, "basic"); err != nil {
		t.Fatalf("first InitProject() error: %v", err)
	}

	// Write custom content to config.yaml.
	configPath := filepath.Join(dir, ProjectConfigDir, ConfigFileName+".yaml")
	customContent := "# my custom config\nagent:\n  type: custom-agent\n"
	if err := os.WriteFile(configPath, []byte(customContent), 0644); err != nil {
		t.Fatal(err)
	}

	// Second init should NOT overwrite.
	if err := InitProject(dir, "bmad-v6"); err != nil {
		t.Fatalf("second InitProject() error: %v", err)
	}

	data, err := os.ReadFile(configPath)
	if err != nil {
		t.Fatal(err)
	}
	if string(data) != customContent {
		t.Error("InitProject() overwrote existing config.yaml — should preserve user files")
	}
}

func TestInitProjectCreatesAllExpectedFiles(t *testing.T) {
	dir := t.TempDir()

	if err := InitProject(dir, "basic"); err != nil {
		t.Fatalf("InitProject() error: %v", err)
	}

	expectedFiles := []string{
		filepath.Join(dir, ProjectConfigDir, ConfigFileName+".yaml"),
		filepath.Join(dir, ProjectConfigDir, "prompt.md"),
		filepath.Join(dir, ProjectConfigDir, "hooks.yaml"),
		filepath.Join(dir, ProjectConfigDir, ".gitignore"),
	}

	for _, path := range expectedFiles {
		t.Run(filepath.Base(path), func(t *testing.T) {
			info, err := os.Stat(path)
			if os.IsNotExist(err) {
				t.Errorf("InitProject() did not create %s", filepath.Base(path))
				return
			}
			if err != nil {
				t.Fatalf("stat %s: %v", filepath.Base(path), err)
			}
			if info.Size() == 0 {
				t.Errorf("%s is empty", filepath.Base(path))
			}
		})
	}
}

func TestInitProjectPresets(t *testing.T) {
	tests := []struct {
		preset       string
		wantWorkflow string
		wantQuality  string
		wantSSH      bool
	}{
		{"basic", "basic", "minimal", false},
		{"bmad-v6", "bmad-v6", "full", true},
		{"tdd-strict", "tdd-strict", "standard", false},
	}

	for _, tt := range tests {
		t.Run(tt.preset, func(t *testing.T) {
			dir := t.TempDir()
			if err := InitProject(dir, tt.preset); err != nil {
				t.Fatalf("InitProject(%q) error: %v", tt.preset, err)
			}

			cfg, err := Load(dir)
			if err != nil {
				t.Fatalf("Load() error: %v", err)
			}

			if cfg.Workflow.Type != tt.wantWorkflow {
				t.Errorf("Workflow.Type = %q, want %q", cfg.Workflow.Type, tt.wantWorkflow)
			}
			if cfg.Quality.Type != tt.wantQuality {
				t.Errorf("Quality.Type = %q, want %q", cfg.Quality.Type, tt.wantQuality)
			}
			if cfg.SSH.Enabled != tt.wantSSH {
				t.Errorf("SSH.Enabled = %v, want %v", cfg.SSH.Enabled, tt.wantSSH)
			}
		})
	}
}

func TestSaveProjectCreatesMissingDirectories(t *testing.T) {
	dir := t.TempDir()
	nestedDir := filepath.Join(dir, "deep", "nested", "project")

	err := SaveProject(nestedDir, "agent.type", "custom")
	if err != nil {
		t.Fatalf("SaveProject() error: %v", err)
	}

	configPath := filepath.Join(nestedDir, ProjectConfigDir, ConfigFileName+".yaml")
	if _, err := os.Stat(configPath); os.IsNotExist(err) {
		t.Error("SaveProject() did not create config file in nested directory")
	}
}

func TestPromptSectionIsEnabled(t *testing.T) {
	boolTrue := true
	boolFalse := false

	tests := []struct {
		name    string
		section PromptSection
		want    bool
	}{
		{
			name:    "nil enabled defaults to true",
			section: PromptSection{Name: "test", Enabled: nil},
			want:    true,
		},
		{
			name:    "explicitly true",
			section: PromptSection{Name: "test", Enabled: &boolTrue},
			want:    true,
		},
		{
			name:    "explicitly false",
			section: PromptSection{Name: "test", Enabled: &boolFalse},
			want:    false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := tt.section.IsEnabled(); got != tt.want {
				t.Errorf("IsEnabled() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestWorkflowConfigCommandsNilMap(t *testing.T) {
	cfg := WorkflowConfig{
		Type:     "basic",
		Commands: nil,
	}

	// Accessing a nil map should not panic and should return zero value.
	val := cfg.Commands["nonexistent"]
	if val != "" {
		t.Errorf("nil Commands map lookup = %q, want empty string", val)
	}
}

func TestWorkflowConfigInstructionsEmptyString(t *testing.T) {
	cfg := WorkflowConfig{
		Type:         "basic",
		Instructions: "",
	}

	if cfg.Instructions != "" {
		t.Errorf("Instructions = %q, want empty string", cfg.Instructions)
	}
}

func TestResearchToolAllFieldsEmpty(t *testing.T) {
	tool := ResearchTool{}

	if tool.Name != "" {
		t.Errorf("Name = %q, want empty", tool.Name)
	}
	if tool.Type != "" {
		t.Errorf("Type = %q, want empty", tool.Type)
	}
	if tool.Priority != 0 {
		t.Errorf("Priority = %d, want 0", tool.Priority)
	}
	if tool.Description != "" {
		t.Errorf("Description = %q, want empty", tool.Description)
	}
	if tool.WhenToUse != "" {
		t.Errorf("WhenToUse = %q, want empty", tool.WhenToUse)
	}
	if tool.HowToUse != "" {
		t.Errorf("HowToUse = %q, want empty", tool.HowToUse)
	}
	if tool.Enabled {
		t.Error("Enabled = true, want false (zero value)")
	}
	if len(tool.Sources) != 0 {
		t.Errorf("Sources = %v, want empty", tool.Sources)
	}
}

func TestAgentConfigDefaultValues(t *testing.T) {
	// Zero-value AgentConfig should have empty/zero fields.
	cfg := AgentConfig{}

	if cfg.Type != "" {
		t.Errorf("Type = %q, want empty", cfg.Type)
	}
	if cfg.Model != "" {
		t.Errorf("Model = %q, want empty", cfg.Model)
	}
	if cfg.MaxStoriesPerSession != 0 {
		t.Errorf("MaxStoriesPerSession = %d, want 0", cfg.MaxStoriesPerSession)
	}
	if cfg.CooldownSeconds != 0 {
		t.Errorf("CooldownSeconds = %d, want 0", cfg.CooldownSeconds)
	}
	if cfg.AllowedTools != "" {
		t.Errorf("AllowedTools = %q, want empty", cfg.AllowedTools)
	}
	if len(cfg.Flags) != 0 {
		t.Errorf("Flags = %v, want empty", cfg.Flags)
	}
}

func TestLoadWithEmptyProjectDir(t *testing.T) {
	// Passing empty string for projectDir should still return defaults.
	cfg, err := Load("")
	if err != nil {
		t.Fatalf("Load('') returned error: %v", err)
	}
	if cfg.Agent.Type != "claude" {
		t.Errorf("Agent.Type = %q, want default %q", cfg.Agent.Type, "claude")
	}
}

func TestInitProjectWithEmptyPreset(t *testing.T) {
	dir := t.TempDir()

	// Empty preset should default to "basic".
	if err := InitProject(dir, ""); err != nil {
		t.Fatalf("InitProject('') error: %v", err)
	}

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}

	// Basic preset creates "basic" workflow.
	if cfg.Workflow.Type != "basic" {
		t.Errorf("Workflow.Type = %q, want %q for empty preset", cfg.Workflow.Type, "basic")
	}
}

// --- Tests for config combinations ---

func TestLoadMinimalConfig(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ProjectConfigDir)
	os.MkdirAll(configDir, 0755)
	// Only agent.type set — everything else should use defaults.
	os.WriteFile(filepath.Join(configDir, "config.yaml"), []byte(`
agent:
  type: aider
`), 0644)

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}
	if cfg.Agent.Type != "aider" {
		t.Errorf("Agent.Type = %q, want %q", cfg.Agent.Type, "aider")
	}
	// All other fields should be defaults.
	if cfg.Workflow.Type != "basic" {
		t.Errorf("Workflow.Type = %q, want default %q", cfg.Workflow.Type, "basic")
	}
	if cfg.Quality.Type != "standard" {
		t.Errorf("Quality.Type = %q, want default %q", cfg.Quality.Type, "standard")
	}
	if cfg.Agent.MaxStoriesPerSession != 5 {
		t.Errorf("MaxStoriesPerSession = %d, want default 5", cfg.Agent.MaxStoriesPerSession)
	}
	if cfg.Tracker.StatusFile != "sprint-status.yaml" {
		t.Errorf("Tracker.StatusFile = %q, want default", cfg.Tracker.StatusFile)
	}
}

func TestLoadWorkflowCommandsOnly(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ProjectConfigDir)
	os.MkdirAll(configDir, 0755)
	os.WriteFile(filepath.Join(configDir, "config.yaml"), []byte(`
workflow:
  type: custom
  commands:
    build: "make build"
    test: "pytest"
    deploy: "kubectl apply"
`), 0644)

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}
	if cfg.Workflow.Type != "custom" {
		t.Errorf("Workflow.Type = %q, want %q", cfg.Workflow.Type, "custom")
	}
	if len(cfg.Workflow.Commands) != 3 {
		t.Errorf("Commands length = %d, want 3", len(cfg.Workflow.Commands))
	}
	if cfg.Workflow.Commands["build"] != "make build" {
		t.Errorf("Commands[build] = %q, want %q", cfg.Workflow.Commands["build"], "make build")
	}
	if cfg.Workflow.Instructions != "" {
		t.Errorf("Instructions = %q, want empty", cfg.Workflow.Instructions)
	}
}

func TestLoadWorkflowInstructionsOnly(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ProjectConfigDir)
	os.MkdirAll(configDir, 0755)
	os.WriteFile(filepath.Join(configDir, "config.yaml"), []byte(`
workflow:
  type: tdd
  instructions: "RED-GREEN-REFACTOR per AC. Never skip tests."
`), 0644)

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}
	if cfg.Workflow.Type != "tdd" {
		t.Errorf("Workflow.Type = %q, want %q", cfg.Workflow.Type, "tdd")
	}
	if cfg.Workflow.Instructions != "RED-GREEN-REFACTOR per AC. Never skip tests." {
		t.Errorf("Instructions = %q", cfg.Workflow.Instructions)
	}
	if len(cfg.Workflow.Commands) != 0 {
		t.Errorf("Commands = %v, want empty", cfg.Workflow.Commands)
	}
}

func TestLoadWorkflowBothCommandsAndInstructions(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ProjectConfigDir)
	os.MkdirAll(configDir, 0755)
	os.WriteFile(filepath.Join(configDir, "config.yaml"), []byte(`
workflow:
  type: bmad-v6
  commands:
    implement: "dev"
    code_review: "cr"
  instructions: "Use Skill tool. TDD per AC."
`), 0644)

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}
	if len(cfg.Workflow.Commands) != 2 {
		t.Errorf("Commands length = %d, want 2", len(cfg.Workflow.Commands))
	}
	if cfg.Workflow.Instructions != "Use Skill tool. TDD per AC." {
		t.Errorf("Instructions = %q", cfg.Workflow.Instructions)
	}
}

func TestLoadResearchDisabled(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ProjectConfigDir)
	os.MkdirAll(configDir, 0755)
	os.WriteFile(filepath.Join(configDir, "config.yaml"), []byte(`
research:
  enabled: false
  tools:
    - name: "RAG"
      type: "rag"
      enabled: true
`), 0644)

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}
	if cfg.Research.Enabled {
		t.Error("Research.Enabled should be false")
	}
	// Tools should still be parsed even when disabled.
	if len(cfg.Research.Tools) != 1 {
		t.Errorf("Research.Tools length = %d, want 1", len(cfg.Research.Tools))
	}
}

func TestLoadResearchNoTools(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ProjectConfigDir)
	os.MkdirAll(configDir, 0755)
	os.WriteFile(filepath.Join(configDir, "config.yaml"), []byte(`
research:
  enabled: true
  strategy: "on-demand"
  tools: []
`), 0644)

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}
	if !cfg.Research.Enabled {
		t.Error("Research.Enabled should be true")
	}
	if cfg.Research.Strategy != "on-demand" {
		t.Errorf("Research.Strategy = %q, want %q", cfg.Research.Strategy, "on-demand")
	}
	if len(cfg.Research.Tools) != 0 {
		t.Errorf("Research.Tools = %v, want empty", cfg.Research.Tools)
	}
}

func TestLoadQualityMinimal(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ProjectConfigDir)
	os.MkdirAll(configDir, 0755)
	os.WriteFile(filepath.Join(configDir, "config.yaml"), []byte(`
quality:
  type: minimal
  gates:
    tests: true
    build: true
    cr: false
    type_check: false
`), 0644)

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}
	if cfg.Quality.Type != "minimal" {
		t.Errorf("Quality.Type = %q, want %q", cfg.Quality.Type, "minimal")
	}
	if !cfg.Quality.Gates.Tests {
		t.Error("Quality.Gates.Tests should be true")
	}
	if !cfg.Quality.Gates.Build {
		t.Error("Quality.Gates.Build should be true")
	}
	if cfg.Quality.Gates.CR {
		t.Error("Quality.Gates.CR should be false")
	}
	if cfg.Quality.Gates.TypeCheck {
		t.Error("Quality.Gates.TypeCheck should be false")
	}
}

func TestLoadQualityFull(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ProjectConfigDir)
	os.MkdirAll(configDir, 0755)
	os.WriteFile(filepath.Join(configDir, "config.yaml"), []byte(`
quality:
  type: full
  max_retries: 3
  gates:
    cr: true
    tests: true
    build: true
    type_check: true
    storybook: true
    e2e: true
    browser: true
    dev_logs: true
`), 0644)

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}
	if cfg.Quality.Type != "full" {
		t.Errorf("Quality.Type = %q, want %q", cfg.Quality.Type, "full")
	}
	if cfg.Quality.MaxRetries != 3 {
		t.Errorf("MaxRetries = %d, want 3", cfg.Quality.MaxRetries)
	}
	if !cfg.Quality.Gates.Storybook {
		t.Error("Quality.Gates.Storybook should be true")
	}
	if !cfg.Quality.Gates.E2E {
		t.Error("Quality.Gates.E2E should be true")
	}
	if !cfg.Quality.Gates.Browser {
		t.Error("Quality.Gates.Browser should be true")
	}
	if !cfg.Quality.Gates.DevLogs {
		t.Error("Quality.Gates.DevLogs should be true")
	}
}

func TestLoadAllFieldsSet(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ProjectConfigDir)
	os.MkdirAll(configDir, 0755)
	os.WriteFile(filepath.Join(configDir, "config.yaml"), []byte(`
engine:
  version: "2.0.0"
  name: "my-engine"
agent:
  type: "codex"
  model: "gpt-4"
  max_turns: 100
  max_stories_per_session: 10
  cooldown_seconds: 60
  allowed_tools: "Bash,Read"
  disallowed_tools: "Write"
  flags: ["--verbose", "--debug"]
workflow:
  type: "custom-flow"
  commands:
    build: "make"
    test: "go test"
  instructions: "Custom instructions"
  custom_phases: ["phase1", "phase2"]
quality:
  type: "strict"
  max_retries: 5
  gates:
    cr: true
    tests: true
    build: true
    type_check: true
    storybook: true
    e2e: true
    browser: true
    dev_logs: true
tracker:
  type: "github"
  status_file: "custom-status.yaml"
resources:
  min_free_ram_mb: 4096
  max_cpu_load_percent: 90
  min_free_disk_gb: 10
  max_log_size_mb: 100
  max_log_files: 20
circuit_breaker:
  max_failures: 5
  cooldown_minutes: 10
ssh:
  enabled: true
  reconnect_script: "./reconnect.sh"
  dev_exec_script: "./dev-exec.sh"
security:
  notice_accepted: true
  require_container: false
  daily_budget_usd: 50.0
  max_cost_per_session_usd: 10.0
paths:
  stories: "stories/"
  architecture: "docs/arch/"
  prd: "docs/prd/"
  ux: "docs/ux/"
  decisions: "docs/adr/"
  status: "status.yaml"
  rules: ".rules/"
research:
  enabled: true
  strategy: "story-start"
  tools:
    - name: "RAG"
      type: "rag"
      priority: 1
      enabled: true
      description: "Knowledge base"
`), 0644)

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}

	// Verify every field is set to the non-default value.
	tests := []struct {
		name string
		got  interface{}
		want interface{}
	}{
		{"Engine.Version", cfg.Engine.Version, "2.0.0"},
		{"Engine.Name", cfg.Engine.Name, "my-engine"},
		{"Agent.Type", cfg.Agent.Type, "codex"},
		{"Agent.Model", cfg.Agent.Model, "gpt-4"},
		{"Agent.MaxTurns", cfg.Agent.MaxTurns, 100},
		{"Agent.MaxStoriesPerSession", cfg.Agent.MaxStoriesPerSession, 10},
		{"Agent.CooldownSeconds", cfg.Agent.CooldownSeconds, 60},
		{"Agent.AllowedTools", cfg.Agent.AllowedTools, "Bash,Read"},
		{"Agent.DisallowedTools", cfg.Agent.DisallowedTools, "Write"},
		{"Workflow.Type", cfg.Workflow.Type, "custom-flow"},
		{"Workflow.Instructions", cfg.Workflow.Instructions, "Custom instructions"},
		{"Quality.Type", cfg.Quality.Type, "strict"},
		{"Quality.MaxRetries", cfg.Quality.MaxRetries, 5},
		{"Tracker.Type", cfg.Tracker.Type, "github"},
		{"Tracker.StatusFile", cfg.Tracker.StatusFile, "custom-status.yaml"},
		{"Resources.MinFreeRAMMB", cfg.Resources.MinFreeRAMMB, 4096},
		{"Resources.MaxCPULoadPercent", cfg.Resources.MaxCPULoadPercent, 90},
		{"Resources.MinFreeDiskGB", cfg.Resources.MinFreeDiskGB, 10},
		{"Resources.MaxLogSizeMB", cfg.Resources.MaxLogSizeMB, 100},
		{"Resources.MaxLogFiles", cfg.Resources.MaxLogFiles, 20},
		{"CircuitBreaker.MaxFailures", cfg.CircuitBreaker.MaxFailures, 5},
		{"CircuitBreaker.CooldownMinutes", cfg.CircuitBreaker.CooldownMinutes, 10},
		{"SSH.Enabled", cfg.SSH.Enabled, true},
		{"SSH.ReconnectScript", cfg.SSH.ReconnectScript, "./reconnect.sh"},
		{"SSH.DevExecScript", cfg.SSH.DevExecScript, "./dev-exec.sh"},
		{"Security.NoticeAccepted", cfg.Security.NoticeAccepted, true},
		{"Security.RequireContainer", cfg.Security.RequireContainer, false},
		{"Security.DailyBudgetUSD", cfg.Security.DailyBudgetUSD, float64(50)},
		{"Security.MaxCostPerSessionUSD", cfg.Security.MaxCostPerSessionUSD, float64(10)},
		{"Paths.Stories", cfg.Paths.Stories, "stories/"},
		{"Paths.Architecture", cfg.Paths.Architecture, "docs/arch/"},
		{"Paths.PRD", cfg.Paths.PRD, "docs/prd/"},
		{"Paths.UX", cfg.Paths.UX, "docs/ux/"},
		{"Paths.Decisions", cfg.Paths.Decisions, "docs/adr/"},
		{"Paths.Status", cfg.Paths.Status, "status.yaml"},
		{"Paths.Rules", cfg.Paths.Rules, ".rules/"},
		{"Research.Enabled", cfg.Research.Enabled, true},
		{"Research.Strategy", cfg.Research.Strategy, "story-start"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.got != tt.want {
				t.Errorf("%s = %v, want %v", tt.name, tt.got, tt.want)
			}
		})
	}

	// Verify collections.
	if len(cfg.Agent.Flags) != 2 {
		t.Errorf("Agent.Flags = %v, want 2 items", cfg.Agent.Flags)
	}
	if len(cfg.Workflow.Commands) != 2 {
		t.Errorf("Workflow.Commands = %v, want 2 items", cfg.Workflow.Commands)
	}
	if len(cfg.Workflow.CustomPhases) != 2 {
		t.Errorf("Workflow.CustomPhases = %v, want 2 items", cfg.Workflow.CustomPhases)
	}
	if len(cfg.Research.Tools) != 1 {
		t.Errorf("Research.Tools = %d, want 1", len(cfg.Research.Tools))
	}
}

func TestLoadTrackerCustomStatusFile(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ProjectConfigDir)
	os.MkdirAll(configDir, 0755)
	os.WriteFile(filepath.Join(configDir, "config.yaml"), []byte(`
tracker:
  type: file
  status_file: "custom-status.yaml"
`), 0644)

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}
	if cfg.Tracker.StatusFile != "custom-status.yaml" {
		t.Errorf("Tracker.StatusFile = %q, want %q", cfg.Tracker.StatusFile, "custom-status.yaml")
	}
}

func TestLoadAgentMaxTurnsZero(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ProjectConfigDir)
	os.MkdirAll(configDir, 0755)
	os.WriteFile(filepath.Join(configDir, "config.yaml"), []byte(`
agent:
  type: claude
  max_turns: 0
`), 0644)

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}
	if cfg.Agent.MaxTurns != 0 {
		t.Errorf("Agent.MaxTurns = %d, want 0 (unlimited)", cfg.Agent.MaxTurns)
	}
}

func TestLoadDisallowedToolsSet(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ProjectConfigDir)
	os.MkdirAll(configDir, 0755)
	os.WriteFile(filepath.Join(configDir, "config.yaml"), []byte(`
agent:
  type: claude
  disallowed_tools: "TodoWrite,Agent"
`), 0644)

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}
	if cfg.Agent.DisallowedTools != "TodoWrite,Agent" {
		t.Errorf("Agent.DisallowedTools = %q, want %q", cfg.Agent.DisallowedTools, "TodoWrite,Agent")
	}
}

func TestLoadPathsConfig(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ProjectConfigDir)
	os.MkdirAll(configDir, 0755)
	os.WriteFile(filepath.Join(configDir, "config.yaml"), []byte(`
paths:
  stories: "_bmad-output/stories/"
  architecture: "_bmad-output/architecture/"
  prd: "_bmad-output/prd/"
  ux: "_bmad-output/ux/"
  decisions: "_bmad-output/decisions/"
  status: "sprint-status.yaml"
  rules: ".claude/rules/"
`), 0644)

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() error: %v", err)
	}
	if cfg.Paths.Stories != "_bmad-output/stories/" {
		t.Errorf("Paths.Stories = %q", cfg.Paths.Stories)
	}
	if cfg.Paths.Architecture != "_bmad-output/architecture/" {
		t.Errorf("Paths.Architecture = %q", cfg.Paths.Architecture)
	}
	if cfg.Paths.PRD != "_bmad-output/prd/" {
		t.Errorf("Paths.PRD = %q", cfg.Paths.PRD)
	}
	if cfg.Paths.UX != "_bmad-output/ux/" {
		t.Errorf("Paths.UX = %q", cfg.Paths.UX)
	}
	if cfg.Paths.Decisions != "_bmad-output/decisions/" {
		t.Errorf("Paths.Decisions = %q", cfg.Paths.Decisions)
	}
	if cfg.Paths.Status != "sprint-status.yaml" {
		t.Errorf("Paths.Status = %q", cfg.Paths.Status)
	}
	if cfg.Paths.Rules != ".claude/rules/" {
		t.Errorf("Paths.Rules = %q", cfg.Paths.Rules)
	}
}
