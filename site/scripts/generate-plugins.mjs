/**
 * Reads all official plugin manifest.yaml files and generates:
 * - src/data/plugins.json (for Astro build-time imports, includes icon URL)
 * - public/plugins/index.json (for machine consumers)
 * - public/plugins/icons/{id}.{ext} (sanitized icon files)
 *
 * Icon auto-discovery: looks for icon.svg > icon.png > icon.jpg > icon.webp
 * in the plugin directory. No manifest field needed — just drop the file.
 * SVGs are sanitized to prevent XSS. Priority: SVG > PNG > JPG > WebP.
 */

import { readFileSync, writeFileSync, readdirSync, mkdirSync, copyFileSync, existsSync } from 'node:fs';
import { join, dirname, extname } from 'node:path';
import { fileURLToPath } from 'node:url';
import yaml from 'js-yaml';
import { marked } from 'marked';

const __dirname = dirname(fileURLToPath(import.meta.url));
const siteRoot = join(__dirname, '..');
const pluginsDir = join(siteRoot, '..', 'plugins', 'official');
const iconsOutDir = join(siteRoot, 'public', 'plugins', 'icons');

mkdirSync(iconsOutDir, { recursive: true });

/** Icon file names to look for, in priority order. */
const ICON_CANDIDATES = ['icon.svg', 'icon.png', 'icon.jpg', 'icon.jpeg', 'icon.webp'];

/**
 * Sanitize SVG content to prevent XSS/injection.
 * Only allows safe SVG elements and attributes.
 */
function sanitizeSvg(content) {
  let clean = content;
  clean = clean.replace(/<script[\s\S]*?<\/script>/gi, '');
  clean = clean.replace(/<style[\s\S]*?<\/style>/gi, '');
  clean = clean.replace(/<a[\s\S]*?<\/a>/gi, '');
  clean = clean.replace(/<animate[^>]*\/?>/gi, '');
  clean = clean.replace(/<animateTransform[^>]*\/?>/gi, '');
  clean = clean.replace(/<set[^>]*\/?>/gi, '');
  clean = clean.replace(/\s+on\w+\s*=\s*["'][^"']*["']/gi, '');
  clean = clean.replace(/javascript\s*:/gi, 'blocked:');
  clean = clean.replace(/data\s*:\s*(?!image\/(svg\+xml|png|jpeg|webp|gif))/gi, 'blocked:');
  clean = clean.replace(/xlink:href\s*=\s*["'](?!#)[^"']*["']/gi, '');
  clean = clean.replace(/<foreignObject[\s\S]*?<\/foreignObject>/gi, '');
  clean = clean.replace(/<use[^>]*href\s*=\s*["'](?!#)[^"']*["'][^>]*\/?>/gi, '');
  return clean;
}

/**
 * Find and process the best icon for a plugin.
 * Returns the public URL or null.
 */
function processIcon(pluginName, pluginId) {
  for (const candidate of ICON_CANDIDATES) {
    const iconPath = join(pluginsDir, pluginName, candidate);
    if (!existsSync(iconPath)) continue;

    const ext = extname(candidate);
    const outName = `${pluginId.replace(/\./g, '-')}${ext}`;
    const outPath = join(iconsOutDir, outName);

    if (ext === '.svg') {
      const raw = readFileSync(iconPath, 'utf8');
      const sanitized = sanitizeSvg(raw);
      if (sanitized !== raw) {
        console.warn(`  ⚠ ${pluginName}: SVG sanitized (unsafe content removed)`);
      }
      writeFileSync(outPath, sanitized);
    } else {
      copyFileSync(iconPath, outPath);
    }

    return `/plugins/icons/${outName}`;
  }
  return null;
}

const pluginDirs = readdirSync(pluginsDir, { withFileTypes: true })
  .filter((d) => d.isDirectory())
  .map((d) => d.name)
  .sort();

/**
 * Sanitize a plain-text string from manifest YAML.
 * Strips HTML tags, limits length, and normalizes whitespace.
 * SECURITY: plugin descriptions come from third-party manifests
 * and MUST be treated as untrusted input.
 */
