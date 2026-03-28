package cli

import (
	"fmt"

	"github.com/diegorodrigo90/ralph-engine/internal/state"
	"github.com/spf13/cobra"
)

var statusCmd = &cobra.Command{
	Use:   "status",
	Short: "Show current engine state and progress",
	RunE: func(cmd *cobra.Command, args []string) error {
		stateDir, _ := cmd.Flags().GetString("state-dir")
		if stateDir == "" {
			stateDir = "."
		}

		s, err := state.Load(stateDir)
		if err != nil {
			return fmt.Errorf("loading state: %w", err)
		}

		fmt.Printf("Status:           %s\n", s.EngineStatus)
		fmt.Printf("Current Story:    %s\n", s.CurrentStory)
		fmt.Printf("Current Phase:    %s\n", s.CurrentPhase)
		fmt.Printf("Stories Done:     %d\n", s.StoriesCompletedTotal)
		fmt.Printf("Session:          %d\n", s.SessionNumber)
		fmt.Printf("Total Cost:       $%.2f\n", s.TotalCostUSD)
		if s.Blocked {
			fmt.Printf("Blocked:          %s\n", s.BlockedReason)
		}
		if s.FindingsCount > 0 {
			fmt.Printf("Findings:         %d\n", s.FindingsCount)
		}
		fmt.Printf("Last Checkpoint:  %s\n", s.LastCheckpoint)

		return nil
	},
}

func init() {
	statusCmd.Flags().String("state-dir", "", "State directory (default: current directory)")
}
