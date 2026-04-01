# Roadmap

Atualizado: 2026-04-01

## Concluído

- [x] Reiniciar o repositório sobre uma fundação em Rust.
- [x] Pinar Rust, Node, commitlint, hooks e as ferramentas principais de validação.
- [x] Estabelecer SemVer + Conventional Commits + release-plz como modelo de release.
- [x] Mover o contrato de validação para scripts no nível do repositório.
- [x] Estabelecer a base inicial de `cargo-dist` no workspace para os artefatos de release em Rust.

## Próximos passos

- [ ] Reconstruir o core do runtime em Rust com TDD e 100% de cobertura significativa.
- [ ] Reintroduzir configuração, state, MCP e ciclo de vida de plugins no novo core.
- [ ] Ligar a publicação em npm e Homebrew ao pipeline de release em Rust.
- [ ] Endurecer o publish do `cargo-dist` ponta a ponta com GitHub Releases, atestações, SBOM, npm e Homebrew.
- [ ] Reconstruir os plugins oficiais sobre os novos contratos em Rust.
- [ ] Introduzir suporte bilíngue de primeira classe para CLI, docs, site e superfície de plugins em inglês e pt-BR.
- [ ] Consolidar um sistema coerente de UX entre site, docs e plugins com navegação clara e baseline A de acessibilidade, performance e SEO.
