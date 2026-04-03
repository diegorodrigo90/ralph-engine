# official.ssh

SSH remote control integration.

## Overview

Enables remote command execution over SSH for Ralph Engine workflows. This plugin provides a typed remote control provider that allows agents to execute commands on remote hosts, transfer files, and manage remote environments — all within the safety of Ralph Engine's plugin system.

## How it works

The SSH plugin exposes a remote control provider (`official.ssh.remote`) that agents can use to:

- Execute commands on remote hosts via SSH
- Transfer files between local and remote machines
- Check connectivity and remote environment health
- Manage remote services (start/stop/restart)

## Use cases

### DevContainer integration

Run tests, migrations, and development servers inside a DevContainer while the agent edits files locally. The SSH plugin bridges the gap between the agent's local environment and the containerized services.

### Multi-machine development

For projects that span multiple machines (e.g., frontend on local, backend on a remote server), the SSH plugin enables the agent to operate across host boundaries seamlessly.

### Deployment workflows

Execute deployment scripts on production or staging servers as part of an automated Ralph Engine workflow.

## Requirements

- SSH client installed on the local machine
- SSH key-based authentication configured for target hosts
- A valid Ralph Engine project

## Security

All SSH operations go through Ralph Engine's plugin system, which means they're auditable and can be gated by policy plugins. The SSH plugin never stores credentials — it relies on your system's SSH agent and key configuration.
