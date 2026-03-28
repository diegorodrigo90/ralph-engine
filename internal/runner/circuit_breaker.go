// Package runner implements the autonomous sprint execution loop.
// It orchestrates AI agent sessions, manages circuit breaker state,
// and ensures graceful shutdown with progress preservation.
package runner

import (
	"fmt"
	"time"
)

// BreakerState represents the circuit breaker state.
type BreakerState string

const (
	BreakerClosed   BreakerState = "closed"
	BreakerHalfOpen BreakerState = "half_open"
	BreakerOpen     BreakerState = "open"
)

// CircuitBreaker prevents runaway token consumption when the AI agent
// gets stuck in a loop without making progress. Uses Michael Nygard's
// Release It! pattern with three states: closed, half-open, open.
type CircuitBreaker struct {
	state               BreakerState
	consecutiveFailures int
	maxFailures         int
	cooldownDuration    time.Duration
	lastFailureTime     time.Time
	lastError           string
}

// NewCircuitBreaker creates a circuit breaker with configurable thresholds.
func NewCircuitBreaker(maxFailures int, cooldownMinutes int) *CircuitBreaker {
	return &CircuitBreaker{
		state:            BreakerClosed,
		maxFailures:      maxFailures,
		cooldownDuration: time.Duration(cooldownMinutes) * time.Minute,
	}
}

// State returns the current breaker state.
func (cb *CircuitBreaker) State() BreakerState {
	// Auto-transition from open to half-open after cooldown
	if cb.state == BreakerOpen && time.Since(cb.lastFailureTime) > cb.cooldownDuration {
		cb.state = BreakerHalfOpen
	}
	return cb.state
}

// IsOpen returns true if the circuit breaker has tripped.
func (cb *CircuitBreaker) IsOpen() bool {
	return cb.State() == BreakerOpen
}

// RecordSuccess resets the failure counter and closes the circuit.
func (cb *CircuitBreaker) RecordSuccess(storiesCompleted int) {
	if storiesCompleted > 0 {
		cb.consecutiveFailures = 0
		cb.state = BreakerClosed
		cb.lastError = ""
	}
}

// RecordFailure increments the failure counter and may trip the breaker.
// Returns an error message if the breaker trips.
func (cb *CircuitBreaker) RecordFailure(err error) string {
	cb.consecutiveFailures++
	cb.lastFailureTime = time.Now()
	cb.lastError = err.Error()

	if cb.consecutiveFailures >= cb.maxFailures {
		cb.state = BreakerOpen
		return fmt.Sprintf(
			"circuit breaker OPEN: %d consecutive failures (max: %d). Last error: %s. Cooldown: %s",
			cb.consecutiveFailures, cb.maxFailures, cb.lastError, cb.cooldownDuration,
		)
	}

	if cb.consecutiveFailures >= cb.maxFailures-1 {
		cb.state = BreakerHalfOpen
	}

	return ""
}

// ConsecutiveFailures returns the current failure count.
func (cb *CircuitBreaker) ConsecutiveFailures() int {
	return cb.consecutiveFailures
}

// Reset forces the circuit breaker back to closed state.
func (cb *CircuitBreaker) Reset() {
	cb.state = BreakerClosed
	cb.consecutiveFailures = 0
	cb.lastError = ""
}
