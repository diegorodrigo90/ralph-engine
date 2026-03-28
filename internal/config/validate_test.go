package config

import (
	"os"
	"path/filepath"
	"testing"
)

func TestValidateMinimalValidConfig(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "sprint-status.yaml"), []byte("epics: []"), 0644)

	cfg := &Config{
		Agent:   AgentConfig{Type: "echo"}, // echo exists in PATH
		Tracker: TrackerConfig{Type: "file", StatusFile: "sprint-status.yaml"},
	}
	r := Validate(cfg, dir)
	if !r.OK() {
		t.Errorf("expected OK, got errors:\n%s", r.Summary())
	}
}

func TestValidateEmptyAgentType(t *testing.T) {
	cfg := &Config{Agent: AgentConfig{Type: ""}}
	r := Validate(cfg, t.TempDir())
	if r.OK() {
		t.Error("expected error for empty agent type")
	}
	if r.Errors[0].Field != "agent.type" {
		t.Errorf("expected agent.type error, got %s", r.Errors[0].Field)
	}
}

func TestValidateAgentNotInPath(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "status.yaml"), []byte("epics: []"), 0644)

	cfg := &Config{
		Agent:   AgentConfig{Type: "nonexistent-binary-xyz"},
		Tracker: TrackerConfig{Type: "file", StatusFile: "status.yaml"},
	}
	r := Validate(cfg, dir)
	// Should be a warning, not error (binary might be installed later).
	if len(r.Warnings) == 0 {
		t.Error("expected warning for missing binary")
	}
}

func TestValidateStatusFileMissing(t *testing.T) {
	cfg := &Config{
		Agent:   AgentConfig{Type: "echo"},
		Tracker: TrackerConfig{Type: "file", StatusFile: "nonexistent.yaml"},
	}
	r := Validate(cfg, t.TempDir())
	if r.OK() {
		t.Error("expected error for missing status file")
	}
}

func TestValidateStatusFileEmpty(t *testing.T) {
	cfg := &Config{
		Agent:   AgentConfig{Type: "echo"},
		Tracker: TrackerConfig{Type: "file", StatusFile: ""},
	}
	r := Validate(cfg, t.TempDir())
	if r.OK() {
		t.Error("expected error for empty status file")
	}
}

func TestValidateUnknownTrackerType(t *testing.T) {
	cfg := &Config{
		Agent:   AgentConfig{Type: "echo"},
		Tracker: TrackerConfig{Type: "jira"},
	}
	r := Validate(cfg, t.TempDir())
	if r.OK() {
		t.Error("expected error for unknown tracker type")
	}
}

func TestValidateNegativeCooldown(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "s.yaml"), []byte("epics: []"), 0644)

	cfg := &Config{
		Agent:   AgentConfig{Type: "echo", CooldownSeconds: -1},
		Tracker: TrackerConfig{Type: "file", StatusFile: "s.yaml"},
	}
	r := Validate(cfg, dir)
	if r.OK() {
		t.Error("expected error for negative cooldown")
	}
}

func TestValidateCPULoadOutOfRange(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "s.yaml"), []byte("epics: []"), 0644)

	cfg := &Config{
		Agent:     AgentConfig{Type: "echo"},
		Tracker:   TrackerConfig{Type: "file", StatusFile: "s.yaml"},
		Resources: ResourceConfig{MaxCPULoadPercent: 150},
	}
	r := Validate(cfg, dir)
	if r.OK() {
		t.Error("expected error for CPU load > 100")
	}
}

func TestValidateUnknownWorkflowWarning(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "s.yaml"), []byte("epics: []"), 0644)

	cfg := &Config{
		Agent:    AgentConfig{Type: "echo"},
		Tracker:  TrackerConfig{Type: "file", StatusFile: "s.yaml"},
		Workflow: WorkflowConfig{Type: "unknown-workflow"},
	}
	r := Validate(cfg, dir)
	if len(r.Warnings) == 0 {
		t.Error("expected warning for unknown workflow type")
	}
}

func TestValidateUnknownQualityWarning(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "s.yaml"), []byte("epics: []"), 0644)

	cfg := &Config{
		Agent:   AgentConfig{Type: "echo"},
		Tracker: TrackerConfig{Type: "file", StatusFile: "s.yaml"},
		Quality: QualityConfig{Type: "ultra"},
	}
	r := Validate(cfg, dir)
	if len(r.Warnings) == 0 {
		t.Error("expected warning for unknown quality type")
	}
}

