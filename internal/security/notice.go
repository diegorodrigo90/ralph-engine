// Package security implements the first-run security notice and acceptance
// system. Users must explicitly acknowledge the security implications of
// running an autonomous AI agent with elevated permissions.
package security

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"
)

const acceptedFile = "security-accepted"

// Notice manages the security acceptance state.
type Notice struct {
	configDir string
	accepted  bool
}

// NewNotice creates a security notice checker. It reads acceptance state
// from the config directory (persisted across sessions).
func NewNotice(configDir string) *Notice {
	n := &Notice{configDir: configDir}
	n.accepted = n.readAccepted()
	return n
}

// IsAccepted returns true if the user has accepted the security notice.
func (n *Notice) IsAccepted() bool {
	return n.accepted
}

// Accept marks the security notice as accepted and persists to disk.
// This is a one-time action — the user is never prompted again.
func (n *Notice) Accept() error {
	if err := os.MkdirAll(n.configDir, 0750); err != nil {
		return fmt.Errorf("creating config dir: %w", err)
	}

	content := fmt.Sprintf("accepted=%s\n", time.Now().UTC().Format(time.RFC3339))
	path := filepath.Join(n.configDir, acceptedFile)
	if err := os.WriteFile(path, []byte(content), 0600); err != nil {
		return fmt.Errorf("writing security acceptance: %w", err)
	}

	n.accepted = true
	return nil
}

// Validate checks if the security notice needs to be shown.
// Returns an error if skip-permissions is requested but notice not accepted.
func (n *Notice) Validate(skipPermissions bool) error {
	if !skipPermissions {
		return nil // No elevated permissions, no notice needed
	}
	if n.accepted {
		return nil
	}
	return fmt.Errorf(
		"security notice not accepted.\n\n"+
			"You are about to run with --dangerously-skip-permissions.\n"+
			"Run 'ralph-engine run --accept-security' to acknowledge risks.\n\n%s",
		NoticeText())
}

// readAccepted validates that the acceptance file exists and contains valid content.
func (n *Notice) readAccepted() bool {
	path := filepath.Join(n.configDir, acceptedFile)
	data, err := os.ReadFile(path) // #nosec G304 -- path is configDir + known acceptedFile constant
	if err != nil {
		return false
	}
	// Validate file contains the acceptance marker, not just any file.
	return strings.Contains(string(data), "accepted=")
}

// NoticeText returns the full security warning text.
func NoticeText() string {
	return `SECURITY NOTICE — ralph-engine autonomous mode

ralph-engine runs AI agents (Claude, ClaudeBox, etc.) with the ability to:
  - Read, write, and delete files in your project directory
  - Execute arbitrary shell commands
  - Make network requests
  - Install packages and modify dependencies

When used with --dangerously-skip-permissions:
  - The AI agent runs WITHOUT permission prompts
  - ALL tool calls are auto-approved (file writes, shell commands, etc.)
  - This is POWERFUL but carries inherent risk

RECOMMENDATIONS:
  1. Run inside a container (ClaudeBox, Docker) for isolation
  2. Never run on production systems or with production credentials
  3. Review commits before pushing to shared branches
  4. Set resource limits to prevent runaway processes
  5. Use budget controls (daily-budget, max-cost-per-session)

BILLING:
  ralph-engine does NOT activate, manage, or interfere with API billing.
  You must manage your Claude usage limits externally (claude.ai, API dashboard).
  The engine only detects when limits are reached and saves progress.

By accepting, you acknowledge these risks and take responsibility
for the AI agent's actions in your environment.`
}
