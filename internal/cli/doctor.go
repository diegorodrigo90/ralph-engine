package cli

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/diegorodrigo90/ralph-engine/internal/config"
	"github.com/diegorodrigo90/ralph-engine/internal/detect"
	"github.com/diegorodrigo90/ralph-engine/internal/hooks"
	"github.com/diegorodrigo90/ralph-engine/internal/tracker"
	"github.com/spf13/cobra"
)

var doctorCmd = &cobra.Command{
	Use:   "doctor",
	Short: "Check project health and readiness for ralph-engine",
	Long: `Runs a comprehensive health check on your project setup.

Checks:
  - Config: validates .ralph-engine/config.yaml
  - Agent: verifies agent binary is available and responds
  - Tracker: validates status file exists and has stories
  - Hooks: validates hooks.yaml syntax and script paths
  - Prompt: checks prompt.md and section files exist
  - Research: validates configured research tools
  - Project: detects tools and suggests improvements`,
	RunE: func(cmd *cobra.Command, args []string) error {
		projectDir, _ := cmd.Flags().GetString("project")
		if projectDir == "" {
			wd, _ := os.Getwd()
			projectDir = wd
		}

		fmt.Println("ralph-engine doctor")
		fmt.Println()
		issues := 0
		warnings := 0

		// 1. Config directory exists.
		configDir := filepath.Join(projectDir, ".ralph-engine")
		if _, err := os.Stat(configDir); os.IsNotExist(err) {
			fmt.Println("  ✗ No .ralph-engine/ directory found")
			fmt.Println("    Run: ralph-engine init")
			return nil
		}
		fmt.Println("  ✓ .ralph-engine/ directory exists")

		// 2. Config validation.
		cfg, err := config.Load(projectDir)
		if err != nil {
			fmt.Printf("  ✗ Config load failed: %v\n", err)
			issues++
		} else {
			fmt.Println("  ✓ config.yaml loaded")

			validation := config.Validate(cfg, projectDir)
			if !validation.OK() {
				for _, e := range validation.Errors {
					fmt.Printf("  ✗ %s\n", e.Error())
					issues++
				}
			}
			for _, w := range validation.Warnings {
				fmt.Printf("  ⚠ %s\n", w.Error())
				warnings++
			}
			if validation.OK() && len(validation.Warnings) == 0 {
				fmt.Println("  ✓ config validation passed")
			}
		}

		// 3. Agent binary.
		if cfg != nil {
			agentBin := cfg.Agent.Type
			if agentBin == "" {
				agentBin = "claude"
			}
			if path, err := exec.LookPath(agentBin); err != nil {
				fmt.Printf("  ✗ Agent '%s' not found in PATH\n", agentBin)
				fmt.Printf("    Install %s or set agent.type in config\n", agentBin)
				issues++
			} else {
				fmt.Printf("  ✓ Agent '%s' found at %s\n", agentBin, path)
			}
		}

		// 4. Tracker / status file.
		if cfg != nil {
			statusFile := cfg.Tracker.StatusFile
			if statusFile == "" {
				statusFile = "sprint-status.yaml"
			}
			fullPath := filepath.Join(projectDir, statusFile)
			if _, err := os.Stat(fullPath); os.IsNotExist(err) {
				fmt.Printf("  ✗ Status file not found: %s\n", statusFile)
				issues++
			} else {
				tk := tracker.AutoDetect(projectDir, statusFile)
				all, err := tk.ListAll()
				if err != nil {
					fmt.Printf("  ✗ Cannot read status file: %v\n", err)
					issues++
				} else {
					pending, _ := tk.ListPending()
					fmt.Printf("  ✓ Status file: %d stories (%d pending, %d done)\n",
						len(all), len(pending), len(all)-len(pending))
					if len(pending) == 0 {
						fmt.Println("  ⚠ No pending stories — nothing to implement")
						warnings++
					}
				}
			}
		}

		// 5. Hooks.
		hooksConfig, err := hooks.Load(projectDir)
		if err != nil {
			fmt.Printf("  ✗ hooks.yaml error: %v\n", err)
			issues++
		} else if hooksConfig == nil {
			fmt.Println("  ⚠ No hooks.yaml found — quality gates won't run")
			warnings++
		} else {
			gateCount := len(hooksConfig.QualityGates.Steps)
			totalSteps := len(hooksConfig.Preflight.Steps) + len(hooksConfig.PreStory.Steps) +
				gateCount + len(hooksConfig.PostStory.Steps) + len(hooksConfig.PostSession.Steps)
			fmt.Printf("  ✓ hooks.yaml: %d steps (%d quality gates)\n", totalSteps, gateCount)
		}

		// 6. Prompt files.
		promptMD := filepath.Join(projectDir, ".ralph-engine", "prompt.md")
		if _, err := os.Stat(promptMD); os.IsNotExist(err) {
			fmt.Println("  ⚠ No prompt.md — agent gets no project context")
			warnings++
		} else {
			fmt.Println("  ✓ prompt.md exists")
		}

		if cfg != nil && len(cfg.Prompt.Sections) > 0 {
			missing := 0
			for _, s := range cfg.Prompt.Sections {
				if s.File != "" {
					fp := filepath.Join(projectDir, s.File)
					if _, err := os.Stat(fp); os.IsNotExist(err) {
						missing++
					}
				}
			}
			if missing > 0 {
				fmt.Printf("  ⚠ %d prompt section file(s) missing\n", missing)
				warnings++
			} else {
				fmt.Printf("  ✓ %d prompt section(s) configured\n", len(cfg.Prompt.Sections))
			}
		}

		// 7. Research tools.
		if cfg != nil && cfg.Research.Enabled {
			enabled := 0
			for _, t := range cfg.Research.Tools {
				if t.Enabled {
					enabled++
				}
			}
			if enabled == 0 {
				fmt.Println("  ⚠ Research enabled but no tools configured")
				warnings++
			} else {
				fmt.Printf("  ✓ Research: %d tool(s) configured\n", enabled)
			}
		}

		// 8. Project detection summary.
		info := detect.Scan(projectDir)
		fmt.Printf("  ✓ Detected: %d tool(s) in project\n", len(info.Tools))

		// Summary.
		fmt.Println()
		if issues == 0 && warnings == 0 {
			fmt.Println("All checks passed! Ready to run: ralph-engine run")
		} else if issues == 0 {
			fmt.Printf("%d warning(s) — can still run but consider fixing\n", warnings)
		} else {
			fmt.Printf("%d issue(s), %d warning(s) — fix issues before running\n", issues, warnings)
		}

		return nil
	},
}

func init() {
	doctorCmd.Flags().StringP("project", "d", "", "Project directory (default: current directory)")
	rootCmd.AddCommand(doctorCmd)
}
