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

## Environment Variables

Every key maps to an env var with `RALPH_` prefix and underscores:

| Config key                     | Environment variable                 |
| ------------------------------ | ------------------------------------ |
| `agent.type`                   | `RALPH_AGENT_TYPE`                   |
| `agent.model`                  | `RALPH_AGENT_MODEL`                  |
| `circuit_breaker.max_failures` | `RALPH_CIRCUIT_BREAKER_MAX_FAILURES` |
| `resources.min_free_ram_mb`    | `RALPH_RESOURCES_MIN_FREE_RAM_MB`    |
| `ssh.enabled`                  | `RALPH_SSH_ENABLED`                  |
