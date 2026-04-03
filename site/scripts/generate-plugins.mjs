/**
 * Reads all official plugin manifest.yaml files and generates:
 * - src/data/plugins.json (for Astro build-time imports)
 * - public/plugins/index.json (for machine consumers / llms.txt)
 */

import { readFileSync, writeFileSync, readdirSync, mkdirSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import yaml from 'js-yaml';

const __dirname = dirname(fileURLToPath(import.meta.url));
const siteRoot = join(__dirname, '..');
const pluginsDir = join(siteRoot, '..', 'plugins', 'official');

const pluginDirs = readdirSync(pluginsDir, { withFileTypes: true })
  .filter((d) => d.isDirectory())
  .map((d) => d.name)
  .sort();

const plugins = pluginDirs.map((name) => {
  const manifestPath = join(pluginsDir, name, 'manifest.yaml');
  const manifest = yaml.load(readFileSync(manifestPath, 'utf8'));
  return manifest;
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
  JSON.stringify(
    {
      version: 1,
      generated_at: new Date().toISOString(),
      count: plugins.length,
      plugins,
    },
    null,
    2,
  ),
);

console.log(`Generated plugin data for ${plugins.length} plugins.`);
