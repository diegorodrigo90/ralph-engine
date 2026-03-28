package engine

import (
	"context"
	"errors"
	"fmt"
	"log"
	"strings"
	"time"

	"github.com/diegorodrigo90/ralph-engine/internal/claude"
	"github.com/diegorodrigo90/ralph-engine/internal/config"
	appcontext "github.com/diegorodrigo90/ralph-engine/internal/context"
	"github.com/diegorodrigo90/ralph-engine/internal/hooks"
	"github.com/diegorodrigo90/ralph-engine/internal/state"
	"github.com/diegorodrigo90/ralph-engine/internal/tracker"
)

// RunResult holds the outcome of an engine run.
type RunResult struct {
	ExitReason      ExitReason
	SessionsRun     int
	StoriesComplete int
	TotalCostUSD    float64
	Error           error
}

// EventHandler receives engine events for display (dashboard, logs).
type EventHandler func(event EngineEvent)

// EngineEvent describes something that happened during the loop.
type EngineEvent struct {
	Type    string
	Message string
	Data    map[string]interface{}
}

// Run executes the autonomous loop: pick story → call agent → check gates → repeat.
// Supports dry-run, max-iterations, and single-story modes for testing.
func (e *Engine) Run(ctx context.Context, tk tracker.TaskTracker, onEvent EventHandler) RunResult {
	e.setStatus(StatusRunning, "")

	engineState, err := state.Load(e.opts.StateDir)
	if err != nil {
		return RunResult{Error: fmt.Errorf("loading state: %w", err)}
	}

	emit := func(eventType, msg string) {
		if onEvent != nil {
			onEvent(EngineEvent{Type: eventType, Message: msg})
		}
	}

	// Report mode.
	mode := "loop"
	if e.opts.DryRun {
		mode = "dry-run"
	}
	if e.opts.MaxIterations > 0 {
		mode = fmt.Sprintf("max-%d-iterations", e.opts.MaxIterations)
	}
	if e.opts.SingleStory != "" {
		mode = fmt.Sprintf("single-story:%s", e.opts.SingleStory)
	}
	emit("info", fmt.Sprintf("Engine started [mode=%s, agent=%s, stories/session=%d]",
		mode, e.opts.Binary, e.opts.StoriesPerSession))

	// Dry-run: show plan and exit.
	if e.opts.DryRun {
		return e.dryRun(tk, emit, engineState)
	}

	// Run preflight hooks (before loop starts).
	if e.opts.Hooks != nil {
		if !e.runHookPhase(ctx, "preflight", e.opts.Hooks.Preflight, nil, emit) {
			return e.stop(ExitBlocked, engineState, emit, "Preflight hooks failed")
		}
	}

	// Use "json" format for maximum compatibility across agent wrappers.
	// "stream-json" requires --verbose which some wrappers (claudebox) consume.
	client := claude.NewClient(claude.ClientConfig{
		Binary:       e.opts.Binary,
		OutputFormat: "json",
	})

	iteration := 0
	for {
		// Max iterations guard.
		if e.opts.MaxIterations > 0 && iteration >= e.opts.MaxIterations {
			emit("info", fmt.Sprintf("Reached max iterations (%d). Stopping.", e.opts.MaxIterations))
			return e.stop(ExitAllComplete, engineState, emit, "")
		}
		iteration++

		if ctx.Err() != nil {
			return e.stop(ExitUserInterrupt, engineState, emit,
				"User interrupt received. Saving progress...")
		}

		if e.circuitBreaker.IsOpen() {
			engineState.Blocked = true
			engineState.BlockedReason = "circuit breaker tripped"
			return e.stop(ExitCircuitBreaker, engineState, emit,
				fmt.Sprintf("Circuit breaker OPEN after %d failures",
					e.circuitBreaker.ConsecutiveFailures()))
		}

		// Check system resources.
		if reason := e.checkResources(ctx, emit); reason != "" {
			return e.stop(ExitReason(reason), engineState, emit, "")
		}

		// Pick next story.
		story, err := e.pickStory(tk)
		if err != nil {
			e.circuitBreaker.RecordFailure(fmt.Errorf("tracker error: %w", err))
			emit("error", fmt.Sprintf("Tracker error: %v", err))
			continue
		}
		if story == nil {
			engineState.EngineStatus = state.StatusAllComplete
			return e.stop(ExitAllComplete, engineState, emit, "All stories complete!")
		}

		// Start session.
		e.mu.Lock()
		e.sessionCount++
		sessionNum := e.sessionCount
		e.mu.Unlock()

		engineState.SessionNumber = sessionNum
		engineState.CurrentStory = story.ID
		engineState.CurrentPhase = "implementation"
		engineState.Save(e.opts.StateDir)

		emit("session_start", fmt.Sprintf("Session #%d: Story %s — %s (Epic %s)",
			sessionNum, story.ID, story.Title, story.EpicID))

		if err := tk.MarkInProgress(story.ID); err != nil {
			log.Printf("Warning: could not mark story in-progress: %v", err)
		}

		// Run pre-story hooks.
		if e.opts.Hooks != nil {
			if !e.runHookPhase(ctx, "pre-story", e.opts.Hooks.PreStory, nil, emit) {
				e.circuitBreaker.RecordFailure(fmt.Errorf("pre-story hooks failed"))
				emit("error", "Pre-story hooks failed, skipping story")
				continue
			}
		}

		// Load story content from paths config or story.FilePath.
		storyContent := e.loadStoryContent(story)

		// Load prompt.md (project-specific context).
		promptMD := appcontext.LoadPromptMD(e.opts.ProjectDir)

		// Resolve workflow and quality gate from config (fallback to defaults).
		workflowType := "basic"
		if e.opts.WorkflowType != "" {
			workflowType = e.opts.WorkflowType
		}
		qualityGate := "standard"
		if e.opts.QualityGate != "" {
			qualityGate = e.opts.QualityGate
		}

		// Collect custom prompt sections from config.
		var sections []config.PromptSection
		if e.opts.Prompt != nil {
			sections = e.opts.Prompt.Sections
		}

		prompt := BuildPrompt(PromptContext{
			StoryID:       story.ID,
			StoryTitle:    story.Title,
			EpicID:        story.EpicID,
			EpicTitle:     story.EpicTitle,
			SessionNumber: sessionNum,
			StoriesDone:   engineState.StoriesCompletedTotal,
			StoriesTotal:  countTotal(tk),
			WorkflowType:  workflowType,
			QualityGate:   qualityGate,
			StoryContent:  storyContent,
			PromptMD:      promptMD,
			Sections:      sections,
			ProjectDir:    e.opts.ProjectDir,
			Research:      e.opts.Research,
		})

		// Build the user prompt — includes story title + file path if available.
		userPrompt := fmt.Sprintf("Implement story %s: %s", story.ID, story.Title)
		if story.FilePath != "" {
			userPrompt += fmt.Sprintf("\n\nStory file: %s", story.FilePath)
		}

		// Show progress during agent session.
		sessionStart := time.Now()
		emit("info", "Agent session started — waiting for completion...")

		sessionResult, err := client.Run(ctx, claude.SessionRequest{
			Prompt:       userPrompt,
			ProjectDir:   e.opts.ProjectDir,
			SystemPrompt: prompt,
		}, func(event claude.StreamEvent) {
			if onEvent != nil {
				onEvent(EngineEvent{Type: "stream", Message: event.Type})
			}
		})
		emit("info", fmt.Sprintf("Agent session finished (%v)", time.Since(sessionStart).Round(time.Second)))

		if err != nil {
			if errors.Is(err, context.Canceled) {
				continue
			}
			msg := e.circuitBreaker.RecordFailure(fmt.Errorf("session error: %w", err))
			emit("error", fmt.Sprintf("Session failed: %v", err))
			if msg != "" {
				emit("error", msg)
			}
			engineState.Save(e.opts.StateDir)
			continue
		}

		if sessionResult != nil && sessionResult.UsageLimit {
			engineState.EngineStatus = state.StatusSessionComplete
			return e.stop(ExitUsageLimit, engineState, emit,
				"Usage limit detected. Saving progress...")
		}

		if sessionResult != nil && sessionResult.CostUSD > 0 {
			engineState.TotalCostUSD += sessionResult.CostUSD
			engineState.SessionCostUSD += sessionResult.CostUSD
		}

		if sessionResult != nil && sessionResult.ExitCode == 0 {
			// Verify agent actually changed files — if not, session was empty.
			changedFiles := hooks.GetChangedFiles(e.opts.ProjectDir)
			if len(changedFiles) == 0 {
				emit("warn", fmt.Sprintf("Agent session for story %s produced no file changes — not marking complete", story.ID))
				msg := e.circuitBreaker.RecordFailure(fmt.Errorf("empty session: no files changed for story %s", story.ID))
				if msg != "" {
					emit("error", msg)
				}
				if err := tk.RevertToReady(story.ID); err != nil {
					log.Printf("Warning: could not revert story status: %v", err)
				}
				engineState.Save(e.opts.StateDir)
				continue
			}

			emit("info", fmt.Sprintf("Agent changed %d file(s) — running quality gates...", len(changedFiles)))

			// Run quality gate hooks (with path filtering).
			// If gates fail, retry: ask agent to fix → re-run gates until pass.
			gatesPassed := true
			if e.opts.Hooks != nil && len(e.opts.Hooks.QualityGates.Steps) > 0 {
				gatesPassed = e.runGatesWithRetry(ctx, client, story, prompt, emit)
			}

			if gatesPassed {
				e.circuitBreaker.RecordSuccess(1)
				engineState.StoriesCompletedTotal++
				engineState.StoriesCompletedThisSession = append(
					engineState.StoriesCompletedThisSession, story.ID)
				if err := tk.MarkComplete(story.ID); err != nil {
					log.Printf("Warning: could not mark story complete: %v", err)
				}
				emit("story_complete", fmt.Sprintf("Story %s complete (%d total, $%.2f session)",
					story.ID, engineState.StoriesCompletedTotal, engineState.SessionCostUSD))

				// Run post-story hooks.
				if e.opts.Hooks != nil {
					e.runHookPhase(ctx, "post-story", e.opts.Hooks.PostStory, nil, emit)
				}
			} else {
				// Gates failed after retries — revert story to ready-for-dev.
				if err := tk.RevertToReady(story.ID); err != nil {
					log.Printf("Warning: could not revert story status: %v", err)
				}
				msg := e.circuitBreaker.RecordFailure(fmt.Errorf("quality gates failed for story %s after retries", story.ID))
				emit("error", fmt.Sprintf("Quality gates FAILED for story %s after retries — reverted to ready-for-dev", story.ID))
				if msg != "" {
					emit("error", msg)
				}
			}
		} else {
			exitCode := 0
			if sessionResult != nil {
				exitCode = sessionResult.ExitCode
			}
			msg := e.circuitBreaker.RecordFailure(fmt.Errorf("session exit code %d", exitCode))
			emit("error", fmt.Sprintf("Session exited with code %d", exitCode))
			if msg != "" {
				emit("error", msg)
			}
		}

		engineState.Save(e.opts.StateDir)

		// Single-story mode: stop after first story.
		if e.opts.SingleStory != "" {
			emit("info", fmt.Sprintf("Single-story mode complete: %s", e.opts.SingleStory))
			return e.stop(ExitAllComplete, engineState, emit, "")
		}

		if len(engineState.StoriesCompletedThisSession) >= e.opts.StoriesPerSession {
			emit("info", fmt.Sprintf("Reached %d stories this batch. Starting fresh context...",
				e.opts.StoriesPerSession))
			engineState.StoriesCompletedThisSession = []string{}
			engineState.SessionCostUSD = 0
		}

		// Cooldown — respects context cancellation.
		emit("info", fmt.Sprintf("Cooldown %v before next story...", e.cooldown))
		select {
		case <-ctx.Done():
			continue
		case <-time.After(e.cooldown):
		}
	}
}

