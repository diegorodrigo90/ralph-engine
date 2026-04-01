# Backlog

Updated: 2026-04-01

- Flesh out `re-cli` beyond the bootstrap surface.
- Introduce the `re-mcp` crate.
- Expand the `re-plugin` contract beyond immutable metadata into configuration, lifecycle, and loading boundaries.
- Add cross-platform release artifact packaging.
- Add explicit validation for `dist-workspace.toml` and generated release artifacts.
- Restore rich docs examples once the runtime contracts are back.
- Define i18n contracts for CLI messages, docs, site, and plugins with English and pt-BR as first-class locales.
- Add performance, accessibility, and SEO budgets for public surfaces so site, docs, and plugins stay at an A-grade baseline.
- Define shared information architecture, menus, and navigation contracts across `/`, `/docs`, and `/plugins`.
- Refactor `site/ui` into a real shared presentation layer for `site/landing` and `site/plugins`.
- Align the public header contract so the logo always returns to `/` and navigation stays coherent across site, docs, and plugins.
- Track the `vitepress`/`vite`/`esbuild` moderate advisory on the docs toolchain and upgrade to the next reviewed stable line when a non-breaking fix becomes available.
