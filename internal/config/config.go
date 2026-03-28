// Package config implements the 4-level configuration cascade for ralph-engine.
// Precedence: CLI flags > ENV vars > Project config > User config > Defaults.
package config

import (
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"strings"

	"github.com/spf13/viper"
)

// Config holds the merged configuration from all sources.
type Config struct {
	// Engine settings
	Engine EngineConfig `mapstructure:"engine"`
	// Agent settings (which AI agent to use)
	Agent AgentConfig `mapstructure:"agent"`
	// Workflow settings
	Workflow WorkflowConfig `mapstructure:"workflow"`
	// Quality gate settings
	Quality QualityConfig `mapstructure:"quality"`
	// Task tracker settings
	Tracker TrackerConfig `mapstructure:"tracker"`
	// Resource limits
	Resources ResourceConfig `mapstructure:"resources"`
	// Circuit breaker settings
	CircuitBreaker CircuitBreakerConfig `mapstructure:"circuit_breaker"`
	// SSH settings
	SSH SSHConfig `mapstructure:"ssh"`
	// Security settings
	Security SecurityConfig `mapstructure:"security"`
	// Project paths for context injection
	Paths PathsConfig `mapstructure:"paths"`
}

// EngineConfig holds core engine settings.
type EngineConfig struct {
	Version string `mapstructure:"version"`
	Name    string `mapstructure:"name"`
}

// AgentConfig defines which AI agent to use and how.
type AgentConfig struct {
	Type                 string   `mapstructure:"type"`
	Flags                []string `mapstructure:"flags"`
	Model                string   `mapstructure:"model"`
	MaxStoriesPerSession int      `mapstructure:"max_stories_per_session"`
	CooldownSeconds      int      `mapstructure:"cooldown_seconds"`
	AllowedTools         string   `mapstructure:"allowed_tools"`
}

// WorkflowConfig defines the development workflow to follow.
type WorkflowConfig struct {
	Type         string   `mapstructure:"type"`
	CustomPhases []string `mapstructure:"custom_phases"`
}

// QualityConfig defines which quality gates to enforce.
type QualityConfig struct {
	Type      string     `mapstructure:"type"`
	Gates     GatesConfig `mapstructure:"gates"`
}

// GatesConfig toggles individual quality gates.
type GatesConfig struct {
	CR        bool `mapstructure:"cr"`
	Tests     bool `mapstructure:"tests"`
	Build     bool `mapstructure:"build"`
	TypeCheck bool `mapstructure:"type_check"`
	Storybook bool `mapstructure:"storybook"`
	E2E       bool `mapstructure:"e2e"`
	Browser   bool `mapstructure:"browser"`
	DevLogs   bool `mapstructure:"dev_logs"`
}

// TrackerConfig defines how to track task progress.
type TrackerConfig struct {
	Type       string `mapstructure:"type"`
	StatusFile string `mapstructure:"status_file"`
}

// ResourceConfig sets host resource safety limits.
type ResourceConfig struct {
	MinFreeRAMMB       int `mapstructure:"min_free_ram_mb"`
	MaxCPULoadPercent  int `mapstructure:"max_cpu_load_percent"`
	MinFreeDiskGB      int `mapstructure:"min_free_disk_gb"`
	MaxLogSizeMB       int `mapstructure:"max_log_size_mb"`
	MaxLogFiles        int `mapstructure:"max_log_files"`
}

// CircuitBreakerConfig controls stagnation detection.
type CircuitBreakerConfig struct {
	MaxFailures     int `mapstructure:"max_failures"`
	CooldownMinutes int `mapstructure:"cooldown_minutes"`
}

// SSHConfig controls remote execution settings.
type SSHConfig struct {
	Enabled          bool   `mapstructure:"enabled"`
	ReconnectScript  string `mapstructure:"reconnect_script"`
	DevExecScript    string `mapstructure:"dev_exec_script"`
}

