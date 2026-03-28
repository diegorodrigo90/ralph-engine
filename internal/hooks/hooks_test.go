package hooks

import (
	"context"
	"os"
	"os/exec"
	"path/filepath"
	"testing"
	"time"
)

func TestLoadHooksFile(t *testing.T) {
	dir := t.TempDir()
	os.MkdirAll(filepath.Join(dir, ".ralph-engine"), 0755)
	os.WriteFile(filepath.Join(dir, ".ralph-engine", "hooks.yaml"), []byte(`
preflight:
  steps:
    - name: "check"
      run: "echo ok"
      required: false
quality_gates:
  steps:
    - name: "tests"
      run: "echo pass"
      required: true
      paths: ["src/**"]
`), 0644)

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if cfg == nil {
		t.Fatal("expected config, got nil")
	}
	if len(cfg.Preflight.Steps) != 1 {
		t.Errorf("expected 1 preflight step, got %d", len(cfg.Preflight.Steps))
	}
	if cfg.Preflight.Steps[0].Name != "check" {
		t.Errorf("expected step name 'check', got %q", cfg.Preflight.Steps[0].Name)
	}
	if len(cfg.QualityGates.Steps) != 1 {
		t.Errorf("expected 1 quality gate step, got %d", len(cfg.QualityGates.Steps))
	}
	if !cfg.QualityGates.Steps[0].Required {
		t.Error("expected quality gate step to be required")
	}
}

func TestLoadHooksFileMissing(t *testing.T) {
	cfg, err := Load(t.TempDir())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if cfg != nil {
		t.Error("expected nil config for missing hooks.yaml")
	}
}

func TestLoadHooksFileInvalidYAML(t *testing.T) {
	dir := t.TempDir()
	os.MkdirAll(filepath.Join(dir, ".ralph-engine"), 0755)
	os.WriteFile(filepath.Join(dir, ".ralph-engine", "hooks.yaml"), []byte("{{invalid"), 0644)

	_, err := Load(dir)
	if err == nil {
		t.Error("expected error for invalid YAML")
	}
}

func TestRunPhaseSuccess(t *testing.T) {
	phase := HookPhase{
		Steps: []HookStep{
			{Name: "step1", Run: "echo hello", Required: true},
			{Name: "step2", Run: "echo world", Required: false},
		},
	}

	result := RunPhase(context.Background(), phase, t.TempDir(), nil, nil)
	if result.Blocked {
		t.Error("expected no block")
	}
	if len(result.Steps) != 2 {
		t.Errorf("expected 2 results, got %d", len(result.Steps))
	}
	for _, sr := range result.Steps {
		if !sr.OK {
			t.Errorf("step %q should have passed", sr.Name)
		}
	}
}

func TestRunPhaseRequiredFailureBlocks(t *testing.T) {
	phase := HookPhase{
		Steps: []HookStep{
			{Name: "will-fail", Run: "exit 1", Required: true},
			{Name: "should-not-run", Run: "echo skip", Required: false},
		},
	}

	result := RunPhase(context.Background(), phase, t.TempDir(), nil, nil)
	if !result.Blocked {
		t.Error("expected block on required failure")
	}
	if len(result.Steps) != 1 {
		t.Errorf("expected 1 result (stopped after failure), got %d", len(result.Steps))
	}
}

func TestRunPhaseOptionalFailureContinues(t *testing.T) {
	phase := HookPhase{
		Steps: []HookStep{
			{Name: "optional-fail", Run: "exit 1", Required: false},
			{Name: "should-run", Run: "echo ok", Required: true},
		},
	}

	result := RunPhase(context.Background(), phase, t.TempDir(), nil, nil)
	if result.Blocked {
		t.Error("should not block on optional failure")
	}
	if len(result.Steps) != 2 {
		t.Errorf("expected 2 results, got %d", len(result.Steps))
	}
	if result.Steps[0].OK {
		t.Error("first step should have failed")
	}
	if !result.Steps[1].OK {
		t.Error("second step should have passed")
	}
}

