# Solução de problemas

## Desalinhamento de toolchain

Rode:

```bash
asdf install
./scripts/bootstrap-dev.sh
```

## Falhas de validação

Rode:

```bash
./scripts/validate.sh --mode local
```

## SonarCloud falha em `Create analysis`

Se o job do SonarCloud falhar na etapa `Create analysis` com `404` vindo de `api.sonarcloud.io`, trate isso primeiro como problema de token ou permissão.

Checklist:

```text
- confirme que o secret SONAR_TOKEN existe no GitHub
- confirme que o token ainda pertence à conta ou organização esperada no SonarCloud
- confirme que o token consegue navegar no projeto
- confirme que o token consegue executar análise no projeto
```

O workflow da CI agora roda um preflight do SonarCloud antes da cobertura e do scan para que esse tipo de falha apareça antes e com mensagem mais clara.

O workflow também resolve a chave do projeto e a organização a partir de `sonar-project.properties` antes do scan e repassa esses valores explicitamente ao scanner. Assim o log mostra com clareza qual projeto o job está tentando analisar e reduz ambiguidade de configuração.
