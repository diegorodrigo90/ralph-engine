package system

import (
	"testing"
)

func TestNewResourceMonitorReturnsDefaults(t *testing.T) {
	rm := NewResourceMonitor(ResourceThresholds{})

	if rm == nil {
		t.Fatal("NewResourceMonitor() returned nil")
	}
	if rm.thresholds.MinFreeRAMMB != DefaultMinFreeRAMMB {
		t.Errorf("MinFreeRAMMB = %d, want %d", rm.thresholds.MinFreeRAMMB, DefaultMinFreeRAMMB)
	}
	if rm.thresholds.CriticalRAMMB != DefaultCriticalRAMMB {
		t.Errorf("CriticalRAMMB = %d, want %d", rm.thresholds.CriticalRAMMB, DefaultCriticalRAMMB)
	}
	if rm.thresholds.MaxCPUPercent != DefaultMaxCPUPercent {
		t.Errorf("MaxCPUPercent = %d, want %d", rm.thresholds.MaxCPUPercent, DefaultMaxCPUPercent)
	}
	if rm.thresholds.MinFreeDiskGB != DefaultMinFreeDiskGB {
		t.Errorf("MinFreeDiskGB = %d, want %d", rm.thresholds.MinFreeDiskGB, DefaultMinFreeDiskGB)
	}
	if rm.thresholds.CriticalDiskGB != DefaultCriticalDiskGB {
		t.Errorf("CriticalDiskGB = %d, want %d", rm.thresholds.CriticalDiskGB, DefaultCriticalDiskGB)
	}
}

func TestNewResourceMonitorRespectsCustomThresholds(t *testing.T) {
	custom := ResourceThresholds{
		MinFreeRAMMB:   4096,
		CriticalRAMMB:  2048,
		MaxCPUPercent:  90,
		MinFreeDiskGB:  10,
		CriticalDiskGB: 5,
	}
	rm := NewResourceMonitor(custom)

	if rm.thresholds.MinFreeRAMMB != 4096 {
		t.Errorf("MinFreeRAMMB = %d, want 4096", rm.thresholds.MinFreeRAMMB)
	}
	if rm.thresholds.CriticalRAMMB != 2048 {
		t.Errorf("CriticalRAMMB = %d, want 2048", rm.thresholds.CriticalRAMMB)
	}
	if rm.thresholds.MaxCPUPercent != 90 {
		t.Errorf("MaxCPUPercent = %d, want 90", rm.thresholds.MaxCPUPercent)
	}
}

func TestEvaluateHealthyResources(t *testing.T) {
	rm := NewResourceMonitor(ResourceThresholds{})
	snapshot := ResourceSnapshot{
		FreeRAMMB:      8192,
		TotalRAMMB:     16384,
		CPUPercent:     30.0,
		FreeDiskGB:     50,
		TotalDiskGB:    100,
		NumCPU:         8,
		LoadAvg1Min:    1.5,
	}

	status := rm.Evaluate(snapshot)

	if status.Level != LevelOK {
		t.Errorf("Level = %q, want %q for healthy resources", status.Level, LevelOK)
	}
	if status.ShouldStop {
		t.Error("ShouldStop should be false for healthy resources")
	}
	if status.ShouldPause {
		t.Error("ShouldPause should be false for healthy resources")
	}
}

func TestEvaluateLowRAMPauses(t *testing.T) {
	rm := NewResourceMonitor(ResourceThresholds{
		MinFreeRAMMB:  2048,
		CriticalRAMMB: 1024,
	})
	snapshot := ResourceSnapshot{
		FreeRAMMB:   1500, // Below MinFreeRAMMB but above CriticalRAMMB
		TotalRAMMB:  16384,
		CPUPercent:  30.0,
		FreeDiskGB:  50,
		TotalDiskGB: 100,
		NumCPU:      8,
	}

	status := rm.Evaluate(snapshot)

	if !status.ShouldPause {
		t.Error("ShouldPause should be true when RAM below MinFreeRAMMB")
	}
	if status.ShouldStop {
		t.Error("ShouldStop should be false when RAM above CriticalRAMMB")
	}
	if status.Level != LevelWarning {
		t.Errorf("Level = %q, want %q", status.Level, LevelWarning)
	}
}

