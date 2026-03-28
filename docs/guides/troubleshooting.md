# Troubleshooting

## Installation Issues

### npm install fails to download binary

**Symptom:** `npm install -g ralph-engine` shows "failed to install binary" message.

**Causes and fixes:**

1. **No internet access** — The postinstall script downloads from GitHub Releases. Check connectivity:

   ```bash
   curl -I https://github.com/diegorodrigo90/ralph-engine/releases
   ```

2. **Behind a corporate proxy** — Set npm proxy config:

   ```bash
   npm config set proxy http://proxy.example.com:8080
   npm config set https-proxy http://proxy.example.com:8080
   ```

3. **No releases published yet** — If this is a development build, install from source instead:

   ```bash
   go install github.com/diegorodrigo90/ralph-engine/cmd/ralph-engine@latest
   ```

4. **Architecture not supported** — Only amd64 and arm64 are supported. Check yours:
   ```bash
   node -e "console.log(process.arch)"
   ```

### pnpm / yarn install

ralph-engine works with all Node.js package managers:

```bash
# pnpm
pnpm add -g ralph-engine
pnpm dlx ralph-engine run --dry-run

# yarn
yarn global add ralph-engine

# npm
npm install -g ralph-engine
npx ralph-engine run --dry-run

# bun
bun add -g ralph-engine
bunx ralph-engine run --dry-run
```

### Homebrew formula not found

**Symptom:** `brew install diegorodrigo90/tap/ralph-engine` fails.

**Fix:** Add the tap first:

```bash
brew tap diegorodrigo90/tap
brew install ralph-engine
```

### Go install fails

**Symptom:** `go install` gives "module not found" error.

**Causes:**

1. **Go version too old** — Need 1.24+. Check: `go version`
2. **GOPATH/bin not in PATH:**
   ```bash
   export PATH=$PATH:$(go env GOPATH)/bin
   echo 'export PATH=$PATH:$(go env GOPATH)/bin' >> ~/.bashrc
   ```

### Permission denied on Linux/macOS

**Symptom:** `ralph-engine: Permission denied`

**Fix:**

```bash
chmod +x $(which ralph-engine)
# Or if installed via curl:
sudo chmod +x /usr/local/bin/ralph-engine
```

## Runtime Issues

### "Agent binary not found"

**Symptom:** `ralph-engine prepare` says agent binary is missing.

**Fix:** Install your AI agent and make sure it's in PATH:

```bash
# Claude Code
which claude

# ClaudeBox
which claudebox

# Custom binary — specify in config
ralph-engine config set agent.type /path/to/my-agent
```

### "No stories found" or tracker errors

**Symptom:** Engine starts but says there are no stories.

**Causes:**

1. **Wrong status file path** — Check your config:

   ```bash
   ralph-engine config list | grep status_file
   ```

   Default: `sprint-status.yaml` in the project root.

2. **Wrong YAML format** — ralph-engine auto-detects two formats:

   **Structured format** (epics + stories):

   ```yaml
   epics:
     - id: "1"
       title: "My Epic"
       stories:
         - id: "1.1"
           title: "First Story"
           status: "ready-for-dev"
   ```

   **Flat format** (BMAD v6):

   ```yaml
   development_status:
     1-1-first-story: ready-for-dev
     1-2-second-story: done
   ```

3. **All stories are done** — Check: `ralph-engine status`

### Circuit breaker triggered

**Symptom:** Engine stops with "circuit breaker: too many consecutive failures."

**Cause:** The agent failed N times in a row (default: 3).

**Fix:**

1. Check the agent's error output in logs
2. Fix the underlying issue (tests failing, build broken, etc.)
3. Resume: `ralph-engine run`
4. Increase threshold if needed: `ralph-engine run --max-failures 5`

### Resource check failures

**Symptom:** Engine pauses or stops due to resource limits.

**Defaults:**
| Resource | Warning | Stop |
|----------|---------|------|
| RAM | < 2 GB free | < 1 GB free |
| CPU | > 80% load | > 95% load |
| Disk | < 5 GB free | < 2 GB free |

**Fix:** Free resources or adjust thresholds in config:

```yaml
resources:
  min_free_ram_mb: 1024
  max_cpu_load_percent: 90
  min_free_disk_gb: 2
```

### Usage limit detected

**Symptom:** Engine saves progress and stops with "usage limit reached."

**Cause:** The AI agent's API hit a rate limit or billing limit.

**Fix:**

1. Wait for the limit to reset, or upgrade your plan
2. Resume exactly where you stopped: `ralph-engine run`
3. Progress is saved in `.ralph-engine/state.json`

### Remote execution failures

**Symptom:** Remote quality gates are skipped.

**Cause:** Remote connection (SSH or other) is not configured or dropped.

**Fix:**

1. Run your reconnect script (configured in `ssh.reconnect_script`)
2. Verify the connection: `ssh your-remote-host echo ok`
3. Engine will auto-reconnect on next iteration

### State file corrupted

**Symptom:** Engine crashes on startup with JSON parse error.

**Fix:**

```bash
# Remove state file — engine starts fresh
rm .ralph-engine/state.json

# Resume from tracker (sprint-status.yaml tracks progress independently)
ralph-engine run
```

No progress is lost — the tracker file is the source of truth for story status.

## Debug Mode

For detailed diagnostics:

```bash
# JSON structured output — great for AI agent analysis
ralph-engine --debug run

# JSON logs without full debug verbosity
ralph-engine --log-format json run
```

Debug output includes:

- `component` — Which module produced the log
- `suggestion` — Suggested fix for errors
- `docs` — Link to relevant documentation

## Getting Help

1. Check [GitHub Issues](https://github.com/diegorodrigo90/ralph-engine/issues) for known issues
2. Open a new issue with:
   - `ralph-engine version` output
   - OS and architecture
   - Full error message
   - Steps to reproduce
3. For security issues, see [SECURITY.md](https://github.com/diegorodrigo90/ralph-engine/blob/main/.github/SECURITY.md)
