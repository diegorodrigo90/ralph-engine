package cli

import (
	"fmt"

	"github.com/diegorodrigo90/ralph-engine/internal/config"
	"github.com/spf13/cobra"
)

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Initialize project configuration",
	Long: `Creates a .ralph-engine/ directory in the project with default configuration.

Presets:
  basic      - Minimal: tests only, file tracker
  bmad-v6    - Full BMAD workflow: claudebox, all quality gates, SSH
  tdd-strict - TDD-focused: strict test-first, no skip`,
	RunE: func(cmd *cobra.Command, args []string) error {
		projectDir, _ := cmd.Flags().GetString("project")
		preset, _ := cmd.Flags().GetString("preset")

		if projectDir == "" {
			projectDir = "."
		}

		if err := config.InitProject(projectDir, preset); err != nil {
			return fmt.Errorf("initializing project: %w", err)
		}

		fmt.Printf("Initialized ralph-engine in %s/.ralph-engine/\n", projectDir)
		if preset != "" {
			fmt.Printf("Applied preset: %s\n", preset)
		}
		fmt.Println("\nNext steps:")
		fmt.Println("  1. Edit .ralph-engine/config.yaml to customize")
		fmt.Println("  2. Run: ralph-engine preflight")
		fmt.Println("  3. Run: ralph-engine run")

		return nil
	},
}

func init() {
	initCmd.Flags().StringP("project", "d", "", "Project directory (default: current directory)")
	initCmd.Flags().StringP("preset", "p", "", "Configuration preset: basic, bmad-v6, tdd-strict")
}