// SecurityConfig controls security-related settings.
type SecurityConfig struct {
	NoticeAccepted       bool    `mapstructure:"notice_accepted"`
	RequireContainer     bool    `mapstructure:"require_container"`
	DailyBudgetUSD       float64 `mapstructure:"daily_budget_usd"`
	MaxCostPerSessionUSD float64 `mapstructure:"max_cost_per_session_usd"`
}

// PathsConfig maps project artifact locations for context injection.
// The engine reads files from these paths and injects them into the agent prompt.
// All paths are relative to the project root. Supports glob patterns.
// The engine is format-agnostic — it reads files as-is and passes them to the agent.
type PathsConfig struct {
	// Stories directory or glob (e.g., "stories/", "_bmad-output/implementation-artifacts/*.md")
	Stories string `mapstructure:"stories"`
	// Architecture docs (e.g., "docs/architecture/", "_bmad-output/planning-artifacts/architecture/")
	Architecture string `mapstructure:"architecture"`
	// Product requirements (e.g., "docs/prd/", "_bmad-output/planning-artifacts/prd/")
	PRD string `mapstructure:"prd"`
	// UX specifications (e.g., "docs/ux/", "_bmad-output/planning-artifacts/ux-design-specification/")
	UX string `mapstructure:"ux"`
	// ADRs / product decisions (e.g., "docs/decisions/", "adr/")
	Decisions string `mapstructure:"decisions"`
	// Sprint/project status (e.g., "sprint-status.yaml", "TODO.md")
	Status string `mapstructure:"status"`
	// Rules / coding standards (e.g., ".claude/rules/", "docs/standards/")
	Rules string `mapstructure:"rules"`
	// Custom paths — arbitrary key-value pairs for project-specific artifacts
	Custom map[string]string `mapstructure:"custom"`
}

const (
	// AppName is the CLI application name.
	AppName = "ralph-engine"
	// ConfigFileName is the config file name without extension.
	ConfigFileName = "config"
	// ProjectConfigDir is the project-level config directory.
	ProjectConfigDir = ".ralph-engine"
)

// setDefaults registers all default values.
func setDefaults(v *viper.Viper) {
	// Engine
	v.SetDefault("engine.version", "1.0.0")
	v.SetDefault("engine.name", "")

	// Agent
	v.SetDefault("agent.type", "claude")
	v.SetDefault("agent.flags", []string{})
	v.SetDefault("agent.model", "")
	v.SetDefault("agent.max_stories_per_session", 5)
	v.SetDefault("agent.cooldown_seconds", 30)
	v.SetDefault("agent.allowed_tools", "Write,Read,Edit,Bash,Glob,Grep,Skill,Agent,WebSearch,WebFetch,ToolSearch")

	// Workflow
	v.SetDefault("workflow.type", "basic")
	v.SetDefault("workflow.custom_phases", []string{})

	// Quality
	v.SetDefault("quality.type", "standard")
	v.SetDefault("quality.gates.cr", true)
	v.SetDefault("quality.gates.tests", true)
	v.SetDefault("quality.gates.build", true)
	v.SetDefault("quality.gates.type_check", true)
	v.SetDefault("quality.gates.storybook", false)
	v.SetDefault("quality.gates.e2e", false)
	v.SetDefault("quality.gates.browser", false)
	v.SetDefault("quality.gates.dev_logs", false)

	// Tracker
	v.SetDefault("tracker.type", "file")
	v.SetDefault("tracker.status_file", "sprint-status.yaml")

	// Resources
	v.SetDefault("resources.min_free_ram_mb", 2048)
	v.SetDefault("resources.max_cpu_load_percent", 80)
	v.SetDefault("resources.min_free_disk_gb", 5)
	v.SetDefault("resources.max_log_size_mb", 50)
	v.SetDefault("resources.max_log_files", 10)

	// Circuit breaker
	v.SetDefault("circuit_breaker.max_failures", 3)
	v.SetDefault("circuit_breaker.cooldown_minutes", 5)

	// SSH
	v.SetDefault("ssh.enabled", false)
	v.SetDefault("ssh.reconnect_script", "")
	v.SetDefault("ssh.dev_exec_script", "")

	// Security
	v.SetDefault("security.notice_accepted", false)
	v.SetDefault("security.require_container", true)
	v.SetDefault("security.daily_budget_usd", 0)
	v.SetDefault("security.max_cost_per_session_usd", 0)

	// Paths — all empty by default, user configures per project.
	// Engine reads files from these paths and injects into agent prompt.
	v.SetDefault("paths.stories", "")
	v.SetDefault("paths.architecture", "")
	v.SetDefault("paths.prd", "")
	v.SetDefault("paths.ux", "")
	v.SetDefault("paths.decisions", "")
	v.SetDefault("paths.status", "")
	v.SetDefault("paths.rules", "")
}

