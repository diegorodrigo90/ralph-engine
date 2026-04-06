---
title: "Arquitetura"
description: "Arquitetura interna para contribuidores"
---

## Posicionamento

Ralph Engine é um runtime open-source, orientado a plugins, para fluxos de desenvolvimento com agentes.

## Estrutura do Repositório

- `core/` — crates Rust do runtime
- `plugins/official/` — plugins oficiais em Rust
- `site/` — site de documentação Astro Starlight, superfícies públicas da web, UI compartilhada e metadados de plugins
- `site/src/content/docs/` — fonte da documentação (EN + PT-BR)
- `packaging/` — superfícies de empacotamento npm e Homebrew
- `tools/create-ralph-engine/` — scaffolding de plugin para `npx create-ralph-engine-plugin`
- `scripts/` — bootstrap, validação e automação de release

## Workspace Rust

- `re-core` — fundações compartilhadas do runtime, topologia e contratos de estado
- `re-config` — contratos, escopos, padrões e regras de resolução compartilhados de configuração do runtime
- `re-mcp` — contratos compartilhados de contribuições MCP, policy de lançamento, modelo de processo, fronteiras de comando e policy
- `re-plugin` — contratos compartilhados de metadados, lifecycle, runtime hooks, fronteira de carregamento e capabilities de plugin
- `re-official` — catálogo tipado embutido que conecta plugins oficiais e servidores MCP em um snapshot reutilizável do runtime
- `re-cli` — crate CLI que produz `ralph-engine`
- Crates de plugins oficiais ficam em `plugins/official/*`

## Modelo de Integração com Agentes

Plugins de agente lançam o binário CLI do próprio agente como subprocesso (`claude -p`, `codex exec`, etc.). A fronteira de integração é o stream stdout do agente — o Ralph Engine lê eventos stream-json para exibir o progresso na TUI. Ele nunca acessa credenciais do agente, intercepta chamadas de API ou atua como proxy. O binário do agente cuida da sua própria autenticação, cobrança e comunicação com a API.

Isso é equivalente a um shell script executando `claude -p "prompt"` — o Ralph Engine adiciona orquestração (resolução de work items, montagem de prompt, quality gates) em torno dessa mesma chamada de subprocesso.

## Regras Arquiteturais

- O core permanece plugin-first e agnóstico de workflow.
- MCP externo permanece como parte nativa da arquitetura.
- Plugins oficiais são em Rust.
- Plugins de terceiros permanecem agnósticos de linguagem.
- Prompt, contexto, governança de MCP, segurança e diagnósticos permanecem como preocupações do core.
- Famílias de comandos da CLI evoluem por módulos e registries isolados, não por um dispatcher central cada vez maior.
- Capabilities de plugin e contribuições MCP evoluem por descritores tipados para que novas capabilities possam ser adicionadas sem lógica acoplada por string no runtime.
- O lifecycle de plugin evolui por estágios tipados compartilhados para que descoberta, configuração, validação e carregamento permaneçam explícitos e extensíveis.
- Runtime hooks de plugin evoluem por descritores tipados compartilhados para que prepare, doctor, prompt, agent, MCP e policy permaneçam modulares sem dispatch ad hoc.
- A resolução de configuração evolui por escopos tipados em camadas para que defaults e futuros overrides permaneçam explícitos em vez de inferidos dentro dos comandos.
- Topologia do runtime, saúde, reporting de issues, reporting de doctor, plano de ações do runtime e registro de runtime hooks evoluem por registros tipados e contratos compartilhados para que ativação de plugin, registro de capability, registro de hook e enablement de MCP permaneçam explícitos em vez de reconstruídos ad hoc por comando.
- Capabilities desabilitadas e runtime hooks desabilitados permanecem visíveis no health e na remediação do runtime; não se tornam metadado invisível só porque a topologia resolveu.
- Fronteiras de carregamento de plugin permanecem tipadas para que integração in-process, subprocess e remota possam evoluir sem branching ad hoc no runtime.

## Pipeline do Comando Run

O comando `run` orquestra a execução de itens de trabalho por meio de um pipeline de cinco etapas:

1. **Verificar agente** — chamar `bootstrap_agent()` no plugin de agente para verificar se o binário está instalado e pronto.
2. **Resolver item de trabalho** — chamar `resolve_work_item()` no plugin de workflow. Retorna o ID canônico, título, caminho de origem e metadados.
3. **Montar prompt** — chamar `build_prompt_context()` no plugin de workflow, depois enriquecer com ferramentas descobertas automaticamente e contribuições de prompt dos plugins.
4. **Exibir info de lançamento** — mostrar o item de trabalho e o agente para o usuário.
5. **Lançar agente** — chamar `launch_agent()` no plugin de agente com o `PromptContext` montado.

### Montagem do Prompt

O prompt é montado em camadas:

- **Contexto da tarefa** — o plugin de workflow lê o arquivo do item de trabalho (story, issue, spec) e constrói o prompt base com descrição da tarefa, critérios de aceitação e regras relevantes do projeto.
- **Contribuições de plugins** — o `prompt_contributions()` de cada plugin habilitado é chamado. Contribuições são adicionadas ao texto do prompt e rastreadas como arquivos de contexto. O plugin `official.findings` usa esse mecanismo para injetar findings anteriores.
- **Restrições** — restrições definidas pelo workflow (quality gates, padrões de código) são adicionadas por último.

### Auto-Discovery de Ferramentas

Em vez de exigir que o usuário liste todas as ferramentas que um agente precisa, o comando `run` coleta ferramentas de todos os plugins habilitados:

1. Cada plugin implementa `required_tools()` retornando nomes ou padrões de ferramentas necessárias (ex: padrões de ferramentas MCP).
2. O core coleta de todos os plugins habilitados, remove duplicatas e mescla com ferramentas configuradas em `.ralph-engine/config.yaml`.
3. A lista mesclada é passada ao plugin de agente via `PromptContext.discovered_tools`.

Isso significa que instalar um plugin que precisa de ferramentas MCP específicas torna automaticamente essas ferramentas disponíveis para o agente.

### Loop de Feedback

O plugin `official.findings` cria um loop de feedback entre sessões de agente:

1. Após uma execução, findings (issues de code review, falhas de quality gate, aprendizados) são escritos em `.ralph-engine/findings.md`.
2. Na execução seguinte, o plugin de findings lê esse arquivo e injeta como uma seção `<findings>` no prompt.
3. O agente vê erros anteriores antes de implementar, reduzindo erros repetidos.

O formato do arquivo é definido pelo projeto — o plugin lê e injeta sem fazer parsing.
