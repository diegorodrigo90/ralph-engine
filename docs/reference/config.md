# Configuration Reference

Every config key, its type, default value, and description.

## agent

| Key                             | Type   | Default      | Description                                      |
| ------------------------------- | ------ | ------------ | ------------------------------------------------ |
| `agent.type`                    | string | `"claude"`   | AI agent CLI binary name or path                 |
| `agent.model`                   | string | `""`         | Model to use (passed to agent)                   |
| `agent.max_stories_per_session` | int    | `5`          | Max stories per agent invocation                 |
| `agent.cooldown_seconds`        | int    | `30`         | Pause between loop iterations (seconds)          |
| `agent.allowed_tools`           | string | _(see note)_ | Comma-separated tool names allowed for the agent |

**Note:** `agent.allowed_tools` defaults to a standard set of Claude Code tools. Override to restrict or expand.

## workflow

| Key             | Type   | Default   | Description          |
| --------------- | ------ | --------- | -------------------- |
| `workflow.type` | string | `"basic"` | Workflow preset name |

## quality

| Key                        | Type   | Default      | Description                     |
| -------------------------- | ------ | ------------ | ------------------------------- |
| `quality.type`             | string | `"standard"` | Quality gate preset name        |
| `quality.gates.cr`         | bool   | `true`       | Run code review between stories |
| `quality.gates.tests`      | bool   | `true`       | Run test suite                  |
| `quality.gates.build`      | bool   | `true`       | Run build                       |
| `quality.gates.type_check` | bool   | `true`       | Run type checker                |

## tracker

| Key                   | Type   | Default                | Description                       |
| --------------------- | ------ | ---------------------- | --------------------------------- |
| `tracker.type`        | string | `"file"`               | Tracker type: `file` or `command` |
| `tracker.status_file` | string | `"sprint-status.yaml"` | Path to sprint status file        |

### Command tracker (when `tracker.type` = `command`)

| Key                             | Type   | Default | Description                                              |
| ------------------------------- | ------ | ------- | -------------------------------------------------------- |
| `tracker.commands.next`         | string |         | Script that outputs next story as JSON                   |
| `tracker.commands.complete`     | string |         | Script to mark story complete (receives story ID as arg) |
| `tracker.commands.in_progress`  | string |         | Script to mark story in-progress                         |
| `tracker.commands.list_pending` | string |         | Script that outputs pending stories as JSON array        |
| `tracker.commands.list_all`     | string |         | Script that outputs all stories as JSON array            |

## circuit_breaker

| Key                                | Type | Default | Description                          |
| ---------------------------------- | ---- | ------- | ------------------------------------ |
| `circuit_breaker.max_failures`     | int  | `3`     | Consecutive failures before stopping |
| `circuit_breaker.cooldown_minutes` | int  | `5`     | Wait time in half-open state         |

## resources

| Key                              | Type | Default | Description                 |
| -------------------------------- | ---- | ------- | --------------------------- |
| `resources.min_free_ram_mb`      | int  | `2048`  | Minimum free RAM in MB      |
| `resources.max_cpu_load_percent` | int  | `80`    | Maximum CPU load percentage |
| `resources.min_free_disk_gb`     | int  | `5`     | Minimum free disk in GB     |

## ssh

| Key                    | Type   | Default | Description                                  |
| ---------------------- | ------ | ------- | -------------------------------------------- |
| `ssh.enabled`          | bool   | `false` | Enable SSH for remote execution              |
| `ssh.reconnect_script` | string | `""`    | Script to run if SSH drops                   |
| `ssh.dev_exec_script`  | string | `""`    | Script to run commands in remote environment |

## paths

| Key                  | Type              | Default | Description                                              |
| -------------------- | ----------------- | ------- | -------------------------------------------------------- |
| `paths.stories`      | string            | `""`    | Story files directory or glob                            |
| `paths.architecture` | string            | `""`    | Architecture docs directory                              |
| `paths.prd`          | string            | `""`    | Product requirements directory                           |
| `paths.ux`           | string            | `""`    | UX specifications directory                              |
| `paths.decisions`    | string            | `""`    | ADRs / product decisions directory                       |
| `paths.status`       | string            | `""`    | Sprint/project status file                               |
| `paths.rules`        | string            | `""`    | Rules / coding standards directory                       |
| `paths.custom`       | map[string]string | `{}`    | Arbitrary key-value pairs for project-specific artifacts |

All paths are relative to the project root. The engine reads files from these locations and injects relevant content into the agent prompt. Supports both directories and individual files.

## research

| Key                 | Type   | Default    | Description                                            |
| ------------------- | ------ | ---------- | ------------------------------------------------------ |
| `research.enabled`  | bool   | `false`    | Enable research-first workflow                         |
| `research.strategy` | string | `"always"` | When to research: `always`, `story-start`, `on-demand` |
| `research.tools`    | array  | `[]`       | List of configured research tools                      |

### research.tools[] (each tool)

| Key                             | Type   | Default | Description                                         |
| ------------------------------- | ------ | ------- | --------------------------------------------------- |
| `tools[].name`                  | string |         | Display name (e.g., "Archon RAG", "Context7")       |
| `tools[].type`                  | string |         | Tool type: `rag`, `mcp`, `search`, `docs`, `custom` |
| `tools[].priority`              | int    |         | Search order (1 = first)                            |
| `tools[].enabled`               | bool   |         | Whether this tool is active                         |
| `tools[].description`           | string |         | What the tool provides (injected into prompt)       |
| `tools[].when_to_use`           | string |         | When the agent should use this tool                 |
| `tools[].how_to_use`            | string |         | Usage examples or MCP tool names                    |
| `tools[].sources`               | array  | `[]`    | Pre-indexed knowledge sources (for RAG tools)       |
| `tools[].sources[].name`        | string |         | Library/framework name                              |
| `tools[].sources[].id`          | string |         | Source identifier for targeted search               |
| `tools[].sources[].description` | string |         | What this source covers                             |

The engine does NOT call research tools directly. It injects instructions into the agent prompt so the agent knows WHAT to use, WHEN, and HOW. This makes the system agnostic to any specific RAG provider, MCP server, or search tool.

**Example:**

```yaml
research:
  enabled: true
  strategy: "always"
  tools:
    - name: "Project RAG"
      type: "rag"
      priority: 1
      enabled: true
      description: "Project knowledge base with indexed docs"
      when_to_use: "First choice for known libraries"
      how_to_use: "rag_search(query='keywords', source_id='id')"
      sources:
        - name: "React"
          id: "src_react"
          description: "UI library docs"
    - name: "WebSearch"
      type: "search"
      priority: 2
      enabled: true
      description: "Broad web search"
      when_to_use: "When RAG has no answer"
      how_to_use: "Search with 2-5 keyword query"
```

## Environment Variables

Every key maps to an env var with `RALPH_` prefix and underscores:

| Config key                     | Environment variable                 |
| ------------------------------ | ------------------------------------ |
| `agent.type`                   | `RALPH_AGENT_TYPE`                   |
| `agent.model`                  | `RALPH_AGENT_MODEL`                  |
| `circuit_breaker.max_failures` | `RALPH_CIRCUIT_BREAKER_MAX_FAILURES` |
| `resources.min_free_ram_mb`    | `RALPH_RESOURCES_MIN_FREE_RAM_MB`    |
| `ssh.enabled`                  | `RALPH_SSH_ENABLED`                  |
