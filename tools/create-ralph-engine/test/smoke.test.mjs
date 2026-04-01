import assert from "node:assert/strict";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { spawnSync } from "node:child_process";

const rootDir = path.resolve(import.meta.dirname, "..");
const binPath = path.join(rootDir, "bin", "create-ralph-engine-plugin.js");

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
  assert.match(manifest, /id: acme\/jira-suite/);
  assert.match(manifest, /kind: mcp_contribution/);
  assert.match(manifest, /- mcp_contribution/);
  assert.match(manifest, /- context_provider/);
  assert.match(manifest, /- data_source/);
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
  assert.equal(fs.existsSync(path.join(targetDir, "template", "config.yaml")), true);
  assert.equal(fs.existsSync(path.join(targetDir, "template", "hooks.yaml")), true);
  assert.equal(fs.existsSync(path.join(targetDir, "template", "prompt.md")), true);
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
