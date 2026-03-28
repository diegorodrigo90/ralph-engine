package logger

import (
	"bytes"
	"encoding/json"
	"strings"
	"testing"
)

func TestNewLoggerDefaults(t *testing.T) {
	l := New(Config{})

	if l.level != LevelInfo {
		t.Errorf("level = %d, want %d (info)", l.level, LevelInfo)
	}
	if l.format != FormatHuman {
		t.Errorf("format = %q, want %q", l.format, FormatHuman)
	}
}

func TestNewLoggerDebugMode(t *testing.T) {
	l := New(Config{Debug: true})

	if l.level != LevelDebug {
		t.Errorf("level = %d, want %d (debug)", l.level, LevelDebug)
	}
}

func TestNewLoggerJSONFormat(t *testing.T) {
	l := New(Config{Format: FormatJSON})

	if l.format != FormatJSON {
		t.Errorf("format = %q, want %q", l.format, FormatJSON)
	}
}

func TestHumanFormatOutput(t *testing.T) {
	var buf bytes.Buffer
	l := New(Config{Output: &buf})

	l.Info("test message", "key", "value")

	output := buf.String()
	if !strings.Contains(output, "test message") {
		t.Errorf("output should contain message, got: %s", output)
	}
	if !strings.Contains(output, "INF") {
		t.Errorf("output should contain level, got: %s", output)
	}
}

func TestJSONFormatOutput(t *testing.T) {
	var buf bytes.Buffer
	l := New(Config{Output: &buf, Format: FormatJSON})

	l.Info("test message", "component", "engine")

	var entry map[string]interface{}
	if err := json.Unmarshal(buf.Bytes(), &entry); err != nil {
		t.Fatalf("JSON output should be valid JSON: %v\nGot: %s", err, buf.String())
	}
	if entry["msg"] != "test message" {
		t.Errorf("msg = %q, want %q", entry["msg"], "test message")
	}
	if entry["level"] != "info" {
		t.Errorf("level = %q, want %q", entry["level"], "info")
	}
	if entry["component"] != "engine" {
		t.Errorf("component = %q, want %q", entry["component"], "engine")
	}
}

func TestDebugNotShownAtInfoLevel(t *testing.T) {
	var buf bytes.Buffer
	l := New(Config{Output: &buf}) // Default = info level

	l.Debug("hidden message")

	if buf.Len() > 0 {
		t.Errorf("debug message should not appear at info level, got: %s", buf.String())
	}
}

func TestDebugShownAtDebugLevel(t *testing.T) {
	var buf bytes.Buffer
	l := New(Config{Output: &buf, Debug: true})

	l.Debug("visible message")

	if buf.Len() == 0 {
		t.Error("debug message should appear at debug level")
	}
}

func TestErrorIncludesContext(t *testing.T) {
	var buf bytes.Buffer
	l := New(Config{Output: &buf, Format: FormatJSON})

	l.Error("something failed",
		"component", "claude-client",
		"story_id", "65.3",
		"exit_code", 1,
		"suggestion", "check if claude binary is in PATH",
	)

	var entry map[string]interface{}
	json.Unmarshal(buf.Bytes(), &entry)

	if entry["component"] != "claude-client" {
		t.Errorf("component = %q, want %q", entry["component"], "claude-client")
	}
	if entry["suggestion"] == nil {
		t.Error("error output should include suggestion for AI consumers")
	}
}

func TestWarnLevel(t *testing.T) {
	var buf bytes.Buffer
	l := New(Config{Output: &buf})

	l.Warn("resource low", "ram_mb", 1500)

	output := buf.String()
	if !strings.Contains(output, "WRN") {
		t.Errorf("warn should show WRN level, got: %s", output)
	}
}

func TestJSONIncludesTimestamp(t *testing.T) {
	var buf bytes.Buffer
	l := New(Config{Output: &buf, Format: FormatJSON})

	l.Info("test")

	var entry map[string]interface{}
	json.Unmarshal(buf.Bytes(), &entry)

	if entry["ts"] == nil {
		t.Error("JSON output should include timestamp")
	}
}

func TestAIFriendlyErrorFormat(t *testing.T) {
	var buf bytes.Buffer
	l := New(Config{Output: &buf, Format: FormatJSON, Debug: true})

	l.Error("circuit breaker tripped",
		"component", "runner",
		"failures", 3,
		"max_failures", 3,
		"last_error", "session exit code 1",
		"suggestion", "check sprint-status.yaml for blocked stories or invalid story format",
		"docs", "https://github.com/diegorodrigo90/ralph-engine#circuit-breaker",
	)

	var entry map[string]interface{}
	json.Unmarshal(buf.Bytes(), &entry)

	// AI agents need structured context to diagnose issues.
	required := []string{"component", "failures", "suggestion", "docs"}
	for _, key := range required {
		if entry[key] == nil {
			t.Errorf("AI-friendly error should include %q", key)
		}
	}
}