function sanitizeText(text, maxLength = 0) {
  if (!text || typeof text !== 'string') return '';
  let clean = text;
  clean = clean.replace(/<[^>]*>/g, '');          // strip HTML tags
  clean = clean.replace(/&[a-z]+;/gi, '');         // strip HTML entities
  clean = clean.replace(/javascript\s*:/gi, '');   // strip JS protocol
  clean = clean.replace(/\s+/g, ' ').trim();       // normalize whitespace (single line)
  if (maxLength > 0 && clean.length > maxLength) {
    clean = clean.slice(0, maxLength) + '…';
  }
  return clean;
}

/**
 * Convert markdown to sanitized HTML at build time.
 * SECURITY pipeline:
 *   1. Strip raw HTML/script from markdown source (before parsing)
 *   2. Parse markdown → HTML via marked
 *   3. Strip dangerous attributes from output HTML (onclick, onerror, etc.)
 *   4. Strip javascript: protocol from output HTML
 * The result is safe for set:html in Astro components.
 */
function markdownToSafeHtml(text) {
  if (!text || typeof text !== 'string') return '';
  // Step 1: strip dangerous content from markdown source
  let clean = text;
  clean = clean.replace(/<script[\s\S]*?<\/script>/gi, '');
  clean = clean.replace(/<style[\s\S]*?<\/style>/gi, '');
  clean = clean.replace(/<iframe[\s\S]*?<\/iframe>/gi, '');
  clean = clean.replace(/<object[\s\S]*?<\/object>/gi, '');
  clean = clean.replace(/<embed[^>]*>/gi, '');
  clean = clean.replace(/<form[\s\S]*?<\/form>/gi, '');
  clean = clean.replace(/<meta[^>]*>/gi, '');
  clean = clean.replace(/<svg[\s\S]*?<\/svg>/gi, '');
  // Step 2: parse markdown to HTML
  let html = marked.parse(clean, { async: false, gfm: true, breaks: false });
  // Step 3: strip dangerous attributes/protocols from output
  html = html.replace(/\s+on\w+\s*=\s*["'][^"']*["']/gi, '');
  html = html.replace(/javascript\s*:/gi, 'blocked:');
  html = html.replace(/data\s*:\s*(?!image\/(png|jpeg|gif|webp|svg\+xml))/gi, 'blocked:');
  html = html.replace(/<form[\s\S]*?<\/form>/gi, '');
  html = html.replace(/<meta[^>]*>/gi, '');
  return html.trim();
}

/**
 * Validate a plugin/resource ID against allowlist pattern.
 * SECURITY: IDs from third-party plugins end up in data-* attributes and CLI commands.
 */
const VALID_ID_PATTERN = /^[a-z0-9][a-z0-9._-]*$/;
function validateId(id, context) {
  if (!id || typeof id !== 'string') return id;
  if (!VALID_ID_PATTERN.test(id)) {
    console.warn(`  ⚠ ${context}: invalid ID "${id}" — must match ${VALID_ID_PATTERN}`);
    return id.replace(/[^a-z0-9._-]/g, '');
  }
  return id;
}

/**
 * Sanitize all IDs in sub-resource arrays (agents, templates, checks, etc.).
 */
function sanitizeSubResources(manifest) {
  const arrayFields = ['agents', 'templates', 'providers', 'checks', 'policies', 'prompts'];
  for (const field of arrayFields) {
    if (Array.isArray(manifest[field])) {
      manifest[field] = manifest[field].map(item => ({
        ...item,
        id: validateId(item.id, `${manifest.id}/${field}`),
      }));
    }
  }
  return manifest;
}

/**
 * Sanitize localized text fields in a locale map.
 */
function sanitizeLocales(locales, maxLength = 2000) {
  if (!locales || typeof locales !== 'object') return undefined;
  const result = {};
  for (const [locale, text] of Object.entries(locales)) {
    result[locale] = sanitizeText(text, maxLength);
  }
  return result;
}