// dryRun shows the execution plan without calling the agent.
func (e *Engine) dryRun(tk tracker.TaskTracker, emit func(string, string), s *state.Engine) RunResult {
	emit("info", "=== DRY RUN — no agent will be called ===")

	// Show all pending stories.
	pending, err := tk.ListPending()
	if err != nil {
		emit("error", fmt.Sprintf("Tracker error: %v", err))
		return RunResult{Error: err}
	}

	if len(pending) == 0 {
		emit("info", "No pending stories found. Nothing to do.")
		return RunResult{ExitReason: ExitAllComplete}
	}

	all, _ := tk.ListAll()
	done := len(all) - len(pending)

	emit("info", fmt.Sprintf("Stories: %d pending, %d done, %d total", len(pending), done, len(all)))
	emit("info", "")

	// Show execution plan.
	maxShow := len(pending)
	if e.opts.MaxIterations > 0 && e.opts.MaxIterations < maxShow {
		maxShow = e.opts.MaxIterations
	}

	for i, story := range pending {
		if i >= maxShow {
			emit("info", fmt.Sprintf("  ... and %d more", len(pending)-maxShow))
			break
		}
		status := "ready"
		if story.Status == tracker.StatusInProgress {
			status = "IN-PROGRESS (will resume)"
		}
		emit("info", fmt.Sprintf("  [%d] Story %s — %s (%s) [%s]",
			i+1, story.ID, story.Title, story.EpicTitle, status))
	}

	// Show config summary.
	emit("info", "")
	emit("info", fmt.Sprintf("Agent:           %s", e.opts.Binary))
	emit("info", fmt.Sprintf("Max iterations:  %s", formatLimit(e.opts.MaxIterations)))
	emit("info", fmt.Sprintf("Stories/session: %d", e.opts.StoriesPerSession))
	emit("info", fmt.Sprintf("Circuit breaker: %d failures → stop", e.opts.MaxFailures))
	emit("info", fmt.Sprintf("Cooldown:        %v", e.cooldown))

	if e.opts.SingleStory != "" {
		emit("info", fmt.Sprintf("Single story:    %s", e.opts.SingleStory))
	}

	// Show prompt preview for first story.
	if len(pending) > 0 {
		story := pending[0]
		emit("info", "")
		emit("info", "=== Prompt preview (first story) ===")

		workflowType := "basic"
		if e.opts.WorkflowType != "" {
			workflowType = e.opts.WorkflowType
		}
		qualityGate := "standard"
		if e.opts.QualityGate != "" {
			qualityGate = e.opts.QualityGate
		}

		var sections []config.PromptSection
		if e.opts.Prompt != nil {
			sections = e.opts.Prompt.Sections
		}

		prompt := BuildPrompt(PromptContext{
			StoryID:      story.ID,
			StoryTitle:   story.Title,
			EpicID:       story.EpicID,
			EpicTitle:    story.EpicTitle,
			StoriesDone:  done,
			StoriesTotal: len(all),
			WorkflowType: workflowType,
			QualityGate:  qualityGate,
			StoryContent: e.loadStoryContent(&story),
			PromptMD:     appcontext.LoadPromptMD(e.opts.ProjectDir),
			Sections:     sections,
			ProjectDir:   e.opts.ProjectDir,
			Research:     e.opts.Research,
		})
		// Show first 20 lines of prompt.
		lines := splitLines(prompt)
		for i, line := range lines {
			if i >= 20 {
				emit("info", fmt.Sprintf("  ... (%d more lines)", len(lines)-20))
				break
			}
			emit("info", fmt.Sprintf("  | %s", line))
		}
	}

	emit("info", "")
	emit("info", "=== DRY RUN complete. Use 'ralph-engine run' to execute. ===")

	return RunResult{
		ExitReason:      ExitAllComplete,
		StoriesComplete: done,
	}
}

