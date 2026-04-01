# Referência de configuração

O reboot em Rust já reintroduziu a primeira fatia tipada de configuração no novo core.

O contrato padrão atual já expõe:

- `schema_version`
- `default_locale`
- entradas padrão de plugins com estado de ativação tipado
- escopos tipados de configuração e a origem da ativação efetiva do plugin
- padrões de MCP

A arquitetura-alvo mais ampla continua incluindo:

- configuração de projeto
- padrões de plugins
- overrides de usuário
- configuração de MCP
- limites de prompt e contexto

A CLI atual consegue renderizar esse contrato tipado com:

```bash
ralph-engine config show-defaults
ralph-engine config show-plugin <plugin-id>
```

O comando `config show-plugin` agora renderiza a ativação efetiva do plugin junto com o escopo que forneceu esse resultado.

Esses contratos continuarão a evoluir sobre o novo core em Rust, guiados por TDD.
