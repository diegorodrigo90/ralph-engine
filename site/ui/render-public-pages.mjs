#!/usr/bin/env node

import { cpSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(scriptDir, "..", "..");
const outputDir = process.argv[2]
  ? path.resolve(process.cwd(), process.argv[2])
  : path.join(rootDir, ".site-pages");

const analyticsLoader = `
    <script>
      if (window.location.hostname === "ralphengine.com") {
        const script = document.createElement("script");
        script.defer = true;
        script.src = "https://static.cloudflareinsights.com/beacon.min.js";
        script.setAttribute("data-cf-beacon", '{"token":"882eb5c78500434c86bc3c6bbde81b4a"}');
        document.head.appendChild(script);
      }
    </script>`;

const themeBootScript = `
    <script>
      (function () {
        const storageKey = "re-theme";
        const root = document.documentElement;
        const storedTheme = localStorage.getItem(storageKey);
        const systemDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
        const nextTheme = storedTheme === "light" || storedTheme === "dark"
          ? storedTheme
          : systemDark ? "dark" : "light";

        root.dataset.theme = nextTheme;
        root.style.colorScheme = nextTheme;
      })();
    </script>`;

const publicScriptTag = `    <script defer src="/public-shell.js"></script>`;

const baseFooterLinks = {
  en: [
    { href: "/", label: "Home" },
    { href: "/docs/", label: "Docs" },
    { href: "/plugins/", label: "Plugins" },
    { href: "/docs/development/roadmap", label: "Roadmap" },
  ],
  "pt-br": [
    { href: "/pt-br/", label: "Início" },
    { href: "/docs/pt-br/", label: "Docs" },
    { href: "/pt-br/plugins/", label: "Plugins" },
    { href: "/docs/pt-br/development/roadmap", label: "Roadmap" },
  ],
};

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
      "Ralph Engine is an open-source plugin-first runtime for agentic coding workflows, built around a Rust-first core, official plugins, MCP-aware integrations, and strict quality gates.",
    socialDescription:
      "A Rust-first runtime for agent loops, plugin contracts, MCP-aware integrations, and quality gates that stay coherent from local development to CI.",
    brandAriaLabel: "Ralph Engine home",
    skipLinkLabel: "Skip to content",
    navigationAriaLabel: "Primary navigation",
    localeSwitcherAriaLabel: "Language switcher",
    menuToggleLabel: "Open navigation menu",
    themeToggleLabel: "Toggle theme",
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
    githubHref: "https://github.com/diegorodrigo90/ralph-engine",
    currentNav: "home",
    footerTagline: "Open-source plugin-first runtime for agentic coding workflows.",
    footerNote: "Built for teams that want durable runtime contracts, disciplined release flow, and public surfaces that stay legible for humans and agents.",
    footerLinks: baseFooterLinks.en,
    footerActions: [
      { href: "/docs/", label: "Read the docs" },
      { href: "/plugins/", label: "Explore plugins" },
    ],
    bodyFile: path.join(rootDir, "site", "landing", "content", "en", "body.html"),
    bodyClass: "surface-home",
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
      "Uma base em Rust para loops de agente, contratos de plugin, integrações com MCP e verificações de qualidade coerentes do desenvolvimento local até a CI.",
    brandAriaLabel: "Página inicial do Ralph Engine",
    skipLinkLabel: "Pular para o conteúdo",
    navigationAriaLabel: "Navegação principal",
    localeSwitcherAriaLabel: "Seletor de idioma",
    menuToggleLabel: "Abrir menu de navegação",
    themeToggleLabel: "Alternar tema",
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
    githubHref: "https://github.com/diegorodrigo90/ralph-engine",
    currentNav: "home",
    footerTagline: "Runtime open source orientado a plugins para fluxos de desenvolvimento com agentes.",
    footerNote: "Feito para times que querem contratos duráveis de runtime, fluxo sério de release e superfícies públicas legíveis tanto para pessoas quanto para agentes.",
    footerLinks: baseFooterLinks["pt-br"],
    footerActions: [
      { href: "/docs/pt-br/", label: "Ler as docs" },
      { href: "/pt-br/plugins/", label: "Explorar plugins" },
    ],
    bodyFile: path.join(rootDir, "site", "landing", "content", "pt-br", "body.html"),
    bodyClass: "surface-home",
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
      "Official plugin metadata, trust framing, and machine-readable paths live here before the marketplace grows denser.",
    brandAriaLabel: "Ralph Engine home",
    skipLinkLabel: "Skip to content",
    navigationAriaLabel: "Primary navigation",
    localeSwitcherAriaLabel: "Language switcher",
    menuToggleLabel: "Open navigation menu",
    themeToggleLabel: "Toggle theme",
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
    githubHref: "https://github.com/diegorodrigo90/ralph-engine",
    currentNav: "plugins",
    footerTagline: "The plugin surface is part of the runtime contract, not a detached marketplace shell.",
    footerNote: "Keep discovery honest, machine-readable, and trust-aware while the ecosystem grows.",
    footerLinks: [
      ...baseFooterLinks.en,
      { href: "/plugins/index.json", label: "index.json" },
    ],
    footerActions: [
      { href: "/docs/guides/plugins", label: "Read the plugin guide" },
      { href: "/plugins/index.json", label: "Open index.json" },
    ],
    bodyFile: path.join(rootDir, "site", "plugins", "content", "en", "body.html"),
    bodyClass: "surface-plugins",
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
      "Metadados de plugins oficiais, sinais de confiança e caminhos legíveis por máquina vivem aqui antes de a vitrine ficar mais densa.",
    brandAriaLabel: "Página inicial do Ralph Engine",
    skipLinkLabel: "Pular para o conteúdo",
    navigationAriaLabel: "Navegação principal",
    localeSwitcherAriaLabel: "Seletor de idioma",
    menuToggleLabel: "Abrir menu de navegação",
    themeToggleLabel: "Alternar tema",
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
    githubHref: "https://github.com/diegorodrigo90/ralph-engine",
    currentNav: "plugins",
    footerTagline: "A superfície de plugins faz parte do contrato do runtime, não de uma vitrine solta.",
    footerNote: "Descoberta precisa continuar honesta, legível por máquina e orientada a confiança enquanto o ecossistema cresce.",
    footerLinks: [
      ...baseFooterLinks["pt-br"],
      { href: "/plugins/index.json", label: "index.json" },
    ],
    footerActions: [
      { href: "/docs/pt-br/guides/plugins", label: "Ler o guia de plugins" },
      { href: "/plugins/index.json", label: "Abrir index.json" },
    ],
    bodyFile: path.join(rootDir, "site", "plugins", "content", "pt-br", "body.html"),
    bodyClass: "surface-plugins",
  },
];

