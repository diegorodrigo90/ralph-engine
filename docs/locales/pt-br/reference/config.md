# Referência de configuração

O reboot em Rust já reintroduziu a primeira fatia tipada de configuração no novo core.

O contrato padrão atual já expõe:

- `schema_version`
- `default_locale`
- entradas padrão de plugins com estado de ativação tipado
- escopos tipados de configuração e a origem da ativação efetiva do plugin
- padrões de MCP
- limites de prompt e contexto

A arquitetura-alvo mais ampla continua incluindo:

- configuração de projeto
- padrões de plugins
- overrides de usuário
- configuração de MCP
- limites de prompt e contexto

A CLI atual consegue renderizar esse contrato tipado com:

```bash
ralph-engine config budgets
ralph-engine config layers
ralph-engine config locale
ralph-engine config show-budgets
ralph-engine config show-defaults
ralph-engine config show-layers
ralph-engine config show-locale
ralph-engine config show-plugin <plugin-id>
```

O comando `config locale` renderiza o contrato tipado do locale padrão atual, para que a localização da CLI permaneça visível e versionada em vez de ficar só como default interno.

O comando `config budgets` renderiza o contrato tipado canônico de limites de prompt e contexto, para que os tetos de tokens permaneçam explícitos na configuração compartilhada do runtime em vez de serem inferidos mais tarde por defaults locais de cada provider.

O comando `config layers` renderiza a pilha canônica tipada de configuração na ordem de resolução, para que defaults, futuras configurações de workspace, configuração de projeto e overrides de usuário continuem explícitos no contrato do runtime.

O comando `config show-plugin` agora renderiza a ativação efetiva do plugin junto com o escopo que forneceu esse resultado.

Esses contratos continuarão a evoluir sobre o novo core em Rust, guiados por TDD.