// pickStory selects the next story, respecting single-story mode.
func (e *Engine) pickStory(tk tracker.TaskTracker) (*tracker.Story, error) {
	if e.opts.SingleStory != "" {
		all, err := tk.ListAll()
		if err != nil {
			return nil, err
		}
		for i := range all {
			if all[i].ID == e.opts.SingleStory && all[i].IsActionable() {
				return &all[i], nil
			}
		}
		return nil, nil // Story not found or already done.
	}
	return tk.NextStory()
}

// setStatus updates engine status with mutex protection.
func (e *Engine) setStatus(status EngineStatus, reason ExitReason) {
	e.mu.Lock()
	e.status = status
	if reason != "" {
		e.exitReason = reason
	}
	e.mu.Unlock()
}

// stop saves state and returns a RunResult.
func (e *Engine) stop(reason ExitReason, s *state.Engine, emit func(string, string), msg string) RunResult {
	e.setStatus(StatusStopped, reason)
	if msg != "" {
		levelType := "info"
		if reason == ExitCircuitBreaker || reason == ExitResourceCritical {
			levelType = "error"
		}
		emit(levelType, msg)
	}

	// Run post-session hooks (best-effort — don't block shutdown).
	if e.opts.Hooks != nil && len(e.opts.Hooks.PostSession.Steps) > 0 {
		ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
		defer cancel()
		e.runHookPhase(ctx, "post-session", e.opts.Hooks.PostSession, nil, emit)
	}

	s.Save(e.opts.StateDir)
	e.mu.RLock()
	defer e.mu.RUnlock()
	return RunResult{
		ExitReason:      reason,
		SessionsRun:     e.sessionCount,
		StoriesComplete: s.StoriesCompletedTotal,
		TotalCostUSD:    s.TotalCostUSD,
	}
}

