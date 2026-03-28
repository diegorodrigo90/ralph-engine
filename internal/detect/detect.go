// Package detect scans a project directory to identify existing tools,
// frameworks, and conventions. Used by the init wizard to suggest config.
// Detection is non-invasive — read-only, no modifications.
package detect

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
)

// ProjectInfo holds all detected project characteristics.
type ProjectInfo struct {
	// Tools detected in the project.
	Tools []DetectedTool
	// Suggested preset based on detected tools.
	SuggestedPreset string
	// Suggested agent binary.
	SuggestedAgent string
	// Suggested status file path.
	SuggestedStatusFile string
	// IsGreenfield is true when the project has no task/story files.
	IsGreenfield bool
}

// DetectedTool represents a single detected tool or framework.
type DetectedTool struct {
	Name     string // e.g., "BMAD", "Claude Code", "Vitest"
	Category string // "workflow", "agent", "testing", "build", "lint", "tracker", "language"
	Path     string // File/dir that triggered detection
}

// Scan detects tools in the given project directory.
func Scan(projectDir string) ProjectInfo {
	info := ProjectInfo{
		SuggestedPreset:     "basic",
		SuggestedAgent:      "claude",
		SuggestedStatusFile: "sprint-status.yaml",
	}

	// Detect workflows / methodologies.
	detectWorkflows(projectDir, &info)

	// Detect AI agent tools.
	detectAgents(projectDir, &info)

	// Detect languages and package managers.
	detectLanguages(projectDir, &info)

	// Detect test runners.
	detectTestRunners(projectDir, &info)

	// Detect linters.
	detectLinters(projectDir, &info)

	// Detect trackers / task systems.
	detectTrackers(projectDir, &info)

	// Detect CI/CD.
	detectCI(projectDir, &info)

	// Determine if greenfield.
	info.IsGreenfield = len(info.Tools) < 3

	return info
}

func detectWorkflows(dir string, info *ProjectInfo) {
	checks := []struct {
		paths []string
		name  string
	}{
		{[]string{"_bmad/", ".bmad/", "sprint-status.yaml"}, "BMAD"},
		{[]string{".claude/", "CLAUDE.md"}, "Claude Code"},
		{[]string{".cursor/", ".cursorrules"}, "Cursor"},
		{[]string{".windsurf/", ".windsurfrules"}, "Windsurf"},
		{[]string{"AGENTS.md"}, "Multi-Agent"},
		{[]string{".gemini/", "GEMINI.md"}, "Gemini"},
	}

	for _, c := range checks {
		for _, p := range c.paths {
			if exists(dir, p) {
				info.Tools = append(info.Tools, DetectedTool{
					Name: c.name, Category: "workflow", Path: p,
				})
				break
			}
		}
	}

	// BMAD → suggest bmad-v6 preset.
	for _, t := range info.Tools {
		if t.Name == "BMAD" {
			info.SuggestedPreset = "bmad-v6"
			break
		}
	}
}

func detectAgents(dir string, info *ProjectInfo) {
	// Check for agent config files that hint at which agent to use.
	if exists(dir, ".claude/") || exists(dir, "CLAUDE.md") {
		info.SuggestedAgent = "claude"
	}
}

func detectLanguages(dir string, info *ProjectInfo) {
	langs := []struct {
		files []string
		name  string
	}{
		{[]string{"package.json", "pnpm-lock.yaml", "yarn.lock", "package-lock.json", "bun.lockb"}, "Node.js"},
		{[]string{"go.mod"}, "Go"},
		{[]string{"Cargo.toml"}, "Rust"},
		{[]string{"pyproject.toml", "setup.py", "requirements.txt", "Pipfile"}, "Python"},
		{[]string{"pom.xml", "build.gradle"}, "Java"},
		{[]string{"Gemfile"}, "Ruby"},
		{[]string{"mix.exs"}, "Elixir"},
		{[]string{"composer.json"}, "PHP"},
	}

	for _, l := range langs {
		for _, f := range l.files {
			if exists(dir, f) {
				info.Tools = append(info.Tools, DetectedTool{
					Name: l.name, Category: "language", Path: f,
				})
				break
			}
		}
	}

	// Check for monorepo markers.
	if exists(dir, "pnpm-workspace.yaml") || exists(dir, "turbo.json") || exists(dir, "lerna.json") || exists(dir, "nx.json") {
		info.Tools = append(info.Tools, DetectedTool{
			Name: "Monorepo", Category: "build",
		})
	}
}