func TestRunPhasePathFiltering(t *testing.T) {
	phase := HookPhase{
		Steps: []HookStep{
			{Name: "ts-tests", Run: "echo ts", Required: true, Paths: []string{"src/**"}},
			{Name: "py-tests", Run: "echo py", Required: true, Paths: []string{"workers/**"}},
			{Name: "always", Run: "echo always", Required: true},
		},
	}

	// Only src/ files changed — py-tests should be skipped.
	changed := []string{"src/index.ts", "src/utils.ts"}
	result := RunPhase(context.Background(), phase, t.TempDir(), changed, nil)

	if result.Blocked {
		t.Error("should not block")
	}
	if len(result.Steps) != 3 {
		t.Fatalf("expected 3 results, got %d", len(result.Steps))
	}
	if result.Steps[0].Skipped {
		t.Error("ts-tests should NOT be skipped (src/** matches)")
	}
	if !result.Steps[1].Skipped {
		t.Error("py-tests SHOULD be skipped (workers/** doesn't match)")
	}
	if result.Steps[2].Skipped {
		t.Error("always should NOT be skipped (no paths filter)")
	}
}

func TestRunPhaseEmptyRunSkipped(t *testing.T) {
	phase := HookPhase{
		Steps: []HookStep{
			{Name: "empty", Run: "", Required: true},
			{Name: "real", Run: "echo ok", Required: true},
		},
	}

	result := RunPhase(context.Background(), phase, t.TempDir(), nil, nil)
	if len(result.Steps) != 1 {
		t.Errorf("expected 1 result (empty step skipped), got %d", len(result.Steps))
	}
}

func TestRunPhaseTimeout(t *testing.T) {
	// Use a command that exits quickly on signal (not sleep which may linger).
	phase := HookPhase{
		Steps: []HookStep{
			{Name: "slow", Run: "sh -c 'while true; do :; done'", Required: true, Timeout: "200ms"},
		},
	}

	start := time.Now()
	result := RunPhase(context.Background(), phase, t.TempDir(), nil, nil)
	elapsed := time.Since(start)

	if len(result.Steps) == 0 {
		t.Fatal("expected at least 1 step result")
	}
	if result.Steps[0].OK {
		t.Error("timed-out step should fail")
	}
	if elapsed > 5*time.Second {
		t.Errorf("timeout should have killed step quickly, took %v", elapsed)
	}
}

func TestRunPhaseContextCancellation(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	cancel() // Cancel immediately.

	phase := HookPhase{
		Steps: []HookStep{
			{Name: "should-not-run", Run: "echo nope", Required: true},
		},
	}

	result := RunPhase(ctx, phase, t.TempDir(), nil, nil)
	if len(result.Steps) != 0 {
		t.Error("no steps should run after context cancellation")
	}
}

func TestRunPhaseCallback(t *testing.T) {
	phase := HookPhase{
		Steps: []HookStep{
			{Name: "step1", Run: "echo hi", Required: true},
		},
	}

	var called int
	RunPhase(context.Background(), phase, t.TempDir(), nil, func(sr StepResult) {
		called++
	})
	if called != 1 {
		t.Errorf("expected callback called 1 time, got %d", called)
	}
}

func TestMatchesAnyPath(t *testing.T) {
	tests := []struct {
		name     string
		changed  []string
		patterns []string
		want     bool
	}{
		{"exact match", []string{"src/index.ts"}, []string{"src/index.ts"}, true},
		{"glob match", []string{"src/index.ts"}, []string{"src/*.ts"}, true},
		{"double star prefix", []string{"apps/api/src/main.ts"}, []string{"apps/**"}, true},
		{"no match", []string{"docs/readme.md"}, []string{"src/**"}, false},
		{"multiple files one matches", []string{"docs/a.md", "src/b.ts"}, []string{"src/**"}, true},
		{"double star suffix match", []string{"packages/graphql/schema.graphql"}, []string{"**/*.graphql"}, true},
		{"double star suffix no match", []string{"src/index.ts"}, []string{"**/*.graphql"}, false},
		{"double star suffix no match py", []string{"workers/scrapper-py/main.py"}, []string{"**/*.graphql"}, false},
		{"prefix + suffix", []string{"apps/api/src/user.resolver.ts"}, []string{"apps/api/src/**/*.resolver.*"}, true},
		{"prefix + suffix no match", []string{"apps/web/src/page.tsx"}, []string{"apps/api/src/**/*.resolver.*"}, false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := MatchesAnyPath(tt.changed, tt.patterns)
			if got != tt.want {
				t.Errorf("MatchesAnyPath(%v, %v) = %v, want %v", tt.changed, tt.patterns, got, tt.want)
			}
		})
	}
}

