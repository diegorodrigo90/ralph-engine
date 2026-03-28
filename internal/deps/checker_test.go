package deps

import (
	"testing"
)

func TestCheckFindsExistingBinary(t *testing.T) {
	// "sh" should exist on any Unix system
	result := CheckBinary("git")
	if !result.Found {
		t.Error("CheckBinary('git') should find sh")
	}
	if result.Path == "" {
		t.Error("Path should be non-empty when found")
	}
}

func TestCheckReportsMissingBinary(t *testing.T) {
	result := CheckBinary("nonexistent-binary-xyz-12345")
	if result.Found {
		t.Error("CheckBinary should not find nonexistent binary")
	}
	if result.Path != "" {
		t.Error("Path should be empty when not found")
	}
}

func TestCheckBinaryHasName(t *testing.T) {
	result := CheckBinary("git")
	if result.Name != "git" {
		t.Errorf("Name = %q, want %q", result.Name, "git")
	}
}

func TestCheckAllReturnsMultipleResults(t *testing.T) {
	deps := []Dependency{
		{Name: "git", Required: true, InstallHint: "install git"},
		{Name: "nonexistent-xyz-12345", Required: true, InstallHint: "install xyz"},
		{Name: "git", Required: false, InstallHint: "install git"},
	}

	results := CheckAll(deps)

	if len(results) != 3 {
		t.Fatalf("CheckAll() = %d results, want 3", len(results))
	}

	// git should be found.
	if !results[0].Found {
		t.Error("git should be found")
	}
	// nonexistent should not be found
	if results[1].Found {
		t.Error("nonexistent should not be found")
	}
}

func TestCheckAllReportsRequiredMissing(t *testing.T) {
	deps := []Dependency{
		{Name: "git", Required: true},
		{Name: "nonexistent-xyz-12345", Required: true, InstallHint: "install it"},
	}

	results := CheckAll(deps)
	missing := MissingRequired(results)

	if len(missing) != 1 {
		t.Errorf("MissingRequired() = %d, want 1", len(missing))
	}
	if len(missing) > 0 && missing[0].Name != "nonexistent-xyz-12345" {
		t.Errorf("missing = %q, want nonexistent-xyz-12345", missing[0].Name)
	}
}

func TestCheckAllOptionalMissingNotBlocking(t *testing.T) {
	deps := []Dependency{
		{Name: "nonexistent-xyz-12345", Required: false, InstallHint: "optional"},
	}

	results := CheckAll(deps)
	missing := MissingRequired(results)

	if len(missing) != 0 {
		t.Errorf("MissingRequired() = %d, want 0 (optional deps not blocking)", len(missing))
	}
}

func TestDefaultDepsIncludesAgent(t *testing.T) {
	deps := DefaultDeps("claude")

	found := false
	for _, d := range deps {
		if d.Name == "claude" {
			found = true
			break
		}
	}
	if !found {
		t.Error("DefaultDeps should include the agent binary")
	}
}

func TestDefaultDepsGit(t *testing.T) {
	deps := DefaultDeps("claude")

	found := false
	for _, d := range deps {
		if d.Name == "git" && d.Required {
			found = true
			break
		}
	}
	if !found {
		t.Error("DefaultDeps should include git as required")
	}
}
