package engine

import (
	"context"
	"errors"
	"fmt"
	"log"
	"time"

	"github.com/diegorodrigo90/ralph-engine/internal/claude"
	"github.com/diegorodrigo90/ralph-engine/internal/config"
	appcontext "github.com/diegorodrigo90/ralph-engine/internal/context"
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

		sessionResult, err := client.Run(ctx, claude.SessionRequest{
			Prompt:       userPrompt,
			ProjectDir:   e.opts.ProjectDir,
			SystemPrompt: prompt,
		}, func(event claude.StreamEvent) {
			if onEvent != nil {
				onEvent(EngineEvent{Type: "stream", Message: event.Type})
			}
		})

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
			e.circuitBreaker.RecordSuccess(1)
			engineState.StoriesCompletedTotal++
			engineState.StoriesCompletedThisSession = append(
				engineState.StoriesCompletedThisSession, story.ID)
			if err := tk.MarkComplete(story.ID); err != nil {
				log.Printf("Warning: could not mark story complete: %v", err)
			}
			emit("story_complete", fmt.Sprintf("Story %s complete (%d total, $%.2f session)",
				story.ID, engineState.StoriesCompletedTotal, engineState.SessionCostUSD))
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
