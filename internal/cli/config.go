package cli

import (
	"fmt"

	"github.com/diegorodrigo90/ralph-engine/internal/config"
	"github.com/spf13/cobra"
)

var configCmd = &cobra.Command{
	Use:   "config",
	Short: "Manage ralph-engine configuration",
	Long: `View and modify user-level configuration.
Settings are saved to ~/.config/ralph-engine/config.yaml`,
}

var configSetCmd = &cobra.Command{
	Use:   "set <key> <value>",
	Short: "Set a user configuration value",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		if err := config.Save(args[0], args[1]); err != nil {
			return fmt.Errorf("saving config: %w", err)
		}
		fmt.Printf("Set %s = %s\n", args[0], args[1])
		return nil
	},
}

var configListCmd = &cobra.Command{
	Use:   "list",
	Short: "Show merged configuration (all levels)",
	RunE: func(cmd *cobra.Command, args []string) error {
		projectDir, _ := cmd.Flags().GetString("project")
		if projectDir == "" {
			projectDir = "."
		}

		cfg, err := config.Load(projectDir)
		if err != nil {
			return fmt.Errorf("loading config: %w", err)
		}

		fmt.Printf("Agent:\n")
		fmt.Printf("  type:            %s\n", cfg.Agent.Type)
		fmt.Printf("  model:           %s\n", cfg.Agent.Model)
		fmt.Printf("  stories/session: %d\n", cfg.Agent.MaxStoriesPerSession)
		fmt.Printf("  cooldown_sec:    %d\n", cfg.Agent.CooldownSeconds)
		fmt.Printf("\nWorkflow:\n")
		fmt.Printf("  type:            %s\n", cfg.Workflow.Type)
		fmt.Printf("\nQuality:\n")
		fmt.Printf("  type:            %s\n", cfg.Quality.Type)
		fmt.Printf("  cr:              %v\n", cfg.Quality.Gates.CR)
		fmt.Printf("  tests:           %v\n", cfg.Quality.Gates.Tests)
		fmt.Printf("  build:           %v\n", cfg.Quality.Gates.Build)
		fmt.Printf("  type_check:      %v\n", cfg.Quality.Gates.TypeCheck)
		fmt.Printf("\nTracker:\n")
		fmt.Printf("  type:            %s\n", cfg.Tracker.Type)
		fmt.Printf("\nCircuit Breaker:\n")
		fmt.Printf("  max_failures:    %d\n", cfg.CircuitBreaker.MaxFailures)
		fmt.Printf("  cooldown_min:    %d\n", cfg.CircuitBreaker.CooldownMinutes)
		fmt.Printf("\nResources:\n")
		fmt.Printf("  min_free_ram_mb: %d\n", cfg.Resources.MinFreeRAMMB)
		fmt.Printf("  min_free_disk_gb: %d\n", cfg.Resources.MinFreeDiskGB)

		return nil
	},
}

func init() {
	configCmd.AddCommand(configSetCmd)
	configCmd.AddCommand(configListCmd)
	configListCmd.Flags().StringP("project", "d", "", "Project directory")
}
