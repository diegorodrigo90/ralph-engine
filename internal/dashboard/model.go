// Package dashboard implements the bubbletea TUI for ralph-engine.
// It displays real-time progress including epic/story status, quality gates,
// system resources, and session statistics.
package dashboard

import (
	"fmt"
	"strings"
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

// Styles for the dashboard.
var (
	titleStyle = lipgloss.NewStyle().
			Bold(true).
			Foreground(lipgloss.Color("99")).
			MarginBottom(1)

	sectionStyle = lipgloss.NewStyle().
			Bold(true).
			Foreground(lipgloss.Color("86"))

	okStyle = lipgloss.NewStyle().
		Foreground(lipgloss.Color("82"))

	warnStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("214"))

	errorStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("196"))

	dimStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("240"))

	progressStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("39"))
)

// DashboardState holds all data displayed on the dashboard.
type DashboardState struct {
	// Engine
	EngineStatus string
	ExitReason   string
	SessionNum   int
	Uptime       time.Duration

	// Story progress
	CurrentEpic    string
	CurrentStory   string
	CurrentPhase   string
	StoriesDone    int
	StoriesTotal   int
	StoriesSession int

	// Quality gates
	GatesCR        string // "pass", "fail", "running", "pending"
	GatesTests     string
	GatesBuild     string
	GatesTypeCheck string
	GatesE2E       string

	// System resources
	RAMMB       int
	RAMTotalMB  int
	CPUPercent  float64
	DiskGB      int
	DiskTotalGB int

	// SSH
	SSHStatus string

	// Circuit breaker
	Failures    int
	MaxFailures int
	BreakerState string

	// Cost
	SessionCost float64
	TotalCost   float64

	// Log
	LastMessage string
	Findings    int
}

// Model is the bubbletea model for the dashboard.
type Model struct {
	State    DashboardState
	width    int
	height   int
	quitting bool
}

// NewModel creates a new dashboard model.
func NewModel() Model {
	return Model{
		State: DashboardState{
			EngineStatus: "idle",
			BreakerState: "closed",
			SSHStatus:    "unknown",
		},
	}
}

// TickMsg triggers periodic refresh.
type TickMsg time.Time

// UpdateMsg updates the dashboard state from the engine.
type UpdateMsg struct {
	State DashboardState
}

// Init starts the periodic tick.
func (m Model) Init() tea.Cmd {
	return tea.Tick(time.Second, func(t time.Time) tea.Msg {
		return TickMsg(t)
	})
}

// Update handles messages.
func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "q", "ctrl+c":
			m.quitting = true
			return m, tea.Quit
		}

	case tea.WindowSizeMsg:
		m.width = msg.Width
		m.height = msg.Height

	case UpdateMsg:
		m.State = msg.State

	case TickMsg:
		return m, tea.Tick(time.Second, func(t time.Time) tea.Msg {
			return TickMsg(t)
		})
	}

	return m, nil
}

