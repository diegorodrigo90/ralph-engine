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

A cobertura agora é gerada uma vez em `Quality (ubuntu-latest)`, publicada como artifact de curta duração e reaproveitada pelo job do SonarCloud. Se o Sonar acusar ausência do arquivo de cobertura, inspecione primeiro o job anterior de qualidade no Ubuntu.

Se o SonarCloud falhar porque o quality gate de cobertura ficou abaixo de `100%`, trate isso como um bloqueio real de release, não como um desencontro de documentação. O contrato do repositório exige que os artifacts reutilizáveis de release só sejam aprovados depois de o gate configurado do SonarCloud passar por completo.
