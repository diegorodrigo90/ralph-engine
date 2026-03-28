package cli

import (
	"fmt"

	"github.com/diegorodrigo90/ralph-engine/internal/config"
	"github.com/diegorodrigo90/ralph-engine/internal/detect"
	"github.com/spf13/cobra"
)

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Initialize project configuration",
	Long: `Creates a .ralph-engine/ directory in the project with default configuration.

Scans the project for existing tools and suggests the best preset.

Presets:
  basic      - Minimal: tests only, file tracker
  bmad-v6    - Full BMAD workflow: all quality gates, SSH
  tdd-strict - TDD-focused: strict test-first, no skip`,
	RunE: func(cmd *cobra.Command, args []string) error {
		projectDir, _ := cmd.Flags().GetString("project")
		preset, _ := cmd.Flags().GetString("preset")
		skipDetect, _ := cmd.Flags().GetBool("skip-detect")

		if projectDir == "" {
			projectDir = "."
		}

		// Auto-detect project tools (unless skipped or preset explicitly set).
		if !skipDetect {
			info := detect.Scan(projectDir)
			printDetectedTools(info)

			// Use detected preset if user didn't specify one.
			if preset == "" {
				preset = info.SuggestedPreset
				fmt.Printf("  Suggested preset: %s\n\n", preset)
			}
		}

		if preset == "" {
			preset = "basic"
		}

		if err := config.InitProject(projectDir, preset); err != nil {
			return fmt.Errorf("initializing project: %w", err)
		}

		fmt.Printf("Initialized ralph-engine in %s/.ralph-engine/\n", projectDir)
		fmt.Printf("Applied preset: %s\n", preset)
		fmt.Println("\nCreated files (never overwritten on re-init):")
		fmt.Println("  config.yaml   — engine configuration")
		fmt.Println("  prompt.md     — project context for agent sessions")
		fmt.Println("  hooks.yaml    — quality gates and lifecycle hooks")
		fmt.Println("\nNext steps:")
		fmt.Println("  1. Edit .ralph-engine/config.yaml to customize")
		fmt.Println("  2. Run: ralph-engine preflight")
		fmt.Println("  3. Run: ralph-engine run --dry-run")

		return nil
	},
}

// printDetectedTools displays what was found in the project.
func printDetectedTools(info detect.ProjectInfo) {
	if len(info.Tools) == 0 {
		fmt.Println("  No tools detected (greenfield project)")
		return
	}

	fmt.Println("Detected project tools:")

	// Group by category.
	categories := []struct {
		key   string
		label string
	}{
		{"workflow", "Workflows"},
		{"language", "Languages"},
		{"testing", "Testing"},
		{"lint", "Linting"},
		{"build", "Build"},
		{"tracker", "Trackers"},
		{"ci", "CI/CD"},
	}

	for _, cat := range categories {
		var tools []detect.DetectedTool
		for _, t := range info.Tools {
			if t.Category == cat.key {
				tools = append(tools, t)
			}
		}
		if len(tools) == 0 {
			continue
		}
		names := ""
		for i, t := range tools {
			if i > 0 {
				names += ", "
			}
			names += t.Name
		}
		fmt.Printf("  %s: %s\n", cat.label, names)
	}
	fmt.Println()
}

func init() {
	initCmd.Flags().StringP("project", "d", "", "Project directory (default: current directory)")
	initCmd.Flags().StringP("preset", "p", "", "Configuration preset: basic, bmad-v6, tdd-strict")
	initCmd.Flags().Bool("skip-detect", false, "Skip auto-detection of project tools")
}
