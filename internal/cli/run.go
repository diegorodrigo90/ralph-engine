package cli

import (
	"context"
	"fmt"
	"os"
	"os/signal"
	"syscall"

	"github.com/diegorodrigo90/ralph-engine/internal/engine"
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

Testing modes:
  --dry-run          Show execution plan without calling the agent
  --max-iterations N Stop after N stories (great for testing)
  --single-story ID  Run only one specific story, then stop

Examples:
  ralph-engine run                          # Full autonomous loop
  ralph-engine run --dry-run                # Preview what would happen
  ralph-engine run --max-iterations 1       # Run exactly one story
  ralph-engine run --single-story 65.3      # Run only story 65.3
  ralph-engine run --binary claudebox       # Use claudebox instead of claude
  ralph-engine --debug run --dry-run        # Dry run with JSON debug output`,
	RunE: runEngine,
}

func runEngine(cmd *cobra.Command, args []string) error {
	projectDir, _ := cmd.Flags().GetString("project")
	stateDir, _ := cmd.Flags().GetString("state-dir")
	binary, _ := cmd.Flags().GetString("binary")
	cooldown, _ := cmd.Flags().GetInt("cooldown")
	maxFailures, _ := cmd.Flags().GetInt("max-failures")
	statusFile, _ := cmd.Flags().GetString("status-file")
	dryRun, _ := cmd.Flags().GetBool("dry-run")
	maxIterations, _ := cmd.Flags().GetInt("max-iterations")
	singleStory, _ := cmd.Flags().GetString("single-story")

	if projectDir == "" {
		wd, err := os.Getwd()
		if err != nil {
			return fmt.Errorf("getting working directory: %w", err)
		}
		projectDir = wd
	}
	if stateDir == "" {
		stateDir = projectDir
	}

	eng, err := engine.New(engine.EngineOpts{
		ProjectDir:    projectDir,
		StateDir:      stateDir,
		Binary:        binary,
		CooldownSeconds: cooldown,
		MaxFailures:   maxFailures,
		DryRun:        dryRun,
		MaxIterations: maxIterations,
		SingleStory:   singleStory,
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

func init() {
	f := runCmd.Flags()

	// Project settings.
	f.StringP("project", "d", "", "Project directory (default: current directory)")
	f.String("state-dir", "", "State directory for state.json (default: project directory)")
	f.String("status-file", "sprint-status.yaml", "Sprint status file name")

	// Agent settings.
	f.StringP("binary", "b", "claude", "Agent binary: claude, claudebox, cursor")

	// Loop control.
	f.Int("cooldown", 10, "Seconds between sessions")
	f.Int("max-failures", 3, "Circuit breaker: stop after N consecutive failures")

	// Testing modes.
	f.Bool("dry-run", false, "Show execution plan without calling the agent")
	f.IntP("max-iterations", "n", 0, "Stop after N iterations (0 = unlimited)")
	f.StringP("single-story", "s", "", "Run only this story ID, then stop")
}
