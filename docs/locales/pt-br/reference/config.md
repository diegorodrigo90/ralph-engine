# Referência de configuração

O reboot em Rust já reintroduziu a primeira fatia tipada de configuração no novo core.

O contrato padrão atual já expõe:

- `schema_version`
- `default_locale`
- entradas padrão de plugins com estado de ativação tipado
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
```

Esses contratos continuarão a evoluir sobre o novo core em Rust, guiados por TDD.