// userConfigDir returns the platform-appropriate user config directory.
func userConfigDir() string {
	if xdg := os.Getenv("XDG_CONFIG_HOME"); xdg != "" {
		return filepath.Join(xdg, AppName)
	}
	home, err := os.UserHomeDir()
	if err != nil {
		return ""
	}
	if runtime.GOOS == "windows" {
		return filepath.Join(home, "AppData", "Roaming", AppName)
	}
	return filepath.Join(home, ".config", AppName)
}

// Load reads configuration from all sources and merges them.
// Precedence: CLI flags > ENV vars > Project config > User config > Defaults.
func Load(projectDir string) (*Config, error) {
	v := viper.New()

	// 1. Set defaults (lowest priority)
	setDefaults(v)

	// 2. User config (~/.config/ralph-engine/config.yaml)
	userDir := userConfigDir()
	if userDir != "" {
		v.SetConfigName(ConfigFileName)
		v.SetConfigType("yaml")
		v.AddConfigPath(userDir)
		// Ignore error if user config doesn't exist
		_ = v.MergeInConfig()
	}

	// 3. Project config (.ralph-engine/config.yaml) — higher priority
	if projectDir != "" {
		projectConfigPath := filepath.Join(projectDir, ProjectConfigDir)
		v.SetConfigName(ConfigFileName)
		v.SetConfigType("yaml")
		v.AddConfigPath(projectConfigPath)
		_ = v.MergeInConfig()
	}

	// 4. Environment variables (higher priority)
	v.SetEnvPrefix("RALPH")
	v.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))
	v.AutomaticEnv()

	// 5. CLI flags are handled by cobra binding (highest priority)
	// Those are bound externally via viper.BindPFlag()

	var cfg Config
	if err := v.Unmarshal(&cfg); err != nil {
		return nil, fmt.Errorf("failed to unmarshal config: %w", err)
	}

	return &cfg, nil
}

// Save writes a key-value pair to the user config file.
// Save writes a value to the user-level config (~/.config/ralph-engine/config.yaml).
func Save(key, value string) error {
	userDir := userConfigDir()
	if userDir == "" {
		return fmt.Errorf("cannot determine user config directory")
	}
	return saveToDir(userDir, key, value)
}

// SaveProject writes a value to the project-level config (.ralph-engine/config.yaml).
func SaveProject(projectDir, key, value string) error {
	configDir := filepath.Join(projectDir, ProjectConfigDir)
	return saveToDir(configDir, key, value)
}

// saveToDir writes a key-value to a config file in the given directory.
func saveToDir(dir, key, value string) error {
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("creating config directory: %w", err)
	}

	v := viper.New()
	v.SetConfigName(ConfigFileName)
	v.SetConfigType("yaml")
	v.AddConfigPath(dir)
	_ = v.ReadInConfig()

	v.Set(key, value)

	configPath := filepath.Join(dir, ConfigFileName+".yaml")
	return v.WriteConfigAs(configPath)
}

