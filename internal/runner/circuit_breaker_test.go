package runner

import (
	"errors"
	"testing"
	"time"
)

func TestNewCircuitBreakerStartsClosed(t *testing.T) {
	cb := NewCircuitBreaker(3, 5)

	if cb.State() != BreakerClosed {
		t.Errorf("State() = %q, want %q", cb.State(), BreakerClosed)
	}
	if cb.IsOpen() {
		t.Error("IsOpen() should be false for new breaker")
	}
	if cb.ConsecutiveFailures() != 0 {
		t.Errorf("ConsecutiveFailures() = %d, want 0", cb.ConsecutiveFailures())
	}
}

func TestRecordSuccessResetFailures(t *testing.T) {
	cb := NewCircuitBreaker(3, 5)
	cb.RecordFailure(errors.New("test error"))
	cb.RecordFailure(errors.New("test error"))

	if cb.ConsecutiveFailures() != 2 {
		t.Fatalf("ConsecutiveFailures() = %d, want 2", cb.ConsecutiveFailures())
	}

	cb.RecordSuccess(1)

	if cb.ConsecutiveFailures() != 0 {
		t.Errorf("ConsecutiveFailures() = %d, want 0 after success", cb.ConsecutiveFailures())
	}
	if cb.State() != BreakerClosed {
		t.Errorf("State() = %q, want %q after success", cb.State(), BreakerClosed)
	}
}

func TestRecordSuccessWithZeroStoriesDoesNotReset(t *testing.T) {
	cb := NewCircuitBreaker(3, 5)
	cb.RecordFailure(errors.New("no progress"))

	cb.RecordSuccess(0)

	if cb.ConsecutiveFailures() != 1 {
		t.Errorf("ConsecutiveFailures() = %d, want 1 (zero stories should not reset)", cb.ConsecutiveFailures())
	}
}

func TestTripsAfterMaxFailures(t *testing.T) {
	cb := NewCircuitBreaker(3, 5)

	cb.RecordFailure(errors.New("error 1"))
	if cb.IsOpen() {
		t.Error("Should not trip after 1 failure")
	}

	cb.RecordFailure(errors.New("error 2"))
	if cb.State() != BreakerHalfOpen {
		t.Errorf("State() = %q, want %q after max-1 failures", cb.State(), BreakerHalfOpen)
	}

	msg := cb.RecordFailure(errors.New("error 3"))
	if !cb.IsOpen() {
		t.Error("Should trip after 3 failures")
	}
	if msg == "" {
		t.Error("RecordFailure() should return message when breaker trips")
	}
}

func TestAutoRecoveryAfterCooldown(t *testing.T) {
	cb := NewCircuitBreaker(3, 5)

	// Trip the breaker
	cb.RecordFailure(errors.New("error"))
	cb.RecordFailure(errors.New("error"))
	cb.RecordFailure(errors.New("error"))

	if cb.State() != BreakerOpen {
		t.Fatalf("State() = %q, want %q", cb.State(), BreakerOpen)
	}

	// Simulate cooldown elapsed by backdating lastFailureTime
	cb.lastFailureTime = time.Now().Add(-10 * time.Minute)

	if cb.State() != BreakerHalfOpen {
		t.Errorf("State() = %q, want %q after cooldown elapsed", cb.State(), BreakerHalfOpen)
	}
}

func TestResetForcesClosedState(t *testing.T) {
	cb := NewCircuitBreaker(3, 60)

	// Trip the breaker
	cb.RecordFailure(errors.New("error"))
	cb.RecordFailure(errors.New("error"))
	cb.RecordFailure(errors.New("error"))

	cb.Reset()

	if cb.State() != BreakerClosed {
		t.Errorf("State() = %q, want %q after reset", cb.State(), BreakerClosed)
	}
	if cb.ConsecutiveFailures() != 0 {
		t.Errorf("ConsecutiveFailures() = %d, want 0 after reset", cb.ConsecutiveFailures())
	}
}

func TestRecordFailureReturnsEmptyBeforeTrip(t *testing.T) {
	cb := NewCircuitBreaker(3, 5)

	msg := cb.RecordFailure(errors.New("first error"))
	if msg != "" {
		t.Errorf("RecordFailure() should return empty before trip, got %q", msg)
	}
}
