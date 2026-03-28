# Writing Plugins

ralph-engine is extensible via custom trackers, quality gates, and workflows.

## Custom Tracker (Command Tracker)

The simplest way to integrate any task system — write scripts that output JSON.

### Setup

```yaml
# .ralph-engine/config.yaml
tracker:
  type: "command"
  commands:
    next: "./scripts/tracker-next.sh"
    complete: "./scripts/tracker-complete.sh"
    in_progress: "./scripts/tracker-in-progress.sh"
    list_pending: "./scripts/tracker-list-pending.sh"
    list_all: "./scripts/tracker-list-all.sh"
```

### Script interface

**`next`** — Output next story as JSON (or empty if none):

```json
{
  "id": "65.2",
  "title": "User Permission Grant/Deny",
  "status": "ready-for-dev",
  "epic_id": "65",
  "epic_title": "Permission System"
}
```

**`complete <story_id>`** — Mark story as done. Receives story ID as argument.

**`in_progress <story_id>`** — Mark story as in-progress.

**`list_pending`** — Output array of pending stories:

```json
[
  {
    "id": "65.2",
    "title": "User Permission Grant/Deny",
    "status": "ready-for-dev"
  },
  { "id": "65.3", "title": "Admin Permission Panel", "status": "ready-for-dev" }
]
```

**`list_all`** — Output array of all stories (any status).

### Examples

**GitHub Issues tracker:**

```bash
#!/bin/bash
# tracker-next.sh — picks oldest "ready" issue
gh issue list --label "ready-for-dev" --limit 1 --json number,title \
  | jq '.[0] | {id: (.number | tostring), title, status: "ready-for-dev"}'
```

**Linear tracker:**

```bash
#!/bin/bash
# tracker-next.sh — picks next from Linear
linear issue list --state "Todo" --first 1 --json \
  | jq '.[0] | {id: .identifier, title: .title, status: "ready-for-dev"}'
```

**Jira tracker:**

```bash
#!/bin/bash
# tracker-next.sh
jira issue list --project MYPROJ --status "To Do" --limit 1 --plain \
  | jq '{id: .key, title: .summary, status: "ready-for-dev"}'
```

## Custom Quality Gates (hooks.yaml)

See [Hooks](hooks.md) for the full hooks.yaml reference.

## Adding a Go Tracker (for contributors)

To add a native tracker implementation:

### 1. Implement the interface

```go
// internal/tracker/my_tracker.go
package tracker

type MyTracker struct {
    // your fields
}

func NewMyTracker(config map[string]string) (*MyTracker, error) {
    // initialize
    return &MyTracker{}, nil
}

func (t *MyTracker) NextStory() (*Story, error) {
    // return next ready story, or nil if none
}

func (t *MyTracker) MarkComplete(storyID string) error {
    // mark story as done
}

func (t *MyTracker) MarkInProgress(storyID string) error {
    // mark story as in-progress
}

func (t *MyTracker) ListPending() ([]Story, error) {
    // return all non-done stories
}

func (t *MyTracker) ListAll() ([]Story, error) {
    // return all stories
}
```

### 2. Register in the factory

```go
// internal/tracker/registry.go
func init() {
    Register("my-tracker", func(cfg map[string]string) (TaskTracker, error) {
        return NewMyTracker(cfg)
    })
}
```

### 3. Add tests

```go
// internal/tracker/my_tracker_test.go
func TestMyTracker_NextStory(t *testing.T) {
    // table-driven tests
}
```

### 4. Document

Add to README.md tracker section and docs/configuration.md.

## Story JSON Schema

All trackers output stories in this format:

```json
{
  "id": "string",
  "title": "string",
  "status": "string",
  "epic_id": "string (optional)",
  "epic_title": "string (optional)",
  "priority": "number (optional, lower = higher priority)",
  "tags": ["string (optional)"]
}
```

Valid status values: `ready-for-dev`, `in-progress`, `review`, `done`, `blocked`