// InitProject creates the project config directory and default config file.
// InitProject creates the .ralph-engine/ directory with config, prompt, and hooks.
// If files already exist, they are NOT overwritten (user configs are sacred).
// The preset parameter selects which boilerplate to use.
func InitProject(projectDir, preset string) error {
	configDir := filepath.Join(projectDir, ProjectConfigDir)
	if err := os.MkdirAll(configDir, 0755); err != nil {
		return fmt.Errorf("creating config directory: %w", err)
	}

	if preset == "" {
		preset = "basic"
	}

	// Write config.yaml (only if not exists).
	configPath := filepath.Join(configDir, ConfigFileName+".yaml")
	if err := writeIfNotExists(configPath, presetConfig(preset)); err != nil {
		return fmt.Errorf("writing config: %w", err)
	}

	// Write prompt.md (only if not exists).
	promptPath := filepath.Join(configDir, "prompt.md")
	if err := writeIfNotExists(promptPath, presetPrompt(preset)); err != nil {
		return fmt.Errorf("writing prompt: %w", err)
	}

	// Write hooks.yaml (only if not exists).
	hooksPath := filepath.Join(configDir, "hooks.yaml")
	if err := writeIfNotExists(hooksPath, presetHooks(preset)); err != nil {
		return fmt.Errorf("writing hooks: %w", err)
	}

	// Write .gitignore for state files (only if not exists).
	gitignorePath := filepath.Join(configDir, ".gitignore")
	writeIfNotExists(gitignorePath, `# Runtime state — do not commit
state.json
state.json.tmp
*.log
`)

	return nil
}

// writeIfNotExists writes content to path only if the file does not exist.
// This ensures ralph-engine init never overwrites user customizations.
func writeIfNotExists(path, content string) error {
	if _, err := os.Stat(path); err == nil {
		return nil // File exists — do not overwrite.
	}
	return os.WriteFile(path, []byte(content), 0644)
}

// presetConfig returns the config.yaml content for a preset.
func presetConfig(preset string) string {
	switch preset {
	case "bmad-v6":
		return `# ralph-engine config — BMAD v6 preset
# Full workflow with all quality gates. Customize freely.
# ralph-engine NEVER overwrites this file.

agent:
  type: "claude"
  model: "opus"
  max_stories_per_session: 4
  cooldown_seconds: 15

workflow:
  type: "bmad-v6"

quality:
  type: "full"
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
  type: "file"
  status_file: "sprint-status.yaml"

circuit_breaker:
  max_failures: 3
  cooldown_minutes: 5

resources:
  min_free_ram_mb: 2048
  max_cpu_load_percent: 80
  min_free_disk_gb: 5

ssh:
  enabled: true
  dev_exec_script: "./scripts/dev-exec.sh"
  reconnect_script: "./scripts/claude-dev.sh"
`
	case "tdd-strict":
		return `# ralph-engine config — TDD strict preset
# Test-first development, stricter circuit breaker.

agent:
  type: "claude"
  max_stories_per_session: 3
  cooldown_seconds: 15

workflow:
  type: "tdd-strict"

quality:
  type: "standard"
  gates:
    cr: true
    tests: true
    build: true
    type_check: true

tracker:
  type: "file"
  status_file: "sprint-status.yaml"

circuit_breaker:
  max_failures: 2
  cooldown_minutes: 5
`
	default: // basic
		return `# ralph-engine config — basic preset
# Minimal setup. Customize as your project grows.
# ralph-engine NEVER overwrites this file.

agent:
  type: "claude"
  max_stories_per_session: 5
  cooldown_seconds: 10

workflow:
  type: "basic"

quality:
  type: "minimal"
  gates:
    tests: true
    build: true

tracker:
  type: "file"
  status_file: "sprint-status.yaml"

circuit_breaker:
  max_failures: 3
  cooldown_minutes: 5
`
	}
}

