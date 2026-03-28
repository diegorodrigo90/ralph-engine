package cli

import (
	"fmt"

	"github.com/diegorodrigo90/ralph-engine/internal/updater"
	"github.com/spf13/cobra"
)

var updateCmd = &cobra.Command{
	Use:   "update",
	Short: "Update ralph-engine to the latest version",
	Long: `Check GitHub Releases for the latest version of ralph-engine
and replace the current binary if a newer version is available.

For npm installs, use: npm update -g ralph-engine
For Homebrew installs, use: brew upgrade ralph-engine`,
	RunE: func(cmd *cobra.Command, args []string) error {
		checkOnly, _ := cmd.Flags().GetBool("check")

		if checkOnly {
			return runCheckOnly()
		}
		return runUpdate()
	},
}

func init() {
	updateCmd.Flags().Bool("check", false, "Only check for updates without installing")
	rootCmd.AddCommand(updateCmd)
}

// runCheckOnly checks for updates and prints the result without installing.
func runCheckOnly() error {
	fmt.Println("Checking for updates...")

	result, err := updater.CheckLatest(Version)
	if err != nil {
		return fmt.Errorf("checking for updates: %w", err)
	}

	if result.UpToDate {
		fmt.Printf("ralph-engine %s is the latest version.\n", result.CurrentVersion)
		return nil
	}

	fmt.Printf("Update available: %s → %s\n", result.CurrentVersion, result.LatestVersion)
	fmt.Println("Run 'ralph-engine update' to install.")
	return nil
}

// runUpdate downloads and installs the latest version.
func runUpdate() error {
	if Version == "dev" {
		return fmt.Errorf("cannot update a development build — install from a release first")
	}

	fmt.Printf("Current version: %s\n", Version)
	fmt.Println("Checking for updates...")

	result, err := updater.Update(Version)
	if err != nil {
		return fmt.Errorf("update failed: %w", err)
	}

	if result.UpToDate {
		fmt.Printf("ralph-engine %s is already the latest version.\n", result.CurrentVersion)
		return nil
	}

	if result.Updated {
		fmt.Printf("Updated ralph-engine: %s → %s\n", result.CurrentVersion, result.LatestVersion)
		fmt.Println("Restart ralph-engine to use the new version.")
	}

	return nil
}