func detectTestRunners(dir string, info *ProjectInfo) {
	// Check package.json scripts for common test runners.
	if pkgJSON := readPackageJSON(dir); pkgJSON != nil {
		scripts := pkgJSON["scripts"]
		if sm, ok := scripts.(map[string]interface{}); ok {
			for _, v := range sm {
				s, _ := v.(string)
				switch {
				case strings.Contains(s, "vitest"):
					info.Tools = append(info.Tools, DetectedTool{Name: "Vitest", Category: "testing"})
				case strings.Contains(s, "jest"):
					info.Tools = append(info.Tools, DetectedTool{Name: "Jest", Category: "testing"})
				case strings.Contains(s, "playwright"):
					info.Tools = append(info.Tools, DetectedTool{Name: "Playwright", Category: "testing"})
				}
			}
		}
	}

	// Go test is implicit with go.mod.
	if exists(dir, "go.mod") {
		info.Tools = append(info.Tools, DetectedTool{Name: "go test", Category: "testing"})
	}

	// Python test runners.
	if exists(dir, "pytest.ini") || exists(dir, "pyproject.toml") {
		info.Tools = append(info.Tools, DetectedTool{Name: "pytest", Category: "testing"})
	}

	if exists(dir, "Makefile") {
		info.Tools = append(info.Tools, DetectedTool{Name: "Make", Category: "build"})
	}
}

func detectLinters(dir string, info *ProjectInfo) {
	linters := []struct {
		files []string
		name  string
	}{
		{[]string{".eslintrc", ".eslintrc.js", ".eslintrc.json", "eslint.config.js", "eslint.config.mjs"}, "ESLint"},
		{[]string{".prettierrc", ".prettierrc.js", "prettier.config.js"}, "Prettier"},
		{[]string{".golangci.yml", ".golangci.yaml"}, "golangci-lint"},
		{[]string{"ruff.toml"}, "Ruff"},
		{[]string{".rubocop.yml"}, "Rubocop"},
	}

	for _, l := range linters {
		for _, f := range l.files {
			if exists(dir, f) {
				info.Tools = append(info.Tools, DetectedTool{
					Name: l.name, Category: "lint", Path: f,
				})
				break
			}
		}
	}

	// Check pyproject.toml for ruff config.
	if exists(dir, "pyproject.toml") {
		data, _ := os.ReadFile(filepath.Join(dir, "pyproject.toml"))
		if strings.Contains(string(data), "[tool.ruff]") {
			info.Tools = append(info.Tools, DetectedTool{Name: "Ruff", Category: "lint", Path: "pyproject.toml"})
		}
	}
}

func detectTrackers(dir string, info *ProjectInfo) {
	trackers := []struct {
		files []string
		name  string
	}{
		{[]string{"sprint-status.yaml", "sprint-status.yml"}, "Sprint Status YAML"},
		{[]string{"TODO.md", "TASKS.md", "tasks.md"}, "Markdown Tasks"},
		{[]string{".linear/"}, "Linear"},
	}

	for _, t := range trackers {
		for _, f := range t.files {
			if exists(dir, f) {
				info.Tools = append(info.Tools, DetectedTool{
					Name: t.name, Category: "tracker", Path: f,
				})
				info.SuggestedStatusFile = f
				break
			}
		}
	}

	// Check for GitHub Issues usage.
	if exists(dir, ".github/") {
		info.Tools = append(info.Tools, DetectedTool{
			Name: "GitHub", Category: "tracker", Path: ".github/",
		})
	}
}

func detectCI(dir string, info *ProjectInfo) {
	ciSystems := []struct {
		paths []string
		name  string
	}{
		{[]string{".github/workflows/"}, "GitHub Actions"},
		{[]string{".gitlab-ci.yml"}, "GitLab CI"},
		{[]string{".circleci/"}, "CircleCI"},
		{[]string{"Jenkinsfile"}, "Jenkins"},
	}

	for _, ci := range ciSystems {
		for _, p := range ci.paths {
			if exists(dir, p) {
				info.Tools = append(info.Tools, DetectedTool{
					Name: ci.name, Category: "ci", Path: p,
				})
				break
			}
		}
	}
}

// exists checks if a file or directory exists relative to dir.
func exists(dir, path string) bool {
	_, err := os.Stat(filepath.Join(dir, path))
	return err == nil
}

// readPackageJSON reads and parses the root package.json.
func readPackageJSON(dir string) map[string]interface{} {
	data, err := os.ReadFile(filepath.Join(dir, "package.json"))
	if err != nil {
		return nil
	}
	var pkg map[string]interface{}
	if err := json.Unmarshal(data, &pkg); err != nil {
		return nil
	}
	return pkg
}
