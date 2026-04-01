# Referência da CLI

A base atual em Rust expõe uma superfície mínima de CLI enquanto o runtime é reconstruído.

## Comandos

```bash
ralph-engine
ralph-engine --version
ralph-engine capabilities
ralph-engine capabilities list
ralph-engine capabilities show <capability-id>
ralph-engine config
ralph-engine config show-defaults
ralph-engine config show-plugin <plugin-id>
ralph-engine plugins
ralph-engine plugins list
ralph-engine plugins show <plugin-id>
ralph-engine runtime
ralph-engine runtime show
ralph-engine mcp
ralph-engine mcp list
ralph-engine mcp show <server-id>
```

O comando `plugins show` imprime o contrato imutável do plugin, incluindo lifecycle, fronteira de carregamento, runtime hooks e o estado de ativação resolvido.

A família `capabilities` imprime o registro tipado de capabilities do runtime para que os providers permaneçam explícitos e modulares.

O comando `mcp show` imprime o contrato tipado de lançamento do MCP, incluindo modelo de processo, policy de lançamento, fronteiras de comando, policy de diretório de trabalho, policy de ambiente e disponibilidade.

O comando `runtime show` imprime a topologia resolvida do runtime, incluindo ativação efetiva de plugin, registro de capability e enablement de MCP.
