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

O campo `tag` DEVE incluir o prefixo `v`, por exemplo `v0.2.0-alpha.1`. O workflow remove esse prefixo antes de preparar as versões dos pacotes npm.
Antes de publicar qualquer coisa, o workflow valida que o SHA selecionado é o HEAD atual de `origin/main` e que o workflow canônico de `CI` já terminou com sucesso exatamente para esse push.
O workflow também rejeita `publish_npm=true` ou `publish_homebrew=true` quando `publish_github_release=false`, porque os dois canais downstream dependem do conjunto revisado de assets do GitHub Release para a tag selecionada.
Quando `publish_npm=true`, o workflow também roda `npm pack --json --dry-run` sobre os payloads staged de `ralph-engine` e `create-ralph-engine-plugin` e rejeita a publicação se entradas obrigatórias, `bin`, scripts, nomes de pacote ou versões reescritas estiverem incorretos.
Esse mesmo workflow de `CI` gera candidates cross-platform de release em paralelo com os gates de qualidade e só publica os artifacts reutilizáveis aprovados para o SHA depois de `Quality`, `Security` e `SonarCloud` terem passado.
Tanto o contrato revisado de `dist-workspace.toml` quanto os assets gerados por release passam por validação explícita antes que os artefatos sejam aprovados ou promovidos.
O quality gate do SonarCloud também é a trava dura de cobertura: se ele cair abaixo da meta configurada de `100%` para o código analisado, o SHA não é aprovado para publicação de artifacts nem para promoção de release.

## Regras

- SemVer é obrigatório.
- Conventional Commits são obrigatórios.
- Actions são pinadas por SHA.
- As ferramentas de release são pinadas em versões revisadas.
- O workflow de release DEVE validar o SHA alvo da `main` contra o workflow canônico de `CI` antes de publicar artefatos.
- O workflow de release DEVE reaproveitar a evidência de `CI` verde desse mesmo SHA da `main`, em vez de rerodar o contrato completo de validação dentro do fluxo de publish.
- O workflow de release DEVE rejeitar publicação de canais downstream quando `publish_github_release=false`.
- O workflow de release DEVE validar que o GitHub Release da tag selecionada existe antes de iniciar publicação em npm ou Homebrew.
- O workflow de release DEVE validar os tarballs staged de npm antes de publicar canais npm.
- O workflow canônico de `CI` DEVE gerar candidates cross-platform de release para o SHA alvo da `main` em paralelo com os gates de qualidade.
- O workflow canônico de `CI` DEVE publicar os artifacts reutilizáveis aprovados para esse SHA só depois de `Quality`, `Security` e `SonarCloud` terem passado.
- O workflow de release DEVE baixar e publicar esse mesmo conjunto aprovado de artifacts, em vez de rebuildá-lo.
- `scripts/verify-dist-workspace.sh` DEVE validar o contrato revisado do workspace `cargo-dist` antes que steps de candidate ou publish dependam dele.
- `scripts/verify-release-assets.sh` DEVE validar assets de release, checksums e completude dos targets antes de aprovação ou publicação.
- O Pages DEVE publicar a partir de releases publicadas e buildar da tag da release para manter site e docs alinhados com versões publicadas.
- `cargo-dist` DEVE ser o builder de artefatos Rust para a distribuição de release.
- `Quality`, `Security` e `SonarCloud` DEVEM passar antes da criação de uma tag de release.
- O quality gate do SonarCloud DEVE exigir `100%` de cobertura para o código analisado antes da aprovação dos artifacts reutilizáveis de release.
- `SONAR_TOKEN` DEVE apontar para um token do SonarCloud com permissão para navegar e analisar o projeto alvo.
- Checksums, SBOMs e atestações de artefato fazem parte do contrato-alvo de release.
- O npm DEVE instalar a partir de assets revisados do `cargo-dist` e verificar o checksum `.sha256` publicado antes da extração.
- O Homebrew DEVE ser derivado dos mesmos assets e checksums do `cargo-dist` usados pelo canal npm.
- A publicação automática NÃO DEVE sair da `main` enquanto GitHub Releases, npm e Homebrew não estiverem conectados ao pipeline em Rust.
