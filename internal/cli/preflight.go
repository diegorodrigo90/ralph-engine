package cli

import (
	"context"
	"fmt"
	"os"

	"github.com/diegorodrigo90/ralph-engine/internal/engine"
	"github.com/spf13/cobra"
)

var preflightCmd = &cobra.Command{
	Use:   "preflight",
	Short: "Run pre-execution checks without starting the loop",
	Long: `Checks project directory, agent binary, system resources,
and state directory writability. Use this to verify your setup.`,
	RunE: func(cmd *cobra.Command, args []string) error {
		projectDir, _ := cmd.Flags().GetString("project")
		binary, _ := cmd.Flags().GetString("binary")

		if projectDir == "" {
			wd, err := os.Getwd()
			if err != nil {
				return fmt.Errorf("getting working directory: %w", err)
			}
			projectDir = wd
		}

		eng, err := engine.New(engine.EngineOpts{
			ProjectDir: projectDir,
			Binary:     binary,
		})
		if err != nil {
			return fmt.Errorf("creating engine: %w", err)
		}

		results := eng.Preflight(context.Background())
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
			return fmt.Errorf("some preflight checks failed")
		}
		fmt.Println("\nAll preflight checks passed.")
		return nil
	},
}

func init() {
	preflightCmd.Flags().StringP("project", "d", "", "Project directory (default: current directory)")
	preflightCmd.Flags().StringP("binary", "b", "claude", "Agent binary to check")
}