// checkResources evaluates host resources and pauses or stops as needed.
func (e *Engine) checkResources(ctx context.Context, emit func(string, string)) ExitReason {
	snap, err := e.resourceMon.Check()
	if err != nil {
		return ""
	}
	status := e.resourceMon.Evaluate(snap)
	if status.ShouldStop {
		emit("error", status.Summary())
		return ExitResourceCritical
	}
	if status.ShouldPause {
		emit("info", fmt.Sprintf("Resources low, pausing 30s... %s", status.Summary()))
		e.setStatus(StatusPaused, "")
		select {
		case <-time.After(30 * time.Second):
		case <-ctx.Done():
		}
		e.setStatus(StatusRunning, "")
	}
	return ""
}

// countTotal returns the total number of stories from the tracker.
func countTotal(tk tracker.TaskTracker) int {
	all, err := tk.ListAll()
	if err != nil {
		return 0
	}
	return len(all)
}

// runGatesWithRetry runs quality gates and, on failure, asks the agent to fix issues.
// Retries until gates pass, context is cancelled, or max retries reached.
// MaxGateRetries = 0 means unlimited retries (default — let agent fix everything).
func (e *Engine) runGatesWithRetry(ctx context.Context, client *claude.Client, story *tracker.Story, systemPrompt string, emit func(string, string)) bool {
	maxRetries := e.opts.MaxGateRetries // 0 = unlimited
	for attempt := 1; ; attempt++ {
		changedFiles := hooks.GetChangedFiles(e.opts.ProjectDir)
		if e.runHookPhase(ctx, "quality-gates", e.opts.Hooks.QualityGates, changedFiles, emit) {
			if attempt > 1 {
				emit("info", fmt.Sprintf("Quality gates passed after %d fix attempts", attempt-1))
			}
			return true // Gates passed.
		}

		if maxRetries > 0 && attempt > maxRetries {
			break // Max retries exhausted.
		}

		if ctx.Err() != nil {
			break // Context cancelled (Ctrl+C).
		}

		// Collect failure details for the agent.
		failures := e.collectGateFailures(ctx, changedFiles)
		retryMsg := fmt.Sprintf("Quality gates failed (attempt %d", attempt)
		if maxRetries > 0 {
			retryMsg += fmt.Sprintf("/%d", maxRetries)
		}
		retryMsg += ") — asking agent to fix..."
		emit("info", retryMsg)

		// Ask agent to fix the failures.
		fixPrompt := fmt.Sprintf("Quality gates FAILED for story %s. Fix these issues:\n\n%s\n\nFix ALL failures, then confirm with a commit.", story.ID, failures)

		_, err := client.Run(ctx, claude.SessionRequest{
			Prompt:       fixPrompt,
			ProjectDir:   e.opts.ProjectDir,
			SystemPrompt: systemPrompt,
		}, func(event claude.StreamEvent) {
			if e.opts.DryRun {
				return
			}
		})

		if err != nil {
			emit("error", fmt.Sprintf("Fix session failed: %v", err))
			break
		}
	}

	return false
}

