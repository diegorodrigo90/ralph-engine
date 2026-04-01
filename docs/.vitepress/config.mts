import { defineConfig } from "vitepress";

export default defineConfig({
  lang: "en-US",
  title: "Ralph Engine",
  description: "Open-source plugin-first runtime for agentic coding workflows",
  base: "/docs/",
  cleanUrls: true,
  sitemap: {
    hostname: "https://ralphengine.com",
  },

  head: [
    ["link", { rel: "preconnect", href: "https://ralphengine.com" }],
    [
      "link",
      { rel: "icon", type: "image/svg+xml", href: "/logo-icon.svg" },
    ],
    [
      "script",
      {
        defer: "",
        src: "https://static.cloudflareinsights.com/beacon.min.js",
        "data-cf-beacon": '{"token": "882eb5c78500434c86bc3c6bbde81b4a"}',
      },
    ],
  ],

  themeConfig: {
    logo: {
      light: "/logo.svg",
      dark: "/logo-dark.svg",
      alt: "Ralph Engine",
    },

    siteTitle: false,

    nav: [
      { text: "Guide", link: "/getting-started/installation" },
      { text: "Reference", link: "/reference/cli" },
      {
        text: "Links",
        items: [
          {
            text: "GitHub",
            link: "https://github.com/diegorodrigo90/ralph-engine",
          },
          {
            text: "Releases",
            link: "https://github.com/diegorodrigo90/ralph-engine/releases",
          },
          {
            text: "llms.txt",
            link: "https://ralphengine.com/llms.txt",
          },
        ],
      },
    ],

    sidebar: [
      {
        text: "Getting Started",
        items: [
          { text: "Installation", link: "/getting-started/installation" },
          { text: "Quick Start", link: "/getting-started/quickstart" },
        ],
      },
      {
        text: "Guides",
        items: [
          { text: "Configuration", link: "/guides/configuration" },
          { text: "Hooks", link: "/guides/hooks" },
          { text: "Extending", link: "/guides/plugins" },
          { text: "Troubleshooting", link: "/guides/troubleshooting" },
        ],
      },
      {
        text: "Reference",
        items: [
          { text: "CLI Commands", link: "/reference/cli" },
          { text: "Config Reference", link: "/reference/config" },
          { text: "Architecture", link: "/reference/architecture" },
        ],
      },
      {
        text: "Development",
        items: [
          { text: "Building", link: "/development/building" },
          { text: "Coding Standards", link: "/development/coding-standards" },
          { text: "Releasing", link: "/development/releasing" },
          { text: "Roadmap", link: "/development/roadmap" },
          { text: "Backlog", link: "/development/backlog" },
        ],
      },
    ],

    socialLinks: [
      {
        icon: "github",
        link: "https://github.com/diegorodrigo90/ralph-engine",
      },
    ],

    search: {
      provider: "local",
    },

    editLink: {
      pattern:
        "https://github.com/diegorodrigo90/ralph-engine/edit/main/docs/:path",
      text: "Edit this page on GitHub",
    },

    footer: {
      message: "Released under the MIT License.",
      copyright: "© 2026 Diego Rodrigo",
    },
  },
});
