import assert from "node:assert/strict";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";

const rootDir = path.resolve(import.meta.dirname, "..");
const binPath = path.join(rootDir, "bin", "create-ralph-engine-plugin.js");
const require = createRequire(import.meta.url);
const { parseManifestDocument, validateManifestDocument } = require("../lib/manifest-contract.js");

test("creates a non-interactive plugin scaffold", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "create-ralph-engine-plugin-"));
  const targetDir = path.join(tempDir, "jira-suite");

  const result = spawnSync(process.execPath, [
    binPath,
    "plugin",
    "jira-suite",
    "--publisher",
    "acme",
    "--kind",
    "mcp_contribution",
    "--capability",
    "context_provider",
    "--capability",
    "data_source",
  ], {
    cwd: tempDir,
    encoding: "utf8",
  });

  assert.equal(result.status, 0, result.stderr);

  const manifest = fs.readFileSync(path.join(targetDir, "manifest.yaml"), "utf8");
  const cargoToml = fs.readFileSync(path.join(targetDir, "Cargo.toml"), "utf8");
  const rustLib = fs.readFileSync(path.join(targetDir, "src", "lib.rs"), "utf8");
  validateManifestDocument(manifest);
  assert.match(manifest, /id: acme\.jira-suite/);
  assert.match(manifest, /kind: mcp_contribution/);
  assert.match(manifest, /display_name_locales:\n  pt-br: Jira Suite/);
  assert.match(manifest, /summary: Jira Suite plugin for Ralph Engine\./);
  assert.match(manifest, /summary_locales:\n  pt-br: Plugin Jira Suite para o Ralph Engine\./);
  assert.match(manifest, /- mcp_contribution/);
  assert.match(manifest, /- context_provider/);
  assert.match(manifest, /- data_source/);
  assert.match(cargoToml, /name = "re-plugin-acme-jira-suite"/);
  assert.match(cargoToml, /re-plugin = \{ git = "https:\/\/github\.com\/diegorodrigo90\/ralph-engine\.git", tag = "v0\.2\.0-alpha\.1", package = "re-plugin" \}/);
  assert.match(rustLib, /pub const PLUGIN_ID: &str = "acme\.jira-suite";/);
  assert.match(rustLib, /PluginTrustLevel::Community/);
  assert.match(rustLib, /PluginRuntimeHook::McpRegistration/);
  assert.match(rustLib, /PluginRuntimeHook::ContextProviderRegistration/);
  assert.match(rustLib, /PluginRuntimeHook::DataSourceRegistration/);
});

test("renders help in pt-br when locale is configured", () => {
  const result = spawnSync(process.execPath, [
    binPath,
    "--help",
  ], {
    cwd: rootDir,
    encoding: "utf8",
    env: {
      ...process.env,
      RALPH_ENGINE_LOCALE: "pt-br",
    },
  });

  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /Uso:/);
  assert.match(result.stdout, /Opções:/);
  assert.match(result.stdout, /Slug do nome do plugin/);
});

test("creates template assets when template capability is present", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "create-ralph-engine-plugin-"));
  const targetDir = path.join(tempDir, "bmad-pack");

  const result = spawnSync(process.execPath, [
    binPath,
    "plugin",
    "bmad-pack",
    "--publisher",
    "community",
    "--kind",
    "template",
  ], {
    cwd: tempDir,
    encoding: "utf8",
  });

  assert.equal(result.status, 0, result.stderr);
  const manifest = fs.readFileSync(path.join(targetDir, "manifest.yaml"), "utf8");
  const rustLib = fs.readFileSync(path.join(targetDir, "src", "lib.rs"), "utf8");
  validateManifestDocument(manifest);
  assert.equal(fs.existsSync(path.join(targetDir, "template", "config.yaml")), true);
  assert.equal(fs.existsSync(path.join(targetDir, "template", "hooks.yaml")), true);
  assert.equal(fs.existsSync(path.join(targetDir, "template", "prompt.md")), true);
  assert.match(rustLib, /PluginRuntimeHook::Scaffold/);
});

