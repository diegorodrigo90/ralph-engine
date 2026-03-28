package tracker

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"

	"gopkg.in/yaml.v3"
)

// flatStatusFile represents a flat key-value YAML sprint status.
// Format: development_status map where keys are "epicN-storyM-slug" and values are statuses.
// This is the BMAD v6 format used by many projects with flat sprint tracking.
type flatStatusFile struct {
	DevelopmentStatus map[string]string `yaml:"development_status"`
	StoryLocation     string            `yaml:"story_location"`
}

var epicKeyPattern = regexp.MustCompile(`^epic-(\d+)$`)
var storyKeyPattern = regexp.MustCompile(`^(\d+)-(\d+)-(.+)$`)

// FlatFileTracker reads stories from a flat key-value YAML file.
// Supports the BMAD v6 sprint-status.yaml format:
//
//	development_status:
//	  epic-1: done
//	  1-1-resolve-critical-issues: done
//	  1-2-unified-permission-system: ready-for-dev
type FlatFileTracker struct {
	dir      string
	filename string
}

// NewFlatFileTracker creates a tracker for flat YAML sprint status files.
func NewFlatFileTracker(dir, filename string) *FlatFileTracker {
	return &FlatFileTracker{dir: dir, filename: filename}
}

// NextStory returns the highest-priority actionable story.
func (ft *FlatFileTracker) NextStory() (*Story, error) {
	stories, err := ft.ListPending()
	if err != nil {
		return nil, err
	}
	if len(stories) == 0 {
		return nil, nil
	}
	return &stories[0], nil
}

// MarkComplete transitions a story to done status.
func (ft *FlatFileTracker) MarkComplete(storyID string) error {
	return ft.updateStatus(storyID, StatusDone)
}

// MarkInProgress transitions a story to in-progress status.
func (ft *FlatFileTracker) MarkInProgress(storyID string) error {
	return ft.updateStatus(storyID, StatusInProgress)
}

// ListPending returns all actionable stories sorted by priority.
func (ft *FlatFileTracker) ListPending() ([]Story, error) {
	all, err := ft.ListAll()
	if err != nil {
		return nil, err
	}
	var pending []Story
	for _, s := range all {
		if s.IsActionable() {
			pending = append(pending, s)
		}
	}
	SortByPriority(pending)
	return pending, nil
}

// ListAll returns all stories from the flat status file.
func (ft *FlatFileTracker) ListAll() ([]Story, error) {
	sf, err := ft.readFile()
	if err != nil {
		return nil, err
	}
	if sf == nil {
		return nil, nil
	}

	// First pass: collect epic titles.
	epics := make(map[string]string) // epicID → epicTitle from key
	for key, status := range sf.DevelopmentStatus {
		if matches := epicKeyPattern.FindStringSubmatch(key); matches != nil {
			epicID := matches[1]
			epics[epicID] = fmt.Sprintf("Epic %s", epicID)
			_ = status
		}
	}

	// Second pass: collect stories.
	var stories []Story
	for key, status := range sf.DevelopmentStatus {
		if epicKeyPattern.MatchString(key) {
			continue // Skip epic-level entries.
		}

		matches := storyKeyPattern.FindStringSubmatch(key)
		if matches == nil {
			continue // Skip non-story entries.
		}

		epicID := matches[1]
		storyNum := matches[2]
		slug := matches[3]

		storyID := fmt.Sprintf("%s.%s", epicID, storyNum)
		title := slugToTitle(slug)

		stories = append(stories, Story{
			ID:        storyID,
			Title:     title,
			Status:    StoryStatus(status),
			EpicID:    epicID,
			EpicTitle: epics[epicID],
			Tags:      []string{key}, // Preserve original key for updates.
		})
	}
	return stories, nil
}

// updateStatus changes a story's status in the flat file.
func (ft *FlatFileTracker) updateStatus(storyID string, status StoryStatus) error {
	path := filepath.Join(ft.dir, ft.filename)

	data, err := os.ReadFile(path)
	if err != nil {
		return fmt.Errorf("reading %s: %w", ft.filename, err)
	}

	// Find the original key for this story ID.
	stories, _ := ft.ListAll()
	var originalKey string
	for _, s := range stories {
		if s.ID == storyID && len(s.Tags) > 0 {
			originalKey = s.Tags[0]
			break
		}
	}
	if originalKey == "" {
		return fmt.Errorf("story %q not found in sprint status", storyID)
	}

	// Replace status in raw text to preserve comments and formatting.
	oldLine := originalKey + ":"
	content := string(data)
	lines := strings.Split(content, "\n")
	found := false
	for i, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, oldLine) {
			// Preserve indentation and any trailing comment.
			indent := line[:len(line)-len(strings.TrimLeft(line, " "))]
			comment := ""
			if idx := strings.Index(trimmed, "#"); idx > 0 {
				comment = "  " + trimmed[idx:]
			}
			lines[i] = fmt.Sprintf("%s%s: %s%s", indent, originalKey, status, comment)
			found = true
			break
		}
	}
	if !found {
		return fmt.Errorf("key %q not found in file", originalKey)
	}

	result := strings.Join(lines, "\n")
	tmpPath := path + ".tmp"
	if err := os.WriteFile(tmpPath, []byte(result), 0644); err != nil {
		return fmt.Errorf("writing %s: %w", ft.filename, err)
	}
	if err := os.Rename(tmpPath, path); err != nil {
		os.Remove(tmpPath)
		return fmt.Errorf("renaming %s: %w", ft.filename, err)
	}
	return nil
}

// readFile loads and parses the flat sprint status YAML.
func (ft *FlatFileTracker) readFile() (*flatStatusFile, error) {
	path := filepath.Join(ft.dir, ft.filename)

	data, err := os.ReadFile(path)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, nil
		}
		return nil, fmt.Errorf("reading %s: %w", ft.filename, err)
	}

	var sf flatStatusFile
	if err := yaml.Unmarshal(data, &sf); err != nil {
		return nil, fmt.Errorf("parsing %s: %w", ft.filename, err)
	}
	return &sf, nil
}

// slugToTitle converts "resolve-critical-issues" to "Resolve Critical Issues".
func slugToTitle(slug string) string {
	words := strings.Split(slug, "-")
	for i, w := range words {
		if len(w) > 0 {
			words[i] = strings.ToUpper(w[:1]) + w[1:]
		}
	}
	return strings.Join(words, " ")
}
