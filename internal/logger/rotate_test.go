package logger

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestLogDirFallback(t *testing.T) {
	// When platform-specific dir cannot be created, falls back to stateDir/logs/.
	stateDir := t.TempDir()
	dir := LogDir(stateDir)

	// The result should be a valid, existing directory.
	info, err := os.Stat(dir)
	if err != nil {
		t.Fatalf("LogDir returned non-existent dir: %v", err)
	}
	if !info.IsDir() {
		t.Fatal("LogDir result is not a directory")
	}
}

func TestLogDirContainsRalphEngine(t *testing.T) {
	stateDir := t.TempDir()
	dir := LogDir(stateDir)

	// Either the platform-specific or fallback path should contain "ralph-engine" or "logs".
	if !strings.Contains(dir, "ralph-engine") && !strings.Contains(dir, "logs") {
		t.Errorf("LogDir = %q, expected to contain 'ralph-engine' or 'logs'", dir)
	}
}

func TestCreateLogFile(t *testing.T) {
	logDir := t.TempDir()
	cfg := DefaultRotateConfig()

	f, path, err := CreateLogFile(logDir, cfg)
	if err != nil {
		t.Fatalf("CreateLogFile error: %v", err)
	}
	defer f.Close()

	// File should exist on disk.
	if _, err := os.Stat(path); os.IsNotExist(err) {
		t.Fatal("CreateLogFile did not create the file")
	}

	// File name should follow the debug-*.log pattern.
	base := filepath.Base(path)
	if !strings.HasPrefix(base, "debug-") || !strings.HasSuffix(base, ".log") {
		t.Errorf("unexpected log file name: %s", base)
	}

	// Should be writable.
	_, err = f.WriteString("test log line\n")
	if err != nil {
		t.Fatalf("log file not writable: %v", err)
	}
}

func TestRotateRemovesOldFiles(t *testing.T) {
	logDir := t.TempDir()
	cfg := RotateConfig{MaxFiles: 10, MaxSizeMB: 50}

	// Create 15 fake log files.
	for i := 0; i < 15; i++ {
		name := filepath.Join(logDir, formatFakeLogName(i))
		if err := os.WriteFile(name, []byte("data"), 0600); err != nil {
			t.Fatalf("creating fake log: %v", err)
		}
	}

	if err := Rotate(logDir, cfg); err != nil {
		t.Fatalf("Rotate error: %v", err)
	}

	// After rotation with MaxFiles=10, should keep at most 9
	// (room for the new file that CreateLogFile would add).
	files, err := ListLogFiles(logDir)
	if err != nil {
		t.Fatalf("ListLogFiles error: %v", err)
	}
	if len(files) >= 10 {
		t.Errorf("expected fewer than 10 files after rotation, got %d", len(files))
	}
}

func TestRotateMaxSize(t *testing.T) {
	logDir := t.TempDir()
	cfg := RotateConfig{MaxFiles: 10, MaxSizeMB: 1} // 1 MB max.

	// Create an oversized file (~2 MB).
	name := filepath.Join(logDir, "debug-20260101-000000.log")
	data := make([]byte, 2*1024*1024) // 2 MB of zeros.
	if err := os.WriteFile(name, data, 0600); err != nil {
		t.Fatalf("creating oversized log: %v", err)
	}

	if err := Rotate(logDir, cfg); err != nil {
		t.Fatalf("Rotate error: %v", err)
	}

	info, err := os.Stat(name)
	if err != nil {
		t.Fatalf("stat after rotate: %v", err)
	}

	maxBytes := int64(1 * 1024 * 1024)
	if info.Size() > maxBytes {
		t.Errorf("file size = %d, want <= %d after truncation", info.Size(), maxBytes)
	}
}

