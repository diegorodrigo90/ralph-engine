// rotate.go provides cross-platform log file management with rotation.
// Debug log files are timestamped per-run and rotated by count and size.
package logger

import (
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"sort"
	"strings"
	"time"
)

// RotateConfig holds log rotation settings.
type RotateConfig struct {
	MaxFiles  int   // Maximum number of log files to keep (default: 10).
	MaxSizeMB int64 // Maximum size per log file in MB (default: 50).
}

// DefaultRotateConfig returns sensible defaults for log rotation.
func DefaultRotateConfig() RotateConfig {
	return RotateConfig{
		MaxFiles:  10,
		MaxSizeMB: 50,
	}
}

// LogDir returns the appropriate log directory for the current platform.
// Falls back to stateDir/logs/ if the platform-specific directory cannot
// be created.
//
// Platform paths:
//
//	Linux:   ~/.local/state/ralph-engine/logs/  (XDG_STATE_HOME compliant)
//	macOS:   ~/Library/Logs/ralph-engine/
//	Windows: %APPDATA%/ralph-engine/logs/
func LogDir(stateDir string) string {
	var dir string

	switch runtime.GOOS {
	case "darwin":
		home, err := os.UserHomeDir()
		if err == nil {
			dir = filepath.Join(home, "Library", "Logs", "ralph-engine")
		}
	case "windows":
		appdata := os.Getenv("APPDATA")
		if appdata != "" {
			dir = filepath.Join(appdata, "ralph-engine", "logs")
		}
	default: // Linux and others — XDG compliant.
		xdgState := os.Getenv("XDG_STATE_HOME")
		if xdgState == "" {
			home, err := os.UserHomeDir()
			if err == nil {
				xdgState = filepath.Join(home, ".local", "state")
			}
		}
		if xdgState != "" {
			dir = filepath.Join(xdgState, "ralph-engine", "logs")
		}
	}

	// Try to create the platform-specific directory.
	if dir != "" {
		if err := os.MkdirAll(dir, 0750); err == nil {
			return dir
		}
	}

	// Fallback: logs inside state dir (project-local, gitignored).
	fallback := filepath.Join(stateDir, "logs")
	_ = os.MkdirAll(fallback, 0750)
	return fallback
}

// CreateLogFile creates a new timestamped log file and performs rotation.
// The caller is responsible for closing the returned file.
func CreateLogFile(logDir string, cfg RotateConfig) (*os.File, string, error) {
	if err := os.MkdirAll(logDir, 0750); err != nil {
		return nil, "", fmt.Errorf("creating log dir: %w", err)
	}

	// Rotate old files first.
	if err := Rotate(logDir, cfg); err != nil {
		// Non-fatal — log rotation failure should not block the engine.
		fmt.Fprintf(os.Stderr, "Warning: log rotation failed: %v\n", err)
	}

	filename := fmt.Sprintf("debug-%s.log", time.Now().Format("20060102-150405"))
	path := filepath.Join(logDir, filename)

	f, err := os.Create(path) // #nosec G304 -- log path in controlled dir
	if err != nil {
		return nil, "", fmt.Errorf("creating log file: %w", err)
	}

	return f, path, nil
}

// Rotate removes old log files beyond MaxFiles and truncates oversized files.
func Rotate(logDir string, cfg RotateConfig) error {
	if cfg.MaxFiles <= 0 {
		cfg.MaxFiles = 10
	}
	if cfg.MaxSizeMB <= 0 {
		cfg.MaxSizeMB = 50
	}

	entries, err := os.ReadDir(logDir)
	if err != nil {
		return fmt.Errorf("reading log dir: %w", err)
	}

	// Collect log files matching the debug-*.log pattern.
	var logFiles []os.DirEntry
	for _, entry := range entries {
		if !entry.IsDir() && strings.HasPrefix(entry.Name(), "debug-") && strings.HasSuffix(entry.Name(), ".log") {
			logFiles = append(logFiles, entry)
		}
	}

	// Sort by name (timestamps sort lexicographically).
	sort.Slice(logFiles, func(i, j int) bool {
		return logFiles[i].Name() < logFiles[j].Name()
	})

	// Remove oldest files beyond MaxFiles (keep room for the new file).
	for len(logFiles) >= cfg.MaxFiles {
		oldest := logFiles[0]
		path := filepath.Join(logDir, oldest.Name())
		if err := os.Remove(path); err != nil {
			return fmt.Errorf("removing old log %s: %w", oldest.Name(), err)
		}
		logFiles = logFiles[1:]
	}

	// Truncate oversized files.
	maxBytes := cfg.MaxSizeMB * 1024 * 1024
	for _, entry := range logFiles {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		if info.Size() > maxBytes {
			path := filepath.Join(logDir, entry.Name())
			if err := os.Truncate(path, maxBytes); err != nil {
				return fmt.Errorf("truncating log %s: %w", entry.Name(), err)
			}
		}
	}

	return nil
}

// ListLogFiles returns all debug log files in the directory, newest first.
func ListLogFiles(logDir string) ([]string, error) {
	entries, err := os.ReadDir(logDir)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, nil
		}
		return nil, err
	}

	var files []string
	for _, entry := range entries {
		if !entry.IsDir() && strings.HasPrefix(entry.Name(), "debug-") && strings.HasSuffix(entry.Name(), ".log") {
			files = append(files, filepath.Join(logDir, entry.Name()))
		}
	}

	// Sort descending (newest first).
	sort.Sort(sort.Reverse(sort.StringSlice(files)))
	return files, nil
}