test("rejects manifests that drift from the typed contract", () => {
  assert.throws(
    () =>
      validateManifestDocument(
        `id: acme.jira-suite
kind: data_source
display_name: Jira Suite
display_name_locales:
  pt-br: Jira Suite
summary: Jira Suite plugin for Ralph Engine.
summary_locales:
  pt-br: Plugin Jira Suite para o Ralph Engine.
publisher: acme
trust_level: community
plugin_version: 0.1.0
capabilities:
  - data_source
project:
  required_files:
    - .ralph-engine/config.yaml
`,
        "manifest.yaml",
      ),
    /project metadata is only valid when the template capability is declared/,
  );
});

test("renders manifest contract errors in pt-br when locale is configured", () => {
  const previousLocale = process.env.RALPH_ENGINE_LOCALE;
  process.env.RALPH_ENGINE_LOCALE = "pt-br";

  try {
    assert.throws(
      () =>
        validateManifestDocument(
          `id: acme.jira-suite
kind: data_source
display_name: Jira Suite
publisher: acme
trust_level: community
plugin_version: 0.1.0
capabilities:
  - data_source
`,
          "manifest.yaml",
        ),
      /campo obrigatório ausente: "summary"/,
    );
  } finally {
    if (previousLocale === undefined) {
      delete process.env.RALPH_ENGINE_LOCALE;
    } else {
      process.env.RALPH_ENGINE_LOCALE = previousLocale;
    }
  }
});

test("rejects reserved publisher", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "create-ralph-engine-plugin-"));
  const result = spawnSync(process.execPath, [
    binPath,
    "plugin",
    "danger",
    "--publisher",
    "official",
  ], {
    cwd: tempDir,
    encoding: "utf8",
  });

  assert.equal(result.status, 1);
});

test("renders validation errors in pt-br when locale is configured", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "create-ralph-engine-plugin-"));
  const result = spawnSync(process.execPath, [
    binPath,
    "plugin",
    "danger",
    "--publisher",
    "official",
  ], {
    cwd: tempDir,
    encoding: "utf8",
    env: {
      ...process.env,
      RALPH_ENGINE_LOCALE: "pt-br",
    },
  });

  assert.equal(result.status, 1);
  assert.match(result.stderr, /O publicador "official" é reservado\./);
});

test("rejects unsupported future kind", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "create-ralph-engine-plugin-"));
  const result = spawnSync(process.execPath, [
    binPath,
    "plugin",
    "danger",
    "--publisher",
    "acme",
    "--kind",
    "tracker_provider",
  ], {
    cwd: tempDir,
    encoding: "utf8",
  });

  assert.equal(result.status, 1);
  assert.match(result.stderr, /Unsupported kind "tracker_provider"/);
});

test("rejects unsupported future capability", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "create-ralph-engine-plugin-"));
  const result = spawnSync(process.execPath, [
    binPath,
    "plugin",
    "danger",
    "--publisher",
    "acme",
    "--capability",
    "dashboard_events",
  ], {
    cwd: tempDir,
    encoding: "utf8",
  });

  assert.equal(result.status, 1);
  assert.match(result.stderr, /Unsupported capability "dashboard_events"/);
});

test("rejects manifests without required summary", () => {
  assert.throws(
    () =>
      validateManifestDocument(
        `id: acme.jira-suite
kind: data_source
display_name: Jira Suite
publisher: acme
trust_level: community
plugin_version: 0.1.0
capabilities:
  - data_source
`,
        "manifest.yaml",
      ),
    /missing required field "summary"/,
  );
});

test("official manifests keep localized display metadata aligned with plugin metadata", () => {
  const officialPluginDir = path.resolve(rootDir, "..", "..", "plugins", "official");
  const manifestFiles = fs.readdirSync(officialPluginDir)
    .map((pluginDir) => path.join(officialPluginDir, pluginDir, "manifest.yaml"));

  for (const manifestFile of manifestFiles) {
    const manifest = parseManifestDocument(fs.readFileSync(manifestFile, "utf8"), manifestFile);

    assert.equal(typeof manifest.id, "string");
    assert.equal(typeof manifest.name, "string");
    assert.equal(typeof manifest.display_name, "string");
    assert.equal(manifest.display_name.length > 0, true);
    assert.equal(typeof manifest.summary, "string");
    assert.equal(manifest.summary.length > 0, true);

    assert.equal(typeof manifest.display_name_locales, "object");
    assert.equal(typeof manifest.summary_locales, "object");
    assert.equal(typeof manifest.display_name_locales["pt-br"], "string");
    assert.equal(typeof manifest.summary_locales["pt-br"], "string");
  }
});
