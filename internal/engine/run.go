package engine

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/diegorodrigo90/ralph-engine/internal/claude"
	"github.com/diegorodrigo90/ralph-engine/internal/config"
	appcontext "github.com/diegorodrigo90/ralph-engine/internal/context"
	"github.com/diegorodrigo90/ralph-engine/internal/hooks"
	applogger "github.com/diegorodrigo90/ralph-engine/internal/logger"
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

	// Reset per-run counters for this fresh invocation.
	engineState.StartNewRun()

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
	agentInfo := e.opts.Binary
	if e.opts.Model != "" {
		agentInfo += " (model=" + e.opts.Model + ")"
	}
	emit("info", fmt.Sprintf("Engine started [mode=%s, agent=%s, stories/session=%d, gate-retries=%s]",
		mode, agentInfo, e.opts.StoriesPerSession, formatLimit(e.opts.MaxGateRetries)))

	// Dry-run: show plan and exit.
	if e.opts.DryRun {
		return e.dryRun(tk, emit, engineState)
	}

	// Run prepare hooks (before loop starts).
	if e.opts.Hooks != nil && len(e.opts.Hooks.Prepare.Steps) > 0 {
		if !e.runHookPhase(ctx, "prepare", e.opts.Hooks.Prepare, nil, emit) {
			return e.stop(ExitBlocked, engineState, emit, "Prepare hooks failed — run 'ralph-engine prepare' to diagnose")
		}
	}

	// Use "stream-json" for real-time progress visibility.
	// Emits events line-by-line so the engine can show tool calls and agent responses.
	var allowedTools []string
	if e.opts.AllowedTools != "" {
		allowedTools = strings.Split(e.opts.AllowedTools, ",")
	}
	var disallowedTools []string
	if e.opts.DisallowedTools != "" {
		disallowedTools = strings.Split(e.opts.DisallowedTools, ",")
	}

	// Debug log: create a verbose log file when --debug is active.
	// Uses cross-platform log directory with automatic rotation.
	var debugLog *os.File
	if e.opts.Debug {
		logDir := applogger.LogDir(e.opts.StateDir)
		cfg := applogger.DefaultRotateConfig()
		if e.opts.LogMaxFiles > 0 {
			cfg.MaxFiles = e.opts.LogMaxFiles
		}
		if e.opts.LogMaxSizeMB > 0 {
			cfg.MaxSizeMB = e.opts.LogMaxSizeMB
		}
		var logPath string
		var err error
		debugLog, logPath, err = applogger.CreateLogFile(logDir, cfg)
		if err == nil {
			defer debugLog.Close()
			emit("info", fmt.Sprintf("Debug log: %s (tail -f to monitor)", logPath))
		}
	}

	client := claude.NewClient(claude.ClientConfig{
		Binary:          e.opts.Binary,
		OutputFormat:    "stream-json",
		Model:           e.opts.Model,
		MaxTurns:        e.opts.MaxTurns,
		AllowedTools:    allowedTools,
		DisallowedTools: disallowedTools,
		SkipPermissions: e.opts.SkipPermissions,
		DebugLog:        debugLog,
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
		if err := engineState.Save(e.opts.StateDir); err != nil {
			log.Printf("Warning: could not save engine state: %v", err)
		}

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
			StoryID:          story.ID,
			StoryTitle:       story.Title,
			EpicID:           story.EpicID,
			EpicTitle:        story.EpicTitle,
			SessionNumber:    sessionNum,
			StoriesDone:      engineState.StoriesCompletedTotal,
			StoriesTotal:     countTotal(tk),
			WorkflowType:     workflowType,
			WorkflowCommands:     e.opts.WorkflowCommands,
			WorkflowInstructions: e.opts.WorkflowInstructions,
			QualityGate:      qualityGate,
			StoryContent:     storyContent,
			PromptMD:         promptMD,
			Sections:         sections,
			ProjectDir:       e.opts.ProjectDir,
			Research:         e.opts.Research,
		})

		// Build the user prompt — explicit implementation instruction.
		userPrompt := fmt.Sprintf("Implement story %s: %s\n\nYou MUST write real source code (not just docs or metadata). Follow the workflow in the system prompt. Use BMAD /dev skill if available.", story.ID, story.Title)
		if story.FilePath != "" {
			userPrompt += fmt.Sprintf("\n\nStory file: %s", story.FilePath)
		}

		// Show real-time progress during agent session via stream-json events.
		sessionStart := time.Now()
		emit("info", "Agent session started...")
		toolCount := 0

		sessionResult, err := client.Run(ctx, claude.SessionRequest{
			Prompt:       userPrompt,
			ProjectDir:   e.opts.ProjectDir,
			SystemPrompt: prompt,
		}, func(event claude.StreamEvent) {
			elapsed := time.Since(sessionStart).Round(time.Second)

			switch event.Type {
			case "assistant":
				// Claude stream-json wraps tool_use inside assistant message content array.
				// Parse the full message to extract tool calls and text responses.
				var msg struct {
					Content json.RawMessage `json:"content"`
				}
				if json.Unmarshal(event.Message, &msg) != nil || msg.Content == nil {
					return
				}

				// Content can be a string or array of content blocks.
				var contentBlocks []struct {
					Type string `json:"type"`
					Text string `json:"text"`
					Name string `json:"name"` // tool name for tool_use blocks
				}
				if json.Unmarshal(msg.Content, &contentBlocks) == nil {
					for _, block := range contentBlocks {
						switch block.Type {
						case "tool_use":
							toolCount++
							emit("info", fmt.Sprintf("  [%v] Tool #%d: %s", elapsed, toolCount, block.Name))
						case "text":
							if len(block.Text) > 10 {
								preview := block.Text
								if len(preview) > 120 {
									preview = preview[:120] + "..."
								}
								emit("info", fmt.Sprintf("  [%v] Agent: %s", elapsed, strings.TrimSpace(preview)))
							}
						}
					}
				}

			case "user":
				// Tool results come back as user messages — shows agent is working.

			case "result":
				// Session ending — extract turn count.
				var res struct {
					NumTurns   int `json:"num_turns"`
					DurationMs int `json:"duration_ms"`
				}
				if event.Result != nil {
					emit("info", fmt.Sprintf("  [%v] Session completing...", elapsed))
				} else if json.Unmarshal(event.Message, &res) == nil && res.NumTurns > 0 {
					emit("info", fmt.Sprintf("  [%v] Session completing (%d turns)...", elapsed, res.NumTurns))
				}
			}
		})
		elapsed := time.Since(sessionStart).Round(time.Second)
		emit("info", fmt.Sprintf("Agent session finished (%v, %d tool calls)", elapsed, toolCount))

		if err != nil {
			if errors.Is(err, context.Canceled) {
				continue
			}
			msg := e.circuitBreaker.RecordFailure(fmt.Errorf("session error: %w", err))
			emit("error", fmt.Sprintf("Session failed: %v", err))
			if msg != "" {
				emit("error", msg)
			}
			if err := engineState.Save(e.opts.StateDir); err != nil {
				log.Printf("Warning: could not save engine state after session error: %v", err)
			}
			continue
		}

		if sessionResult != nil && sessionResult.UsageLimit {
			engineState.EngineStatus = state.StatusSessionComplete
			// Save handoff with session context (no AI needed).
			e.saveHandoff(engineState, story, toolCount, time.Since(sessionStart), emit)
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
				if err := engineState.Save(e.opts.StateDir); err != nil {
					log.Printf("Warning: could not save engine state after empty session: %v", err)
				}
				continue
			}

			// Show changed files so user can see what the agent did.
			emit("info", fmt.Sprintf("Agent changed %d file(s):", len(changedFiles)))
			for _, f := range changedFiles {
				emit("info", fmt.Sprintf("  → %s", f))
			}

			// Run quality gate hooks (with path filtering).
			// If gates fail, retry: ask agent to fix → re-run gates until pass.
			gatesPassed := true
			allGatesSkipped := false
			if e.opts.Hooks != nil && len(e.opts.Hooks.QualityGates.Steps) > 0 {
				gatesPassed, allGatesSkipped = e.runGatesWithRetry(ctx, client, story, prompt, changedFiles, emit)
			}

			// If ALL quality gates were skipped (no changed files matched any gate path),
			// the agent only changed metadata files — not real code. Don't mark complete.
			if allGatesSkipped {
				emit("warn", fmt.Sprintf("ALL quality gates skipped for story %s — agent changed only non-code files. Not marking complete.", story.ID))
				msg := e.circuitBreaker.RecordFailure(fmt.Errorf("no code changes: all gates skipped for story %s", story.ID))
				if msg != "" {
					emit("error", msg)
				}
				if err := tk.RevertToReady(story.ID); err != nil {
					log.Printf("Warning: could not revert story status: %v", err)
				}
				if err := engineState.Save(e.opts.StateDir); err != nil {
					log.Printf("Warning: could not save engine state after gates skipped: %v", err)
				}
				continue
			}

			if gatesPassed {
				e.circuitBreaker.RecordSuccess(1)
				engineState.StoriesCompletedTotal++
				engineState.StoriesCompletedThisRun++
				engineState.StoriesCompletedThisSession = append(
					engineState.StoriesCompletedThisSession, story.ID)
				if err := tk.MarkComplete(story.ID); err != nil {
					log.Printf("Warning: could not mark story complete: %v", err)
				}
				emit("story_complete", fmt.Sprintf("Story %s complete (%d this run, %d total, $%.2f session)",
					story.ID, engineState.StoriesCompletedThisRun, engineState.StoriesCompletedTotal, engineState.SessionCostUSD))

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

		if err := engineState.Save(e.opts.StateDir); err != nil {
			log.Printf("Warning: could not save engine state after iteration: %v", err)
		}

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

// saveHandoff writes a handoff file with session context for resume.
// Called when usage limit hits — saves without AI (engine memory only).
func (e *Engine) saveHandoff(s *state.Engine, story *tracker.Story, toolCount int, elapsed time.Duration, emit func(string, string)) {
	handoff := map[string]interface{}{
		"story_id":            story.ID,
		"story_title":         story.Title,
		"epic_id":             story.EpicID,
		"session_number":      s.SessionNumber,
		"tool_calls":          toolCount,
		"elapsed":             elapsed.String(),
		"files_changed":       hooks.GetChangedFiles(e.opts.ProjectDir),
		"stories_done_this_run": s.StoriesCompletedThisRun,
		"stories_done_total":   s.StoriesCompletedTotal,
		"cost_usd":            s.SessionCostUSD,
		"exit_reason":         "usage_limit",
		"timestamp":           time.Now().UTC().Format(time.RFC3339),
	}

	data, err := json.MarshalIndent(handoff, "", "  ")
	if err != nil {
		emit("warn", fmt.Sprintf("Could not marshal handoff: %v", err))
		return
	}

	path := filepath.Join(e.opts.StateDir, fmt.Sprintf("handoff-%s.json", story.ID))
	if err := os.WriteFile(path, data, 0600); err != nil {
		emit("warn", fmt.Sprintf("Could not write handoff: %v", err))
		return
	}
	emit("info", fmt.Sprintf("Handoff saved: %s", path))
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
			StoryID:          story.ID,
			StoryTitle:       story.Title,
			EpicID:           story.EpicID,
			EpicTitle:        story.EpicTitle,
			StoriesDone:      done,
			StoriesTotal:     len(all),
			WorkflowType:     workflowType,
			WorkflowCommands:     e.opts.WorkflowCommands,
			WorkflowInstructions: e.opts.WorkflowInstructions,
			QualityGate:      qualityGate,
			StoryContent:     e.loadStoryContent(&story),
			PromptMD:         appcontext.LoadPromptMD(e.opts.ProjectDir),
			Sections:         sections,
			ProjectDir:       e.opts.ProjectDir,
			Research:         e.opts.Research,
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

	if err := s.Save(e.opts.StateDir); err != nil {
		log.Printf("Warning: could not save engine state on stop: %v", err)
	}

	// Remind user where debug logs are stored.
	if e.opts.Debug {
		logDir := applogger.LogDir(e.opts.StateDir)
		emit("info", fmt.Sprintf("Debug logs: %s", logDir))
	}

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
// Returns (passed, allSkipped) — allSkipped is true when ALL gates were skipped due to path filters.
func (e *Engine) runGatesWithRetry(ctx context.Context, client *claude.Client, story *tracker.Story, systemPrompt string, changedFiles []string, emit func(string, string)) (bool, bool) {
	maxRetries := e.opts.MaxGateRetries // 0 = unlimited

	// Check if ALL gates would be skipped (no changed files match any gate path).
	// This indicates the agent only changed non-code files (metadata, docs, etc.).
	allSkipped := e.allGatesWouldBeSkipped(changedFiles)
	if allSkipped {
		// Still run the phase to show skip messages, then return.
		e.runHookPhase(ctx, "quality-gates", e.opts.Hooks.QualityGates, changedFiles, emit)
		return true, true
	}

	for attempt := 1; ; attempt++ {
		freshChangedFiles := hooks.GetChangedFiles(e.opts.ProjectDir)
		if e.runHookPhase(ctx, "quality-gates", e.opts.Hooks.QualityGates, freshChangedFiles, emit) {
			if attempt > 1 {
				emit("info", fmt.Sprintf("Quality gates passed after %d fix attempts", attempt-1))
			}
			return true, false // Gates passed.
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

	return false, false
}

// allGatesWouldBeSkipped checks if every quality gate step would be skipped
// because none of the changed files match any gate's path filter.
// Gates without path filters always run and are NOT considered "skipped".
func (e *Engine) allGatesWouldBeSkipped(changedFiles []string) bool {
	if changedFiles == nil || e.opts.Hooks == nil || len(e.opts.Hooks.QualityGates.Steps) == 0 {
		return false
	}
	for _, step := range e.opts.Hooks.QualityGates.Steps {
		if strings.TrimSpace(step.Run) == "" {
			continue
		}
		// No path filter = always runs = not skipped.
		if len(step.Paths) == 0 {
			return false
		}
		// If any gate matches, not all are skipped.
		if hooks.MatchesAnyPath(changedFiles, step.Paths) {
			return false
		}
	}
	return true
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
