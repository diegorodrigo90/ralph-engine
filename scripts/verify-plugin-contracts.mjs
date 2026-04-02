#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import process from "node:process";

const ROOT_DIR = path.resolve(import.meta.dirname, "..");
const RUST_PLUGIN_CONTRACT_PATH = path.join(
  ROOT_DIR,
  "core",
  "crates",
  "re-plugin",
  "src",
  "lib.rs",
);
const SCAFFOLDER_PATH = path.join(
  ROOT_DIR,
  "tools",
  "create-ralph-engine",
  "bin",
  "create-ralph-engine-plugin.js",
);

function fail(message) {
  process.stderr.write(`${message}\n`);
  process.exit(1);
}

function readUtf8(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

function parseRustCapabilityConstants(source) {
  const matches = [...source.matchAll(/pub const [A-Z_]+: PluginCapability = PluginCapability::new\("([^"]+)"\);/g)];
  return new Set(matches.map((match) => match[1]));
}

function parseRustPluginKinds(source) {
  const kindImplMatch = source.match(/impl PluginKind \{([\s\S]*?)\n\}/);
  if (!kindImplMatch) {
    fail("could not find PluginKind implementation in Rust plugin contract");
  }

  return new Set(
    [...kindImplMatch[1].matchAll(/Self::[A-Za-z]+ => "([^"]+)"/g)].map(
      (match) => match[1],
    ),
  );
}

function parseScaffolderSet(source, setName) {
  const regex = new RegExp(
    `const ${setName} = new Set\\(\\[([\\s\\S]*?)\\]\\);`,
    "m",
  );
  const match = source.match(regex);
  if (!match) {
    fail(`could not find ${setName} in scaffolder`);
  }

  return new Set(
    [...match[1].matchAll(/"([^"]+)"/g)].map((innerMatch) => innerMatch[1]),
  );
}

function parseDefaultKind(source) {
  const match = source.match(/const DEFAULT_KIND = "([^"]+)";/);
  if (!match) {
    fail("could not find DEFAULT_KIND in scaffolder");
  }

  return match[1];
}

function parseDefaultCapabilitiesByKind(source) {
  const match = source.match(/function defaultCapabilitiesForKind\(kind\) \{([\s\S]*?)\n\}/);
  if (!match) {
    fail("could not find defaultCapabilitiesForKind in scaffolder");
  }

  const branchMatches = [
    ...match[1].matchAll(/case "([^"]+)":\s+return \[(.*?)\];/g),
  ];

  return new Map(
    branchMatches.map(([_, kind, rawCapabilities]) => [
      kind,
      [...rawCapabilities.matchAll(/"([^"]+)"/g)].map((capabilityMatch) => capabilityMatch[1]),
    ]),
  );
}

function assertSubset(actualSet, allowedSet, label) {
  const unexpected = [...actualSet].filter((value) => !allowedSet.has(value));
  if (unexpected.length > 0) {
    fail(`${label} contains unsupported values: ${unexpected.join(", ")}`);
  }
}

function assertExactSet(actualSet, expectedSet, label) {
  const missing = [...expectedSet].filter((value) => !actualSet.has(value));
  const extra = [...actualSet].filter((value) => !expectedSet.has(value));

  if (missing.length > 0 || extra.length > 0) {
    fail(
      `${label} drift detected.\nmissing: ${missing.join(", ") || "(none)"}\nextra: ${extra.join(", ") || "(none)"}`,
    );
  }
}

const rustPluginContract = readUtf8(RUST_PLUGIN_CONTRACT_PATH);
const scaffolderSource = readUtf8(SCAFFOLDER_PATH);

const rustCapabilities = parseRustCapabilityConstants(rustPluginContract);
const rustKinds = parseRustPluginKinds(rustPluginContract);
const supportedCapabilities = parseScaffolderSet(scaffolderSource, "SUPPORTED_CAPABILITIES");
const supportedKinds = parseScaffolderSet(scaffolderSource, "SUPPORTED_KINDS");
const defaultKind = parseDefaultKind(scaffolderSource);
const defaultCapabilitiesByKind = parseDefaultCapabilitiesByKind(scaffolderSource);

assertExactSet(
  supportedCapabilities,
  rustCapabilities,
  "scaffolder supported capabilities",
);
assertExactSet(supportedKinds, rustKinds, "scaffolder supported kinds");

if (!supportedKinds.has(defaultKind)) {
  fail(`DEFAULT_KIND must stay inside SUPPORTED_KINDS: ${defaultKind}`);
}

for (const [kind, capabilities] of defaultCapabilitiesByKind.entries()) {
  if (!supportedKinds.has(kind)) {
    fail(`defaultCapabilitiesForKind declares unsupported kind: ${kind}`);
  }

  assertSubset(
    new Set(capabilities),
    supportedCapabilities,
    `defaultCapabilitiesForKind(${kind})`,
  );
}

process.stdout.write("Plugin contracts verified.\n");