// presetPrompt returns the prompt.md content for a preset.
func presetPrompt(preset string) string {
	switch preset {
	case "bmad-v6":
		return `# Ralph Engine — Project Prompt (BMAD v6)

<!-- Injected into every AI session. Customize for YOUR project. -->
<!-- ralph-engine NEVER overwrites this file. -->

## BMAD v6 Workflow

1. Read story file — understand all acceptance criteria.
2. DoR validation — ACs testable, tasks sequenced, deps resolved.
3. TDD per AC — RED (failing test) → GREEN (implement) → REFACTOR.
4. Code review — fix ALL findings before commit.
5. Quality gates — tests → build → type-check → dev logs.
6. Commit — conventional message with story ID.
7. Update tracker — mark story done.
8. Note findings — report discovered issues.

## Quality Rules

- ALL tests SHALL pass before commit.
- Build SHALL pass for the full project.
- Type-check SHALL show zero errors.
- Code review findings SHALL be fixed.
- Dev logs SHALL show ZERO errors.

## Project Context

<!-- Add YOUR project-specific context here: -->
<!-- - Tech stack and key libraries -->
<!-- - Architecture patterns (DDD, FSD, etc.) -->
<!-- - Testing conventions -->
<!-- - File structure -->
<!-- - Domain terminology -->

## Progress Persistence

After EVERY commit, report stories completed.
If usage limit approaching: commit → update tracker → save handoff.
`
	case "tdd-strict":
		return `# Ralph Engine — Project Prompt (TDD Strict)

<!-- Customize for YOUR project. ralph-engine never overwrites this. -->

## TDD Rules (MANDATORY)

For EVERY acceptance criterion:
1. RED — Write failing test. Run it. Confirm failure.
2. GREEN — Write MINIMAL code to pass. Nothing more.
3. REFACTOR — Clean up while tests stay green.

NEVER write implementation before the test fails.

## Commit per Cycle

Each commit: failing test + implementation + refactoring.

## Project Context

<!-- Add YOUR project details here -->
`
	default:
		return `# Ralph Engine — Project Prompt

<!-- Injected into every AI session. Customize for YOUR project. -->
<!-- ralph-engine NEVER overwrites this file. -->

## Project Context

<!-- Replace with your project description, tech stack, conventions. -->

## Development Rules

- Write tests for important functionality.
- Use conventional commit messages.
- Keep code clean and well-organized.

## Progress

After completing a story: commit, report complete.
If approaching usage limits: save progress immediately.
`
	}
}

// presetHooks returns the hooks.yaml content for a preset.
func presetHooks(preset string) string {
	switch preset {
	case "bmad-v6":
		return `# ralph-engine hooks — BMAD v6 preset
# Customize every step. ralph-engine never overwrites this file.
# Empty run = skip. required: false = warn and continue.

preflight:
  steps:
    - name: "SSH connectivity"
      run: "./scripts/dev-exec.sh echo ok"
      required: false
    - name: "Dev server"
      run: "./scripts/dev-exec.sh curl -sf http://localhost:3000/health || true"
      required: false

pre_story:
  steps: []

post_story:
  steps: []

quality_gates:
  steps:
    - name: "Tests"
      run: "./scripts/dev-exec.sh NODE_ENV=development pnpm test"
      required: true
    - name: "Type check"
      run: "./scripts/dev-exec.sh pnpm type-check"
      required: true
    - name: "Build"
      run: "./scripts/dev-exec.sh pnpm build"
      required: true

post_session:
  steps: []
`
	default:
		return `# ralph-engine hooks
# Customize every step. ralph-engine never overwrites this file.
# Empty run = skip. required: false = warn and continue.

preflight:
  steps: []

pre_story:
  steps: []

post_story:
  steps: []

quality_gates:
  steps:
    - name: "Tests"
      run: "echo 'Replace with your test command'"
      required: true
    - name: "Build"
      run: "echo 'Replace with your build command'"
      required: true

post_session:
  steps: []
`
	}
}