func TestValidatePathsMissing(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "s.yaml"), []byte("epics: []"), 0644)

	cfg := &Config{
		Agent:   AgentConfig{Type: "echo"},
		Tracker: TrackerConfig{Type: "file", StatusFile: "s.yaml"},
		Paths:   PathsConfig{Stories: "nonexistent-dir/"},
	}
	r := Validate(cfg, dir)
	if len(r.Warnings) == 0 {
		t.Error("expected warning for missing stories path")
	}
}

func TestValidatePathsCustomMissing(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "s.yaml"), []byte("epics: []"), 0644)

	cfg := &Config{
		Agent:   AgentConfig{Type: "echo"},
		Tracker: TrackerConfig{Type: "file", StatusFile: "s.yaml"},
		Paths:   PathsConfig{Custom: map[string]string{"epics": "nonexistent/"}},
	}
	r := Validate(cfg, dir)
	if len(r.Warnings) == 0 {
		t.Error("expected warning for missing custom path")
	}
}

func TestValidateResearchEnabledNoTools(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "s.yaml"), []byte("epics: []"), 0644)

	cfg := &Config{
		Agent:    AgentConfig{Type: "echo"},
		Tracker:  TrackerConfig{Type: "file", StatusFile: "s.yaml"},
		Research: ResearchConfig{Enabled: true, Tools: []ResearchTool{}},
	}
	r := Validate(cfg, dir)
	if len(r.Warnings) == 0 {
		t.Error("expected warning for research enabled without tools")
	}
}

func TestValidateResearchToolNoPriority(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "s.yaml"), []byte("epics: []"), 0644)

	cfg := &Config{
		Agent:   AgentConfig{Type: "echo"},
		Tracker: TrackerConfig{Type: "file", StatusFile: "s.yaml"},
		Research: ResearchConfig{
			Enabled: true,
			Tools:   []ResearchTool{{Name: "RAG", Enabled: true, Priority: 0}},
		},
	}
	r := Validate(cfg, dir)
	hasWarning := false
	for _, w := range r.Warnings {
		if w.Field == "research.tools[0].priority" {
			hasWarning = true
		}
	}
	if !hasWarning {
		t.Error("expected warning for tool with priority 0")
	}
}

func TestValidateResearchDisabledSkips(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "s.yaml"), []byte("epics: []"), 0644)

	cfg := &Config{
		Agent:    AgentConfig{Type: "echo"},
		Tracker:  TrackerConfig{Type: "file", StatusFile: "s.yaml"},
		Research: ResearchConfig{Enabled: false, Tools: []ResearchTool{}},
	}
	r := Validate(cfg, dir)
	// Should not warn about empty tools when disabled.
	for _, w := range r.Warnings {
		if w.Field == "research.tools" {
			t.Error("should not warn about tools when research is disabled")
		}
	}
}

func TestValidateSSHScriptMissing(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "s.yaml"), []byte("epics: []"), 0644)

	cfg := &Config{
		Agent:   AgentConfig{Type: "echo"},
		Tracker: TrackerConfig{Type: "file", StatusFile: "s.yaml"},
		SSH:     SSHConfig{Enabled: true, DevExecScript: "nonexistent.sh"},
	}
	r := Validate(cfg, dir)
	if len(r.Warnings) == 0 {
		t.Error("expected warning for missing SSH script")
	}
}

func TestValidateSummaryOutput(t *testing.T) {
	cfg := &Config{Agent: AgentConfig{Type: ""}}
	r := Validate(cfg, t.TempDir())
	summary := r.Summary()
	if summary == "Config OK" {
		t.Error("summary should not be OK with errors")
	}
}

func TestValidateAllOKSummary(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "s.yaml"), []byte("epics: []"), 0644)

	cfg := &Config{
		Agent:          AgentConfig{Type: "echo", MaxStoriesPerSession: 5, CooldownSeconds: 10},
		Tracker:        TrackerConfig{Type: "file", StatusFile: "s.yaml"},
		CircuitBreaker: CircuitBreakerConfig{MaxFailures: 3},
	}
	r := Validate(cfg, dir)
	if r.Summary() != "Config OK" {
		t.Errorf("expected 'Config OK', got %q", r.Summary())
	}
}
