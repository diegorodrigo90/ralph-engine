// Package context loads project artifacts for injection into agent prompts.
// It reads story files, prompt.md, and paths-configured artifacts,
// returning their content as strings for the prompt builder.
// The loader is format-agnostic — it reads files as-is without parsing.
package context

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/diegorodrigo90/ralph-engine/internal/config"
)

// maxFileSize is the maximum size of a single file to read (512 KB).
// Files larger than this are truncated with a warning appended.
const maxFileSize = 512 * 1024

// maxTotalSize is the maximum total size of all injected content (2 MB).
// This prevents prompt overflow with very large projects.
const maxTotalSize = 2 * 1024 * 1024

// LoadedContext holds all loaded content ready for prompt injection.
type LoadedContext struct {
	// StoryContent is the full content of the story file.
	StoryContent string
	// PromptMD is the content of .ralph-engine/prompt.md.
	PromptMD string
}

// LoadPromptMD reads the project's .ralph-engine/prompt.md file.
// Returns empty string if the file doesn't exist (not an error).
func LoadPromptMD(projectDir string) string {
	path := filepath.Join(projectDir, config.ProjectConfigDir, "prompt.md")
	content, err := readFileSafe(path)
	if err != nil {
		return ""
	}
	return content
}

// LoadStoryFile reads a story file by its path.
// If storyFilePath is relative, it's resolved against projectDir.
// Returns empty string if the file doesn't exist.
func LoadStoryFile(projectDir, storyFilePath string) string {
	if storyFilePath == "" {
		return ""
	}
	path := resolvePath(projectDir, storyFilePath)
	content, err := readFileSafe(path)
	if err != nil {
		return ""
	}
	return content
}

// FindStoryFile searches for a story file matching the given story ID.
// It looks in the paths.stories directory for files containing the story ID.
// Returns the file path (relative to projectDir) or empty string if not found.
func FindStoryFile(projectDir, storiesPath, storyID string) string {
	if storiesPath == "" || storyID == "" {
		return ""
	}

	dir := resolvePath(projectDir, storiesPath)
	info, err := os.Stat(dir)
	if err != nil || !info.IsDir() {
		// Maybe it's a single file or glob — try reading directly.
		return ""
	}

	// Normalize story ID for file matching (e.g., "65.3" → "65-3", "65.3").
	idDash := strings.ReplaceAll(storyID, ".", "-")
	idDot := storyID

	// Walk the stories directory looking for a matching file.
	var match string
	filepath.WalkDir(dir, func(path string, d os.DirEntry, err error) error {
		if err != nil || d.IsDir() || match != "" {
			return nil
		}
		name := strings.ToLower(d.Name())
		// Match files containing the story ID in their name.
		if strings.Contains(name, idDash) || strings.Contains(name, idDot) {
			rel, _ := filepath.Rel(projectDir, path)
			match = rel
			return filepath.SkipAll
		}
		return nil
	})

	return match
}

// resolvePath makes a path absolute relative to projectDir.
func resolvePath(projectDir, path string) string {
	if filepath.IsAbs(path) {
		return path
	}
	return filepath.Join(projectDir, path)
}

// readFileSafe reads a file with size limits.
// Returns the content or an error. Files larger than maxFileSize are truncated.
func readFileSafe(path string) (string, error) {
	info, err := os.Stat(path)
	if err != nil {
		return "", err
	}

	if info.IsDir() {
		return readDirIndex(path)
	}

	data, err := os.ReadFile(path)
	if err != nil {
		return "", fmt.Errorf("reading %s: %w", path, err)
	}

	content := string(data)
	if len(content) > maxFileSize {
		content = content[:maxFileSize] + fmt.Sprintf("\n\n[TRUNCATED — file exceeds %d KB limit]", maxFileSize/1024)
	}

	return content, nil
}

// readDirIndex reads the index.md or README.md from a directory.
// This handles sharded directories where the index file provides context.
func readDirIndex(dir string) (string, error) {
	// Try common index file names in order of preference.
	indexFiles := []string{"index.md", "INDEX.md", "README.md", "readme.md"}
	for _, name := range indexFiles {
		path := filepath.Join(dir, name)
		if _, err := os.Stat(path); err == nil {
			data, err := os.ReadFile(path)
			if err != nil {
				return "", err
			}
			content := string(data)
			if len(content) > maxFileSize {
				content = content[:maxFileSize]
			}
			return content, nil
		}
	}
	return "", fmt.Errorf("no index file found in %s", dir)
}
