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
        {
          label: 'Getting Started',
          translations: { 'pt-BR': 'Primeiros passos' },
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
          ],
        },
        {
          label: 'Guides',
          translations: { 'pt-BR': 'Guias' },
          items: [
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
              slug: 'guides/plugins',
              label: 'Extending',
              translations: { 'pt-BR': 'Extensões' },
            },
            {
              slug: 'guides/troubleshooting',
              label: 'Troubleshooting',
              translations: { 'pt-BR': 'Solução de problemas' },
            },
          ],
        },
        {
          label: 'Reference',
          translations: { 'pt-BR': 'Referência' },
          items: [
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
              slug: 'reference/architecture',
              label: 'Architecture',
              translations: { 'pt-BR': 'Arquitetura' },
            },
            {
              slug: 'reference/mcp',
              label: 'MCP',
            },
          ],
        },
        {
          label: 'Development',
          translations: { 'pt-BR': 'Desenvolvimento' },
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
      customCss: [
        './src/styles/starlight-custom.css',
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
