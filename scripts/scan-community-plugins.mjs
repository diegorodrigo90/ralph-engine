#!/usr/bin/env node

/**
 * Scans GitHub for community Ralph Engine plugins and updates the
 * public catalog index. Runs as a scheduled GitHub Action (daily).
 *
 * Discovery convention (same as Terraform providers):
 * - Repo topic: "ralph-engine-plugin"
 * - Repo name: "ralph-engine-plugin-{name}" (recommended, not required)
 * - Must have manifest.yaml at repo root
 *
 * Cost: zero — uses GitHub API (5000 req/hr free for authenticated),
 * GitHub Actions (unlimited for public repos), static JSON on Cloudflare.
 */

import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
// Single source of truth — both Astro and public API read from here.
const CATALOG_PATH = join(__dirname, "..", "site", "src", "data", "plugins.json");
const PUBLIC_CATALOG_PATH = join(__dirname, "..", "site", "public", "plugins", "index.json");
const TOPIC = "ralph-engine-plugin";
const GITHUB_API = "https://api.github.com";
const MANIFEST_FILE = "manifest.yaml";

// GitHub token from environment (set by Actions or local dev)
const TOKEN = process.env.GITHUB_TOKEN || process.env.GH_TOKEN || "";

async function githubFetch(url) {
  const headers = {
    Accept: "application/vnd.github.v3+json",
    "User-Agent": "ralph-engine-catalog-scan",
  };
  if (TOKEN) {
    headers.Authorization = `token ${TOKEN}`;
  }
  const response = await fetch(url, { headers });
  if (!response.ok) {
    const text = await response.text();
    throw new Error(`GitHub API ${response.status}: ${text.slice(0, 200)}`);
  }
  return response.json();
}

/**
 * Search GitHub for repos with the ralph-engine-plugin topic.
 * Returns array of { full_name, html_url, description, default_branch, owner, topics }.
 */
async function discoverCommunityRepos() {
  const repos = [];
  let page = 1;
  const perPage = 100;

  while (true) {
    const url = `${GITHUB_API}/search/repositories?q=topic:${TOPIC}+fork:false&sort=updated&per_page=${perPage}&page=${page}`;
    const data = await githubFetch(url);

    for (const repo of data.items || []) {
      // Skip the ralph-engine repo itself (official plugins are already indexed)
      if (repo.full_name === "diegorodrigo90/ralph-engine") continue;

      repos.push({
        full_name: repo.full_name,
        html_url: repo.html_url,
        description: repo.description || "",
        default_branch: repo.default_branch,
        owner: repo.owner?.login || "",
        topics: repo.topics || [],
        archived: repo.archived || false,
        updated_at: repo.updated_at,
      });
    }

    if ((data.items || []).length < perPage) break;
    page++;
  }

  return repos;
}

/**
 * Fetch manifest.yaml from a repo's default branch.
 * Returns raw YAML string or null if not found.
 */
async function fetchManifest(repo) {
  try {
    const url = `${GITHUB_API}/repos/${repo.full_name}/contents/${MANIFEST_FILE}?ref=${repo.default_branch}`;
    const data = await githubFetch(url);
    if (data.encoding === "base64" && data.content) {
      return Buffer.from(data.content, "base64").toString("utf-8");
    }
    return null;
  } catch {
    return null;
  }
}

/**
 * Parse manifest YAML (simple key: value extraction, no dependency).
 * Returns object with id, kind, display_name, summary, etc.
 */
function parseManifestSimple(yaml) {
  const result = {};
  const lines = yaml.split("\n");

  for (const line of lines) {
    const match = line.match(/^(\w[\w_]*)\s*:\s*(.+)$/);
    if (match) {
      const key = match[1].trim();
      let value = match[2].trim();
      // Strip quotes
      if ((value.startsWith('"') && value.endsWith('"')) ||
          (value.startsWith("'") && value.endsWith("'"))) {
        value = value.slice(1, -1);
      }
      result[key] = value;
    }
  }

  // Parse capabilities array
  const capsMatch = yaml.match(/capabilities:\s*\n((?:\s+-\s+\w+\n?)+)/);
  if (capsMatch) {
    result.capabilities = capsMatch[1]
      .split("\n")
      .map((l) => l.replace(/^\s*-\s*/, "").trim())
      .filter(Boolean);
  }

  return result;
}

/**
 * Detects if a version string is a prerelease (alpha, beta, rc).
 * SemVer: prerelease identifiers follow a hyphen after the patch.
 * Examples: "1.0.0-alpha.1", "2.0.0-beta", "3.0.0-rc.2"
 */
function isPrerelease(version) {
  if (!version) return false;
  return /-(alpha|beta|rc|dev|pre|canary|nightly)/i.test(version);
}

/**
 * Returns the stability level of a version string.
 */
function stabilityLevel(version) {
  if (!version) return "unknown";
  if (/-(alpha|dev|nightly)/i.test(version)) return "alpha";
  if (/-(beta|canary)/i.test(version)) return "beta";
  if (/-(rc|pre)/i.test(version)) return "rc";
  return "stable";
}