func TestEvaluateCriticalRAMStops(t *testing.T) {
	rm := NewResourceMonitor(ResourceThresholds{
		CriticalRAMMB: 1024,
	})
	snapshot := ResourceSnapshot{
		FreeRAMMB:   512, // Below CriticalRAMMB
		TotalRAMMB:  16384,
		CPUPercent:  30.0,
		FreeDiskGB:  50,
		TotalDiskGB: 100,
		NumCPU:      8,
	}

	status := rm.Evaluate(snapshot)

	if !status.ShouldStop {
		t.Error("ShouldStop should be true when RAM below CriticalRAMMB")
	}
	if status.Level != LevelCritical {
		t.Errorf("Level = %q, want %q", status.Level, LevelCritical)
	}
}

func TestEvaluateHighCPUPauses(t *testing.T) {
	rm := NewResourceMonitor(ResourceThresholds{
		MaxCPUPercent: 80,
	})
	snapshot := ResourceSnapshot{
		FreeRAMMB:   8192,
		TotalRAMMB:  16384,
		CPUPercent:  85.0, // Above MaxCPUPercent
		FreeDiskGB:  50,
		TotalDiskGB: 100,
		NumCPU:      8,
	}

	status := rm.Evaluate(snapshot)

	if !status.ShouldPause {
		t.Error("ShouldPause should be true when CPU above MaxCPUPercent")
	}
	if status.Level != LevelWarning {
		t.Errorf("Level = %q, want %q", status.Level, LevelWarning)
	}
}

func TestEvaluateLowDiskWarns(t *testing.T) {
	rm := NewResourceMonitor(ResourceThresholds{
		MinFreeDiskGB:  5,
		CriticalDiskGB: 2,
	})
	snapshot := ResourceSnapshot{
		FreeRAMMB:   8192,
		TotalRAMMB:  16384,
		CPUPercent:  30.0,
		FreeDiskGB:  3, // Below MinFreeDiskGB but above CriticalDiskGB
		TotalDiskGB: 100,
		NumCPU:      8,
	}

	status := rm.Evaluate(snapshot)

	if !status.ShouldPause {
		t.Error("ShouldPause should be true when disk below MinFreeDiskGB")
	}
	if status.ShouldStop {
		t.Error("ShouldStop should be false when disk above CriticalDiskGB")
	}
}

func TestEvaluateCriticalDiskStops(t *testing.T) {
	rm := NewResourceMonitor(ResourceThresholds{
		CriticalDiskGB: 2,
	})
	snapshot := ResourceSnapshot{
		FreeRAMMB:   8192,
		TotalRAMMB:  16384,
		CPUPercent:  30.0,
		FreeDiskGB:  1, // Below CriticalDiskGB
		TotalDiskGB: 100,
		NumCPU:      8,
	}

	status := rm.Evaluate(snapshot)

	if !status.ShouldStop {
		t.Error("ShouldStop should be true when disk below CriticalDiskGB")
	}
	if status.Level != LevelCritical {
		t.Errorf("Level = %q, want %q", status.Level, LevelCritical)
	}
}

func TestEvaluateMultipleIssuesReportsAll(t *testing.T) {
	rm := NewResourceMonitor(ResourceThresholds{
		CriticalRAMMB:  1024,
		CriticalDiskGB: 2,
	})
	snapshot := ResourceSnapshot{
		FreeRAMMB:   512,  // Critical RAM
		TotalRAMMB:  16384,
		CPUPercent:  90.0, // High CPU
		FreeDiskGB:  1,    // Critical disk
		TotalDiskGB: 100,
		NumCPU:      8,
	}

	status := rm.Evaluate(snapshot)

	if !status.ShouldStop {
		t.Error("ShouldStop should be true with multiple critical issues")
	}
	if len(status.Messages) < 2 {
		t.Errorf("Messages count = %d, want ≥2 for multiple issues", len(status.Messages))
	}
}

func TestCheckReturnsRealSnapshot(t *testing.T) {
	rm := NewResourceMonitor(ResourceThresholds{})

	snapshot, err := rm.Check()
	if err != nil {
		t.Fatalf("Check() error: %v", err)
	}

	// Real machine should have > 0 for all values
	if snapshot.TotalRAMMB == 0 {
		t.Error("TotalRAMMB should be > 0")
	}
	if snapshot.NumCPU == 0 {
		t.Error("NumCPU should be > 0")
	}
	if snapshot.TotalDiskGB == 0 {
		t.Error("TotalDiskGB should be > 0")
	}
}

func TestStatusSummaryReturnsHumanReadable(t *testing.T) {
	status := ResourceStatus{
		Level:       LevelWarning,
		ShouldPause: true,
		Messages:    []string{"RAM low: 1500 MB free (min: 2048 MB)"},
	}

	summary := status.Summary()
	if summary == "" {
		t.Error("Summary() should return non-empty string")
	}
}
