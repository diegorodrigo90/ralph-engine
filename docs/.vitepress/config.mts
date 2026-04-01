import { defineConfig } from "vitepress";

function buildSidebar(prefix: string, labels: {
  gettingStarted: string;
  guides: string;
  reference: string;
  development: string;
  installation: string;
  quickStart: string;
  configuration: string;
  hooks: string;
  extending: string;
  troubleshooting: string;
  cliCommands: string;
  configReference: string;
  architecture: string;
  building: string;
  codingStandards: string;
  releasing: string;
  roadmap: string;
  backlog: string;
}) {
  return [
    {
      text: labels.gettingStarted,
      items: [
        { text: labels.installation, link: `${prefix}/getting-started/installation` },
        { text: labels.quickStart, link: `${prefix}/getting-started/quickstart` },
      ],
    },
    {
      text: labels.guides,
      items: [
        { text: labels.configuration, link: `${prefix}/guides/configuration` },
        { text: labels.hooks, link: `${prefix}/guides/hooks` },
        { text: labels.extending, link: `${prefix}/guides/plugins` },
        { text: labels.troubleshooting, link: `${prefix}/guides/troubleshooting` },
      ],
    },
    {
      text: labels.reference,
      items: [
        { text: labels.cliCommands, link: `${prefix}/reference/cli` },
        { text: labels.configReference, link: `${prefix}/reference/config` },
        { text: labels.architecture, link: `${prefix}/reference/architecture` },
      ],
    },
    {
      text: labels.development,
      items: [
        { text: labels.building, link: `${prefix}/development/building` },
        { text: labels.codingStandards, link: `${prefix}/development/coding-standards` },
        { text: labels.releasing, link: `${prefix}/development/releasing` },
        { text: labels.roadmap, link: `${prefix}/development/roadmap` },
        { text: labels.backlog, link: `${prefix}/development/backlog` },
      ],
    },
  ];
}

export default defineConfig({
  lang: "en-US",
  title: "Ralph Engine",
  description: "Open-source plugin-first runtime for agentic coding workflows",
  base: "/docs/",
  cleanUrls: true,
  sitemap: {
    hostname: "https://ralphengine.com/docs",
  },

  head: [
    ["link", { rel: "preconnect", href: "https://ralphengine.com" }],
    [
      "link",
      { rel: "icon", type: "image/svg+xml", href: "/logo-icon.svg" },
    ],
    [
      "script",
      {},
      `if (window.location.hostname === "ralphengine.com") {
        const script = document.createElement("script");
        script.defer = true;
        script.src = "https://static.cloudflareinsights.com/beacon.min.js";
        script.setAttribute("data-cf-beacon", '{"token":"882eb5c78500434c86bc3c6bbde81b4a"}');
        document.head.appendChild(script);
      }`,
    ],
  ],

  locales: {
    root: {
      label: "English",
      lang: "en-US",
      title: "Ralph Engine",
      description: "Open-source plugin-first runtime for agentic coding workflows",
      themeConfig: {
        nav: [
          { text: "Home", link: "https://ralphengine.com/" },
          { text: "Docs", link: "/" },
          { text: "Plugins", link: "https://ralphengine.com/plugins/" },
          { text: "Roadmap", link: "/development/roadmap" },
          { text: "GitHub", link: "https://github.com/diegorodrigo90/ralph-engine" },
        ],
        sidebar: buildSidebar("", {
          gettingStarted: "Getting Started",
          guides: "Guides",
          reference: "Reference",
          development: "Development",
          installation: "Installation",
          quickStart: "Quick Start",
          configuration: "Configuration",
          hooks: "Hooks",
          extending: "Extending",
          troubleshooting: "Troubleshooting",
          cliCommands: "CLI Commands",
          configReference: "Config Reference",
          architecture: "Architecture",
          building: "Building",
          codingStandards: "Coding Standards",
          releasing: "Releasing",
          roadmap: "Roadmap",
          backlog: "Backlog",
        }),
        editLink: {
          pattern:
            "https://github.com/diegorodrigo90/ralph-engine/edit/main/docs/locales/en/:path",
          text: "Edit this page on GitHub",
        },
      },
    },
    "pt-br": {
      label: "Português (Brasil)",
      lang: "pt-BR",
      link: "/pt-br/",
      title: "Ralph Engine",
      description: "Runtime open source, orientado a plugins, para fluxos de desenvolvimento com agentes",
      themeConfig: {
        nav: [
          { text: "Início", link: "https://ralphengine.com/pt-br/" },
          { text: "Docs", link: "/pt-br/" },
          { text: "Plugins", link: "https://ralphengine.com/pt-br/plugins/" },
          { text: "Roadmap", link: "/pt-br/development/roadmap" },
          { text: "GitHub", link: "https://github.com/diegorodrigo90/ralph-engine" },
        ],
        sidebar: buildSidebar("/pt-br", {
          gettingStarted: "Primeiros passos",
          guides: "Guias",
          reference: "Referência",
          development: "Desenvolvimento",
          installation: "Instalação",
          quickStart: "Início rápido",
          configuration: "Configuração",
          hooks: "Hooks",
          extending: "Plugins",
          troubleshooting: "Solução de problemas",
          cliCommands: "Comandos CLI",
          configReference: "Referência de configuração",
          architecture: "Arquitetura",
          building: "Compilação",
          codingStandards: "Padrões de código",
          releasing: "Releases",
          roadmap: "Roadmap",
          backlog: "Backlog",
        }),
        editLink: {
          pattern:
            "https://github.com/diegorodrigo90/ralph-engine/edit/main/docs/locales/pt-br/:path",
          text: "Editar esta página no GitHub",
        },
      },
    },
  },

  themeConfig: {
    logo: {
      light: "/logo.svg",
      dark: "/logo-dark.svg",
      alt: "Ralph Engine",
    },

    siteTitle: false,
    logoLink: "https://ralphengine.com/",

    socialLinks: [
      {
        icon: "github",
        link: "https://github.com/diegorodrigo90/ralph-engine",
      },
    ],

    search: {
      provider: "local",
    },

    outline: {
      level: [2, 3],
    },

    footer: {
      message: "Released under the MIT License.",
      copyright: "© 2026 Diego Rodrigo",
    },
  },
});
