package context

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestLoadPromptMDExists(t *testing.T) {
	dir := t.TempDir()
	configDir := filepath.Join(dir, ".ralph-engine")
	os.MkdirAll(configDir, 0755)
	os.WriteFile(filepath.Join(configDir, "prompt.md"), []byte("# My Project\nTech: Go"), 0644)

	content := LoadPromptMD(dir)
	if content != "# My Project\nTech: Go" {
		t.Errorf("expected prompt.md content, got %q", content)
	}
}

func TestLoadPromptMDMissing(t *testing.T) {
	dir := t.TempDir()
	content := LoadPromptMD(dir)
	if content != "" {
		t.Errorf("expected empty string for missing prompt.md, got %q", content)
	}
}

func TestLoadStoryFileExists(t *testing.T) {
	dir := t.TempDir()
	storyDir := filepath.Join(dir, "stories")
	os.MkdirAll(storyDir, 0755)
	os.WriteFile(filepath.Join(storyDir, "65-3-permission-ui.md"), []byte("### Story 65.3\nGiven..."), 0644)

	content := LoadStoryFile(dir, "stories/65-3-permission-ui.md")
	if !strings.Contains(content, "Story 65.3") {
		t.Errorf("expected story content, got %q", content)
	}
}

func TestLoadStoryFileAbsolutePath(t *testing.T) {
	dir := t.TempDir()
	storyFile := filepath.Join(dir, "story.md")
	os.WriteFile(storyFile, []byte("# Story"), 0644)

	content := LoadStoryFile(dir, storyFile)
	if content != "# Story" {
		t.Errorf("expected story content, got %q", content)
	}
}

func TestLoadStoryFileMissing(t *testing.T) {
	dir := t.TempDir()
	content := LoadStoryFile(dir, "nonexistent.md")
	if content != "" {
		t.Errorf("expected empty string for missing file, got %q", content)
	}
}

func TestLoadStoryFileEmpty(t *testing.T) {
	dir := t.TempDir()
	content := LoadStoryFile(dir, "")
	if content != "" {
		t.Errorf("expected empty string for empty path, got %q", content)
	}
}

func TestFindStoryFileByDashID(t *testing.T) {
	dir := t.TempDir()
	storyDir := filepath.Join(dir, "stories")
	os.MkdirAll(storyDir, 0755)
	os.WriteFile(filepath.Join(storyDir, "65-3-permission-ui.md"), []byte("story"), 0644)
	os.WriteFile(filepath.Join(storyDir, "65-4-other-story.md"), []byte("other"), 0644)

	match := FindStoryFile(dir, "stories", "65.3")
	if match == "" {
		t.Fatal("expected to find story file")
	}
	if !strings.Contains(match, "65-3") {
		t.Errorf("expected match to contain 65-3, got %q", match)
	}
}

func TestFindStoryFileByDotID(t *testing.T) {
	dir := t.TempDir()
	storyDir := filepath.Join(dir, "stories")
	os.MkdirAll(storyDir, 0755)
	os.WriteFile(filepath.Join(storyDir, "story-65.3.md"), []byte("story"), 0644)

	match := FindStoryFile(dir, "stories", "65.3")
	if match == "" {
		t.Fatal("expected to find story file")
	}
}

func TestFindStoryFileNotFound(t *testing.T) {
	dir := t.TempDir()
	storyDir := filepath.Join(dir, "stories")
	os.MkdirAll(storyDir, 0755)
	os.WriteFile(filepath.Join(storyDir, "65-1-first.md"), []byte("story"), 0644)

	match := FindStoryFile(dir, "stories", "99.9")
	if match != "" {
		t.Errorf("expected no match, got %q", match)
	}
}

func TestFindStoryFileEmptyInputs(t *testing.T) {
	if FindStoryFile("", "", "") != "" {
		t.Error("expected empty for empty inputs")
	}
	if FindStoryFile("/tmp", "", "65.3") != "" {
		t.Error("expected empty for empty stories path")
	}
	if FindStoryFile("/tmp", "stories", "") != "" {
		t.Error("expected empty for empty story ID")
	}
}

func TestReadFileSafeTruncatesLargeFile(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "large.md")

	// Create file larger than maxFileSize.
	data := strings.Repeat("x", maxFileSize+100)
	os.WriteFile(path, []byte(data), 0644)

	content, err := readFileSafe(path)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !strings.Contains(content, "TRUNCATED") {
		t.Error("expected truncation warning")
	}
	if len(content) > maxFileSize+200 {
		t.Error("content should be truncated near maxFileSize")
	}
}

func TestReadDirIndex(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "index.md"), []byte("# Index"), 0644)

	content, err := readDirIndex(dir)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if content != "# Index" {
		t.Errorf("expected index content, got %q", content)
	}
}

func TestReadDirIndexFallsBackToREADME(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "README.md"), []byte("# README"), 0644)

	content, err := readDirIndex(dir)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if content != "# README" {
		t.Errorf("expected README content, got %q", content)
	}
}

func TestReadDirIndexNoIndexFile(t *testing.T) {
	dir := t.TempDir()
	_, err := readDirIndex(dir)
	if err == nil {
		t.Error("expected error when no index file exists")
	}
}

func TestReadFileSafeReadsDirectory(t *testing.T) {
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "index.md"), []byte("# Dir Index"), 0644)

	content, err := readFileSafe(dir)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if content != "# Dir Index" {
		t.Errorf("expected dir index content, got %q", content)
	}
}
