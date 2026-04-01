# Backlog

Atualizado: 2026-04-01

- Expandir `re-cli` além da superfície inicial de bootstrap.
- Expandir o contrato de `re-mcp` de metadados tipados de lançamento para contratos mais ricos de registro e orquestração de runtime, sem voltar para branching baseado em strings.
- Expandir os contratos do runtime além de metadados tipados, escopos de ativação, registro de capability, lifecycle, fronteiras de carregamento, runtime hooks, registro de topologia e health status para uma orquestração de estado mais rica.
- Adicionar empacotamento cross-platform para os artefatos de release.
- Adicionar validação explícita para `dist-workspace.toml` e para os artefatos gerados por release.
- Restaurar exemplos mais ricos nas docs quando os contratos do runtime estiverem de volta.
- Definir contratos de i18n para mensagens da CLI, docs, site e plugins com inglês e pt-BR como idiomas de primeira classe.
- Definir metas de performance, acessibilidade e SEO para as superfícies públicas, para que site, docs e plugins continuem em baseline A.
- Definir a arquitetura de informação, menus e contratos de navegação compartilhados entre `/`, `/docs` e `/plugins`.
- Refatorar `site/ui` para virar uma camada real de apresentação compartilhada entre `site/landing` e `site/plugins`.
- Alinhar o contrato do header público para que o logo sempre volte para `/` e a navegação continue coerente entre site, docs e plugins.
- Acompanhar o advisory moderado de `vitepress`/`vite`/`esbuild` na toolchain das docs e subir para a próxima linha estável revisada assim que existir correção sem quebra desnecessária.
