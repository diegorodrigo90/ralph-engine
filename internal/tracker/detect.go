package tracker

import (
	"os"
	"path/filepath"
	"strings"
)

// AutoDetect creates the right tracker based on the file format.
// Returns a FlatFileTracker if the file contains "development_status:",
// otherwise returns a FileTracker (structured YAML with epics array).
func AutoDetect(dir, filename string) TaskTracker {
	path := filepath.Join(dir, filename)
	data, err := os.ReadFile(path)
	if err != nil {
		// File doesn't exist yet — default to structured format.
		return NewFileTracker(dir, filename)
	}

	if strings.Contains(string(data), "development_status:") {
		return NewFlatFileTracker(dir, filename)
	}
	return NewFileTracker(dir, filename)
}
