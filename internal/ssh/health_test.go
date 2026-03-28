package ssh

import (
	"context"
	"testing"
	"time"
)

func TestNewHealthCheckerDefaults(t *testing.T) {
	hc := NewHealthChecker(HealthConfig{})

	if hc.config.Host != DefaultHost {
		t.Errorf("Host = %q, want %q", hc.config.Host, DefaultHost)
	}
	if hc.config.Port != DefaultPort {
		t.Errorf("Port = %d, want %d", hc.config.Port, DefaultPort)
	}
	if hc.config.Timeout != DefaultTimeout {
		t.Errorf("Timeout = %v, want %v", hc.config.Timeout, DefaultTimeout)
	}
	if hc.config.MaxRetries != DefaultMaxRetries {
		t.Errorf("MaxRetries = %d, want %d", hc.config.MaxRetries, DefaultMaxRetries)
	}
}

func TestNewHealthCheckerCustomConfig(t *testing.T) {
	hc := NewHealthChecker(HealthConfig{
		Host:       "192.168.1.100",
		Port:       2222,
		Timeout:    10 * time.Second,
		MaxRetries: 5,
	})

	if hc.config.Host != "192.168.1.100" {
		t.Errorf("Host = %q, want %q", hc.config.Host, "192.168.1.100")
	}
	if hc.config.Port != 2222 {
		t.Errorf("Port = %d, want 2222", hc.config.Port)
	}
}

func TestHealthStatusConstants(t *testing.T) {
	tests := []struct {
		status HealthStatus
		want   string
	}{
		{StatusHealthy, "healthy"},
		{StatusUnhealthy, "unhealthy"},
		{StatusReconnecting, "reconnecting"},
		{StatusUnavailable, "unavailable"},
	}
	for _, tt := range tests {
		if string(tt.status) != tt.want {
			t.Errorf("HealthStatus %q != %q", tt.status, tt.want)
		}
	}
}

func TestBuildSSHCommand(t *testing.T) {
	hc := NewHealthChecker(HealthConfig{
		Host: "devcontainer",
		Port: 22,
	})

	args := hc.buildExecArgs("echo ok")

	assertContains(t, args, "-o")
	assertContains(t, args, "ConnectTimeout=5")
	assertContains(t, args, "devcontainer")
	assertContains(t, args, "echo ok")
}

func TestBuildSSHCommandWithUser(t *testing.T) {
	hc := NewHealthChecker(HealthConfig{
		Host: "devcontainer",
		User: "node",
	})

	args := hc.buildExecArgs("echo ok")

	found := false
	for _, a := range args {
		if a == "node@devcontainer" {
			found = true
			break
		}
	}
	if !found {
		t.Errorf("args should contain user@host, got %v", args)
	}
}

func TestBuildSSHCommandWithPort(t *testing.T) {
	hc := NewHealthChecker(HealthConfig{
		Host: "devcontainer",
		Port: 2222,
	})

	args := hc.buildExecArgs("echo ok")
	assertContains(t, args, "-p")
	assertContains(t, args, "2222")
}

func TestCheckHealthTimesOutOnBadHost(t *testing.T) {
	hc := NewHealthChecker(HealthConfig{
		Host:    "nonexistent-host-12345.invalid",
		Port:    22,
		Timeout: 1 * time.Second,
	})

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	result := hc.Check(ctx)

	if result.Status == StatusHealthy {
		t.Error("Check() should not be healthy for nonexistent host")
	}
	if result.Error == "" {
		t.Error("Check() should have an error message")
	}
}

func TestHealthResultIsOK(t *testing.T) {
	healthy := HealthResult{Status: StatusHealthy}
	unhealthy := HealthResult{Status: StatusUnhealthy, Error: "connection refused"}

	if !healthy.IsOK() {
		t.Error("healthy result should be OK")
	}
	if unhealthy.IsOK() {
		t.Error("unhealthy result should not be OK")
	}
}

func TestReconnectScriptPath(t *testing.T) {
	hc := NewHealthChecker(HealthConfig{
		ReconnectScript: "/workspace/scripts/claude-dev.sh",
	})

	if hc.config.ReconnectScript != "/workspace/scripts/claude-dev.sh" {
		t.Errorf("ReconnectScript = %q, want /workspace/scripts/claude-dev.sh",
			hc.config.ReconnectScript)
	}
}

func TestRemoteExecBuildsCorrectCommand(t *testing.T) {
	hc := NewHealthChecker(HealthConfig{
		ExecScript: "./scripts/dev-exec.sh",
	})

	args := hc.buildRemoteExecArgs("pnpm test")

	assertContains(t, args, "./scripts/dev-exec.sh")
	assertContains(t, args, "pnpm test")
}

func TestRemoteExecFallsBackToSSH(t *testing.T) {
	hc := NewHealthChecker(HealthConfig{
		Host: "devcontainer",
	})

	// No ExecScript — should use SSH directly
	args := hc.buildRemoteExecArgs("pnpm test")

	assertContains(t, args, "ssh")
	assertContains(t, args, "devcontainer")
	assertContains(t, args, "pnpm test")
}

func assertContains(t *testing.T, haystack []string, needle string) {
	t.Helper()
	for _, s := range haystack {
		if s == needle {
			return
		}
	}
	t.Errorf("args %v does not contain %q", haystack, needle)
}
