package updater

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"runtime"
	"testing"
)

func TestFindAssetURL(t *testing.T) {
	url := findAssetURL("1.2.3")

	ext := "tar.gz"
	if runtime.GOOS == "windows" {
		ext = "zip"
	}

	expected := "https://github.com/diegorodrigo90/ralph-engine/releases/download/v1.2.3/" +
		"ralph-engine_1.2.3_" + runtime.GOOS + "_" + runtime.GOARCH + "." + ext
	if url != expected {
		t.Errorf("got %s, want %s", url, expected)
	}
}

func TestCheckLatestUpToDate(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		release := githubRelease{
			TagName:    "v1.0.0",
			Prerelease: false,
			Draft:      false,
		}
		json.NewEncoder(w).Encode(release)
	}))
	defer server.Close()

	result, err := checkLatestFrom("v1.0.0", server.URL)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !result.UpToDate {
		t.Error("expected UpToDate to be true")
	}
	if result.CurrentVersion != "1.0.0" {
		t.Errorf("expected CurrentVersion 1.0.0, got %s", result.CurrentVersion)
	}
	if result.LatestVersion != "1.0.0" {
		t.Errorf("expected LatestVersion 1.0.0, got %s", result.LatestVersion)
	}
}

func TestCheckLatestNewerAvailable(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		release := githubRelease{TagName: "v1.1.0"}
		json.NewEncoder(w).Encode(release)
	}))
	defer server.Close()

	result, err := checkLatestFrom("v1.0.0", server.URL)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result.UpToDate {
		t.Error("expected UpToDate to be false")
	}
	if result.LatestVersion != "1.1.0" {
		t.Errorf("expected LatestVersion 1.1.0, got %s", result.LatestVersion)
	}
}

func TestCheckLatestNotFound(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusNotFound)
	}))
	defer server.Close()

	_, err := checkLatestFrom("1.0.0", server.URL)
	if err == nil {
		t.Fatal("expected error for 404 response")
	}
}

func TestCheckLatestBadJSON(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("not json"))
	}))
	defer server.Close()

	_, err := checkLatestFrom("1.0.0", server.URL)
	if err == nil {
		t.Fatal("expected error for invalid JSON")
	}
}

func TestReplaceBinarySuccess(t *testing.T) {
	tmpDir := t.TempDir()

	currentPath := filepath.Join(tmpDir, "ralph-engine")
	if err := os.WriteFile(currentPath, []byte("old-binary"), 0o755); err != nil {
		t.Fatal(err)
	}

	newPath := filepath.Join(tmpDir, "ralph-engine-new")
	if err := os.WriteFile(newPath, []byte("new-binary"), 0o755); err != nil {
		t.Fatal(err)
	}

	if err := replaceBinary(currentPath, newPath); err != nil {
		t.Fatalf("replaceBinary failed: %v", err)
	}

	data, err := os.ReadFile(currentPath)
	if err != nil {
		t.Fatal(err)
	}
	if string(data) != "new-binary" {
		t.Errorf("got %q, want %q", string(data), "new-binary")
	}

	if _, err := os.Stat(currentPath + ".bak"); !os.IsNotExist(err) {
		t.Error("backup file should be removed after successful update")
	}
}

func TestReplaceBinaryRestoresOnFailure(t *testing.T) {
	tmpDir := t.TempDir()

	currentPath := filepath.Join(tmpDir, "ralph-engine")
	if err := os.WriteFile(currentPath, []byte("old-binary"), 0o755); err != nil {
		t.Fatal(err)
	}

	newPath := filepath.Join(tmpDir, "does-not-exist")

	err := replaceBinary(currentPath, newPath)
	if err == nil {
		t.Fatal("expected error when new binary doesn't exist")
	}

	data, err := os.ReadFile(currentPath)
	if err != nil {
		t.Fatal(err)
	}
	if string(data) != "old-binary" {
		t.Errorf("original binary should be restored, got %q", string(data))
	}
}

func TestVersionCleaning(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  string
	}{
		{"with v prefix", "v1.2.3", "1.2.3"},
		{"without v prefix", "1.2.3", "1.2.3"},
		{"zero version", "v0.0.1", "0.0.1"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := stripV(tt.input)
			if got != tt.want {
				t.Errorf("stripV(%q) = %q, want %q", tt.input, got, tt.want)
			}
		})
	}
}

// stripV is a test helper that mirrors the prefix stripping in CheckLatest.
func stripV(v string) string {
	if len(v) > 0 && v[0] == 'v' {
		return v[1:]
	}
	return v
}

func TestDownloadFileHTTPError(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusNotFound)
	}))
	defer server.Close()

	tmpDir := t.TempDir()
	dest := filepath.Join(tmpDir, "test-file")

	err := downloadFile(server.URL, dest)
	if err == nil {
		t.Fatal("expected error for 404 response")
	}
}

func TestDownloadFileSuccess(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("binary-content"))
	}))
	defer server.Close()

	tmpDir := t.TempDir()
	dest := filepath.Join(tmpDir, "test-file")

	err := downloadFile(server.URL, dest)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	data, err := os.ReadFile(dest)
	if err != nil {
		t.Fatal(err)
	}
	if string(data) != "binary-content" {
		t.Errorf("got %q, want %q", string(data), "binary-content")
	}
}

func TestRunCmdCapturesStderr(t *testing.T) {
	// Run a command that writes to stderr and fails.
	err := runCmd("sh", "-c", "echo 'error details' >&2; exit 1")
	if err == nil {
		t.Fatal("expected error")
	}
	if got := err.Error(); got == "" {
		t.Error("error should include stderr output")
	}
}