func TestListLogFilesNewestFirst(t *testing.T) {
	logDir := t.TempDir()

	// Create files with known timestamps in lexicographic order.
	names := []string{
		"debug-20260101-100000.log",
		"debug-20260102-100000.log",
		"debug-20260103-100000.log",
	}
	for _, name := range names {
		path := filepath.Join(logDir, name)
		if err := os.WriteFile(path, []byte("x"), 0600); err != nil {
			t.Fatalf("creating file: %v", err)
		}
	}

	files, err := ListLogFiles(logDir)
	if err != nil {
		t.Fatalf("ListLogFiles error: %v", err)
	}

	if len(files) != 3 {
		t.Fatalf("expected 3 files, got %d", len(files))
	}

	// Newest should be first.
	if !strings.Contains(files[0], "20260103") {
		t.Errorf("first file should be newest, got %s", files[0])
	}
	if !strings.Contains(files[2], "20260101") {
		t.Errorf("last file should be oldest, got %s", files[2])
	}
}

func TestListLogFilesIgnoresNonLogFiles(t *testing.T) {
	logDir := t.TempDir()

	// Create a mix of log and non-log files.
	os.WriteFile(filepath.Join(logDir, "debug-20260101-100000.log"), []byte("x"), 0600)
	os.WriteFile(filepath.Join(logDir, "state.json"), []byte("x"), 0600)
	os.WriteFile(filepath.Join(logDir, "readme.txt"), []byte("x"), 0600)

	files, err := ListLogFiles(logDir)
	if err != nil {
		t.Fatalf("ListLogFiles error: %v", err)
	}

	if len(files) != 1 {
		t.Errorf("expected 1 log file, got %d", len(files))
	}
}

func TestRotateEmptyDir(t *testing.T) {
	logDir := t.TempDir()
	cfg := DefaultRotateConfig()

	// Should not panic or error on empty directory.
	if err := Rotate(logDir, cfg); err != nil {
		t.Errorf("Rotate on empty dir should not error, got: %v", err)
	}
}

func TestRotateNonExistentDir(t *testing.T) {
	cfg := DefaultRotateConfig()

	err := Rotate("/nonexistent/path/that/does/not/exist", cfg)
	if err == nil {
		t.Error("Rotate on non-existent dir should return an error")
	}
}

func TestListLogFilesNonExistentDir(t *testing.T) {
	files, err := ListLogFiles("/nonexistent/path")
	if err != nil {
		t.Errorf("ListLogFiles on non-existent dir should return nil error, got: %v", err)
	}
	if files != nil {
		t.Errorf("expected nil files, got %v", files)
	}
}

func TestDefaultRotateConfig(t *testing.T) {
	cfg := DefaultRotateConfig()
	if cfg.MaxFiles != 10 {
		t.Errorf("MaxFiles = %d, want 10", cfg.MaxFiles)
	}
	if cfg.MaxSizeMB != 50 {
		t.Errorf("MaxSizeMB = %d, want 50", cfg.MaxSizeMB)
	}
}

func TestRotateDefaultsOnZeroConfig(t *testing.T) {
	logDir := t.TempDir()

	// Zero config should use defaults (10 files, 50 MB) and not panic.
	cfg := RotateConfig{} // All zeros.
	if err := Rotate(logDir, cfg); err != nil {
		t.Errorf("Rotate with zero config should not error: %v", err)
	}
}

// formatFakeLogName generates a fake log filename for testing.
// Files are named debug-20260101-HHMMSS.log with incrementing times.
func formatFakeLogName(i int) string {
	h := i / 3600
	m := (i % 3600) / 60
	s := i % 60
	return filepath.Base(
		filepath.Join(".", // Ensure clean path.
			strings.Join([]string{
				"debug-20260101-",
				padInt(h), padInt(m), padInt(s),
				".log",
			}, ""),
		),
	)
}

func padInt(n int) string {
	if n < 10 {
		return "0" + string(rune('0'+n))
	}
	return string(rune('0'+n/10)) + string(rune('0'+n%10))
}
