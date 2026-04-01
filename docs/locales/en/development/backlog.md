# Backlog

Updated: 2026-04-01

- Flesh out `re-cli` beyond the bootstrap surface.
- Expand the `re-mcp` contract from typed launch metadata into richer runtime registration and orchestration contracts without falling back to stringly typed branching.
- Expand the runtime contracts beyond typed metadata, activation scopes, capability registration, lifecycle, load boundaries, runtime hooks, topology registration, and health status into richer runtime state orchestration.
- Add cross-platform release artifact packaging.
- Add explicit validation for `dist-workspace.toml` and generated release artifacts.
- Restore rich docs examples once the runtime contracts are back.
- Define i18n contracts for CLI messages, docs, site, and plugins with English and pt-BR as first-class locales.
- Add performance, accessibility, and SEO budgets for public surfaces so site, docs, and plugins stay at an A-grade baseline.
- Define shared information architecture, menus, and navigation contracts across `/`, `/docs`, and `/plugins`.
- Refactor `site/ui` into a real shared presentation layer for `site/landing` and `site/plugins`.
- Align the public header contract so the logo always returns to `/` and navigation stays coherent across site, docs, and plugins.
- Track the `vitepress`/`vite`/`esbuild` moderate advisory on the docs toolchain and upgrade to the next reviewed stable line when a non-breaking fix becomes available.
