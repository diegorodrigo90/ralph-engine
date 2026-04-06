# CLAUDE.md — Ralph Engine companion notes

Use `AGENTS.md` as the primary contract.

## Working model

- Repository root validation contract: `./scripts/validate.sh`
- Rust toolchain contract: `rust-toolchain.toml`
- asdf contract: `.tool-versions`
- Hooks: `lefthook.yml`
- Versioning: Conventional Commits + release-plz + SemVer

## Design System: ratatui-themekit (GOLDEN RULE)

ALL TUI colors in core and plugins use `ratatui-themekit`. No exceptions.

```rust
// YES — themekit
let block = t.block(" Title ").focused(true).build();
let line = t.line().accent("A").dim("|").success("B").build();
let status = t.status_line().kv("Key", "Val").build();
let ts = t.table_styles();
let ns = t.notification_styles();

// NO — never do this
Style::default().fg(Color::Rgb(137, 180, 250))  // hardcoded color
Block::default().border_style(...)               // raw Block
Modifier::BOLD                                   // raw modifier
```

CR flags: `Style::default()`, `Color::Rgb`, `Modifier::`, `Block::default()`.

## Important commands

```bash
./scripts/bootstrap-dev.sh
./scripts/validate.sh --mode local
cargo test --workspace --all-targets --all-features
cd site && npm run build    # Astro + Starlight + Pagefind
cd site && npm run dev      # Dev server on port 4500
```

## Site and docs

- Site + docs are a single Astro + Starlight build in `site/`
- Docs content: `site/src/content/docs/` (EN) and `site/src/content/docs/pt-br/`
- Plugin pages: `site/src/pages/plugins/` (custom Astro with StarlightPage wrapper)
- Landing page: `site/src/pages/index.astro` (custom Astro with StarlightPage wrapper)
- Custom header with Docs/Plugins nav links: `site/src/components/starlight/Header.astro`
- Design tokens: `site/src/styles/design-tokens.css` (imported by starlight-custom.css)
- Component styles: `site/src/styles/site-components.css` (buttons, cards, terminals, pills)
- Starlight theme: `site/src/styles/starlight-custom.css` (color overrides)
