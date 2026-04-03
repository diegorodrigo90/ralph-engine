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

const plugins = pluginDirs.map((name) => {
  const manifestPath = join(pluginsDir, name, 'manifest.yaml');
  const manifest = yaml.load(readFileSync(manifestPath, 'utf8'));
  const iconUrl = processIcon(name, manifest.id);
  return { ...manifest, iconUrl };
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