function renderNavLink(link) {
  return `              <a${link.current ? ' aria-current="page"' : ""} href="${link.href}">${link.label}</a>`;
}

function renderFooterLinks(links) {
  return links.map((link) => `            <a href="${link.href}">${link.label}</a>`).join("\n");
}

function renderFooterActions(actions) {
  return actions
    .map((action, index) => {
      const className = index === 0 ? "button primary subtle-lift" : "button secondary subtle-lift";
      return `            <a class="${className}" href="${action.href}">${action.label}</a>`;
    })
    .join("\n");
}

function renderPage(page) {
  const body = readFileSync(page.bodyFile, "utf8").trim();
  const targetFile = path.join(outputDir, page.outputPath);
  mkdirSync(path.dirname(targetFile), { recursive: true });

  const navLinks = [
    { href: page.homeHref, label: page.homeLabel, current: page.currentNav === "home" },
    { href: page.docsHref, label: page.docsLabel, current: false },
    { href: page.pluginsHref, label: page.pluginsLabel, current: page.currentNav === "plugins" },
    { href: page.roadmapHref, label: page.roadmapLabel, current: false },
    { href: page.githubHref, label: page.githubLabel, current: false },
  ];

  const html = `<!doctype html>
<html lang="${page.htmlLang}">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover" />
    <title>${page.title}</title>
    <meta name="description" content="${page.metaDescription}" />
    <meta name="robots" content="index,follow" />
    <meta name="theme-color" content="#5a67d8" />
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
    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:title" content="${page.title}" />
    <meta name="twitter:description" content="${page.socialDescription}" />
    <meta name="twitter:image" content="https://ralphengine.com/logo-icon.svg" />
    <link rel="icon" type="image/svg+xml" href="/logo-icon.svg" />
    <link rel="stylesheet" href="/styles.css" />
${themeBootScript}
${publicScriptTag}
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
  <body class="${page.bodyClass}">
    <a class="skip-link" href="#content">${page.skipLinkLabel}</a>
    <div class="site-backdrop" aria-hidden="true">
      <div class="site-orb orb-one"></div>
      <div class="site-orb orb-two"></div>
      <div class="site-grid"></div>
    </div>
    <main class="shell">
      <header class="shell-header">
        <div class="page shell-header-inner">
          <nav class="topbar" aria-label="${page.navigationAriaLabel}">
            <a class="brand subtle-lift" href="/" aria-label="${page.brandAriaLabel}">
              <span class="brand-mark">
                <img src="/logo-icon.svg" alt="" width="28" height="28" decoding="async" />
              </span>
              <span class="brand-copy">
                <strong>Ralph Engine</strong>
                <span>${page.locale === "pt-br" ? "Runtime para desenvolvimento com agentes" : "Runtime for agentic coding workflows"}</span>
              </span>
            </a>
            <div class="topbar-actions">
              <button
                class="icon-button theme-toggle subtle-lift"
                type="button"
                data-theme-toggle
                aria-label="${page.themeToggleLabel}"
                title="${page.themeToggleLabel}"
              >
                <span class="theme-icon theme-icon-light" aria-hidden="true">☀</span>
                <span class="theme-icon theme-icon-dark" aria-hidden="true">☾</span>
              </button>
              <button
                class="icon-button menu-toggle subtle-lift"
                type="button"
                data-menu-toggle
                aria-expanded="false"
                aria-controls="site-nav"
                aria-label="${page.menuToggleLabel}"
              >
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
              </button>
            </div>
            <div class="topbar-panel" id="site-nav" data-menu-panel>
              <div class="nav-links">
${navLinks.map(renderNavLink).join("\n")}
              </div>
              <div class="locale-switcher" aria-label="${page.localeSwitcherAriaLabel}">
                <a${page.locale === "en" ? ' aria-current="page"' : ""} href="${page.locale === "en" ? page.localeSelfHref : page.localeOtherHref}">EN</a>
                <a${page.locale === "pt-br" ? ' aria-current="page"' : ""} href="${page.locale === "pt-br" ? page.localeSelfHref : page.localeOtherHref}">PT-BR</a>
              </div>
            </div>
          </nav>
        </div>
      </header>

${body}

      <footer class="site-footer">
        <div class="page footer-shell">
          <div class="footer-copy">
            <p class="footer-kicker">${page.locale === "pt-br" ? "Superfícies públicas unificadas" : "Unified public surfaces"}</p>
            <h2>${page.footerTagline}</h2>
            <p>${page.footerNote}</p>
          </div>
          <div class="footer-meta">
            <div class="footer-links">
${renderFooterLinks(page.footerLinks)}
            </div>
            <div class="footer-actions">
${renderFooterActions(page.footerActions)}
            </div>
          </div>
        </div>
      </footer>
    </main>
${analyticsLoader}
  </body>
</html>
`;

  writeFileSync(targetFile, html);
}

mkdirSync(outputDir, { recursive: true });

for (const page of pages) {
  renderPage(page);
}

cpSync(path.join(rootDir, "site", "plugins", "index.json"), path.join(outputDir, "plugins", "index.json"));
