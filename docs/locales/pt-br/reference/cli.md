# Referência da CLI

A base atual em Rust expõe uma superfície mínima de CLI enquanto o runtime é reconstruído.

## Comandos

```bash
ralph-engine
ralph-engine --version
ralph-engine capabilities
ralph-engine capabilities list
ralph-engine capabilities show <capability-id>
ralph-engine doctor
ralph-engine doctor runtime
ralph-engine hooks
ralph-engine hooks list
ralph-engine hooks show <hook-id>
ralph-engine policies
ralph-engine policies list
ralph-engine policies show <policy-id>
ralph-engine config
ralph-engine config budgets
ralph-engine config layers
ralph-engine config show-budgets
ralph-engine config show-defaults
ralph-engine config show-layers
ralph-engine config show-plugin <plugin-id>
ralph-engine plugins
ralph-engine plugins list
ralph-engine plugins show <plugin-id>
ralph-engine runtime
ralph-engine runtime show
ralph-engine runtime status
ralph-engine runtime issues
ralph-engine runtime plan
ralph-engine mcp
ralph-engine mcp list
ralph-engine mcp show <server-id>
```

O comando `plugins show` imprime o contrato imutável do plugin, incluindo lifecycle, fronteira de carregamento, runtime hooks e o estado de ativação resolvido.

A família `capabilities` imprime o registro tipado de capabilities do runtime para que os providers permaneçam explícitos e modulares.

A família `doctor` imprime o relatório tipado de diagnóstico do runtime, compondo status, issues pendentes e ações de remediação a partir de um snapshot compartilhado do runtime, em vez de espalhar o diagnóstico por lógicas ad hoc em cada comando.

A família `config budgets` imprime o contrato tipado de limites de prompt e contexto, para que os tetos de tokens permaneçam explícitos na fundação do runtime em vez de ficarem escondidos em lógica futura de provider.

A família `hooks` imprime o registro tipado de runtime hooks do runtime para que os providers permaneçam explícitos e modulares.

A família `policies` imprime o registro tipado de policies do runtime para que os providers de policy permaneçam explícitos, inspecionáveis e separados de listagens genéricas de capability.

O comando `mcp show` imprime o contrato tipado de lançamento do MCP, incluindo modelo de processo, policy de lançamento, fronteiras de comando, policy de diretório de trabalho, policy de ambiente e disponibilidade.

O comando `runtime show` imprime a topologia resolvida do runtime, incluindo ativação efetiva de plugin, registro de capability, registro de policy, registro de hook e enablement de MCP.

O comando `runtime status` imprime o resumo tipado de health do runtime, incluindo providers habilitados e desabilitados em plugins, capabilities, policies, runtime hooks e servidores MCP.

O comando `runtime issues` imprime a lista tipada de issues pendentes do runtime e as ações recomendadas, incluindo providers de policy e de runtime hook desabilitados, em vez de depender de heurísticas locais em cada comando.

O comando `runtime plan` imprime o plano tipado de remediação derivado da topologia resolvida, incluindo enablement de providers de policy e de hook, para que o próximo passo de enablement permaneça explícito e modular em vez de ser inferido ad hoc na CLI.
