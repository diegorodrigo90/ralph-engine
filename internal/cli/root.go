// Package cli implements the cobra command tree for ralph-engine.
// It provides subcommands for running, configuring, and monitoring
// the autonomous AI development loop.
package cli

import (
	"fmt"

	"github.com/diegorodrigo90/ralph-engine/internal/logger"
	"github.com/spf13/cobra"
)

// Version is set at build time via -ldflags.
var Version = "dev"

// Log is the global logger instance, configured by root flags.
var Log *logger.Logger

// rootCmd is the base command for ralph-engine.
var rootCmd = &cobra.Command{
	Use:   "ralph-engine",
	Short: "Autonomous AI development loop engine",
	Long: `ralph-engine orchestrates AI agent sessions in an infinite loop
with quality gates, resource monitoring, and progress persistence.

It calls Claude Code (or other agents) repeatedly, each invocation
getting fresh context. State persists in files between sessions.

The engine NEVER manages Claude billing or usage limits — it only
detects limits and saves progress for graceful resume.`,
	PersistentPreRun: func(cmd *cobra.Command, args []string) {
		debug, _ := cmd.Flags().GetBool("debug")
		logFormat, _ := cmd.Flags().GetString("log-format")

		format := logger.FormatHuman
		if logFormat == "json" {
			format = logger.FormatJSON
		}
		// Auto-select JSON in debug mode for AI-friendly output.
		if debug && logFormat == "" {
			format = logger.FormatJSON
		}

		Log = logger.New(logger.Config{
			Debug:  debug,
			Format: format,
		})
	},
}

// Execute runs the root command. Called from main.
func Execute() error {
	return rootCmd.Execute()
}

func init() {
	// Global flags available to all subcommands.
	rootCmd.PersistentFlags().Bool("debug", false, "Enable debug mode (verbose JSON output for AI agents)")
	rootCmd.PersistentFlags().String("log-format", "", "Log format: human, json (default: human, auto-json in debug)")

	rootCmd.AddCommand(runCmd)
	rootCmd.AddCommand(prepareCmd)
	rootCmd.AddCommand(statusCmd)
	rootCmd.AddCommand(configCmd)
	rootCmd.AddCommand(initCmd)
	rootCmd.AddCommand(versionCmd)
}

// BuildCommit is set at build time via -ldflags (short SHA).
var BuildCommit = "unknown"

// BuildDate is set at build time via -ldflags.
var BuildDate = "unknown"

// versionCmd prints the build version.
var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Show version information",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Printf("ralph-engine %s\n", Version)
		if Version == "dev" {
			fmt.Println("  (development build — install from a release for version tracking)")
		} else {
			fmt.Printf("  commit: %s\n  date:   %s\n", BuildCommit, BuildDate)
		}
	},
}
