// Package updater provides self-update functionality for ralph-engine.
// It checks GitHub Releases for newer versions and replaces the running binary.
//
// IMPORTANT: The updater ONLY replaces the binary. It NEVER touches user files:
// config.yaml, prompt.md, hooks.yaml, state.json, or any project artifacts.
// User customizations are sacred and must survive every update.
package updater

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"time"
)

const (
	// githubRepo is the GitHub repository for release downloads.
	githubRepo = "diegorodrigo90/ralph-engine"

	// apiTimeout is the maximum time for GitHub API calls.
	apiTimeout = 15 * time.Second

	// downloadTimeout is the maximum time for binary downloads.
	downloadTimeout = 120 * time.Second

	// maxRedirects is the maximum number of HTTP redirects to follow.
	maxRedirects = 5

	// maxDownloadBytes is the maximum size for a downloaded archive (100 MB).
	maxDownloadBytes = 100 * 1024 * 1024
)

// githubRelease represents a GitHub Releases API response.
type githubRelease struct {
	TagName    string        `json:"tag_name"`
	Prerelease bool          `json:"prerelease"`
	Draft      bool          `json:"draft"`
	Assets     []githubAsset `json:"assets"`
}

// githubAsset represents a single asset in a release.
type githubAsset struct {
	Name               string `json:"name"`
	BrowserDownloadURL string `json:"browser_download_url"`
}

// UpdateResult contains the result of an update check or update.
type UpdateResult struct {
	CurrentVersion string
	LatestVersion  string
	Updated        bool
	UpToDate       bool
	Error          error
}

// CheckLatest queries GitHub Releases API for the latest non-prerelease version.
func CheckLatest(currentVersion string) (*UpdateResult, error) {
	return checkLatestFrom(currentVersion, fmt.Sprintf("https://api.github.com/repos/%s/releases/latest", githubRepo))
}

// checkLatestFrom is the internal implementation with injectable URL for testing.
func checkLatestFrom(currentVersion, apiURL string) (*UpdateResult, error) {
	client := &http.Client{Timeout: apiTimeout}
	req, err := http.NewRequest("GET", apiURL, nil)
	if err != nil {
		return nil, fmt.Errorf("creating request: %w", err)
	}
	req.Header.Set("User-Agent", "ralph-engine-updater")
	req.Header.Set("Accept", "application/vnd.github.v3+json")

	resp, err := client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("fetching latest release: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode == http.StatusNotFound {
		return nil, fmt.Errorf("no releases found — have you published a release?")
	}
	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("GitHub API returned HTTP %d", resp.StatusCode)
	}

	var release githubRelease
	if err := json.NewDecoder(resp.Body).Decode(&release); err != nil {
		return nil, fmt.Errorf("decoding release: %w", err)
	}

	latestVersion := strings.TrimPrefix(release.TagName, "v")
	currentClean := strings.TrimPrefix(currentVersion, "v")

	return &UpdateResult{
		CurrentVersion: currentClean,
		LatestVersion:  latestVersion,
		UpToDate:       currentClean == latestVersion,
	}, nil
}

// Update downloads and installs the latest version, replacing the running binary.
func Update(currentVersion string) (*UpdateResult, error) {
	result, err := CheckLatest(currentVersion)
	if err != nil {
		return nil, err
	}

	if result.UpToDate {
		return result, nil
	}

	// Construct the download URL for this platform.
	assetURL := findAssetURL(result.LatestVersion)

	// Download to a temp file.
	tmpDir, err := os.MkdirTemp("", "ralph-engine-update-")
	if err != nil {
		return nil, fmt.Errorf("creating temp dir: %w", err)
	}
	defer os.RemoveAll(tmpDir)

	ext := "tar.gz"
	if runtime.GOOS == "windows" {
		ext = "zip"
	}
	archivePath := filepath.Join(tmpDir, "ralph-engine."+ext)

	if err := downloadFile(assetURL, archivePath); err != nil {
		return nil, fmt.Errorf("downloading release: %w", err)
	}

	// Extract the archive.
	if err := extractArchive(archivePath, tmpDir, ext); err != nil {
		return nil, fmt.Errorf("extracting archive: %w", err)
	}

	// Find the new binary.
	binaryName := "ralph-engine"
	if runtime.GOOS == "windows" {
		binaryName = "ralph-engine.exe"
	}
	newBinary := filepath.Join(tmpDir, binaryName)
	if _, err := os.Stat(newBinary); os.IsNotExist(err) {
		return nil, fmt.Errorf("binary not found in archive: %s", binaryName)
	}

	// Replace the running binary.
	currentBinary, err := os.Executable()
	if err != nil {
		return nil, fmt.Errorf("locating current binary: %w", err)
	}
	currentBinary, err = filepath.EvalSymlinks(currentBinary)
	if err != nil {
		return nil, fmt.Errorf("resolving symlinks: %w", err)
	}

	if err := replaceBinary(currentBinary, newBinary); err != nil {
		return nil, fmt.Errorf("replacing binary: %w", err)
	}

	result.Updated = true
	return result, nil
}

