# official.hello-world

Plugin de exemplo mínimo — use como referência para criar o seu.

## Visão geral

Este é o plugin mais simples possível do Ralph Engine. Ele demonstra a arquitetura de plugins com um único template starter e nada mais. Se você está aprendendo como plugins funcionam, comece aqui.

## O que ele faz

Inclui um único template starter (`official.hello-world.starter`) que cria um diretório básico `.ralph-engine/` com:

- `config.yaml` — configuração mínima do projeto
- `hooks.yaml` — definições de hooks vazias
- `prompt.md` — prompt de projeto placeholder

## Como usar

```
ralph-engine templates materialize official.hello-world.starter ./meu-projeto
```

## Por que ele existe

Este plugin serve como referência viva para o tutorial de Desenvolvimento de Plugins. Cada arquivo neste diretório mapeia diretamente para uma seção nas docs. Quando você cria um novo plugin com `npx create-ralph-engine-plugin`, o código gerado segue esta mesma estrutura.

## Arquivos explicados

| Arquivo | Propósito |
|---------|-----------|
| `manifest.yaml` | Metadados do plugin — ID, tipo, capabilities, i18n |
| `src/lib.rs` | Descriptor do plugin + implementação do trait PluginRuntime |
| `src/i18n/mod.rs` | Inclui código i18n gerado no build |
| `build.rs` | Lê locales/*.toml e gera código Rust de i18n |
| `locales/en.toml` | Traduções em inglês |
| `locales/pt-br.toml` | Traduções em português |
| `template/` | Arquivos materializados pelo template starter |
| `Cargo.toml` | Manifesto do crate Rust com deps do workspace |
