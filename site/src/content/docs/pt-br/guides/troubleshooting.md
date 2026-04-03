---
title: "Solução de problemas"
description: "Problemas comuns ao usar o Ralph Engine e como resolvê-los"
---

## Desalinhamento de Toolchain

Se as versões de Rust ou Node estiverem desalinhadas com os pins do repositório, resincronize com o asdf:

```bash
asdf install
```

Depois re-execute o script de bootstrap:

```bash
./scripts/bootstrap-dev.sh
```

## Falhas de Validação

Execute o contrato completo de validação local para identificar o que está quebrado:

```bash
./scripts/validate.sh --mode local
```

## SonarCloud Falha em `Create analysis`

Se o job do SonarCloud falhar durante `Create analysis` com um `404` de `api.sonarcloud.io`, trate isso primeiro como um problema de token ou permissão.

### Checklist

- Confirme que o secret `SONAR_TOKEN` existe no GitHub.
- Confirme que o token ainda pertence à conta ou organização esperada no SonarCloud.
- Confirme que o token consegue navegar no projeto.
- Confirme que o token consegue executar análise no projeto.

### Como a CI Trata o SonarCloud

O workflow da CI roda um preflight do SonarCloud antes das etapas de coverage e scan para que esse tipo de falha apareça mais cedo e com uma mensagem mais clara.

O workflow resolve a chave do projeto e a organização a partir de `sonar-project.properties` antes do scan e repassa esses valores explicitamente ao scanner. Isso mantém a entrada do scan visível no log do job e remove ambiguidade sobre qual projeto do SonarCloud o workflow está analisando.

### Fluxo de Artefatos de Coverage

A coverage é gerada uma vez em `Quality (ubuntu-latest)`, publicada como artefato de curta duração e reutilizada pelo job do SonarCloud. Se o SonarCloud reportar ausência do arquivo de coverage, inspecione primeiro o job anterior de qualidade no Ubuntu antes de investigar o step de scan.

### Quality Gate de Coverage

Se o SonarCloud falhar porque o quality gate de coverage ficou abaixo de `100%`, trate isso como um bloqueio real de release, não como um desencontro de documentação. O contrato do repositório exige que os artefatos reutilizáveis de release só sejam aprovados depois de o gate configurado do SonarCloud passar por completo.
