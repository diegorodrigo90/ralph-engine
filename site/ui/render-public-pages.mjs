#!/usr/bin/env node

import { cpSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(scriptDir, "..", "..");
const outputDir = process.argv[2]
  ? path.resolve(process.cwd(), process.argv[2])
  : path.join(rootDir, ".site-pages");

const cloudflareAnalytics = `
    <script>
      if (window.location.hostname === "ralphengine.com") {
        const script = document.createElement("script");
        script.defer = true;
        script.src = "https://static.cloudflareinsights.com/beacon.min.js";
        script.setAttribute("data-cf-beacon", '{"token": "882eb5c78500434c86bc3c6bbde81b4a"}');
        document.head.appendChild(script);
      }
    </script>`;

const pages = [
  {
    page: "landing",
    locale: "en",
    outputPath: "index.html",
    htmlLang: "en",
    localeCode: "en_US",
    canonicalUrl: "https://ralphengine.com/",
    alternateDefaultUrl: "https://ralphengine.com/",
    alternateLocaleUrl: "https://ralphengine.com/pt-br/",
    title: "Ralph Engine | The runtime for agentic coding workflows",
    metaDescription:
      "Ralph Engine is an open-source plugin-first runtime for agentic coding workflows, built with a Rust-first core, official plugins, MCP-aware integrations, and disciplined quality gates.",
    socialDescription:
      "A Rust-first runtime for agent loops, plugins, MCP-aware integrations, and quality gates that stay coherent from local development to CI.",
    brandAriaLabel: "Ralph Engine home",
    skipLinkLabel: "Skip to content",
    navigationAriaLabel: "Primary",
    localeSwitcherAriaLabel: "Language switcher",
    docsHref: "/docs/",
    roadmapHref: "/docs/development/roadmap",
    homeHref: "/",
    pluginsHref: "/plugins/",
    localeSelfHref: "/",
    localeOtherHref: "/pt-br/",
    localeSelfLabel: "EN",
    localeOtherLabel: "PT-BR",
    homeLabel: "Home",
    docsLabel: "Docs",
    pluginsLabel: "Plugins",
    roadmapLabel: "Roadmap",
    githubLabel: "GitHub",
    currentNav: "home",
    footerText: "Released under the MIT License.",
    footerLinks: [
      { href: "/", label: "Home" },
      { href: "/docs/", label: "Docs" },
      { href: "/plugins/", label: "Plugins" },
      { href: "https://github.com/diegorodrigo90/ralph-engine", label: "GitHub" },
    ],
    bodyFile: path.join(rootDir, "site", "landing", "content", "en", "body.html"),
    structuredData: {
      "@context": "https://schema.org",
      "@type": "SoftwareApplication",
      name: "Ralph Engine",
      applicationCategory: "DeveloperApplication",
      operatingSystem: "Linux, macOS, Windows",
      url: "https://ralphengine.com/",
      description: "Open-source plugin-first runtime for agentic coding workflows.",
      softwareVersion: "0.2.0-alpha.1",
    },
  },
  {
    page: "landing",
    locale: "pt-br",
    outputPath: "pt-br/index.html",
    htmlLang: "pt-BR",
    localeCode: "pt_BR",
    canonicalUrl: "https://ralphengine.com/pt-br/",
    alternateDefaultUrl: "https://ralphengine.com/",
    alternateLocaleUrl: "https://ralphengine.com/pt-br/",
    title: "Ralph Engine | O runtime para fluxos de desenvolvimento com agentes",
    metaDescription:
      "Ralph Engine é um runtime open source, orientado a plugins, para fluxos de desenvolvimento com agentes, com base em Rust, plugins oficiais, integrações com MCP e verificações rígidas de qualidade.",
    socialDescription:
      "Uma base em Rust para loops de agente, plugins, integrações com MCP e verificações de qualidade coerentes do desenvolvimento local até a CI.",
    brandAriaLabel: "Página inicial do Ralph Engine",
    skipLinkLabel: "Pular para o conteúdo",
    navigationAriaLabel: "Primária",
    localeSwitcherAriaLabel: "Seletor de idioma",
    docsHref: "/docs/pt-br/",
    roadmapHref: "/docs/pt-br/development/roadmap",
    homeHref: "/pt-br/",
    pluginsHref: "/pt-br/plugins/",
    localeSelfHref: "/pt-br/",
    localeOtherHref: "/",
    localeSelfLabel: "PT-BR",
    localeOtherLabel: "EN",
    homeLabel: "Início",
    docsLabel: "Docs",
    pluginsLabel: "Plugins",
    roadmapLabel: "Roadmap",
    githubLabel: "GitHub",
    currentNav: "home",
    footerText: "Distribuído sob a licença MIT.",
    footerLinks: [
      { href: "/pt-br/", label: "Início" },
      { href: "/docs/pt-br/", label: "Docs" },
      { href: "/pt-br/plugins/", label: "Plugins" },
      { href: "https://github.com/diegorodrigo90/ralph-engine", label: "GitHub" },
    ],
    bodyFile: path.join(rootDir, "site", "landing", "content", "pt-br", "body.html"),
  },
  {
    page: "plugins",
    locale: "en",
    outputPath: "plugins/index.html",
    htmlLang: "en",
    localeCode: "en_US",
    canonicalUrl: "https://ralphengine.com/plugins/",
    alternateDefaultUrl: "https://ralphengine.com/plugins/",
    alternateLocaleUrl: "https://ralphengine.com/pt-br/plugins/",
    title: "Ralph Engine Plugins | Discover official and custom extensions",
    metaDescription:
      "Explore the Ralph Engine plugins surface for official plugins, trust framing, stable metadata paths, and machine-readable indexes.",
    socialDescription:
      "Official plugin metadata, trust framing, and machine-readable paths are landing here before the marketplace grows denser.",
    brandAriaLabel: "Ralph Engine home",
    skipLinkLabel: "Skip to content",
    navigationAriaLabel: "Primary",
    localeSwitcherAriaLabel: "Language switcher",
    docsHref: "/docs/",
    roadmapHref: "/docs/development/roadmap",
    homeHref: "/",
    pluginsHref: "/plugins/",
    localeSelfHref: "/plugins/",
    localeOtherHref: "/pt-br/plugins/",
    localeSelfLabel: "EN",
    localeOtherLabel: "PT-BR",
    homeLabel: "Home",
    docsLabel: "Docs",
    pluginsLabel: "Plugins",
    roadmapLabel: "Roadmap",
    githubLabel: "GitHub",
    currentNav: "plugins",
    footerText: "Plugin paths stay public and stable while the ecosystem grows.",
    footerLinks: [
      { href: "/", label: "Home" },
      { href: "/docs/", label: "Docs" },
      { href: "/plugins/", label: "Plugins" },
      { href: "/plugins/index.json", label: "index.json" },
    ],
    bodyFile: path.join(rootDir, "site", "plugins", "content", "en", "body.html"),
  },
  {
    page: "plugins",
    locale: "pt-br",
    outputPath: "pt-br/plugins/index.html",
    htmlLang: "pt-BR",
    localeCode: "pt_BR",
    canonicalUrl: "https://ralphengine.com/pt-br/plugins/",
    alternateDefaultUrl: "https://ralphengine.com/plugins/",
    alternateLocaleUrl: "https://ralphengine.com/pt-br/plugins/",
    title: "Plugins Ralph Engine | Descubra extensões oficiais e customizadas",
    metaDescription:
      "Explore a superfície pública de plugins do Ralph Engine para plugins oficiais, sinais de confiança, caminhos estáveis de metadados e índices legíveis por máquina.",
    socialDescription:
      "Metadados de plugins oficiais, sinais de confiança e caminhos legíveis por máquina entram aqui antes de a vitrine visual ficar mais densa.",
    brandAriaLabel: "Página inicial do Ralph Engine",
    skipLinkLabel: "Pular para o conteúdo",
    navigationAriaLabel: "Primária",
    localeSwitcherAriaLabel: "Seletor de idioma",
    docsHref: "/docs/pt-br/",
    roadmapHref: "/docs/pt-br/development/roadmap",
    homeHref: "/pt-br/",
    pluginsHref: "/pt-br/plugins/",
    localeSelfHref: "/pt-br/plugins/",
    localeOtherHref: "/plugins/",
    localeSelfLabel: "PT-BR",
    localeOtherLabel: "EN",
    homeLabel: "Início",
    docsLabel: "Docs",
    pluginsLabel: "Plugins",
    roadmapLabel: "Roadmap",
    githubLabel: "GitHub",
    currentNav: "plugins",
    footerText: "Os caminhos públicos de plugins ficam estáveis enquanto o ecossistema cresce.",
    footerLinks: [
      { href: "/pt-br/", label: "Início" },
      { href: "/docs/pt-br/", label: "Docs" },
      { href: "/pt-br/plugins/", label: "Plugins" },
      { href: "/plugins/index.json", label: "index.json" },
    ],
    bodyFile: path.join(rootDir, "site", "plugins", "content", "pt-br", "body.html"),
  },
];

mkdirSync(outputDir, { recursive: true });

for (const page of pages) {
  const body = readFileSync(page.bodyFile, "utf8").trim();
  const targetFile = path.join(outputDir, page.outputPath);
  mkdirSync(path.dirname(targetFile), { recursive: true });

  const navLinks = [
    { href: page.homeHref, label: page.homeLabel, current: page.currentNav === "home" },
    { href: page.docsHref, label: page.docsLabel, current: false },
    { href: page.pluginsHref, label: page.pluginsLabel, current: page.currentNav === "plugins" },
    { href: page.roadmapHref, label: page.roadmapLabel, current: false },
    { href: "https://github.com/diegorodrigo90/ralph-engine", label: page.githubLabel, current: false },
  ];

  const html = `<!doctype html>
<html lang="${page.htmlLang}">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>${page.title}</title>
    <meta name="description" content="${page.metaDescription}" />
    <meta name="robots" content="index,follow" />
    <meta name="theme-color" content="#5b6ad0" />
    <link rel="canonical" href="${page.canonicalUrl}" />
    <link rel="alternate" hreflang="en" href="${page.alternateDefaultUrl}" />
    <link rel="alternate" hreflang="pt-BR" href="${page.alternateLocaleUrl}" />
    ${
      page.locale === "en"
        ? `    <link rel="alternate" hreflang="x-default" href="${page.alternateDefaultUrl}" />`
        : ""
    }
    <meta property="og:url" content="${page.canonicalUrl}" />
    <meta property="og:type" content="website" />
    <meta property="og:locale" content="${page.localeCode}" />
    <meta property="og:title" content="${page.title}" />
    <meta property="og:description" content="${page.socialDescription}" />
    <meta property="og:image" content="https://ralphengine.com/logo-icon.svg" />
    <meta property="og:image:alt" content="${page.locale === "pt-br" ? "Logo do Ralph Engine" : "Ralph Engine logo"}" />
    <meta name="twitter:card" content="summary" />
    <meta name="twitter:title" content="${page.title}" />
    <meta name="twitter:description" content="${page.socialDescription}" />
    <meta name="twitter:image" content="https://ralphengine.com/logo-icon.svg" />
    <link rel="icon" type="image/svg+xml" href="/logo-icon.svg" />
    <link rel="stylesheet" href="/styles.css" />
    ${
      page.structuredData
        ? `    <script type="application/ld+json">
${JSON.stringify(page.structuredData, null, 2)
  .split("\n")
  .map((line) => `      ${line}`)
  .join("\n")}
    </script>`
        : ""
    }
  </head>
  <body>
    <a class="skip-link" href="#content">${page.skipLinkLabel}</a>
    <main class="shell">
      <header class="topbar-wrap">
        <nav class="topbar" aria-label="${page.navigationAriaLabel}">
          <a class="brand" href="/" aria-label="${page.brandAriaLabel}">
            <img src="/logo-icon.svg" alt="" width="28" height="28" decoding="async" />
            <span>Ralph Engine</span>
          </a>
          <div class="topbar-group">
            <div class="nav-links">
${navLinks
  .map(
    (link) =>
      `              <a${link.current ? ' aria-current="page"' : ""} href="${link.href}">${link.label}</a>`,
  )
  .join("\n")}
            </div>
            <div class="locale-switcher" aria-label="${page.localeSwitcherAriaLabel}">
              <a${page.locale === "en" ? ' aria-current="page"' : ""} href="${page.locale === "en" ? page.localeSelfHref : page.localeOtherHref}">EN</a>
              <a${page.locale === "pt-br" ? ' aria-current="page"' : ""} href="${page.locale === "pt-br" ? page.localeSelfHref : page.localeOtherHref}">PT-BR</a>
            </div>
          </div>
        </nav>
      </header>

${body}

      <footer class="site-footer">
        <div class="page footer-inner">
          <p>${page.footerText}</p>
          <div class="footer-links">
${page.footerLinks.map((link) => `            <a href="${link.href}">${link.label}</a>`).join("\n")}
          </div>
        </div>
      </footer>
    </main>
${cloudflareAnalytics}
  </body>
</html>
`;

  writeFileSync(targetFile, html);
}

cpSync(path.join(rootDir, "site", "plugins", "index.json"), path.join(outputDir, "plugins", "index.json"));
