// Package logger provides structured logging optimized for AI agent consumption.
// In debug mode, output is JSON with rich context (component, suggestion, docs)
// so AI agents can diagnose and fix issues autonomously.
// In normal mode, output is human-friendly with colors.
package logger

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
	"strings"
	"time"
)

// Level controls which messages are emitted.
type Level int

const (
	LevelDebug Level = iota
	LevelInfo
	LevelWarn
	LevelError
)

// Format controls the output format.
type Format string

const (
	FormatHuman Format = "human" // Colored, concise — for terminal users.
	FormatJSON  Format = "json"  // Structured — for AI agents and log aggregation.
)

// Config configures the logger.
type Config struct {
	Debug  bool      // Enable debug level + verbose output.
	Format Format    // "human" or "json".
	Output io.Writer // Where to write (default: os.Stderr).
}

// Logger writes structured log messages.
type Logger struct {
	level  Level
	format Format
	output io.Writer
}

// New creates a logger with the given config.
func New(config Config) *Logger {
	level := LevelInfo
	if config.Debug {
		level = LevelDebug
	}
	if config.Format == "" {
		config.Format = FormatHuman
	}
	if config.Output == nil {
		config.Output = os.Stderr
	}
	return &Logger{
		level:  level,
		format: config.Format,
		output: config.Output,
	}
}

// Debug logs at debug level — only visible with --debug flag.
func (l *Logger) Debug(msg string, kvs ...interface{}) {
	l.log(LevelDebug, msg, kvs...)
}

// Info logs at info level — normal operation messages.
func (l *Logger) Info(msg string, kvs ...interface{}) {
	l.log(LevelInfo, msg, kvs...)
}

// Warn logs at warn level — something unexpected but not fatal.
func (l *Logger) Warn(msg string, kvs ...interface{}) {
	l.log(LevelWarn, msg, kvs...)
}

// Error logs at error level — something failed.
// For AI-friendly errors, include these keys:
//   - "component": which module failed (e.g., "claude-client", "tracker")
//   - "suggestion": what to try to fix it
//   - "docs": link to relevant documentation
func (l *Logger) Error(msg string, kvs ...interface{}) {
	l.log(LevelError, msg, kvs...)
}

func (l *Logger) log(level Level, msg string, kvs ...interface{}) {
	if level < l.level {
		return
	}

	if l.format == FormatJSON {
		l.logJSON(level, msg, kvs...)
	} else {
		l.logHuman(level, msg, kvs...)
	}
}

func (l *Logger) logJSON(level Level, msg string, kvs ...interface{}) {
	entry := map[string]interface{}{
		"ts":    time.Now().UTC().Format(time.RFC3339),
		"level": levelStr(level),
		"msg":   msg,
	}

	// Parse key-value pairs.
	for i := 0; i+1 < len(kvs); i += 2 {
		key, ok := kvs[i].(string)
		if !ok {
			continue
		}
		entry[key] = kvs[i+1]
	}

	data, err := json.Marshal(entry)
	if err != nil {
		fmt.Fprintf(l.output, `{"level":"error","msg":"log marshal error: %v"}`+"\n", err)
		return
	}
	fmt.Fprintln(l.output, string(data))
}

func (l *Logger) logHuman(level Level, msg string, kvs ...interface{}) {
	prefix := levelPrefix(level)
	var parts []string
	parts = append(parts, fmt.Sprintf("%s %s", prefix, msg))

	for i := 0; i+1 < len(kvs); i += 2 {
		key, ok := kvs[i].(string)
		if !ok {
			continue
		}
		parts = append(parts, fmt.Sprintf("%s=%v", key, kvs[i+1]))
	}

	fmt.Fprintln(l.output, strings.Join(parts, " "))
}

func levelStr(level Level) string {
	switch level {
	case LevelDebug:
		return "debug"
	case LevelInfo:
		return "info"
	case LevelWarn:
		return "warn"
	case LevelError:
		return "error"
	default:
		return "unknown"
	}
}

func levelPrefix(level Level) string {
	switch level {
	case LevelDebug:
		return "DBG"
	case LevelInfo:
		return "INF"
	case LevelWarn:
		return "WRN"
	case LevelError:
		return "ERR"
	default:
		return "???"
	}
}
