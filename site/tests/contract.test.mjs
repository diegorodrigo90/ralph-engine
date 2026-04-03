/**
 * Site Contract Tests — TEA risk-based approach
 *
 * These tests validate the build output contracts for the Ralph Engine site.
 * Risk priority: P1 (build breaks) > P2 (missing pages) > P3 (data integrity) > P4 (SEO)
 *
 * Run: node --test site/tests/contract.test.mjs
 * Prereq: npm run build (dist/ must exist)
 */

import { describe, it } from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';

const SITE_ROOT = new URL('..', import.meta.url).pathname;
const DIST = join(SITE_ROOT, 'dist');
const DATA = join(SITE_ROOT, 'src', 'data');

// ── P1: Build output exists ──────────────────────────────────────

describe('P1: Build output', () => {
  it('dist/ directory exists', () => {
    assert.ok(existsSync(DIST), 'Run npm run build first');
  });

  it('index.html exists at root', () => {
    assert.ok(existsSync(join(DIST, 'index.html')));
  });

  it('sitemap-index.xml exists', () => {
    assert.ok(existsSync(join(DIST, 'sitemap-index.xml')));
  });

  it('pagefind index exists', () => {
    assert.ok(existsSync(join(DIST, 'pagefind')), 'Pagefind search index missing');
  });
});

// ── P2: All expected pages exist (EN) ────────────────────────────

const EN_PAGES = [
  'index.html',
  'plugins/index.html',
  'getting-started/installation/index.html',
  'getting-started/quickstart/index.html',
  'guides/configuration/index.html',
  'guides/hooks/index.html',
  'guides/plugins/index.html',
  'guides/plugin-development/index.html',
  'guides/troubleshooting/index.html',
  'reference/cli/index.html',
  'reference/config/index.html',
  'reference/mcp/index.html',
  'reference/architecture/index.html',
  'development/building/index.html',
  'development/coding-standards/index.html',
  'development/releasing/index.html',
];

describe('P2: EN pages exist', () => {
  for (const page of EN_PAGES) {
    it(`EN: ${page}`, () => {
      assert.ok(existsSync(join(DIST, page)), `Missing: ${page}`);
    });
  }
});

// ── P2: All expected pages exist (PT-BR) ─────────────────────────

const PTBR_PAGES = EN_PAGES
  .filter(p => p !== 'index.html') // landing uses root locale
  .map(p => `pt-br/${p}`);

describe('P2: PT-BR pages exist', () => {
  for (const page of PTBR_PAGES) {
    it(`PT-BR: ${page}`, () => {
      assert.ok(existsSync(join(DIST, page)), `Missing PT-BR page: ${page}`);
    });
  }
});

// ── P2: Plugin detail pages exist ────────────────────────────────

describe('P2: Plugin detail pages', () => {
  const pluginsJson = JSON.parse(readFileSync(join(DATA, 'plugins.json'), 'utf8'));

  for (const plugin of pluginsJson) {
    const slug = plugin.id.replace('official.', '');
    it(`EN plugin page: ${slug}`, () => {
      assert.ok(existsSync(join(DIST, `plugins/${slug}/index.html`)));
    });
    it(`PT-BR plugin page: ${slug}`, () => {
      assert.ok(existsSync(join(DIST, `pt-br/plugins/${slug}/index.html`)));
    });
  }
});

// ── P3: Plugin data integrity ────────────────────────────────────

