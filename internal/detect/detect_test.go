package detect

import (
	"os"
	"path/filepath"
	"testing"
)

func TestScanEmptyDir(t *testing.T) {
	info := Scan(t.TempDir())
	if !info.IsGreenfield {
		t.Error("empty dir should be greenfield")
	}
	if info.SuggestedPreset != "basic" {
		t.Errorf("expected basic preset, got %s", info.SuggestedPreset)
	}
}

func TestScanDetectsBMAD(t *testing.T) {
	dir := t.TempDir()
	os.MkdirAll(filepath.Join(dir, "_bmad"), 0755)
	os.WriteFile(filepath.Join(dir, "sprint-status.yaml"), []byte("epics: []"), 0644)

	info := Scan(dir)
	if info.SuggestedPreset != "bmad-v6" {
		t.Errorf("expected bmad-v6 preset, got %s", info.SuggestedPreset)
	}
	found := false
	for _, tool := range info.Tools {
		if tool.Name == "BMAD" {
			found = true
		}
	}
	if !found {
		t.Error("should detect BMAD")
	}
}

func TestScanDetectsNodeJS(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "package.json"), []byte(`{"scripts":{"test":"vitest"}}`), 0644)

	info := Scan(dir)
	hasNode := false
	hasVitest := false
	for _, tool := range info.Tools {
		if tool.Name == "Node.js" {
			hasNode = true
		}
		if tool.Name == "Vitest" {
			hasVitest = true
		}
	}
	if !hasNode {
		t.Error("should detect Node.js")
	}
	if !hasVitest {
		t.Error("should detect Vitest from package.json scripts")
	}
}

func TestScanDetectsGo(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "go.mod"), []byte("module test"), 0644)

	info := Scan(dir)
	hasGo := false
	for _, tool := range info.Tools {
		if tool.Name == "Go" {
			hasGo = true
		}
	}
	if !hasGo {
		t.Error("should detect Go")
	}
}

func TestScanDetectsGitHubActions(t *testing.T) {
	dir := t.TempDir()
	os.MkdirAll(filepath.Join(dir, ".github", "workflows"), 0755)

	info := Scan(dir)
	hasCI := false
	for _, tool := range info.Tools {
		if tool.Name == "GitHub Actions" {
			hasCI = true
		}
	}
	if !hasCI {
		t.Error("should detect GitHub Actions")
	}
}

func TestScanDetectsStatusFile(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "sprint-status.yaml"), []byte("epics: []"), 0644)

	info := Scan(dir)
	if info.SuggestedStatusFile != "sprint-status.yaml" {
		t.Errorf("expected sprint-status.yaml, got %s", info.SuggestedStatusFile)
	}
}

func TestScanDetectsMonorepo(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "pnpm-workspace.yaml"), []byte("packages:\n  - apps/*"), 0644)
	os.WriteFile(filepath.Join(dir, "turbo.json"), []byte("{}"), 0644)

	info := Scan(dir)
	hasMono := false
	for _, tool := range info.Tools {
		if tool.Name == "Monorepo" {
			hasMono = true
		}
	}
	if !hasMono {
		t.Error("should detect Monorepo")
	}
}

func TestScanDetectsLinters(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, ".golangci.yml"), []byte("run:"), 0644)
	os.WriteFile(filepath.Join(dir, ".prettierrc"), []byte("{}"), 0644)

	info := Scan(dir)
	hasGolangci := false
	hasPrettier := false
	for _, tool := range info.Tools {
		if tool.Name == "golangci-lint" {
			hasGolangci = true
		}
		if tool.Name == "Prettier" {
			hasPrettier = true
		}
	}
	if !hasGolangci {
		t.Error("should detect golangci-lint")
	}
	if !hasPrettier {
		t.Error("should detect Prettier")
	}
}

func TestScanClaudeAgent(t *testing.T) {
	dir := t.TempDir()
	os.MkdirAll(filepath.Join(dir, ".claude"), 0755)

	info := Scan(dir)
	if info.SuggestedAgent != "claude" {
		t.Errorf("expected claude agent, got %s", info.SuggestedAgent)
	}
}
