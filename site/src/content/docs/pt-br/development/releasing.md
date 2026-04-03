---
title: "Publicação"
description: "Processo de release e pipeline CI para mantenedores"
---

## Fluxo de Release

1. Faça merge de mudanças revisadas com Conventional Commits em `main`.
2. `release-plz release-pr` abre ou atualiza a PR de release.
3. Faça merge da PR de release.
4. A publicação de release fica desabilitada até o pipeline de distribuição em Rust estar conectado e validado ponta a ponta.
5. O workflow endurecido de publicação cria a tag SemVer somente depois que os gates obrigatórios passarem para o commit alvo em `main`.

## Workflow Manual de Publicação

O workflow `Release` é manual e espera as seguintes entradas:

- `tag` — deve incluir o prefixo `v` (ex: `v0.2.0-alpha.1`). O workflow remove esse prefixo antes de preparar as versões dos pacotes npm.
- `publish_github_release`
- `publish_npm`
- `publish_homebrew`
- `homebrew_tap_repository` — quando o tap não deve ser inferido por outro meio

### Secrets Necessários

- `NPM_TOKEN` — quando `publish_npm=true`
- `HOMEBREW_TAP_TOKEN` — quando `publish_homebrew=true`

### Verificações Pré-Publicação

Antes de publicar qualquer coisa, o workflow:

1. Verifica que o SHA selecionado é o HEAD atual de `origin/main`.
2. Verifica que o workflow canônico de `CI` completou com sucesso para esse exato push.
3. Rejeita `publish_npm=true` ou `publish_homebrew=true` quando `publish_github_release=true` não estiver marcado (ambos os canais downstream dependem do conjunto revisado de assets do GitHub Release).

## Verificação npm

Quando `publish_npm=true`, o workflow:

1. Executa `npm pack --json --dry-run` nos payloads staged de `ralph-engine` e `create-ralph-engine-plugin`.
2. Rejeita a publicação se entradas obrigatórias, wiring de `bin`, scripts, nomes de pacote ou versões reescritas estiverem incorretos.
3. Instala os tarballs staged em um projeto consumidor temporário e executa seus binários públicos antes do publish.

## Verificação Homebrew

Quando `publish_homebrew=true`, o workflow:

1. Renderiza a fórmula a partir dos assets aprovados de release.
2. Valida no macOS com `brew audit`, `brew install` e `brew test`.
3. Somente então atualiza o repositório do tap.

## Pipeline CI

O workflow canônico de `CI` gera release candidates cross-platform em paralelo com os gates de qualidade e publica artefatos reutilizáveis aprovados para o SHA somente depois que `Quality`, `Security` e `SonarCloud` tenham todos passado.

Tanto o contrato revisado de `dist-workspace.toml` quanto os assets gerados de release são validados explicitamente antes que os artefatos sejam aprovados ou promovidos.

O quality gate do SonarCloud é a trava dura de coverage para release: se ficar abaixo da meta configurada de `100%` para o código analisado, o SHA não é aprovado para publicação de artefatos nem para promoção de release.

## Regras

- SemVer é obrigatório.
- Conventional Commits são obrigatórios.
- Actions são pinadas por SHA.
- As ferramentas de release são pinadas em versões revisadas.
- O workflow de release verifica o SHA alvo em `main` contra o workflow canônico de `CI` antes de publicar artefatos.
- O workflow de release reaproveita a evidência de CI verde em vez de re-executar o contrato completo de validação dentro do fluxo de publish.
- O workflow de release rejeita publicação de canais downstream quando `publish_github_release=false`.
- O workflow de release verifica que o GitHub Release da tag selecionada existe antes de iniciar publicação em npm ou Homebrew.
- O workflow de release verifica os tarballs staged de npm antes de publicar canais npm.
- O workflow de release executa smoke de install dos pacotes npm staged antes de publicar os canais npm.
- O workflow canônico de `CI` gera release candidates cross-platform em paralelo com os gates de qualidade.
- O workflow canônico de `CI` publica artefatos reutilizáveis aprovados somente depois que `Quality`, `Security` e `SonarCloud` tenham todos passado.
- O workflow de release baixa e publica esse mesmo conjunto aprovado de artefatos, em vez de reconstruí-lo.
- `scripts/verify-dist-workspace.sh` valida o contrato revisado do workspace `cargo-dist` antes que steps de candidate ou publish dependam dele.
- `scripts/verify-release-assets.sh` valida assets de release, checksums e completude de targets antes de aprovação ou publicação.
- `scripts/verify-homebrew-formula.sh` valida a fórmula Homebrew renderizada com `brew audit`, `brew install` e `brew test` antes da atualização do tap.
- O Pages publica a partir de releases publicadas e compila da tag de release para manter site e docs alinhados com versões publicadas.
- `cargo-dist` é o builder de artefatos Rust para distribuição de release.
- `Quality`, `Security` e `SonarCloud` devem todos passar antes da criação de uma tag de release.
- O quality gate do SonarCloud exige `100%` de coverage para o código analisado antes da aprovação dos artefatos reutilizáveis de release.
- `SONAR_TOKEN` deve apontar para um token do SonarCloud com permissão para navegar e analisar o projeto alvo.
- Checksums, SBOMs e atestações de artefato fazem parte do contrato-alvo de release.
- O npm instala a partir de assets revisados do `cargo-dist` e verifica o checksum `.sha256` publicado antes da extração.
- O Homebrew é derivado dos mesmos assets e checksums do `cargo-dist` usados pelo canal npm.
- A publicação automática não acontece a partir de `main` até que GitHub Releases, npm e Homebrew estejam conectados ao pipeline em Rust.
