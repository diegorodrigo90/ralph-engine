//go:build windows

package system

import (
	"os"
	"syscall"
	"unsafe"
)

// readDiskInfo uses GetDiskFreeSpaceEx on Windows.
func readDiskInfo(snap *ResourceSnapshot) error {
	wd, err := os.Getwd()
	if err != nil {
		wd = "C:\\"
	}

	kernel32 := syscall.NewLazyDLL("kernel32.dll")
	getDiskFreeSpace := kernel32.NewProc("GetDiskFreeSpaceExW")

	var freeBytesAvailable, totalBytes, totalFreeBytes int64
	pathPtr, _ := syscall.UTF16PtrFromString(wd)

	ret, _, _ := getDiskFreeSpace.Call(
		uintptr(unsafe.Pointer(pathPtr)),
		uintptr(unsafe.Pointer(&freeBytesAvailable)),
		uintptr(unsafe.Pointer(&totalBytes)),
		uintptr(unsafe.Pointer(&totalFreeBytes)),
	)
	if ret == 0 {
		// Fallback: assume 100GB total, 50GB free
		snap.TotalDiskGB = 100
		snap.FreeDiskGB = 50
		return nil
	}

	snap.TotalDiskGB = int(totalBytes / (1024 * 1024 * 1024))
	snap.FreeDiskGB = int(freeBytesAvailable / (1024 * 1024 * 1024))
	return nil
}
