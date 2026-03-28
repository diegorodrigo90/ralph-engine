// Package tracker defines the pluggable task tracking interface.
// Implementations can read stories from files (BMAD, TODO.md),
// external services (GitHub Issues, Linear), or custom scripts.
package tracker

import (
	"fmt"
	"sort"
)

// StoryStatus represents the lifecycle state of a story.
type StoryStatus string

const (
	StatusBacklog     StoryStatus = "backlog"
	StatusReadyForDev StoryStatus = "ready-for-dev"
	StatusInProgress  StoryStatus = "in-progress"
	StatusReview      StoryStatus = "review"
	StatusDone        StoryStatus = "done"
	StatusBlocked     StoryStatus = "blocked"
)

// Story represents a unit of work the engine can execute.
type Story struct {
	ID        string      `json:"id" yaml:"id"`
	Title     string      `json:"title" yaml:"title"`
	Status    StoryStatus `json:"status" yaml:"status"`
	EpicID    string      `json:"epic_id" yaml:"epic_id"`
	EpicTitle string      `json:"epic_title" yaml:"epic_title"`
	FilePath  string      `json:"file_path,omitempty" yaml:"file_path,omitempty"`
	Tags      []string    `json:"tags,omitempty" yaml:"tags,omitempty"`
	DependsOn []string    `json:"depends_on,omitempty" yaml:"depends_on,omitempty"`
}

// IsActionable returns true if the story can be worked on.
func (s Story) IsActionable() bool {
	return s.Status == StatusReadyForDev || s.Status == StatusInProgress
}

// Priority returns a sort weight — lower means higher priority.
// In-progress stories should be picked first (resume interrupted work).
func (s Story) priority() int {
	switch s.Status {
	case StatusInProgress:
		return 0
	case StatusReadyForDev:
		return 1
	case StatusReview:
		return 2
	default:
		return 99
	}
}

// TaskTracker is the interface that all tracker implementations must satisfy.
// The engine uses this to discover, pick, and update story progress.
type TaskTracker interface {
	// NextStory returns the highest-priority actionable story, or nil if none.
	NextStory() (*Story, error)
	// MarkComplete transitions a story to done status.
	MarkComplete(storyID string) error
	// MarkInProgress transitions a story to in-progress status.
	MarkInProgress(storyID string) error
	// ListPending returns all actionable stories sorted by priority.
	ListPending() ([]Story, error)
	// ListAll returns all stories regardless of status.
	ListAll() ([]Story, error)
}

// SortByPriority sorts stories with in-progress first, then ready-for-dev.
func SortByPriority(stories []Story) {
	sort.Slice(stories, func(i, j int) bool {
		return stories[i].priority() < stories[j].priority()
	})
}

// TrackerFactory creates a new TaskTracker instance.
type TrackerFactory func() TaskTracker

// Registry holds registered tracker implementations.
type Registry struct {
	factories map[string]TrackerFactory
}

// NewRegistry creates an empty tracker registry.
func NewRegistry() *Registry {
	return &Registry{
		factories: make(map[string]TrackerFactory),
	}
}

// Register adds a tracker factory to the registry.
func (r *Registry) Register(name string, factory TrackerFactory) {
	r.factories[name] = factory
}

// Get creates a tracker instance by name.
func (r *Registry) Get(name string) (TaskTracker, error) {
	factory, ok := r.factories[name]
	if !ok {
		return nil, fmt.Errorf("unknown tracker: %q (available: %v)", name, r.Available())
	}
	return factory(), nil
}

// Available returns the names of all registered trackers.
func (r *Registry) Available() []string {
	names := make([]string, 0, len(r.factories))
	for name := range r.factories {
		names = append(names, name)
	}
	sort.Strings(names)
	return names
}