func TestGetChangedFilesInGitRepo(t *testing.T) {
	// Create a real git repo to test GetChangedFiles.
	dir := t.TempDir()

	// Init repo with an initial commit.
	run := func(args ...string) {
		cmd := exec.Command(args[0], args[1:]...)
		cmd.Dir = dir
		cmd.Env = append(os.Environ(), "GIT_AUTHOR_NAME=test", "GIT_AUTHOR_EMAIL=test@test.com",
			"GIT_COMMITTER_NAME=test", "GIT_COMMITTER_EMAIL=test@test.com")
		if out, err := cmd.CombinedOutput(); err != nil {
			t.Fatalf("command %v failed: %v\n%s", args, err, out)
		}
	}

	run("git", "init")
	os.WriteFile(filepath.Join(dir, "initial.txt"), []byte("init"), 0644)
	run("git", "add", ".")
	run("git", "commit", "-m", "initial")

	// Case 1: Uncommitted changes should be detected.
	os.WriteFile(filepath.Join(dir, "uncommitted.txt"), []byte("new"), 0644)
	files := GetChangedFiles(dir)
	found := false
	for _, f := range files {
		if f == "uncommitted.txt" {
			found = true
		}
	}
	if !found {
		t.Errorf("expected uncommitted.txt in changed files, got: %v", files)
	}

	// Case 2: Committed changes should be detected (HEAD vs HEAD~1).
	run("git", "add", ".")
	run("git", "commit", "-m", "add file")
	files = GetChangedFiles(dir)
	found = false
	for _, f := range files {
		if f == "uncommitted.txt" {
			found = true
		}
	}
	if !found {
		t.Errorf("expected uncommitted.txt in post-commit changed files, got: %v", files)
	}

	// Case 3: No duplicates when file is in both uncommitted and last commit.
	os.WriteFile(filepath.Join(dir, "uncommitted.txt"), []byte("modified"), 0644)
	files = GetChangedFiles(dir)
	count := 0
	for _, f := range files {
		if f == "uncommitted.txt" {
			count++
		}
	}
	if count != 1 {
		t.Errorf("expected exactly 1 occurrence of uncommitted.txt, got %d in: %v", count, files)
	}
}

func TestGetChangedFilesNonGitDir(t *testing.T) {
	// Non-git directory should return nil (not panic).
	dir := t.TempDir()
	files := GetChangedFiles(dir)
	if files != nil {
		t.Errorf("expected nil for non-git dir, got: %v", files)
	}
}

func TestTruncateOutput(t *testing.T) {
	short := "hello"
	if truncateOutput(short) != short {
		t.Error("short string should not be truncated")
	}

	long := make([]byte, 3000)
	for i := range long {
		long[i] = 'x'
	}
	result := truncateOutput(string(long))
	if len(result) > 2100 {
		t.Error("long output should be truncated")
	}
}

// --- Additional tests for MatchesAnyPath edge cases ---

