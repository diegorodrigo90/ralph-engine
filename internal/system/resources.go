// Package system provides host resource monitoring to prevent the engine
// from freezing the host machine during autonomous execution loops.
// It checks RAM, CPU, and disk space against configurable thresholds.
package system

import (
	"fmt"
	"os"
	"runtime"
	"strings"
)

// Default resource thresholds for safe operation.
const (
	DefaultMinFreeRAMMB   = 2048 // 2 GB — pause engine
	DefaultCriticalRAMMB  = 1024 // 1 GB — stop engine
	DefaultMaxCPUPercent  = 80   // % of cores — pause engine
	DefaultMinFreeDiskGB  = 5    // 5 GB — warn user
	DefaultCriticalDiskGB = 2    // 2 GB — stop engine
)

// StatusLevel indicates the severity of resource status.
type StatusLevel string

const (
	LevelOK       StatusLevel = "ok"
	LevelWarning  StatusLevel = "warning"
	LevelCritical StatusLevel = "critical"
)

// ResourceThresholds defines configurable limits for host resource usage.
// Zero values are replaced with defaults.
type ResourceThresholds struct {
	MinFreeRAMMB   int
	CriticalRAMMB  int
	MaxCPUPercent  int
	MinFreeDiskGB  int
	CriticalDiskGB int
}

// ResourceSnapshot holds a point-in-time reading of host resources.
type ResourceSnapshot struct {
	FreeRAMMB   int
	TotalRAMMB  int
	CPUPercent  float64
	FreeDiskGB  int
	TotalDiskGB int
	NumCPU      int
	LoadAvg1Min float64
}

// ResourceStatus is the result of evaluating a snapshot against thresholds.
type ResourceStatus struct {
	Level       StatusLevel
	ShouldPause bool
	ShouldStop  bool
	Messages    []string
}

// Summary returns a human-readable status string.
func (rs ResourceStatus) Summary() string {
	if len(rs.Messages) == 0 {
		return fmt.Sprintf("[%s] All resources healthy", rs.Level)
	}
	return fmt.Sprintf("[%s] %s", rs.Level, strings.Join(rs.Messages, "; "))
}

// ResourceMonitor checks host resources against thresholds.
type ResourceMonitor struct {
	thresholds ResourceThresholds
}

// NewResourceMonitor creates a monitor with the given thresholds.
// Zero-value fields in thresholds are replaced with safe defaults.
func NewResourceMonitor(thresholds ResourceThresholds) *ResourceMonitor {
	if thresholds.MinFreeRAMMB == 0 {
		thresholds.MinFreeRAMMB = DefaultMinFreeRAMMB
	}
	if thresholds.CriticalRAMMB == 0 {
		thresholds.CriticalRAMMB = DefaultCriticalRAMMB
	}
	if thresholds.MaxCPUPercent == 0 {
		thresholds.MaxCPUPercent = DefaultMaxCPUPercent
	}
	if thresholds.MinFreeDiskGB == 0 {
		thresholds.MinFreeDiskGB = DefaultMinFreeDiskGB
	}
	if thresholds.CriticalDiskGB == 0 {
		thresholds.CriticalDiskGB = DefaultCriticalDiskGB
	}
	return &ResourceMonitor{thresholds: thresholds}
}

