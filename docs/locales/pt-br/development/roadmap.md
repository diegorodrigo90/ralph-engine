# Roadmap

Atualizado: 2026-04-02

## Concluído

- [x] Reiniciar o repositório sobre uma fundação em Rust.
- [x] Pinar Rust, Node, commitlint, hooks e as ferramentas principais de validação.
- [x] Estabelecer SemVer + Conventional Commits + release-plz como modelo de release.
- [x] Mover o contrato de validação para scripts no nível do repositório.
- [x] Estabelecer a base inicial de `cargo-dist` no workspace para os artefatos de release em Rust.
- [x] Introduzir suporte bilíngue de primeira classe para CLI, docs, site e superfície de plugins em inglês e pt-BR.
- [x] Consolidar um sistema coerente de UX entre site, docs e plugins com navegação clara e baseline A de acessibilidade, performance e SEO.
- [x] Reconstruir a superfície pública de inspeção da CLI e do runtime sobre contratos tipados em Rust para capabilities, templates, prompts, agents, checks, providers, policies, hooks, MCP, health, issues e planos de remediação.
- [x] Estabelecer um contrato tipado compartilhado de locale em `re-config`, para que CLI, crates de runtime, plugins oficiais e plugins gerados cresçam a partir da mesma base canônica com fallback para inglês.
- [x] Endurecer o pipeline do GitHub Actions em torno de `Quality`, `Security`, `SonarCloud`, artefatos reutilizáveis aprovados e releases no modelo promote-later.

## Próximos passos

- [ ] Levar o runtime de metadados tipados e diagnósticos para uma orquestração executável mais rica e tratamento de estado, com TDD e 100% de cobertura significativa.
- [ ] Reconstruir os plugins oficiais além dos descritores tipados, para que executem comportamento real sobre os novos contratos do runtime em Rust.
- [ ] Fechar o pipeline ponta a ponta de release para GitHub Releases, npm e Homebrew com provenance, checksums, atestações e gates finais de publish.
- [x] Adicionar validação explícita para `dist-workspace.toml` e para os artefatos gerados por release.
- [ ] Restaurar exemplos mais ricos do runtime nas docs quando a camada executável estiver mais avançada.
