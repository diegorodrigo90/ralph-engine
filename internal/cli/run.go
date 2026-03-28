package cli

import (
	"context"
	"fmt"
	"os"
	"os/signal"
	"path/filepath"
	"strings"
	"syscall"

	"github.com/diegorodrigo90/ralph-engine/internal/config"
	"github.com/diegorodrigo90/ralph-engine/internal/engine"
	"github.com/diegorodrigo90/ralph-engine/internal/hooks"
	"github.com/diegorodrigo90/ralph-engine/internal/tracker"
	"github.com/spf13/cobra"
)

var runCmd = &cobra.Command{
	Use:   "run",
	Short: "Start the autonomous development loop",
	Long: `Starts the loop: preflight → pick story → call agent → quality gates → repeat.

The loop continues until all stories are complete, the circuit breaker trips,
a usage limit is detected, or the user interrupts (Ctrl+C).

Progress is saved after every commit. Resume with: ralph-engine run

Configuration is read from .ralph-engine/config.yaml in the project directory.
CLI flags override config values. Run 'ralph-engine init' to create config.

Testing modes:
  --dry-run          Show execution plan without calling the agent
  --max-iterations N Stop after N stories (great for testing)
  --single-story ID  Run only one specific story, then stop

Examples:
  ralph-engine run                          # Full autonomous loop
  ralph-engine run --dry-run                # Preview what would happen
  ralph-engine run --max-iterations 1       # Run exactly one story
  ralph-engine run --single-story 65.3      # Run only story 65.3
  ralph-engine run --binary claudebox       # Override agent from config
  ralph-engine --debug run --dry-run        # Dry run with JSON debug output`,
	RunE: runEngine,
}

func runEngine(cmd *cobra.Command, args []string) error {
	projectDir, _ := cmd.Flags().GetString("project")
	if projectDir == "" {
		wd, err := os.Getwd()
		if err != nil {
			return fmt.Errorf("getting working directory: %w", err)
		}
		projectDir = wd
	}

	// Load config from .ralph-engine/config.yaml (if exists).
	cfg, _ := config.Load(projectDir)

	// CLI flags override config. Only override if flag was explicitly set.
	binary := configOrFlag(cmd, "binary", cfg.Agent.Type)
	cooldown := configOrFlagInt(cmd, "cooldown", cfg.Agent.CooldownSeconds)
	maxFailures := configOrFlagInt(cmd, "max-failures", cfg.CircuitBreaker.MaxFailures)
	statusFile := configOrFlag(cmd, "status-file", cfg.Tracker.StatusFile)

	// State dir defaults to .ralph-engine/ inside project.
	stateDir, _ := cmd.Flags().GetString("state-dir")
	if stateDir == "" {
		stateDir = filepath.Join(projectDir, ".ralph-engine")
		os.MkdirAll(stateDir, 0755)
	}

	// Persist explicitly-changed flags to config if --save was passed.
	save, _ := cmd.Flags().GetBool("save")
	if save {
		saved := saveChangedFlags(cmd, projectDir, map[string]string{
			"binary":       "agent.type",
			"cooldown":     "agent.cooldown_seconds",
			"max-failures": "circuit_breaker.max_failures",
			"status-file":  "tracker.status_file",
		})
		if len(saved) > 0 {
			fmt.Printf("Saved to .ralph-engine/config.yaml: %s\n\n", strings.Join(saved, ", "))
		}
	}

	// Testing flags (no config equivalent — always from CLI).
	dryRun, _ := cmd.Flags().GetBool("dry-run")
	maxIterations, _ := cmd.Flags().GetInt("max-iterations")
	singleStory, _ := cmd.Flags().GetString("single-story")

	// Load hooks.yaml (lifecycle hooks for quality gates, etc.).
	hooksConfig, err := hooks.Load(projectDir)
	if err != nil {
		return fmt.Errorf("loading hooks: %w", err)
	}

	eng, err := engine.New(engine.EngineOpts{
		ProjectDir:      projectDir,
		StateDir:        stateDir,
		Binary:          binary,
		CooldownSeconds: cooldown,
		MaxFailures:     maxFailures,
		DryRun:          dryRun,
		MaxIterations:   maxIterations,
		SingleStory:     singleStory,
		WorkflowType:    cfg.Workflow.Type,
		QualityGate:     cfg.Quality.Type,
		Paths:           &cfg.Paths,
		Prompt:          &cfg.Prompt,
		Research:        &cfg.Research,
		Hooks:           hooksConfig,
		MaxGateRetries:  cfg.Quality.MaxRetries,
	})
	if err != nil {
		return fmt.Errorf("creating engine: %w", err)
	}

	// Graceful shutdown on Ctrl+C.
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	sigCh := make(chan os.Signal, 1)
	signal.Notify(sigCh, syscall.SIGINT, syscall.SIGTERM)
	go func() {
		<-sigCh
		fmt.Println("\nReceived interrupt. Saving progress...")
		cancel()
	}()

	// Validate config before running.
	validation := config.Validate(cfg, projectDir)
	if !validation.OK() {
		fmt.Println("Config validation failed:")
		fmt.Print(validation.Summary())
		return fmt.Errorf("fix config errors above before running")
	}
	if len(validation.Warnings) > 0 {
		fmt.Println("Config warnings:")
		fmt.Print(validation.Summary())
		fmt.Println()
	}

	// Run preflight (skip in dry-run for speed).
	if !dryRun {
		fmt.Println("Running preflight checks...")
		results := eng.Preflight(ctx)
		allOK := true
		for _, r := range results {
			icon := "✓"
			if !r.OK {
				icon = "✗"
				allOK = false
			}
			fmt.Printf("  %s %s: %s\n", icon, r.Name, r.Message)
		}
		if !allOK {
			return fmt.Errorf("preflight checks failed — fix issues above before running")
		}
		fmt.Println("\nPreflight passed. Starting engine...")
	}

	fmt.Println("Press Ctrl+C to save progress and stop.")
	fmt.Println()

	// Create tracker (auto-detects flat vs structured YAML format).
	tk := tracker.AutoDetect(projectDir, statusFile)

	// Run the loop.
	result := eng.Run(ctx, tk, func(event engine.EngineEvent) {
		switch event.Type {
		case "session_start":
			fmt.Printf("\n▶ %s\n", event.Message)
		case "story_complete":
			fmt.Printf("  ✓ %s\n", event.Message)
		case "error":
			fmt.Printf("  ✗ %s\n", event.Message)
		case "info":
			fmt.Printf("  · %s\n", event.Message)
		}
	})

	// Report result.
	fmt.Printf("\n─── Engine stopped ───\n")
	fmt.Printf("  Reason:     %s\n", result.ExitReason)
	fmt.Printf("  Sessions:   %d\n", result.SessionsRun)
	fmt.Printf("  Stories:    %d completed\n", result.StoriesComplete)
	if result.TotalCostUSD > 0 {
		fmt.Printf("  Cost:       $%.2f\n", result.TotalCostUSD)
	}

	if result.ExitReason == engine.ExitUsageLimit {
		fmt.Println("\nUsage limit reached. Progress saved. Resume with:")
		fmt.Println("  ralph-engine run")
	}

	if result.Error != nil {
		return result.Error
	}
	return nil
}

