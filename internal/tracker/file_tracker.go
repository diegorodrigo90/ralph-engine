package tracker

import (
	"fmt"
	"os"
	"path/filepath"

	"gopkg.in/yaml.v3"
)

// sprintStatusFile represents the YAML structure of sprint-status.yaml.
// Compatible with BMAD v6 and generic sprint tracking.
type sprintStatusFile struct {
	Epics []epicEntry `yaml:"epics"`
}

// epicEntry represents an epic in sprint-status.yaml.
type epicEntry struct {
	ID      string       `yaml:"id"`
	Title   string       `yaml:"title"`
	Status  string       `yaml:"status"`
	Stories []storyEntry `yaml:"stories"`
}

// storyEntry represents a story within an epic.
type storyEntry struct {
	ID     string `yaml:"id"`
	Title  string `yaml:"title"`
	Status string `yaml:"status"`
}

// FileTracker reads stories from a YAML file (sprint-status.yaml).
// It supports BMAD v6 format and generic sprint tracking.
type FileTracker struct {
	dir      string
	filename string
}

// NewFileTracker creates a tracker that reads from a YAML file.
func NewFileTracker(dir, filename string) *FileTracker {
	return &FileTracker{dir: dir, filename: filename}
}

// NextStory returns the highest-priority actionable story.
// In-progress stories are returned first (resume interrupted work),
// then ready-for-dev stories in file order.
func (ft *FileTracker) NextStory() (*Story, error) {
	stories, err := ft.ListPending()
	if err != nil {
		return nil, err
	}
	if len(stories) == 0 {
		return nil, nil
	}
	return &stories[0], nil
}

// MarkComplete transitions a story to done status and writes back.
func (ft *FileTracker) MarkComplete(storyID string) error {
	return ft.updateStatus(storyID, StatusDone)
}

// MarkInProgress transitions a story to in-progress status and writes back.
func (ft *FileTracker) MarkInProgress(storyID string) error {
	return ft.updateStatus(storyID, StatusInProgress)
}

// ListPending returns all actionable stories sorted by priority.
func (ft *FileTracker) ListPending() ([]Story, error) {
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

// ListAll returns all stories from the sprint status file.
func (ft *FileTracker) ListAll() ([]Story, error) {
	sf, err := ft.readFile()
	if err != nil {
		return nil, err
	}
	if sf == nil {
		return nil, nil
	}

	var stories []Story
	for _, epic := range sf.Epics {
		for _, se := range epic.Stories {
			stories = append(stories, Story{
				ID:        se.ID,
				Title:     se.Title,
				Status:    StoryStatus(se.Status),
				EpicID:    epic.ID,
				EpicTitle: epic.Title,
			})
		}
	}
	return stories, nil
}

// updateStatus changes a story's status and writes the file back.
func (ft *FileTracker) updateStatus(storyID string, status StoryStatus) error {
	sf, err := ft.readFile()
	if err != nil {
		return err
	}
	if sf == nil {
		return fmt.Errorf("sprint status file not found")
	}

	found := false
	for i := range sf.Epics {
		for j := range sf.Epics[i].Stories {
			if sf.Epics[i].Stories[j].ID == storyID {
				sf.Epics[i].Stories[j].Status = string(status)
				found = true
				break
			}
		}
		if found {
			break
		}
	}

	if !found {
		return fmt.Errorf("story %q not found in sprint status", storyID)
	}

	return ft.writeFile(sf)
}

// readFile loads and parses the sprint status YAML.
func (ft *FileTracker) readFile() (*sprintStatusFile, error) {
	path := filepath.Join(ft.dir, ft.filename)

	data, err := os.ReadFile(path)
	if os.IsNotExist(err) {
		return nil, nil
	}
	if err != nil {
		return nil, fmt.Errorf("reading %s: %w", ft.filename, err)
	}

	var sf sprintStatusFile
	if err := yaml.Unmarshal(data, &sf); err != nil {
		return nil, fmt.Errorf("parsing %s: %w", ft.filename, err)
	}

	return &sf, nil
}

// writeFile serializes and writes the sprint status back to YAML.
func (ft *FileTracker) writeFile(sf *sprintStatusFile) error {
	path := filepath.Join(ft.dir, ft.filename)

	data, err := yaml.Marshal(sf)
	if err != nil {
		return fmt.Errorf("marshaling %s: %w", ft.filename, err)
	}

	// Atomic write via temp + rename
	tmpPath := path + ".tmp"
	if err := os.WriteFile(tmpPath, data, 0644); err != nil {
		return fmt.Errorf("writing %s: %w", ft.filename, err)
	}
	if err := os.Rename(tmpPath, path); err != nil {
		os.Remove(tmpPath)
		return fmt.Errorf("renaming %s: %w", ft.filename, err)
	}

	return nil
}
