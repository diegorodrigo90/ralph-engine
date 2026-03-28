//go:build !windows

package hooks

import (
	"os/exec"
	"syscall"
)

// setSysProcAttr configures the command to create a new process group.
func setSysProcAttr(cmd *exec.Cmd) {
	cmd.SysProcAttr = &syscall.SysProcAttr{Setpgid: true}
}

// killProcessGroup sends SIGKILL to the entire process group,
// ensuring child processes (like sleep) are also killed on timeout.
func killProcessGroup(cmd *exec.Cmd) {
	if cmd.Process != nil {
		syscall.Kill(-cmd.Process.Pid, syscall.SIGKILL)
	}
}