// collectGateFailures runs gates in dry mode and collects failure messages.
func (e *Engine) collectGateFailures(ctx context.Context, changedFiles []string) string {
	var failures []string
	hooks.RunPhase(ctx, e.opts.Hooks.QualityGates, e.opts.ProjectDir, changedFiles, func(sr hooks.StepResult) {
		if !sr.OK && !sr.Skipped {
			msg := fmt.Sprintf("- %s: FAILED", sr.Name)
			if sr.Error != nil {
				msg += fmt.Sprintf(" (%s)", sr.Error.Error())
			}
			if sr.Output != "" {
				// Include last 500 chars of output for context.
				out := sr.Output
				if len(out) > 500 {
					out = out[len(out)-500:]
				}
				msg += fmt.Sprintf("\n  Output: %s", out)
			}
			failures = append(failures, msg)
		}
	})
	if len(failures) == 0 {
		return "Unknown failure"
	}
	return strings.Join(failures, "\n")
}

// runHookPhase executes a hook phase and emits events for each step.
// Returns true if the phase passed (no required step failed).
func (e *Engine) runHookPhase(ctx context.Context, name string, phase hooks.HookPhase, changedFiles []string, emit func(string, string)) bool {
	if len(phase.Steps) == 0 {
		return true
	}

	emit("info", fmt.Sprintf("Running %s hooks...", name))
	result := hooks.RunPhase(ctx, phase, e.opts.ProjectDir, changedFiles, func(sr hooks.StepResult) {
		if sr.Skipped {
			emit("info", fmt.Sprintf("  ⊘ %s (skipped — no matching files)", sr.Name))
			return
		}
		if sr.OK {
			emit("info", fmt.Sprintf("  ✓ %s (%v)", sr.Name, sr.Duration.Round(time.Millisecond)))
		} else {
			level := "warn"
			if sr.Required {
				level = "error"
			}
			errMsg := ""
			if sr.Error != nil {
				errMsg = sr.Error.Error()
			}
			emit(level, fmt.Sprintf("  ✗ %s — %s", sr.Name, errMsg))
		}
	})

	if result.Blocked {
		emit("error", fmt.Sprintf("Hook blocked: %s", result.Reason))
		return false
	}
	return true
}

