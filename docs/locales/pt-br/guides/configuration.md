# Configuração

A configuração do Ralph Engine é gerenciada por contratos tipados em Rust. A CLI expõe toda a superfície de configuração:

```bash
ralph-engine config show-defaults    # Config padrão do projeto (YAML)
ralph-engine config locale           # Configuração de idioma
ralph-engine config budgets          # Limites de tokens e contexto
ralph-engine config layers           # Camadas de resolução de config
ralph-engine config show-plugin <id> # Config resolvida de um plugin
ralph-engine config show-mcp-server <id> # Config resolvida de um servidor MCP
```

## Configuração do Projeto

Execute `ralph-engine templates scaffold official.basic.starter .` para criar o diretório `.ralph-engine/` com os arquivos de configuração:

- `.ralph-engine/config.yaml` — configuração do projeto
- `.ralph-engine/prompt.md` — conteúdo de prompt do projeto
- `.ralph-engine/hooks.yaml` — configuração de hooks (ao usar o plugin BMAD)

## Camadas de Configuração

A configuração é resolvida em camadas:

1. **Padrões do runtime** — embutidos no binário
2. **Padrões dos plugins** — declarados por cada plugin
3. **Config do projeto** — de `.ralph-engine/config.yaml`

Use `ralph-engine config layers` para inspecionar a cadeia completa.

## Diagnóstico

O comando `doctor` analisa a configuração do projeto e sugere correções:

```bash
ralph-engine doctor                         # Relatório de diagnóstico
ralph-engine doctor apply-config config.yaml # Grava config corrigida em arquivo
```