// Evaluate checks a resource snapshot against the configured thresholds.
// Returns a status indicating whether the engine should pause or stop.
func (rm *ResourceMonitor) Evaluate(snap ResourceSnapshot) ResourceStatus {
	status := ResourceStatus{Level: LevelOK}

	// RAM checks
	if snap.FreeRAMMB < rm.thresholds.CriticalRAMMB {
		status.ShouldStop = true
		status.Level = LevelCritical
		status.Messages = append(status.Messages,
			fmt.Sprintf("RAM critical: %d MB free (min: %d MB)", snap.FreeRAMMB, rm.thresholds.CriticalRAMMB))
	} else if snap.FreeRAMMB < rm.thresholds.MinFreeRAMMB {
		status.ShouldPause = true
		if status.Level != LevelCritical {
			status.Level = LevelWarning
		}
		status.Messages = append(status.Messages,
			fmt.Sprintf("RAM low: %d MB free (min: %d MB)", snap.FreeRAMMB, rm.thresholds.MinFreeRAMMB))
	}

	// CPU checks
	if snap.CPUPercent > float64(rm.thresholds.MaxCPUPercent) {
		status.ShouldPause = true
		if status.Level != LevelCritical {
			status.Level = LevelWarning
		}
		status.Messages = append(status.Messages,
			fmt.Sprintf("CPU high: %.1f%% (max: %d%%)", snap.CPUPercent, rm.thresholds.MaxCPUPercent))
	}

	// Disk checks
	if snap.FreeDiskGB < rm.thresholds.CriticalDiskGB {
		status.ShouldStop = true
		status.Level = LevelCritical
		status.Messages = append(status.Messages,
			fmt.Sprintf("disk critical: %d GB free (min: %d GB)", snap.FreeDiskGB, rm.thresholds.CriticalDiskGB))
	} else if snap.FreeDiskGB < rm.thresholds.MinFreeDiskGB {
		status.ShouldPause = true
		if status.Level != LevelCritical {
			status.Level = LevelWarning
		}
		status.Messages = append(status.Messages,
			fmt.Sprintf("disk low: %d GB free (min: %d GB)", snap.FreeDiskGB, rm.thresholds.MinFreeDiskGB))
	}

	return status
}

// Check reads current host resources and returns a snapshot.
// Uses /proc/meminfo for RAM on Linux, syscall.Statfs for disk,
// and runtime.NumCPU for CPU count.
func (rm *ResourceMonitor) Check() (ResourceSnapshot, error) {
	snap := ResourceSnapshot{
		NumCPU: runtime.NumCPU(),
	}

	// RAM: read from /proc/meminfo (Linux) or fall back to runtime
	if err := readMemInfo(&snap); err != nil {
		// Fallback: use Go runtime stats (less accurate but cross-platform)
		var memStats runtime.MemStats
		runtime.ReadMemStats(&memStats)
		snap.TotalRAMMB = int(memStats.Sys / (1024 * 1024))
		snap.FreeRAMMB = snap.TotalRAMMB / 2 // Conservative estimate
	}

	// Disk: use syscall.Statfs on current working directory
	if err := readDiskInfo(&snap); err != nil {
		return snap, fmt.Errorf("reading disk info: %w", err)
	}

	return snap, nil
}

// readMemInfo parses /proc/meminfo for accurate RAM data on Linux.
func readMemInfo(snap *ResourceSnapshot) error {
	data, err := os.ReadFile("/proc/meminfo")
	if err != nil {
		return err
	}

	var totalKB, freeKB, availableKB, buffersKB, cachedKB int
	for _, line := range strings.Split(string(data), "\n") {
		switch {
		case strings.HasPrefix(line, "MemTotal:"):
			fmt.Sscanf(line, "MemTotal: %d kB", &totalKB)
		case strings.HasPrefix(line, "MemFree:"):
			fmt.Sscanf(line, "MemFree: %d kB", &freeKB)
		case strings.HasPrefix(line, "MemAvailable:"):
			fmt.Sscanf(line, "MemAvailable: %d kB", &availableKB)
		case strings.HasPrefix(line, "Buffers:"):
			fmt.Sscanf(line, "Buffers: %d kB", &buffersKB)
		case strings.HasPrefix(line, "Cached:"):
			fmt.Sscanf(line, "Cached: %d kB", &cachedKB)
		}
	}

	snap.TotalRAMMB = totalKB / 1024
	// Prefer MemAvailable (includes reclaimable), fall back to free+buffers+cached
	if availableKB > 0 {
		snap.FreeRAMMB = availableKB / 1024
	} else {
		snap.FreeRAMMB = (freeKB + buffersKB + cachedKB) / 1024
	}
	return nil
}

// readDiskInfo is implemented in platform-specific files:
// disk_unix.go (linux, darwin) and disk_windows.go
