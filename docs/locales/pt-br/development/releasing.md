# Releases

## Fluxo

1. Faça merge de mudanças revisadas com Conventional Commits em `main`.
2. `release-plz release-pr` abre ou atualiza a PR de release.
3. Faça merge da PR de release.
4. A publicação de release fica desativada até o pipeline de distribuição em Rust estar fechado e validado ponta a ponta.
5. O workflow endurecido de publicação vai criar a tag SemVer só depois de os gates obrigatórios passarem para o commit alvo da `main`.

## Regras

- SemVer é obrigatório.
- Conventional Commits são obrigatórios.
- Actions são pinadas por SHA.
- As ferramentas de release são pinadas em versões revisadas.
- O contrato de release SHALL rodar via `./scripts/validate.sh --mode release` antes da publicação.
- `cargo-dist` DEVE ser o builder de artefatos Rust para a distribuição de release.
- `Quality`, `Security` e `SonarCloud` DEVEM passar antes da criação de uma tag de release.
- `SONAR_TOKEN` DEVE apontar para um token do SonarCloud com permissão para navegar e analisar o projeto alvo.
- Checksums, SBOMs e atestações de artefato fazem parte do contrato-alvo de release.
- npm e Homebrew continuam canais oficiais, mas ainda não estão integrados.
- A publicação automática NÃO DEVE sair da `main` enquanto GitHub Releases, npm e Homebrew não estiverem conectados ao pipeline em Rust.
