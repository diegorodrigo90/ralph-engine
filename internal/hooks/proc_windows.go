//go:build windows

package hooks

import "os/exec"

// setSysProcAttr is a no-op on Windows.
func setSysProcAttr(cmd *exec.Cmd) {}

// killProcessGroup is a no-op on Windows.
func killProcessGroup(cmd *exec.Cmd) {}
