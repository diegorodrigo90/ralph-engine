# Releases

## Fluxo

1. Faça merge de mudanças revisadas com Conventional Commits em `main`.
2. `release-plz release-pr` abre ou atualiza a PR de release.
3. Faça merge da PR de release.
4. `release-plz release` cria a tag SemVer e o GitHub Release a partir da `main`.
5. Os workflows por tag executam o contrato de release e geram os artefatos.

## Regras

- SemVer é obrigatório.
- Conventional Commits são obrigatórios.
- Actions são pinadas por SHA.
- As ferramentas de release são pinadas em versões revisadas.
- O contrato de release SHALL rodar via `./scripts/validate.sh --mode release` antes da publicação.
- Checksums, SBOMs e atestações de artefato fazem parte do contrato-alvo de release.
- npm e Homebrew continuam canais oficiais e publicarão a partir do pipeline em Rust, não de empacotamento local ad hoc.
