import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import sitemap from '@astrojs/sitemap';

export default defineConfig({
  site: 'https://ralphengine.com',
  output: 'static',
  integrations: [
    starlight({
      title: {
        en: 'Ralph Engine',
        'pt-br': 'Ralph Engine',
      },
      logo: {
        light: './src/assets/logo.svg',
        dark: './src/assets/logo-dark.svg',
        alt: 'Ralph Engine',
        replacesTitle: true,
      },
      defaultLocale: 'root',
      locales: {
        root: {
          label: 'English',
          lang: 'en',
        },
        'pt-br': {
          label: 'Português (Brasil)',
          lang: 'pt-BR',
        },
      },
      sidebar: [
        // ── For users: install, configure, use ──
        {
          label: 'Using Ralph Engine',
          translations: { 'pt-BR': 'Usando o Ralph Engine' },
          items: [
            {
              slug: 'getting-started/installation',
              label: 'Installation',
              translations: { 'pt-BR': 'Instalação' },
            },
            {
              slug: 'getting-started/quickstart',
              label: 'Quick Start',
              translations: { 'pt-BR': 'Início rápido' },
            },
            {
              slug: 'guides/configuration',
              label: 'Configuration',
              translations: { 'pt-BR': 'Configuração' },
            },
            {
              slug: 'guides/hooks',
              label: 'Hooks',
            },
            {
              slug: 'reference/cli',
              label: 'CLI Commands',
              translations: { 'pt-BR': 'Comandos CLI' },
            },
            {
              slug: 'reference/config',
              label: 'Config Reference',
              translations: { 'pt-BR': 'Referência de config' },
            },
            {
              slug: 'reference/mcp',
              label: 'MCP',
            },
            {
              slug: 'guides/troubleshooting',
              label: 'Troubleshooting',
              translations: { 'pt-BR': 'Solução de problemas' },
            },
          ],
        },
        // ── For plugin authors: create your own plugins ──
        {
          label: 'Plugin Development',
          translations: { 'pt-BR': 'Desenvolvimento de Plugins' },
          items: [
            {
              slug: 'guides/plugins',
              label: 'Overview',
              translations: { 'pt-BR': 'Visão geral' },
            },
            {
              slug: 'guides/plugin-development',
              label: 'Tutorial',
            },
          ],
        },
        // ── For contributors: build the core, submit PRs ──
        {
          label: 'Contributing',
          translations: { 'pt-BR': 'Contribuindo' },
          items: [
            {
              slug: 'development/building',
              label: 'Building',
              translations: { 'pt-BR': 'Compilação' },
            },
            {
              slug: 'development/coding-standards',
              label: 'Coding Standards',
              translations: { 'pt-BR': 'Padrões de código' },
            },
            {
              slug: 'reference/architecture',
              label: 'Architecture',
              translations: { 'pt-BR': 'Arquitetura' },
            },
            {
              slug: 'development/releasing',
              label: 'Releasing',
              translations: { 'pt-BR': 'Publicação' },
            },
          ],
        },
      ],
      social: {
        github: 'https://github.com/diegorodrigo90/ralph-engine',
      },
      components: {
        Header: './src/components/starlight/Header.astro',
      },
      expressiveCode: {
        themes: ['github-dark-dimmed', 'github-dark-default'],
        styleOverrides: {
          borderRadius: '0.75rem',
        },
      },
      customCss: [
        './src/styles/starlight-custom.css',
        './src/styles/site-components.css',
      ],
      lastUpdated: true,
      editLink: {
        baseUrl: 'https://github.com/diegorodrigo90/ralph-engine/edit/main/site/src/content/docs/',
      },
      head: [
        {
          tag: 'link',
          attrs: { rel: 'icon', href: '/logo-icon.svg', type: 'image/svg+xml' },
        },
      ],
      pagefind: true,
      tableOfContents: { minHeadingLevel: 2, maxHeadingLevel: 3 },
    }),
    sitemap(),
  ],
  build: { format: 'directory' },
});
