# Referência da CLI

A base atual em Rust expõe uma superfície mínima de CLI enquanto o runtime é reconstruído.

## Comandos

```bash
ralph-engine
ralph-engine --locale <locale-id>
ralph-engine --version
ralph-engine --locale <locale-id> --version
ralph-engine agents
ralph-engine agents list
ralph-engine agents show <agent-id>
ralph-engine capabilities
ralph-engine capabilities list
ralph-engine capabilities show <capability-id>
ralph-engine checks
ralph-engine checks list
ralph-engine checks show <check-id>
ralph-engine checks run <check-id>
ralph-engine doctor
ralph-engine doctor runtime
ralph-engine doctor config
ralph-engine doctor apply-config <output-path>
ralph-engine doctor write-config <output-path>
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
ralph-engine runtime agent-plans
ralph-engine runtime provider-plans
ralph-engine runtime check-plans
ralph-engine runtime policy-plans
ralph-engine runtime mcp-plans
ralph-engine runtime patch
ralph-engine runtime patched-config
ralph-engine runtime apply-config <output-path>
ralph-engine runtime write-patched-config <output-path>
ralph-engine prompts
ralph-engine prompts list
ralph-engine prompts show <prompt-id>
ralph-engine prompts asset <prompt-id> <asset-path>
ralph-engine prompts materialize <prompt-id> <output-dir>
ralph-engine templates
ralph-engine templates list
ralph-engine templates show <template-id>
ralph-engine templates asset <template-id> <asset-path>
ralph-engine templates materialize <template-id> <output-dir>
ralph-engine mcp
ralph-engine mcp list
ralph-engine mcp show <server-id>
ralph-engine mcp plan <server-id>
```

A CLI também aceita o flag global `--locale <locale-id>` ou `-L <locale-id>` para que uma única invocação troque o idioma explicitamente sem depender do ambiente. Quando nenhum flag explícito é passado, a CLI continua fazendo fallback para `RALPH_ENGINE_LOCALE` e depois para o contrato tipado de locale padrão.

O comando `plugins show` imprime o contrato imutável do plugin, incluindo lifecycle, fronteira de carregamento, runtime hooks e o estado de ativação resolvido.

A família `agents` imprime o registro tipado de agent runtimes para que integrações oficiais de agente permaneçam explícitas, em vez de ficarem escondidas só em listagens genéricas de capability.

A família `capabilities` imprime o registro tipado de capabilities do runtime para que os providers permaneçam explícitos e modulares.

A família `templates` imprime o registro tipado de templates do runtime para que providers de template permaneçam explícitos e separados das listagens genéricas de capability, enquanto a responsabilidade de scaffolding continua pertencendo ao tooling.

O comando `templates materialize` grava o conjunto de assets embutidos pertencente a um template tipado em um diretório de saída, para que o scaffolding oficial continue explícito, pertencente ao plugin e inspecionável em vez de ficar escondido atrás de comportamento implícito de gerador.

A família `prompts` imprime o registro tipado de prompts do runtime para que providers de prompt permaneçam explícitos e separados das listagens genéricas de capability, enquanto a montagem de prompt continua sendo uma superfície modular do runtime em vez de virar comportamento implícito em comandos locais.

O comando `prompts materialize` grava o conjunto de assets embutidos pertencente a uma superfície tipada de prompt em um diretório de saída, para que assets reutilizáveis de prompt permaneçam explícitos, pertencentes ao plugin e executáveis em vez de ficarem escondidos atrás de comportamento ad hoc de comando.

A família `checks` imprime o registro tipado de checks do runtime para que contribuições de validação de prepare e doctor permaneçam explícitas, em vez de ficarem escondidas só como capabilities genéricas.

O comando `checks run` executa uma verificação tipada do runtime contra a topologia resolvida canônica e retorna um resultado localizado de aprovação ou reprovação com os findings atuais e as ações de remediação, para que os providers oficiais de check deixem de ser só metadado e passem a ser uma superfície executável do runtime.

A família `doctor` imprime o relatório tipado de diagnóstico do runtime, compondo status, issues pendentes e ações de remediação a partir de um snapshot compartilhado do runtime, em vez de espalhar o diagnóstico por lógicas ad hoc em cada comando.

O comando `doctor config` renderiza a mesma configuração de projeto resultante da aplicação do patch de remediação do runtime sobre os defaults atuais, para que o fluxo de diagnóstico aponte diretamente para um alvo de remediação inspecionável em vez de parar só na análise.

O comando `doctor apply-config` persiste esse mesmo alvo de remediação em um caminho de saída, para que o fluxo de diagnóstico produza um artefato concreto de próximo passo em vez de parar em YAML renderizado.