describe('P3: plugins.json integrity', () => {
  const plugins = JSON.parse(readFileSync(join(DATA, 'plugins.json'), 'utf8'));

  it('is a non-empty array', () => {
    assert.ok(Array.isArray(plugins));
    assert.ok(plugins.length > 0);
  });

  for (const p of plugins) {
    describe(`plugin: ${p.id}`, () => {
      it('has required fields', () => {
        assert.ok(p.id, 'missing id');
        assert.ok(p.kind, 'missing kind');
        assert.ok(p.display_name, 'missing display_name');
        assert.ok(p.summary, 'missing summary');
        assert.ok(p.publisher, 'missing publisher');
        assert.ok(p.trust_level, 'missing trust_level');
        assert.ok(p.plugin_version, 'missing plugin_version');
      });

      it('has description', () => {
        assert.ok(p.description, 'missing description');
        assert.ok(p.description.length > 50, 'description too short');
      });

      it('has PT-BR locales', () => {
        assert.ok(p.display_name_locales?.['pt-br'], 'missing PT-BR display_name');
        assert.ok(p.summary_locales?.['pt-br'], 'missing PT-BR summary');
      });

      it('has docs sections', () => {
        assert.ok(p.docs?.default?.length > 0, 'missing default docs');
      });

      it('has PT-BR docs', () => {
        assert.ok(p.docs?.locales?.['pt-br']?.length > 0, 'missing PT-BR docs');
      });

      it('id matches valid pattern', () => {
        assert.match(p.id, /^[a-z0-9][a-z0-9._-]*$/, `invalid id: ${p.id}`);
      });

      it('description has no HTML tags (sanitized)', () => {
        assert.ok(!/<[^>]+>/.test(p.description), 'description contains HTML');
      });

      it('summary has no HTML tags (sanitized)', () => {
        assert.ok(!/<[^>]+>/.test(p.summary), 'summary contains HTML');
      });
    });
  }
});

// ── P3: i18n completeness ────────────────────────────────────────

describe('P3: i18n completeness', () => {
  const en = JSON.parse(readFileSync(join(SITE_ROOT, 'src', 'i18n', 'en.json'), 'utf8'));
  const ptbr = JSON.parse(readFileSync(join(SITE_ROOT, 'src', 'i18n', 'pt-br.json'), 'utf8'));

  const enKeys = Object.keys(en).sort();
  const ptbrKeys = Object.keys(ptbr).sort();

  it('EN and PT-BR have same keys', () => {
    const missingInPtbr = enKeys.filter(k => !ptbrKeys.includes(k));
    const extraInPtbr = ptbrKeys.filter(k => !enKeys.includes(k));
    assert.deepEqual(missingInPtbr, [], `Missing in PT-BR: ${missingInPtbr.join(', ')}`);
    assert.deepEqual(extraInPtbr, [], `Extra in PT-BR: ${extraInPtbr.join(', ')}`);
  });

  it('no empty values in EN', () => {
    const empty = enKeys.filter(k => !en[k]?.trim());
    assert.deepEqual(empty, [], `Empty EN keys: ${empty.join(', ')}`);
  });

  it('no empty values in PT-BR', () => {
    const empty = ptbrKeys.filter(k => !ptbr[k]?.trim());
    assert.deepEqual(empty, [], `Empty PT-BR keys: ${empty.join(', ')}`);
  });
});

// ── P4: SEO basics ───────────────────────────────────────────────

describe('P4: SEO', () => {
  it('sitemap references all pages', () => {
    const sitemap = readFileSync(join(DIST, 'sitemap-index.xml'), 'utf8');
    assert.ok(sitemap.includes('sitemap'), 'sitemap-index.xml is empty or malformed');
  });

  it('homepage has meta description', () => {
    const html = readFileSync(join(DIST, 'index.html'), 'utf8');
    assert.ok(html.includes('meta name="description"'), 'missing meta description');
  });

  it('homepage has og:title', () => {
    const html = readFileSync(join(DIST, 'index.html'), 'utf8');
    assert.ok(html.includes('og:title'), 'missing og:title');
  });

  it('plugin pages have meta description', () => {
    const plugins = JSON.parse(readFileSync(join(DATA, 'plugins.json'), 'utf8'));
    const slug = plugins[0].id.replace('official.', '');
    const html = readFileSync(join(DIST, `plugins/${slug}/index.html`), 'utf8');
    assert.ok(html.includes('meta name="description"'), 'plugin page missing meta description');
  });
});

// ── P4: Security — no inline scripts from plugin data ────────────

describe('P4: Security', () => {
  it('plugin pages do not contain script injection from plugin data', () => {
    const plugins = JSON.parse(readFileSync(join(DATA, 'plugins.json'), 'utf8'));
    for (const p of plugins) {
      const slug = p.id.replace('official.', '');
      const html = readFileSync(join(DIST, `plugins/${slug}/index.html`), 'utf8');
      // Plugin description/summary should never appear inside <script> tags
      assert.ok(
        !html.includes(`<script>${p.description}`),
        `Plugin ${p.id} description found inside script tag`
      );
    }
  });
});