// loadStoryContent reads the story specification file for prompt injection.
// It tries: (1) story.FilePath from tracker, (2) search in paths.stories by ID.
func (e *Engine) loadStoryContent(story *tracker.Story) string {
	// Try explicit file path from tracker first.
	if story.FilePath != "" {
		content := appcontext.LoadStoryFile(e.opts.ProjectDir, story.FilePath)
		if content != "" {
			return content
		}
	}

	// Search in configured stories path by story ID.
	if e.opts.Paths != nil && e.opts.Paths.Stories != "" {
		foundPath := appcontext.FindStoryFile(e.opts.ProjectDir, e.opts.Paths.Stories, story.ID)
		if foundPath != "" {
			content := appcontext.LoadStoryFile(e.opts.ProjectDir, foundPath)
			if content != "" {
				return content
			}
		}
	}

	return ""
}

func formatLimit(n int) string {
	if n <= 0 {
		return "unlimited"
	}
	return fmt.Sprintf("%d", n)
}

func splitLines(s string) []string {
	var lines []string
	start := 0
	for i := 0; i < len(s); i++ {
		if s[i] == '\n' {
			lines = append(lines, s[start:i])
			start = i + 1
		}
	}
	if start < len(s) {
		lines = append(lines, s[start:])
	}
	return lines
}