func TestMatchesAnyPathEdgeCases(t *testing.T) {
	tests := []struct {
		name     string
		changed  []string
		patterns []string
		want     bool
	}{
		{
			name:     "empty patterns returns false",
			changed:  []string{"src/index.ts"},
			patterns: []string{},
			want:     false,
		},
		{
			name:     "empty files returns false",
			changed:  []string{},
			patterns: []string{"src/**"},
			want:     false,
		},
		{
			name:     "both empty returns false",
			changed:  []string{},
			patterns: []string{},
			want:     false,
		},
		{
			name:     "exact file match no glob",
			changed:  []string{"README.md"},
			patterns: []string{"README.md"},
			want:     true,
		},
		{
			name:     "exact file match no glob miss",
			changed:  []string{"README.md"},
			patterns: []string{"CHANGELOG.md"},
			want:     false,
		},
		{
			name:     "deeply nested double star only splits first occurrence",
			changed:  []string{"a/b/c/d/e/f.ts"},
			patterns: []string{"**/**/**/*.ts"},
			want:     false, // SplitN on first ** leaves "**/*.ts" as suffix; filepath.Match won't match "b/c/d/e/f.ts"
		},
		{
			name:     "deeply nested file with single double star",
			changed:  []string{"a/b/c/d/e/f.ts"},
			patterns: []string{"**/*.ts"},
			want:     true,
		},
		{
			name:     "triple star prefix pattern",
			changed:  []string{"deep/nested/file.go"},
			patterns: []string{"**/*.go"},
			want:     true,
		},
		{
			name:     "file with spaces in name",
			changed:  []string{"docs/my file.md"},
			patterns: []string{"docs/**"},
			want:     true,
		},
		{
			name:     "file with dots in name",
			changed:  []string{"src/user.resolver.spec.ts"},
			patterns: []string{"src/**"},
			want:     true,
		},
		{
			name:     "file with hyphens in path",
			changed:  []string{"my-app/src/my-component.tsx"},
			patterns: []string{"my-app/**"},
			want:     true,
		},
		{
			name:     "file with hyphens pattern exact glob",
			changed:  []string{"my-app/src/my-component.tsx"},
			patterns: []string{"my-app/src/*.tsx"},
			want:     true,
		},
		{
			name:     "single file single exact pattern match",
			changed:  []string{"package.json"},
			patterns: []string{"package.json"},
			want:     true,
		},
		{
			name:     "nil changed files treated as empty",
			changed:  nil,
			patterns: []string{"src/**"},
			want:     false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := MatchesAnyPath(tt.changed, tt.patterns)
			if got != tt.want {
				t.Errorf("MatchesAnyPath(%v, %v) = %v, want %v", tt.changed, tt.patterns, got, tt.want)
			}
		})
	}
}

// --- Additional tests for RunPhase edge cases ---

func TestRunPhaseEmptyStepsList(t *testing.T) {
	phase := HookPhase{Steps: []HookStep{}}
	result := RunPhase(context.Background(), phase, t.TempDir(), nil, nil)

	if result.Blocked {
		t.Error("empty steps list should not block")
	}
	if len(result.Steps) != 0 {
		t.Errorf("expected 0 results for empty steps, got %d", len(result.Steps))
	}
	if result.Reason != "" {
		t.Errorf("expected empty reason, got %q", result.Reason)
	}
}

func TestRunPhaseNilSteps(t *testing.T) {
	phase := HookPhase{Steps: nil}
	result := RunPhase(context.Background(), phase, t.TempDir(), nil, nil)

	if result.Blocked {
		t.Error("nil steps should not block")
	}
	if len(result.Steps) != 0 {
		t.Errorf("expected 0 results for nil steps, got %d", len(result.Steps))
	}
}

func TestRunPhaseInvalidTimeout(t *testing.T) {
	// An invalid timeout string should fall back to DefaultTimeout (not crash).
	// We use a fast command so it completes well within any timeout.
	phase := HookPhase{
		Steps: []HookStep{
			{Name: "invalid-timeout", Run: "echo ok", Required: true, Timeout: "invalid"},
		},
	}

	result := RunPhase(context.Background(), phase, t.TempDir(), nil, nil)
	if len(result.Steps) != 1 {
		t.Fatalf("expected 1 result, got %d", len(result.Steps))
	}
	if !result.Steps[0].OK {
		t.Errorf("step with invalid timeout should still succeed (falls back to default), error: %v", result.Steps[0].Error)
	}
}

