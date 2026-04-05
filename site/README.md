# site/

Public website, documentation, and plugin catalog for Ralph Engine.

Built with [Astro](https://astro.build/) + [Starlight](https://starlight.astro.build/).

## Structure

```
site/
├── src/
│   ├── content/docs/         # Documentation (EN at root, PT-BR under pt-br/)
│   ├── pages/                # Custom pages (landing, plugin catalog)
│   ├── components/           # Astro components (header, features, etc.)
│   └── styles/               # CSS (design tokens, Starlight theme, components)
├── public/
│   └── plugins/index.json    # Plugin catalog data
└── astro.config.mjs          # Astro + Starlight configuration
```

## Commands

```bash
cd site
npm install
npm run dev       # Dev server (localhost:4321)
npm run build     # Production build (dist/)
```

## Bilingual

All documentation is available in English and Brazilian Portuguese (pt-BR). Pages under `src/content/docs/pt-br/` mirror the English structure.
