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
