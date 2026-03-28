// Package cli implements the cobra command tree for ralph-engine.
package cli

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/diegorodrigo90/ralph-engine/internal/config"
	"github.com/diegorodrigo90/ralph-engine/internal/detect"
	"github.com/diegorodrigo90/ralph-engine/internal/engine"
	"github.com/diegorodrigo90/ralph-engine/internal/hooks"
	"github.com/diegorodrigo90/ralph-engine/internal/tracker"
	"github.com/spf13/cobra"
)

var prepareCmd = &cobra.Command{
	Use:   "prepare",
	Short: "Validate project readiness and run preparation hooks",
	Long: `Runs a comprehensive readiness check before starting the engine loop.

Built-in checks:
  - Project directory and state directory exist and are writable
  - Agent binary is available in PATH
  - System resources (RAM, disk) are sufficient
  - config.yaml loads and validates
  - Status file exists and has pending stories
  - hooks.yaml syntax is valid
  - prompt.md and section files exist

Custom hooks (hooks.yaml "prepare" phase):
  - Run any custom validation steps (e.g., DoR check, story format, RAG index)
  - Each step can be required (blocks) or optional (warns)
  - Use this to integrate your workflow tool (BMAD, Claude Tasks, custom scripts)

Example hooks.yaml:
  prepare:
    steps:
      - name: "Validate stories have ACs"
        run: "grep -l 'Given.*When.*Then' stories/*.md | wc -l"
        required: true
      - name: "Check architecture review"
        run: "./scripts/check-dor.sh"
        required: false

This command combines and replaces the old 'preflight' and 'doctor' commands.`,
	RunE: func(cmd *cobra.Command, args []string) error {
		projectDir, _ := cmd.Flags().GetString("project")
		if projectDir == "" {
			wd, _ := os.Getwd()
			projectDir = wd
		}

		fmt.Println("ralph-engine prepare")
		fmt.Println()
		issues := 0
		warnings := 0

		// Phase 1: Built-in checks (same as doctor + preflight).
		fmt.Println("== Built-in Checks ==")
		fmt.Println()

		// 1. Project directory + state dir.
		eng, err := engine.New(engine.EngineOpts{
			ProjectDir: projectDir,
			Binary:     "claude",
		})
		if err != nil {
			fmt.Printf("  ✗ Engine init failed: %v\n", err)
			return err
		}

		results := eng.Preflight(context.Background())
		for _, r := range results {
			icon := "✓"
			if !r.OK {
				icon = "✗"
				issues++
			}
			fmt.Printf("  %s %s: %s\n", icon, r.Name, r.Message)
		}

		// 2. Config directory exists.
		configDir := filepath.Join(projectDir, ".ralph-engine")
		if _, err := os.Stat(configDir); os.IsNotExist(err) {
			fmt.Println("  ✗ No .ralph-engine/ directory found")
			fmt.Println("    Run: ralph-engine init")
			issues++
		}

		// 3. Config validation.
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

		// 4. Agent binary.
		if cfg != nil {
			agentBin := cfg.Agent.Type
			if agentBin == "" {
				agentBin = "claude"
			}
			if path, lookErr := exec.LookPath(agentBin); lookErr != nil {
				fmt.Printf("  ✗ Agent '%s' not found in PATH\n", agentBin)
				issues++
			} else {
				fmt.Printf("  ✓ Agent '%s' found at %s\n", agentBin, path)
			}
		}

		// 5. Tracker / status file.
		if cfg != nil {
			statusFile := cfg.Tracker.StatusFile
			if statusFile == "" {
				statusFile = "sprint-status.yaml"
			}
			fullPath := filepath.Join(projectDir, statusFile)
			if _, statErr := os.Stat(fullPath); os.IsNotExist(statErr) {
				fmt.Printf("  ✗ Status file not found: %s\n", statusFile)
				fmt.Println("    Create stories first (manually, BMAD, Claude Tasks, or any tool)")
				issues++
			} else {
				tk := tracker.AutoDetect(projectDir, statusFile)
				all, listErr := tk.ListAll()
				if listErr != nil {
					fmt.Printf("  ✗ Cannot read status file: %v\n", listErr)
					issues++
				} else {
					pending, _ := tk.ListPending()
					fmt.Printf("  ✓ Status file: %d stories (%d pending, %d done)\n",
						len(all), len(pending), len(all)-len(pending))
					if len(pending) == 0 {
						fmt.Println("  ⚠ No pending stories — nothing to implement")
						fmt.Println("    Create stories using your preferred tool:")
						fmt.Println("      - BMAD: /create-story, /sprint-planning")
						fmt.Println("      - Claude Tasks: claude tasks add \"story description\"")
						fmt.Println("      - Manual: edit sprint-status.yaml directly")
						fmt.Println("      - Custom: any tool that produces YAML/Markdown stories")
						warnings++
					}
				}
			}
		}

		// 6. Hooks.
		hooksConfig, hooksErr := hooks.Load(projectDir)
		if hooksErr != nil {
			fmt.Printf("  ✗ hooks.yaml error: %v\n", hooksErr)
			issues++
		} else if hooksConfig == nil {
			fmt.Println("  ⚠ No hooks.yaml found — quality gates won't run")
			warnings++
		} else {
			gateCount := len(hooksConfig.QualityGates.Steps)
			prepareCount := len(hooksConfig.Prepare.Steps)
			totalSteps := len(hooksConfig.Prepare.Steps) + prepareCount +
				len(hooksConfig.PreStory.Steps) + gateCount +
				len(hooksConfig.PostStory.Steps) + len(hooksConfig.PostSession.Steps)
			fmt.Printf("  ✓ hooks.yaml: %d steps (%d quality gates, %d prepare)\n",
				totalSteps, gateCount, prepareCount)
		}

		// 7. Prompt files.
		promptMD := filepath.Join(projectDir, ".ralph-engine", "prompt.md")
		if _, statErr := os.Stat(promptMD); os.IsNotExist(statErr) {
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
					if _, statErr := os.Stat(fp); os.IsNotExist(statErr) {
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

		// 8. Research tools.
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

		// 9. Project detection.
		info := detect.Scan(projectDir)
		fmt.Printf("  ✓ Detected: %d tool(s) in project\n", len(info.Tools))

		// Phase 2: Custom prepare hooks from hooks.yaml.
		if hooksConfig != nil && len(hooksConfig.Prepare.Steps) > 0 {
			fmt.Println()
			fmt.Println("== Custom Prepare Hooks ==")
			fmt.Println()

			ctx := context.Background()
			result := hooks.RunPhase(ctx, hooksConfig.Prepare, projectDir, nil, func(sr hooks.StepResult) {
				if sr.Skipped {
					fmt.Printf("  ⊘ %s (skipped)\n", sr.Name)
					return
				}
				if sr.OK {
					fmt.Printf("  ✓ %s (%v)\n", sr.Name, sr.Duration.Round(1000000)) // ms
				} else {
					errMsg := ""
					if sr.Error != nil {
						errMsg = sr.Error.Error()
					}
					if sr.Required {
						fmt.Printf("  ✗ %s — %s\n", sr.Name, errMsg)
						issues++
					} else {
						fmt.Printf("  ⚠ %s — %s\n", sr.Name, errMsg)
						warnings++
					}
				}
			})
			if result.Blocked {
				fmt.Printf("  ✗ Blocked: %s\n", result.Reason)
				issues++
			}
		}

		// Summary.
		fmt.Println()
		if issues == 0 && warnings == 0 {
			fmt.Println("All checks passed! Ready to run: ralph-engine run")
		} else if issues == 0 {
			fmt.Printf("%d warning(s) — can still run but consider fixing\n", warnings)
		} else {
			fmt.Printf("%d issue(s), %d warning(s) — fix issues before running\n", issues, warnings)
			return fmt.Errorf("%d issues found", issues)
		}

		return nil
	},
}

func init() {
	prepareCmd.Flags().StringP("project", "d", "", "Project directory (default: current directory)")
}