// View renders the dashboard.
func (m Model) View() string {
	if m.quitting {
		return "Shutting down...\n"
	}

	var b strings.Builder
	s := m.State

	// Title
	b.WriteString(titleStyle.Render("ralph-engine"))
	b.WriteString(dimStyle.Render(fmt.Sprintf("  session #%d", s.SessionNum)))
	b.WriteString("\n\n")

	// Engine status
	b.WriteString(sectionStyle.Render("ENGINE"))
	b.WriteString(fmt.Sprintf("  %s", statusIcon(s.EngineStatus)))
	if s.Uptime > 0 {
		b.WriteString(dimStyle.Render(fmt.Sprintf("  %s", formatDuration(s.Uptime))))
	}
	b.WriteString("\n")

	// Story progress
	b.WriteString(sectionStyle.Render("STORY "))
	if s.CurrentStory != "" {
		b.WriteString(fmt.Sprintf("  %s", s.CurrentStory))
		if s.CurrentPhase != "" {
			b.WriteString(dimStyle.Render(fmt.Sprintf(" [%s]", s.CurrentPhase)))
		}
	} else {
		b.WriteString(dimStyle.Render("  waiting"))
	}
	b.WriteString("\n")

	if s.CurrentEpic != "" {
		b.WriteString(dimStyle.Render(fmt.Sprintf("        epic: %s", s.CurrentEpic)))
		b.WriteString("\n")
	}

	// Progress bar
	b.WriteString(sectionStyle.Render("PROGRESS"))
	b.WriteString(fmt.Sprintf(" %s", renderProgress(s.StoriesDone, s.StoriesTotal, 20)))
	b.WriteString(fmt.Sprintf(" %d/%d", s.StoriesDone, s.StoriesTotal))
	if s.StoriesSession > 0 {
		b.WriteString(dimStyle.Render(fmt.Sprintf(" (+%d this session)", s.StoriesSession)))
	}
	b.WriteString("\n\n")

	// Quality gates
	b.WriteString(sectionStyle.Render("GATES"))
	b.WriteString(fmt.Sprintf("  CR:%s  Tests:%s  Build:%s  Types:%s",
		gateIcon(s.GatesCR), gateIcon(s.GatesTests),
		gateIcon(s.GatesBuild), gateIcon(s.GatesTypeCheck)))
	if s.GatesE2E != "" {
		b.WriteString(fmt.Sprintf("  E2E:%s", gateIcon(s.GatesE2E)))
	}
	b.WriteString("\n\n")

	// System resources
	b.WriteString(sectionStyle.Render("SYSTEM"))
	b.WriteString(fmt.Sprintf("  RAM: %s  CPU: %s  Disk: %s",
		resourceStr(s.RAMMB, s.RAMTotalMB, "MB"),
		cpuStr(s.CPUPercent),
		resourceStr(s.DiskGB, s.DiskTotalGB, "GB")))
	b.WriteString("\n")

	// SSH + Circuit Breaker
	b.WriteString(sectionStyle.Render("INFRA "))
	b.WriteString(fmt.Sprintf("  SSH:%s  Breaker:%s (%d/%d)",
		sshIcon(s.SSHStatus),
		breakerIcon(s.BreakerState),
		s.Failures, s.MaxFailures))
	b.WriteString("\n")

	// Cost
	if s.TotalCost > 0 || s.SessionCost > 0 {
		b.WriteString(sectionStyle.Render("COST  "))
		b.WriteString(fmt.Sprintf("  session: $%.2f  total: $%.2f", s.SessionCost, s.TotalCost))
		b.WriteString("\n")
	}

	// Findings
	if s.Findings > 0 {
		b.WriteString(warnStyle.Render(fmt.Sprintf("\nFindings: %d", s.Findings)))
		b.WriteString("\n")
	}

	// Last message
	if s.LastMessage != "" {
		b.WriteString("\n")
		b.WriteString(dimStyle.Render(s.LastMessage))
		b.WriteString("\n")
	}

	b.WriteString(dimStyle.Render("\n[q] quit"))

	return b.String()
}

// renderProgress draws a progress bar.
func renderProgress(done, total, width int) string {
	if total == 0 {
		return strings.Repeat("░", width)
	}
	filled := (done * width) / total
	if filled > width {
		filled = width
	}
	return progressStyle.Render(strings.Repeat("█", filled)) +
		dimStyle.Render(strings.Repeat("░", width-filled))
}

func statusIcon(status string) string {
	switch status {
	case "running":
		return okStyle.Render("● running")
	case "paused":
		return warnStyle.Render("◑ paused")
	case "stopped":
		return errorStyle.Render("○ stopped")
	case "blocked":
		return errorStyle.Render("✗ blocked")
	default:
		return dimStyle.Render("○ idle")
	}
}

func gateIcon(status string) string {
	switch status {
	case "pass":
		return okStyle.Render("✓")
	case "fail":
		return errorStyle.Render("✗")
	case "running":
		return warnStyle.Render("⟳")
	default:
		return dimStyle.Render("·")
	}
}

func sshIcon(status string) string {
	switch status {
	case "healthy":
		return okStyle.Render("✓")
	case "unhealthy":
		return errorStyle.Render("✗")
	case "reconnecting":
		return warnStyle.Render("⟳")
	default:
		return dimStyle.Render("?")
	}
}

func breakerIcon(state string) string {
	switch state {
	case "closed":
		return okStyle.Render("closed")
	case "half_open":
		return warnStyle.Render("half-open")
	case "open":
		return errorStyle.Render("OPEN")
	default:
		return dimStyle.Render(state)
	}
}

func resourceStr(used, total int, unit string) string {
	if total == 0 {
		return dimStyle.Render("?")
	}
	pct := float64(used) / float64(total) * 100
	s := fmt.Sprintf("%d/%d%s", used, total, unit)
	if pct > 90 {
		return errorStyle.Render(s)
	}
	if pct > 70 {
		return warnStyle.Render(s)
	}
	return okStyle.Render(s)
}

func cpuStr(pct float64) string {
	s := fmt.Sprintf("%.0f%%", pct)
	if pct > 90 {
		return errorStyle.Render(s)
	}
	if pct > 70 {
		return warnStyle.Render(s)
	}
	return okStyle.Render(s)
}

func formatDuration(d time.Duration) string {
	h := int(d.Hours())
	m := int(d.Minutes()) % 60
	s := int(d.Seconds()) % 60
	if h > 0 {
		return fmt.Sprintf("%dh%02dm", h, m)
	}
	return fmt.Sprintf("%dm%02ds", m, s)
}
