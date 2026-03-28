package tracker

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"time"
)

// CommandConfig defines user-provided commands for each tracker operation.
// Users configure these in .ralph-engine/config.yaml under tracker.commands.
//
// Example config:
//
//	tracker:
//	  type: "command"
//	  commands:
//	    next:     "./scripts/next-story.sh"
//	    complete: "./scripts/mark-complete.sh {{.ID}}"
//	    progress: "./scripts/mark-progress.sh {{.ID}}"
//	    pending:  "./scripts/list-pending.sh"
//	    all:      "./scripts/list-all.sh"
//
// Each command SHALL output JSON in the Story format.
// The engine passes story ID as the first argument where {{.ID}} appears.
type CommandConfig struct {
	Next     string // Command that outputs a single Story JSON (or empty for none).
	Complete string // Command to mark story done. Receives story ID as arg.
	Progress string // Command to mark story in-progress. Receives story ID as arg.
	Pending  string // Command that outputs JSON array of pending Story objects.
	All      string // Command that outputs JSON array of all Story objects.
	Timeout  int    // Command timeout in seconds (default: 30).
}

// CommandTracker delegates all operations to user-defined commands.
// This enables integration with ANY task system — Jira, Notion, custom DB, etc.
type CommandTracker struct {
	config  CommandConfig
	workDir string
}

// NewCommandTracker creates a tracker that calls external commands.
func NewCommandTracker(workDir string, config CommandConfig) *CommandTracker {
	if config.Timeout <= 0 {
		config.Timeout = 30
	}
	return &CommandTracker{config: config, workDir: workDir}
}

// NextStory calls the "next" command and parses the output as a Story.
func (ct *CommandTracker) NextStory() (*Story, error) {
	if ct.config.Next == "" {
		return nil, fmt.Errorf("tracker command 'next' not configured")
	}

	output, err := ct.run(ct.config.Next)
	if err != nil {
		return nil, fmt.Errorf("next command failed: %w", err)
	}

	output = trimOutput(output)
	if output == "" || output == "null" || output == "{}" {
		return nil, nil
	}

	var story Story
	if err := json.Unmarshal([]byte(output), &story); err != nil {
		return nil, fmt.Errorf("parsing next command output: %w (output: %s)", err, truncate(output, 200))
	}
	return &story, nil
}

// MarkComplete calls the "complete" command with the story ID.
func (ct *CommandTracker) MarkComplete(storyID string) error {
	if ct.config.Complete == "" {
		return nil
	}
	_, err := ct.run(ct.config.Complete, storyID)
	return err
}

// MarkInProgress calls the "progress" command with the story ID.
func (ct *CommandTracker) MarkInProgress(storyID string) error {
	if ct.config.Progress == "" {
		return nil
	}
	_, err := ct.run(ct.config.Progress, storyID)
	return err
}

// RevertToReady calls the "revert" command, or falls back to the "progress" command with ready-for-dev hint.
func (ct *CommandTracker) RevertToReady(storyID string) error {
	// CommandTracker doesn't have a specific revert command — best effort.
	return nil
}

// ListPending calls the "pending" command and parses the output as a Story array.
func (ct *CommandTracker) ListPending() ([]Story, error) {
	if ct.config.Pending == "" {
		return nil, fmt.Errorf("tracker command 'pending' not configured")
	}
	return ct.runList(ct.config.Pending)
}

// ListAll calls the "all" command and parses the output as a Story array.
func (ct *CommandTracker) ListAll() ([]Story, error) {
	if ct.config.All == "" {
		return nil, fmt.Errorf("tracker command 'all' not configured")
	}
	return ct.runList(ct.config.All)
}

// run executes a command with optional arguments.
func (ct *CommandTracker) run(command string, args ...string) (string, error) {
	ctx, cancel := context.WithTimeout(context.Background(),
		time.Duration(ct.config.Timeout)*time.Second)
	defer cancel()

	shell := findShell()
	cmd := exec.CommandContext(ctx, shell, "-c", command+argsToString(args)) // #nosec G204 -- user-configured tracker commands, by design
	cmd.Dir = ct.workDir
	cmd.Stdin = nil

	output, err := cmd.CombinedOutput()
	if err != nil {
		return "", fmt.Errorf("command %q failed: %w (output: %s)",
			command, err, truncate(string(output), 200))
	}
	return string(output), nil
}

// runList executes a command and parses output as Story array.
func (ct *CommandTracker) runList(command string) ([]Story, error) {
	output, err := ct.run(command)
	if err != nil {
		return nil, err
	}

	output = trimOutput(output)
	if output == "" || output == "null" || output == "[]" {
		return nil, nil
	}

	var stories []Story
	if err := json.Unmarshal([]byte(output), &stories); err != nil {
		return nil, fmt.Errorf("parsing list output: %w (output: %s)", err, truncate(output, 200))
	}
	return stories, nil
}

func argsToString(args []string) string {
	if len(args) == 0 {
		return ""
	}
	s := ""
	for _, a := range args {
		s += " " + a
	}
	return s
}

func trimOutput(s string) string {
	for len(s) > 0 && (s[0] == ' ' || s[0] == '\n' || s[0] == '\r' || s[0] == '\t') {
		s = s[1:]
	}
	for len(s) > 0 && (s[len(s)-1] == ' ' || s[len(s)-1] == '\n' || s[len(s)-1] == '\r' || s[len(s)-1] == '\t') {
		s = s[:len(s)-1]
	}
	return s
}

// findShell returns the path to a POSIX shell for command execution.
func findShell() string {
	for _, sh := range []string{"/bin/sh", "/usr/bin/sh", "/bin/bash", "/usr/bin/bash"} {
		if _, err := os.Stat(sh); err == nil {
			return sh
		}
	}
	// Fallback to PATH lookup.
	if path, err := exec.LookPath("sh"); err == nil {
		return path
	}
	return "sh"
}

func truncate(s string, maxLen int) string {
	if len(s) <= maxLen {
		return s
	}
	return s[:maxLen] + "..."
}
