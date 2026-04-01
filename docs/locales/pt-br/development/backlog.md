# Backlog

Atualizado: 2026-04-01

- Expandir `re-cli` além da superfície inicial de bootstrap.
- Expandir o contrato de `re-mcp` além de metadados imutáveis de servidor para configuração de transporte, inicialização de processo e fronteiras de policy.
- Expandir o contrato de `re-plugin` além de metadados tipados, lifecycle, estado de ativação, fronteiras de carregamento e runtime hooks para resolução de configuração e registro de runtime.
- Adicionar empacotamento cross-platform para os artefatos de release.
- Adicionar validação explícita para `dist-workspace.toml` e para os artefatos gerados por release.
- Restaurar exemplos mais ricos nas docs quando os contratos do runtime estiverem de volta.
- Definir contratos de i18n para mensagens da CLI, docs, site e plugins com inglês e pt-BR como idiomas de primeira classe.
- Definir metas de performance, acessibilidade e SEO para as superfícies públicas, para que site, docs e plugins continuem em baseline A.
- Definir a arquitetura de informação, menus e contratos de navegação compartilhados entre `/`, `/docs` e `/plugins`.
- Refatorar `site/ui` para virar uma camada real de apresentação compartilhada entre `site/landing` e `site/plugins`.
- Alinhar o contrato do header público para que o logo sempre volte para `/` e a navegação continue coerente entre site, docs e plugins.
- Acompanhar o advisory moderado de `vitepress`/`vite`/`esbuild` na toolchain das docs e subir para a próxima linha estável revisada assim que existir correção sem quebra desnecessária.
