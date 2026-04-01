# Referência de MCP

O reboot em Rust agora já expõe um contrato tipado de MCP no novo core.

O descritor compartilhado atual já modela:

- identificador do servidor
- identificador do plugin dono
- transporte
- modelo de processo
- policy de disponibilidade

A CLI atual consegue inspecionar o catálogo MCP embutido com:

```bash
ralph-engine mcp list
ralph-engine mcp show <server-id>
```

Esses contratos vão continuar evoluindo sob TDD para que inicialização de processo e fronteiras de policy permaneçam tipadas, em vez de ficarem espalhadas por branches específicas de runtime.
