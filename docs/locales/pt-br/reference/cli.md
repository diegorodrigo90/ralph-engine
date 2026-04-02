# Referência da CLI

A base atual em Rust expõe uma superfície mínima de CLI enquanto o runtime é reconstruído.

## Comandos

```bash
ralph-engine
ralph-engine --version
ralph-engine agents
ralph-engine agents list
ralph-engine agents show <agent-id>
ralph-engine capabilities
ralph-engine capabilities list
ralph-engine capabilities show <capability-id>
ralph-engine checks
ralph-engine checks list
ralph-engine checks show <check-id>
ralph-engine doctor
ralph-engine doctor runtime
ralph-engine doctor config
ralph-engine hooks
ralph-engine hooks list
ralph-engine hooks show <hook-id>
ralph-engine locales
ralph-engine locales list
ralph-engine locales show <locale-id>
ralph-engine policies
ralph-engine policies list
ralph-engine policies show <policy-id>
ralph-engine providers
ralph-engine providers list
ralph-engine providers show <provider-id>
ralph-engine config
ralph-engine config budgets
ralph-engine config layers
ralph-engine config locale
ralph-engine config show-budgets
ralph-engine config show-defaults
ralph-engine config show-layers
ralph-engine config show-locale
ralph-engine config show-mcp-server <server-id>
ralph-engine config show-plugin <plugin-id>
ralph-engine plugins
ralph-engine plugins list
ralph-engine plugins show <plugin-id>
ralph-engine runtime
ralph-engine runtime show
ralph-engine runtime status
ralph-engine runtime issues
ralph-engine runtime plan
ralph-engine runtime patch
ralph-engine runtime patched-config
ralph-engine prompts
ralph-engine prompts list
ralph-engine prompts show <prompt-id>
ralph-engine templates
ralph-engine templates list
ralph-engine templates show <template-id>
ralph-engine mcp
ralph-engine mcp list
ralph-engine mcp show <server-id>
```

O comando `plugins show` imprime o contrato imutável do plugin, incluindo lifecycle, fronteira de carregamento, runtime hooks e o estado de ativação resolvido.

A família `agents` imprime o registro tipado de agent runtimes para que integrações oficiais de agente permaneçam explícitas, em vez de ficarem escondidas só em listagens genéricas de capability.

A família `capabilities` imprime o registro tipado de capabilities do runtime para que os providers permaneçam explícitos e modulares.

A família `templates` imprime o registro tipado de templates do runtime para que providers de template permaneçam explícitos e separados das listagens genéricas de capability, enquanto a responsabilidade de scaffolding continua pertencendo ao tooling.

A família `prompts` imprime o registro tipado de prompts do runtime para que providers de prompt permaneçam explícitos e separados das listagens genéricas de capability, enquanto a montagem de prompt continua sendo uma superfície modular do runtime em vez de virar comportamento implícito em comandos locais.

A família `checks` imprime o registro tipado de checks do runtime para que contribuições de validação de prepare e doctor permaneçam explícitas, em vez de ficarem escondidas só como capabilities genéricas.

A família `doctor` imprime o relatório tipado de diagnóstico do runtime, compondo status, issues pendentes e ações de remediação a partir de um snapshot compartilhado do runtime, em vez de espalhar o diagnóstico por lógicas ad hoc em cada comando.

O comando `doctor config` renderiza a mesma configuração de projeto resultante da aplicação do patch de remediação do runtime sobre os defaults atuais, para que o fluxo de diagnóstico aponte diretamente para um alvo de remediação inspecionável em vez de parar só na análise.

A família `config locale` imprime o contrato tipado do locale padrão, para que o i18n da CLI permaneça inspecionável em vez de ficar implícito nos defaults do runtime.

A família `locales` imprime o catálogo tipado de locales suportados, para que cobertura de idioma, nome nativo e regra de fallback para inglês permaneçam explícitos e versionados.

A família `config budgets` imprime o contrato tipado de limites de prompt e contexto, para que os tetos de tokens permaneçam explícitos na fundação do runtime em vez de ficarem escondidos em lógica futura de provider.

A família `hooks` imprime o registro tipado de runtime hooks do runtime para que os providers permaneçam explícitos e modulares.

A família `policies` imprime o registro tipado de policies do runtime para que os providers de policy permaneçam explícitos, inspecionáveis e separados de listagens genéricas de capability.

A família `providers` imprime o registro tipado de providers do runtime para que contribuições de data source, context provider, forge provider e remote control permaneçam explícitas, em vez de ficarem escondidas só na saída genérica de capability.

O comando `mcp show` imprime o contrato tipado de lançamento do MCP, incluindo modelo de processo, policy de lançamento, fronteiras de comando, policy de diretório de trabalho, policy de ambiente e disponibilidade.

O comando `runtime show` imprime a topologia resolvida do runtime, incluindo ativação efetiva de plugin, registro de capability, registro de template, registro de prompt, registro de agent runtime, registro de check, registro de provider, registro de policy, registro de hook e enablement de MCP.

O comando `runtime status` imprime o resumo tipado de health do runtime, incluindo providers habilitados e desabilitados em plugins, capabilities, templates, prompts, agent runtimes tipados, checks tipados, providers tipados, policies, runtime hooks e servidores MCP.

O comando `runtime issues` imprime a lista tipada de issues pendentes do runtime e as ações recomendadas, incluindo templates, prompts, agent runtimes tipados, checks tipados, providers tipados, providers de policy e providers de runtime hook desabilitados, em vez de depender de heurísticas locais em cada comando.

O comando `runtime plan` imprime o plano tipado de remediação derivado da topologia resolvida, incluindo enablement de templates, de prompts, de agent runtimes tipados, de checks tipados, de providers tipados, de policy e de hook, para que o próximo passo de enablement permaneça explícito e modular em vez de ser inferido ad hoc na CLI.

O comando `runtime patch` renderiza o patch tipado de configuração que remedia a topologia degradada atual, incluindo ativações de plugin e enablement por servidor MCP, para que a recuperação do runtime permaneça explícita e reutilizável em vez de ficar apenas como plano textual.

O comando `runtime patched-config` renderiza a configuração de projeto resultante da aplicação do patch tipado de runtime sobre os defaults atuais, para que o operador possa inspecionar o resultado efetivo da remediação antes de persistir essa configuração em outro lugar.
