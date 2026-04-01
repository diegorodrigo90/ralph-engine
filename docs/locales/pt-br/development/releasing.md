# Releases

## Fluxo

1. Faça merge de mudanças revisadas com Conventional Commits em `main`.
2. `release-plz release-pr` abre ou atualiza a PR de release.
3. Faça merge da PR de release.
4. A publicação de release fica desativada até o pipeline de distribuição em Rust estar fechado e validado ponta a ponta.
5. O workflow endurecido de publicação vai criar a tag SemVer só depois de os gates obrigatórios passarem para o commit alvo da `main`.

## Workflow manual de publicação

O workflow `Release` é manual e espera:

- `tag`
- `publish_github_release`
- `publish_npm`
- `publish_homebrew`
- `homebrew_tap_repository` quando o tap não for inferido por outro meio

Secrets usados por esse workflow:

- `NPM_TOKEN` quando `publish_npm=true`
- `HOMEBREW_TAP_TOKEN` quando `publish_homebrew=true`

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
- O npm DEVE instalar a partir de assets revisados do `cargo-dist` e verificar o checksum `.sha256` publicado antes da extração.
- O Homebrew DEVE ser derivado dos mesmos assets e checksums do `cargo-dist` usados pelo canal npm.
- A publicação automática NÃO DEVE sair da `main` enquanto GitHub Releases, npm e Homebrew não estiverem conectados ao pipeline em Rust.