O comando `doctor write-config` continua disponível como alias de compatibilidade para `doctor apply-config`.

A família `config locale` imprime o contrato tipado do locale padrão, para que o i18n da CLI permaneça inspecionável em vez de ficar implícito nos defaults do runtime.

A família `locales` imprime o catálogo tipado de locales suportados, para que cobertura de idioma, nome nativo e regra de fallback para inglês permaneçam explícitos e versionados.

A família `config budgets` imprime o contrato tipado de limites de prompt e contexto, para que os tetos de tokens permaneçam explícitos na fundação do runtime em vez de ficarem escondidos em lógica futura de provider.

A família `hooks` imprime o registro tipado de runtime hooks do runtime para que os providers permaneçam explícitos e modulares.

A família `policies` imprime o registro tipado de policies do runtime para que os providers de policy permaneçam explícitos, inspecionáveis e separados de listagens genéricas de capability.

A família `providers` imprime o registro tipado de providers do runtime para que contribuições de data source, context provider, forge provider e remote control permaneçam explícitas, em vez de ficarem escondidas só na saída genérica de capability.

O comando `mcp show` imprime o contrato tipado de lançamento do MCP, incluindo modelo de processo, policy de lançamento, fronteiras de comando, policy de diretório de trabalho, policy de ambiente e disponibilidade.

O comando `mcp plan` imprime o plano tipado de lançamento derivado desse contrato, para que bootstrap gerenciado por plugin e execução por spawn de processo permaneçam reutilizáveis fora de formatação local do comando.

O comando `runtime show` imprime a topologia resolvida do runtime, incluindo ativação efetiva de plugin, registro de capability, registro de template, registro de prompt, registro de agent runtime, registro de check, registro de provider, registro de policy, registro de hook e enablement de MCP.

O comando `runtime status` imprime o resumo tipado de health do runtime, incluindo providers habilitados e desabilitados em plugins, capabilities, templates, prompts, agent runtimes tipados, checks tipados, providers tipados, policies, runtime hooks e servidores MCP.

O comando `runtime issues` imprime a lista tipada de issues pendentes do runtime e as ações recomendadas, incluindo templates, prompts, agent runtimes tipados, checks tipados, providers tipados, providers de policy e providers de runtime hook desabilitados, em vez de depender de heurísticas locais em cada comando.

O comando `runtime plan` imprime o plano tipado de remediação derivado da topologia resolvida, incluindo enablement de templates, de prompts, de agent runtimes tipados, de checks tipados, de providers tipados, de policy e de hook, para que o próximo passo de enablement permaneça explícito e modular em vez de ser inferido ad hoc na CLI.

O comando `runtime agent-plans` imprime os planos executáveis de bootstrap de agentes que permanecem habilitados no snapshot resolvido do runtime, para que os passos operacionais de inicialização de agentes fiquem visíveis ao lado de topologia, health, issues e remediação.

O comando `runtime provider-plans` imprime os planos executáveis de registro de providers que permanecem habilitados no snapshot resolvido do runtime, para que o registro operacional de providers permaneça visível ao lado de topologia, health, issues, remediação e bootstrap de agentes.

O comando `runtime check-plans` imprime os planos executáveis de verificações tipadas que permanecem habilitados no snapshot resolvido do runtime, para que os passos de execução de `prepare` e `doctor` permaneçam visíveis ao lado de topologia, health, issues, remediação e outros planos do runtime.

O comando `runtime policy-plans` imprime os planos executáveis de enforcement de políticas que permanecem habilitados no snapshot resolvido do runtime, para que os passos de guardrail e enforcement permaneçam visíveis ao lado de topologia, health, issues, remediação e outros planos do runtime.

O comando `runtime mcp-plans` imprime os planos executáveis de lançamento MCP que permanecem habilitados no snapshot resolvido do runtime, para que os passos operacionais de lançamento fiquem visíveis ao lado de topologia, health, issues e remediação.

O comando `runtime patch` renderiza o patch tipado de configuração que remedia a topologia degradada atual, incluindo ativações de plugin e enablement por servidor MCP, para que a recuperação do runtime permaneça explícita e reutilizável em vez de ficar apenas como plano textual.

O comando `runtime patched-config` renderiza a configuração de projeto resultante da aplicação do patch tipado de runtime sobre os defaults atuais, para que o operador possa inspecionar o resultado efetivo da remediação antes de persistir essa configuração em outro lugar.

O comando `runtime apply-config` persiste esse alvo de remediação totalmente materializado em um caminho de saída, para que a recuperação do runtime saia da inspeção e vire um passo explícito e reproduzível de escrita em arquivo.

O comando `runtime write-patched-config` continua disponível como alias de compatibilidade para `runtime apply-config`.
