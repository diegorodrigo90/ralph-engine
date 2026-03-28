package security

import (
	"os"
	"path/filepath"
	"testing"
)

func TestNoticeNotAcceptedByDefault(t *testing.T) {
	dir := t.TempDir()
	n := NewNotice(dir)

	if n.IsAccepted() {
		t.Error("IsAccepted() should be false on first run")
	}
}

func TestNoticeAcceptPersists(t *testing.T) {
	dir := t.TempDir()
	n := NewNotice(dir)

	if err := n.Accept(); err != nil {
		t.Fatalf("Accept() error: %v", err)
	}

	if !n.IsAccepted() {
		t.Error("IsAccepted() should be true after Accept()")
	}

	// New instance should read from file
	n2 := NewNotice(dir)
	if !n2.IsAccepted() {
		t.Error("IsAccepted() should be true from persisted file")
	}
}

func TestNoticeFileCreated(t *testing.T) {
	dir := t.TempDir()
	n := NewNotice(dir)
	n.Accept()

	path := filepath.Join(dir, "security-accepted")
	if _, err := os.Stat(path); os.IsNotExist(err) {
		t.Error("security-accepted file should exist after Accept()")
	}
}

func TestNoticeTextContainsWarnings(t *testing.T) {
	text := NoticeText()

	if text == "" {
		t.Fatal("NoticeText() should not be empty")
	}

	// Should warn about key security concerns
	mustContain := []string{
		"dangerously-skip-permissions",
		"container",
		"billing",
	}
	for _, keyword := range mustContain {
		found := false
		for _, line := range []byte(text) {
			_ = line
		}
		if !containsStr(text, keyword) {
			t.Errorf("NoticeText() should contain %q", keyword)
		}
		_ = found
	}
}

func TestNoticeRequiredWhenSkipPermissions(t *testing.T) {
	dir := t.TempDir()
	n := NewNotice(dir)

	err := n.Validate(true) // skipPermissions=true
	if err == nil {
		t.Error("Validate() should error when not accepted and skip-permissions is true")
	}
}

func TestNoticeNotRequiredWithoutSkipPermissions(t *testing.T) {
	dir := t.TempDir()
	n := NewNotice(dir)

	err := n.Validate(false) // skipPermissions=false
	if err != nil {
		t.Errorf("Validate() should not error without skip-permissions: %v", err)
	}
}

func TestNoticeValidatePassesAfterAccept(t *testing.T) {
	dir := t.TempDir()
	n := NewNotice(dir)
	n.Accept()

	err := n.Validate(true)
	if err != nil {
		t.Errorf("Validate() should pass after Accept(): %v", err)
	}
}

func containsStr(s, substr string) bool {
	return len(s) >= len(substr) && searchStr(s, substr)
}

func searchStr(s, substr string) bool {
	for i := 0; i <= len(s)-len(substr); i++ {
		if s[i:i+len(substr)] == substr {
			return true
		}
	}
	return false
}