func TestRunPhaseZeroTimeout(t *testing.T) {
	// timeout="0s" means zero duration — command should be killed immediately.
	phase := HookPhase{
		Steps: []HookStep{
			{Name: "zero-timeout", Run: "sh -c 'while true; do :; done'", Required: true, Timeout: "0s"},
		},
	}

	start := time.Now()
	result := RunPhase(context.Background(), phase, t.TempDir(), nil, nil)
	elapsed := time.Since(start)

	if len(result.Steps) != 1 {
		t.Fatalf("expected 1 result, got %d", len(result.Steps))
	}
	if result.Steps[0].OK {
		t.Error("step with 0s timeout should fail")
	}
	if elapsed > 5*time.Second {
		t.Errorf("zero timeout should resolve quickly, took %v", elapsed)
	}
}

// --- Additional tests for truncateOutput boundary ---

func TestTruncateOutputBoundary(t *testing.T) {
	tests := []struct {
		name           string
		inputLen       int
		wantTruncated  bool
		wantExactMatch bool // when not truncated, output should equal input
	}{
		{"exactly 2000 chars not truncated", 2000, false, true},
		{"2001 chars truncated", 2001, true, false},
		{"1999 chars not truncated", 1999, false, true},
		{"empty string not truncated", 0, false, true},
		{"1 char not truncated", 1, false, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			input := make([]byte, tt.inputLen)
			for i := range input {
				input[i] = 'a'
			}
			s := string(input)
			result := truncateOutput(s)

			if tt.wantTruncated {
				if result == s {
					t.Error("expected truncation but output equals input")
				}
				if len(result) > 2000+len("\n[output truncated]") {
					t.Errorf("truncated output too long: %d", len(result))
				}
				suffix := "\n[output truncated]"
				if len(result) < len(suffix) || result[len(result)-len(suffix):] != suffix {
					t.Error("truncated output should end with truncation marker")
				}
			} else {
				if result != s {
					t.Errorf("expected no truncation, but output differs (input len=%d, output len=%d)", len(s), len(result))
				}
			}
		})
	}
}

// --- Additional tests for Load edge cases ---

func TestLoadEmptyFile(t *testing.T) {
	dir := t.TempDir()
	os.MkdirAll(filepath.Join(dir, ".ralph-engine"), 0755)
	os.WriteFile(filepath.Join(dir, ".ralph-engine", "hooks.yaml"), []byte(""), 0644)

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("empty file should not error, got: %v", err)
	}
	// Empty YAML unmarshals to zero-value struct.
	if cfg == nil {
		t.Fatal("expected non-nil config for empty YAML (zero-value struct)")
	}
	if len(cfg.Preflight.Steps) != 0 {
		t.Errorf("expected 0 preflight steps, got %d", len(cfg.Preflight.Steps))
	}
}

func TestLoadMalformedYAML(t *testing.T) {
	tests := []struct {
		name    string
		content string
	}{
		{"yaml with tabs in wrong place", "\t\t\tbad:\n\t\tyaml"},
		{"unclosed bracket", "preflight: ["},
		{"binary-like content", "\x00\x01\x02\x03"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			dir := t.TempDir()
			os.MkdirAll(filepath.Join(dir, ".ralph-engine"), 0755)
			os.WriteFile(filepath.Join(dir, ".ralph-engine", "hooks.yaml"), []byte(tt.content), 0644)

			_, err := Load(dir)
			if err == nil {
				t.Error("expected error for malformed YAML")
			}
		})
	}
}

func TestLoadNoDotRalphEngineDir(t *testing.T) {
	// Directory exists but has no .ralph-engine/ subdirectory.
	dir := t.TempDir()
	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("missing .ralph-engine dir should not error, got: %v", err)
	}
	if cfg != nil {
		t.Error("expected nil config when .ralph-engine dir doesn't exist")
	}
}

func TestLoadDotRalphEngineDirExistsButNoHooksYaml(t *testing.T) {
	dir := t.TempDir()
	os.MkdirAll(filepath.Join(dir, ".ralph-engine"), 0755)
	// Create some other file, not hooks.yaml.
	os.WriteFile(filepath.Join(dir, ".ralph-engine", "config.yaml"), []byte("key: val"), 0644)

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("missing hooks.yaml should not error, got: %v", err)
	}
	if cfg != nil {
		t.Error("expected nil config when hooks.yaml doesn't exist")
	}
}