// configOrFlag returns the CLI flag value if explicitly set, otherwise the config value.
func configOrFlag(cmd *cobra.Command, flagName, configValue string) string {
	if cmd.Flags().Changed(flagName) {
		val, _ := cmd.Flags().GetString(flagName)
		return val
	}
	if configValue != "" {
		return configValue
	}
	val, _ := cmd.Flags().GetString(flagName)
	return val
}

// configOrFlagInt returns the CLI flag value if explicitly set, otherwise the config value.
func configOrFlagInt(cmd *cobra.Command, flagName string, configValue int) int {
	if cmd.Flags().Changed(flagName) {
		val, _ := cmd.Flags().GetInt(flagName)
		return val
	}
	if configValue > 0 {
		return configValue
	}
	val, _ := cmd.Flags().GetInt(flagName)
	return val
}

// saveChangedFlags persists explicitly-set CLI flags to the project config file.
// Only flags the user actually typed are saved — defaults are never persisted.
func saveChangedFlags(cmd *cobra.Command, projectDir string, flagToKey map[string]string) []string {
	var saved []string
	for flag, key := range flagToKey {
		if !cmd.Flags().Changed(flag) {
			continue
		}
		val, _ := cmd.Flags().GetString(flag)
		if val == "" {
			// Try int.
			if intVal, err := cmd.Flags().GetInt(flag); err == nil {
				val = fmt.Sprintf("%d", intVal)
			}
		}
		if err := config.SaveProject(projectDir, key, val); err == nil {
			saved = append(saved, fmt.Sprintf("%s=%s", key, val))
		}
	}
	return saved
}

func init() {
	f := runCmd.Flags()

	// Project settings.
	f.StringP("project", "d", "", "Project directory (default: current directory)")
	f.String("state-dir", "", "State directory (default: .ralph-engine/ in project)")
	f.String("status-file", "sprint-status.yaml", "Sprint status file (overrides config)")

	// Agent settings (override .ralph-engine/config.yaml).
	f.StringP("binary", "b", "claude", "Agent binary (overrides config)")

	// Loop control (override config).
	f.Int("cooldown", 30, "Seconds between sessions (overrides config)")
	f.Int("max-failures", 3, "Circuit breaker threshold (overrides config)")

	// Persistence.
	f.Bool("save", false, "Save explicitly-set flags to .ralph-engine/config.yaml")

	// Testing modes (CLI only, no config equivalent).
	f.Bool("dry-run", false, "Show execution plan without calling the agent")
	f.IntP("max-iterations", "n", 0, "Stop after N iterations (0 = unlimited)")
	f.StringP("single-story", "s", "", "Run only this story ID, then stop")
}