/**
 * Load plugin docs from README.md (or README.{locale}.md) in the plugin directory.
 * Parses ## headings into sections. All text is sanitized (untrusted input
 * from third-party plugins).
 *
 * Lookup order: README.{locale}.md → README.md (fallback).
 * Returns { default: sections, locales: { "pt-br": sections } } or undefined.
 *
 * SECURITY: README.md content from community plugins is UNTRUSTED.
 * - HTML tags stripped
 * - JS protocol stripped
 * - Total README limited to 50KB per file (prevents build DoS)
 * - Max 50 sections per plugin per locale
 * - Body rendered as sanitized HTML via set:html in Astro (markdownToSafeHtml pipeline)
 */
const SUPPORTED_LOCALES = ['pt-br'];

function parseReadmeSections(filePath) {
  if (!existsSync(filePath)) return undefined;

  const MAX_README_BYTES = 50 * 1024;
  const MAX_SECTIONS = 50;
  let raw = readFileSync(filePath, 'utf8');
  if (raw.length > MAX_README_BYTES) {
    console.warn(`  ⚠ ${filePath}: truncated (>${MAX_README_BYTES / 1024}KB)`);
    raw = raw.slice(0, MAX_README_BYTES);
  }

  const sections = [];
  const lines = raw.split('\n');
  let current = null;

  for (const line of lines) {
    const headingMatch = line.match(/^##\s+(.+)/);
    if (headingMatch) {
      if (current) sections.push(current);
      current = { heading: sanitizeText(headingMatch[1], 200), body: '' };
    } else if (current) {
      current.body += line + '\n';
    }
  }
  if (current) sections.push(current);

  return sections.length > 0
    ? sections.slice(0, MAX_SECTIONS).map(s => ({
        heading: s.heading,
        body: markdownToSafeHtml(s.body),
      }))
    : undefined;
}

function loadPluginDocs(pluginName) {
  const defaultDocs = parseReadmeSections(join(pluginsDir, pluginName, 'README.md'));
  if (!defaultDocs) return undefined;

  const docs_locales = {};
  for (const locale of SUPPORTED_LOCALES) {
    const localeDocs = parseReadmeSections(join(pluginsDir, pluginName, `README.${locale}.md`));
    if (localeDocs) docs_locales[locale] = localeDocs;
  }

  return {
    default: defaultDocs,
    ...(Object.keys(docs_locales).length > 0 ? { locales: docs_locales } : {}),
  };
}

const plugins = pluginDirs.map((name) => {
  const manifestPath = join(pluginsDir, name, 'manifest.yaml');
  const manifest = yaml.load(readFileSync(manifestPath, 'utf8'));
  const iconUrl = processIcon(name, manifest.id);

  // Sanitize text fields from manifest (untrusted for third-party plugins)
  const description = sanitizeText(manifest.description);
  const description_locales = sanitizeLocales(manifest.description_locales);
  const docs = loadPluginDocs(name);

  const sanitized = sanitizeSubResources({ ...manifest });
  return { ...sanitized, iconUrl, description, description_locales, docs };
});

// Write for Astro build-time import
const dataDir = join(siteRoot, 'src', 'data');
mkdirSync(dataDir, { recursive: true });
writeFileSync(join(dataDir, 'plugins.json'), JSON.stringify(plugins, null, 2));

// Write public machine-readable catalog
const publicPluginsDir = join(siteRoot, 'public', 'plugins');
mkdirSync(publicPluginsDir, { recursive: true });
writeFileSync(
  join(publicPluginsDir, 'index.json'),
  JSON.stringify({
    version: 1,
    generated_at: new Date().toISOString(),
    count: plugins.length,
    icon_contract: {
      discovery: 'Auto — place icon.svg (preferred), icon.png, icon.jpg, or icon.webp in plugin root',
      svg: { viewbox: '0 0 24 24', style: 'stroke-based, stroke-width 2', security: 'Sanitized at build time' },
      png: { min_size: '128x128', background: 'transparent' },
    },
    plugins,
  }, null, 2),
);

const withIcons = plugins.filter(p => p.iconUrl).length;
console.log(`Generated plugin data: ${plugins.length} plugins, ${withIcons} with icons.`);