/**
 * Validate that a parsed manifest has minimum required fields.
 */
function isValidManifest(manifest) {
  return (
    typeof manifest.id === "string" &&
    manifest.id.length > 0 &&
    typeof manifest.kind === "string" &&
    typeof manifest.display_name === "string" &&
    typeof manifest.summary === "string" &&
    // Must NOT be an official plugin
    !manifest.id.startsWith("official.")
  );
}

/**
 * Check if a previously known repo still exists.
 */
async function repoExists(fullName) {
  try {
    const headers = {
      "User-Agent": "ralph-engine-catalog-scan",
    };
    if (TOKEN) {
      headers.Authorization = `token ${TOKEN}`;
    }
    const response = await fetch(`${GITHUB_API}/repos/${fullName}`, {
      method: "HEAD",
      headers,
    });
    return response.ok;
  } catch {
    return false;
  }
}

async function main() {
  console.log("Ralph Engine Community Plugin Catalog Scan");
  console.log("==========================================\n");

  // Load existing index (create empty seed if missing — first run or fresh checkout)
  let index;
  if (existsSync(CATALOG_PATH)) {
    try {
      index = JSON.parse(readFileSync(CATALOG_PATH, "utf-8"));
    } catch {
      console.error(`Failed to parse ${CATALOG_PATH}, starting with empty catalog`);
      index = { plugins: [] };
    }
  } else {
    console.log(`${CATALOG_PATH} not found, starting with empty catalog`);
    index = { plugins: [] };
  }

  const officialPlugins = index.plugins.filter(
    (p) => !p.source || p.source === "official"
  );
  const existingCommunity = index.plugins.filter(
    (p) => p.source === "community"
  );

  console.log(`Official plugins: ${officialPlugins.length}`);
  console.log(`Existing community plugins: ${existingCommunity.length}\n`);

  // Discover repos with topic
  console.log(`Searching GitHub for topic: ${TOPIC}...`);
  let repos;
  try {
    repos = await discoverCommunityRepos();
  } catch (err) {
    console.error(`GitHub API error: ${err.message}`);
    // Don't fail — keep existing catalog
    console.log("Keeping existing catalog unchanged.");
    process.exit(0);
  }
  console.log(`Found ${repos.length} candidate repos\n`);

  // Fetch and validate manifests
  const communityPlugins = [];
  for (const repo of repos) {
    if (repo.archived) {
      console.log(`  SKIP ${repo.full_name} (archived)`);
      continue;
    }

    const yaml = await fetchManifest(repo);
    if (!yaml) {
      console.log(`  SKIP ${repo.full_name} (no manifest.yaml)`);
      continue;
    }

    const manifest = parseManifestSimple(yaml);
    if (!isValidManifest(manifest)) {
      console.log(`  SKIP ${repo.full_name} (invalid manifest)`);
      continue;
    }

    console.log(`  OK   ${repo.full_name} → ${manifest.id}`);

    const version = manifest.plugin_version || "0.0.0";
    communityPlugins.push({
      id: manifest.id,
      kind: manifest.kind,
      display_name: manifest.display_name,
      summary: manifest.summary,
      publisher: manifest.publisher || repo.owner,
      plugin_version: version,
      stability: stabilityLevel(version),
      prerelease: isPrerelease(version),
      trust_level: "community",
      source: "community",
      repository: repo.html_url,
      capabilities: manifest.capabilities || [],
      updated_at: repo.updated_at,
    });
  }

  // Check for removed repos (were in catalog but not found in scan)
  const scannedIds = new Set(communityPlugins.map((p) => p.id));
  for (const existing of existingCommunity) {
    if (!scannedIds.has(existing.id)) {
      // Check if repo still exists
      const repoUrl = existing.repository || "";
      const fullName = repoUrl.replace("https://github.com/", "");
      if (fullName && !(await repoExists(fullName))) {
        console.log(`  REMOVED ${existing.id} (repo ${fullName} no longer exists)`);
      } else {
        // Repo exists but topic was removed — keep for one more cycle
        console.log(`  KEEP ${existing.id} (repo exists but topic missing)`);
        communityPlugins.push(existing);
      }
    }
  }

  // Merge: official + community
  const allPlugins = [...officialPlugins, ...communityPlugins];

  // Update index
  index.plugins = allPlugins;
  index.count = allPlugins.length;
  index.generated_at = new Date().toISOString();
  index.community_count = communityPlugins.length;
  index.last_scan = new Date().toISOString();

  const catalogJson = JSON.stringify(index, null, 2) + "\n";
  writeFileSync(CATALOG_PATH, catalogJson);
  // Keep public API in sync (DRY — same content, two locations for
  // different consumers: Astro build vs static HTTP).
  writeFileSync(PUBLIC_CATALOG_PATH, catalogJson);

  console.log(`\nCatalog updated: ${officialPlugins.length} official + ${communityPlugins.length} community = ${allPlugins.length} total`);
}

main().catch((err) => {
  console.error(`Fatal: ${err.message}`);
  process.exit(1);
});
