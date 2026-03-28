//go:build !windows

package system

import (
	"os"
	"syscall"
)

const bytesPerGB = 1024 * 1024 * 1024

// readDiskInfo uses syscall.Statfs to get disk usage on Unix systems.
func readDiskInfo(snap *ResourceSnapshot) error {
	wd, err := os.Getwd()
	if err != nil {
		wd = "/"
	}

	var stat syscall.Statfs_t
	if err := syscall.Statfs(wd, &stat); err != nil {
		return err
	}

	// Use int64 arithmetic to avoid uint64 overflow on large filesystems.
	bsize := int64(stat.Bsize)
	snap.TotalDiskGB = int(int64(stat.Blocks) * bsize / bytesPerGB)
	snap.FreeDiskGB = int(int64(stat.Bavail) * bsize / bytesPerGB)
	return nil
}