// findAssetURL constructs the download URL for the current platform and architecture.
// Asset naming matches GoReleaser template: ralph-engine_VERSION_OS_ARCH.tar.gz
func findAssetURL(version string) string {
	goos := runtime.GOOS
	goarch := runtime.GOARCH
	ext := "tar.gz"
	if goos == "windows" {
		ext = "zip"
	}

	assetName := fmt.Sprintf("ralph-engine_%s_%s_%s.%s", version, goos, goarch, ext)
	return fmt.Sprintf("https://github.com/%s/releases/download/v%s/%s", githubRepo, version, assetName)
}

// downloadFile downloads a URL to a local file path, following redirects.
// Downloads are limited to maxDownloadBytes to prevent resource exhaustion.
func downloadFile(url, dest string) error {
	client := &http.Client{
		Timeout: downloadTimeout,
		CheckRedirect: func(req *http.Request, via []*http.Request) error {
			if len(via) >= maxRedirects {
				return fmt.Errorf("too many redirects (>%d)", maxRedirects)
			}
			return nil
		},
	}

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return fmt.Errorf("creating request: %w", err)
	}
	req.Header.Set("User-Agent", "ralph-engine-updater")

	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("fetching %s: %w", url, err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("HTTP %d for %s", resp.StatusCode, url)
	}

	out, err := os.Create(dest) // #nosec G304 -- dest is constructed internally from temp dir + known filename
	if err != nil {
		return fmt.Errorf("creating %s: %w", dest, err)
	}
	defer out.Close()

	// Limit download size to prevent resource exhaustion from compromised releases.
	limited := io.LimitReader(resp.Body, maxDownloadBytes)
	if _, err = io.Copy(out, limited); err != nil {
		return fmt.Errorf("writing response to %s: %w", dest, err)
	}
	return nil
}

// extractArchive extracts a tar.gz or zip archive to a destination directory.
func extractArchive(archive, dest, ext string) error {
	switch ext {
	case "tar.gz":
		return runCmd("tar", "-xzf", archive, "-C", dest)
	case "zip":
		// Windows has tar.exe since 2018 that supports zip.
		return runCmd("tar", "-xf", archive, "-C", dest)
	default:
		return fmt.Errorf("unsupported archive format: %s", ext)
	}
}

// replaceBinary atomically replaces the current binary with the new one.
// On Unix: rename old → old.bak, copy new → old path, remove old.bak.
// On Windows: rename old → old.bak (can't delete running exe), copy new → old path.
// Uses streaming copy to avoid loading the entire binary into memory.
func replaceBinary(currentPath, newPath string) error {
	backupPath := currentPath + ".bak"

	// Remove any stale backup from previous update (best-effort).
	_ = os.Remove(backupPath)

	// Move current binary to backup.
	if err := os.Rename(currentPath, backupPath); err != nil {
		return fmt.Errorf("backing up current binary: %w", err)
	}

	// Stream new binary to current path (avoids loading entire file into memory).
	if err := copyFile(newPath, currentPath); err != nil {
		// Restore backup on failure (best-effort).
		_ = os.Rename(backupPath, currentPath)
		return err
	}

	// Remove backup (best-effort — on Windows, running exe can't be deleted).
	_ = os.Remove(backupPath)

	return nil
}

// copyFile copies src to dst using streaming io.Copy.
func copyFile(src, dst string) error {
	in, err := os.Open(src) // #nosec G304 -- src is the extracted binary from temp dir
	if err != nil {
		// Restore is handled by caller.
		return fmt.Errorf("reading new binary: %w", err)
	}
	defer in.Close()

	out, err := os.OpenFile(dst, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0o755) // #nosec G302,G304 -- binary needs 0755 execute permission; dst is from os.Executable()
	if err != nil {
		return fmt.Errorf("writing new binary: %w", err)
	}
	defer out.Close()

	if _, err = io.Copy(out, in); err != nil {
		return fmt.Errorf("copying binary data: %w", err)
	}
	return nil
}

// runCmd executes a command and returns any error, including stderr output.
func runCmd(name string, args ...string) error {
	var stderr strings.Builder
	cmd := exec.Command(name, args...) // #nosec G204 -- runs tar/unzip for self-update extraction, by design
	cmd.Stdout = io.Discard
	cmd.Stderr = &stderr
	if err := cmd.Run(); err != nil {
		msg := strings.TrimSpace(stderr.String())
		if msg != "" {
			return fmt.Errorf("%w: %s", err, msg)
		}
		return err
	}
	return nil
}
