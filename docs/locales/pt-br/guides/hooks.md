# Hooks

Hooks definem gatilhos genéricos do runtime e a execução de etapas em `.ralph-engine/hooks.yaml`.

O Ralph Engine usa hooks como mecanismo genérico. O core do runtime **não** atribui significado de workflow aos nomes dos gatilhos além dos pontos técnicos que ele suporta.

Se um workflow precisar de conceitos como `prepare`, `review` ou `validation`, esses significados pertencem ao plugin ou boilerplate que gera e valida a configuração do projeto.

## Gatilhos do runtime

Hoje o core reconhece estes gatilhos:

| Gatilho           | Quando roda                                 |
| ----------------- | ------------------------------------------- |
| `session_start`    | Uma vez antes de o loop começar              |
| `work_item_start`  | Antes de o work item atual começar           |
| `after_agent`      | Depois que a sessão do agente termina        |
| `work_item_finish` | Depois que o work item termina com sucesso   |
| `session_end`      | Uma vez quando o engine para                 |

## Formato do hooks.yaml

```yaml
session_start:
  steps:
    - name: "Bootstrap environment"
      run: "npm install"
      required: true

work_item_start:
  steps:
    - name: "Refresh generated files"
      run: "npm run codegen"
      required: false

after_agent:
  steps:
    - name: "Unit tests"
      run: "npm test"
      required: true
      paths: ["src/**"]

    - name: "Build"
      run: "npm run build"
      required: true

work_item_finish:
  steps:
    - name: "Update docs"
      run: "npm run docs:generate"
      required: false

session_end:
  steps:
    - name: "Clean temporary files"
      run: "npm run clean"
      required: false
```

## Propriedades da etapa

| Propriedade | Tipo     | Obrigatória | Descrição                                   |
| ----------- | -------- | ----------- | ------------------------------------------- |
| `name`     | string   | sim         | Nome legível da etapa                          |
| `run`      | string   | sim         | Comando shell que será executado              |
| `timeout`  | string   | não         | Duração, por exemplo `"5m"` ou `"30s"`        |
| `required` | bool     | não         | Se `true`, a falha bloqueia o progresso       |
| `paths`    | string[] | não         | Só roda se os arquivos alterados baterem globs |

## Filtro por caminhos

Etapas em `after_agent` podem rodar condicionalmente com base nos arquivos alterados:

```yaml
after_agent:
  steps:
    - name: "TypeScript tests"
      run: "pnpm test"
      paths:
        - "apps/**/*.ts"
        - "packages/**/*.ts"

    - name: "Python tests"
      run: "pytest"
      paths:
        - "**/*.py"
```

Se nenhum arquivo alterado combinar com os globs configurados, a etapa é ignorada.

## Responsabilidade do workflow

O core do runtime é responsável por:

- carregar `.ralph-engine/hooks.yaml`
- executar etapas por gatilho
- aplicar filtro por caminho
- controlar timeout
- bloquear quando uma etapa obrigatória falha

Plugins ou boilerplates são responsáveis por:

- decidir quais arquivos e gatilhos um workflow exige
- decidir o significado de cada gatilho naquele workflow
- validar se o projeto está pronto para aquele workflow

## Comportamento cross-platform

Comandos em `run` são executados pela abstração interna de shell:

- Linux/macOS: `sh -c`
- Windows: `cmd /c`

Isso mantém a execução cross-platform sem hardcode de caminhos de shell na configuração.
