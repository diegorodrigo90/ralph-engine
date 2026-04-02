# official/github

GitHub integration plugin for data, context, forge, and MCP surfaces.

## Surfaces

- Kind: `data_source`
- Runtime hooks:
  - `mcp_registration`
  - `data_source_registration`
  - `context_provider_registration`
  - `forge_provider_registration`
- MCP server contributions:
  - `official.github.mcp`
- Provider contributions:
  - `official.github.data`
  - `official.github.context`
  - `official.github.forge`

## What it owns

- the typed GitHub provider descriptors exported by the crate
- the MCP server contract for GitHub-backed runtime integrations
- localized plugin metadata
