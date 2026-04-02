#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { createRequire } from "node:module";

const ROOT_DIR = path.resolve(import.meta.dirname, "..");
const require = createRequire(import.meta.url);
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
const MANIFEST_CONTRACT_PATH = path.join(
  ROOT_DIR,
  "tools",
  "create-ralph-engine",
  "lib",
  "manifest-contract.js",
);
const RUNTIME_SURFACES_PATH = path.join(
  ROOT_DIR,
  "tools",
  "create-ralph-engine",
  "lib",
  "runtime-surfaces.js",
);
const SCAFFOLDER_I18N_INDEX_PATH = path.join(
  ROOT_DIR,
  "tools",
  "create-ralph-engine",
  "lib",
  "i18n",
  "index.js",
);
const WORKSPACE_CARGO_TOML_PATH = path.join(ROOT_DIR, "Cargo.toml");
const OFFICIAL_PLUGIN_DIR = path.join(ROOT_DIR, "plugins", "official");

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

function parseRustPluginTrustLevels(source) {
  const trustImplMatch = source.match(/impl PluginTrustLevel \{([\s\S]*?)\n\}/);
  if (!trustImplMatch) {
    fail("could not find PluginTrustLevel implementation in Rust plugin contract");
  }

  return new Set(
    [...trustImplMatch[1].matchAll(/Self::[A-Za-z]+ => "([^"]+)"/g)].map(
      (match) => match[1],
    ),
  );
}

function parseRustRuntimeSurfaces(source) {
  const surfaceImplMatch = source.match(/impl PluginRuntimeSurface \{([\s\S]*?)\n\}/);
  if (!surfaceImplMatch) {
    fail("could not find PluginRuntimeSurface implementation in Rust plugin contract");
  }

  return new Set(
    [...surfaceImplMatch[1].matchAll(/Self::[A-Za-z]+ => "([^"]+)"/g)].map(
      (match) => match[1],
    ),
  );
}

function parseRustRuntimeHooks(source) {
  const hookImplMatch = source.match(/impl PluginRuntimeHook \{([\s\S]*?)\n\}/);
  if (!hookImplMatch) {
    fail("could not find PluginRuntimeHook implementation in Rust plugin contract");
  }

  return new Set(
    [...hookImplMatch[1].matchAll(/Self::[A-Za-z]+ => "([^"]+)"/g)].map(
      (match) => match[1],
    ),
  );
}

function parseRustDefaultLocale(source) {
  const match = source.match(/pub const DEFAULT_LOCALE: &str = "([^"]+)";/);
  if (!match) {
    fail("could not find DEFAULT_LOCALE in Rust config contract");
  }

  return match[1];
}

function parseRustSupportedLocales(source) {
  const localeImplMatch = source.match(/impl SupportedLocale \{([\s\S]*?)\n\}/);
  if (!localeImplMatch) {
    fail("could not find SupportedLocale implementation in Rust config contract");
  }

  return new Set(
    [...localeImplMatch[1].matchAll(/Self::[A-Za-z]+ => "([^"]+)"/g)].map(
      (match) => match[1],
    ),
  );
}

function parseWorkspaceVersion(source) {
  const workspacePackageMatch = source.match(/\[workspace\.package\]([\s\S]*?)\n\[/);
  const section = workspacePackageMatch ? workspacePackageMatch[1] : source;
  const versionMatch = section.match(/version = "([^"]+)"/);
  if (!versionMatch) {
    fail("could not find workspace package version in Cargo.toml");
  }

  return versionMatch[1];
}

function assertOfficialManifestsStayLocalizedAndVersioned(
  parseManifestDocument,
  supportedLocales,
  workspaceVersion,
) {
  const manifestFiles = fs.readdirSync(OFFICIAL_PLUGIN_DIR)
    .map((pluginDir) => path.join(OFFICIAL_PLUGIN_DIR, pluginDir, "manifest.yaml"));

  for (const manifestFile of manifestFiles) {
    const manifest = parseManifestDocument(readUtf8(manifestFile), manifestFile);

    if (manifest.publisher !== "official") {
      fail(`official manifest publisher drift detected in ${manifestFile}: ${manifest.publisher}`);
    }

    const manifestVersion = manifest.plugin_version ?? manifest.version;

    if (manifestVersion !== workspaceVersion) {
      fail(
        `official manifest version drift detected in ${manifestFile}.\nexpected: ${workspaceVersion}\nactual: ${manifestVersion}`,
      );
    }

    if (typeof manifest.display_name !== "string" || manifest.display_name.trim().length === 0) {
      fail(`official manifest ${manifestFile} must declare a non-empty display_name`);
    }

    if (typeof manifest.summary !== "string" || manifest.summary.trim().length === 0) {
      fail(`official manifest ${manifestFile} must declare a non-empty summary`);
    }

    if (!manifest.display_name_locales || typeof manifest.display_name_locales !== "object") {
      fail(`official manifest ${manifestFile} must declare display_name_locales`);
    }

    if (!manifest.summary_locales || typeof manifest.summary_locales !== "object") {
      fail(`official manifest ${manifestFile} must declare summary_locales`);
    }

    for (const locale of supportedLocales) {
      if (locale === "en") {
        continue;
      }

      const localizedName = manifest.display_name_locales[locale];
      const localizedSummary = manifest.summary_locales[locale];

      if (typeof localizedName !== "string" || localizedName.trim().length === 0) {
        fail(`official manifest ${manifestFile} must declare display_name_locales.${locale}`);
      }

      if (typeof localizedSummary !== "string" || localizedSummary.trim().length === 0) {
        fail(`official manifest ${manifestFile} must declare summary_locales.${locale}`);
      }
    }
  }
}

function localeModuleFileName(locale) {
  return locale.replace(/-/g, "_");
}

function assertOfficialPluginLocaleModules(supportedLocales) {
  const pluginDirs = fs.readdirSync(OFFICIAL_PLUGIN_DIR, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => path.join(OFFICIAL_PLUGIN_DIR, entry.name));

  for (const pluginDir of pluginDirs) {
    const pluginName = path.basename(pluginDir);
    const sourceDir = path.join(pluginDir, "src");
    const libPath = path.join(sourceDir, "lib.rs");
    const i18nDir = path.join(sourceDir, "i18n");
    const modPath = path.join(i18nDir, "mod.rs");

    if (!fs.existsSync(libPath)) {
      fail(`official plugin ${pluginName} is missing src/lib.rs`);
    }

    if (!fs.existsSync(i18nDir)) {
      fail(`official plugin ${pluginName} is missing src/i18n/`);
    }

    if (!fs.existsSync(modPath)) {
      fail(`official plugin ${pluginName} is missing src/i18n/mod.rs`);
    }

    const libSource = readUtf8(libPath);
    const modSource = readUtf8(modPath);

    if (!libSource.includes("mod i18n;")) {
      fail(`official plugin ${pluginName} must declare mod i18n; in src/lib.rs`);
    }

    for (const locale of supportedLocales) {
      const localeFile = `${localeModuleFileName(locale)}.rs`;
      const localePath = path.join(i18nDir, localeFile);

      if (!fs.existsSync(localePath)) {
        fail(`official plugin ${pluginName} is missing src/i18n/${localeFile}`);
      }

      if (!modSource.includes(`pub mod ${localeModuleFileName(locale)};`)) {
        fail(
          `official plugin ${pluginName} must re-export locale module ${localeModuleFileName(locale)} in src/i18n/mod.rs`,
        );
      }
    }
  }
}

function assertOfficialPluginOwnedTests() {
  const pluginDirs = fs.readdirSync(OFFICIAL_PLUGIN_DIR, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => path.join(OFFICIAL_PLUGIN_DIR, entry.name));

  for (const pluginDir of pluginDirs) {
    const pluginName = path.basename(pluginDir);
    const libPath = path.join(pluginDir, "src", "lib.rs");

    if (!fs.existsSync(libPath)) {
      fail(`official plugin ${pluginName} is missing src/lib.rs`);
    }

    const libSource = readUtf8(libPath);
    const requiredTestMarkers = [
      "#[cfg(test)]",
      "mod tests {",
      "fn plugin_descriptor_is_consistent()",
      "fn plugin_manifest_matches_typed_contract_surface()",
    ];

    for (const marker of requiredTestMarkers) {
      if (!libSource.includes(marker)) {
        fail(
          `official plugin ${pluginName} must own the test marker '${marker}' in src/lib.rs`,
        );
      }
    }
  }
}

function parseDefaultKind(source) {
  const match = source.match(/const DEFAULT_KIND = "([^"]+)";/);
  if (!match) {
    fail("could not find DEFAULT_KIND in scaffolder");
  }

  return match[1];
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
const rustConfigContract = readUtf8(
  path.join(ROOT_DIR, "core", "crates", "re-config", "src", "lib.rs"),
);
const workspaceCargoToml = readUtf8(WORKSPACE_CARGO_TOML_PATH);
const scaffolderSource = readUtf8(SCAFFOLDER_PATH);
const manifestContract = require(MANIFEST_CONTRACT_PATH);
const runtimeSurfaces = require(RUNTIME_SURFACES_PATH);
const scaffolderI18n = require(SCAFFOLDER_I18N_INDEX_PATH);
const manifestSchema = manifestContract.loadManifestSchema();

const rustCapabilities = parseRustCapabilityConstants(rustPluginContract);
const rustKinds = parseRustPluginKinds(rustPluginContract);
const rustTrustLevels = parseRustPluginTrustLevels(rustPluginContract);
const rustRuntimeSurfaces = parseRustRuntimeSurfaces(rustPluginContract);
const rustRuntimeHooks = parseRustRuntimeHooks(rustPluginContract);
const rustDefaultLocale = parseRustDefaultLocale(rustConfigContract);
const rustSupportedLocales = parseRustSupportedLocales(rustConfigContract);
const workspaceVersion = parseWorkspaceVersion(workspaceCargoToml);
const supportedCapabilities = new Set(runtimeSurfaces.SUPPORTED_CAPABILITIES);
const supportedKinds = new Set(runtimeSurfaces.SUPPORTED_KINDS);
const manifestCapabilities = new Set(manifestSchema.properties.capabilities.items.enum);
const manifestKinds = new Set(manifestSchema.properties.kind.enum);
const manifestTrustLevels = new Set(manifestSchema.properties.trust_level.enum);
const defaultKind = parseDefaultKind(scaffolderSource);
const defaultCapabilitiesByKind = new Map(runtimeSurfaces.DEFAULT_CAPABILITIES_BY_KIND);
const scaffolderCapabilityImports = new Set(runtimeSurfaces.CAPABILITY_IMPORT_NAMES.keys());
const scaffolderCapabilityHooks = new Set(runtimeSurfaces.CAPABILITY_RUNTIME_HOOKS.keys());
const scaffolderKindVariants = new Set(runtimeSurfaces.KIND_VARIANTS.keys());
const scaffolderRuntimeHooks = new Set(
  [...runtimeSurfaces.CAPABILITY_RUNTIME_HOOKS.values()].map((hook) =>
    hook.replace("PluginRuntimeHook::", "").replace(/[A-Z]/g, (letter, index) =>
      index === 0 ? letter.toLowerCase() : `_${letter.toLowerCase()}`,
    ),
  ),
);
const scaffolderSupportedLocales = new Set(scaffolderI18n.SUPPORTED_LOCALES);
const scaffolderDefaultLocale = scaffolderI18n.DEFAULT_LOCALE;

assertExactSet(
  supportedCapabilities,
  rustCapabilities,
  "scaffolder supported capabilities",
);
assertExactSet(supportedKinds, rustKinds, "scaffolder supported kinds");
assertExactSet(manifestCapabilities, rustCapabilities, "manifest schema capabilities");
assertExactSet(manifestKinds, rustKinds, "manifest schema kinds");
assertSubset(manifestTrustLevels, rustTrustLevels, "manifest schema trust levels");
assertExactSet(scaffolderKindVariants, rustKinds, "scaffolder kind variant catalog");
assertExactSet(
  scaffolderCapabilityImports,
  rustCapabilities,
  "scaffolder capability import catalog",
);
assertExactSet(
  scaffolderCapabilityHooks,
  rustCapabilities,
  "scaffolder capability runtime-hook catalog",
);
assertSubset(
  scaffolderRuntimeHooks,
  rustRuntimeHooks,
  "scaffolder runtime-hook catalog",
);
assertExactSet(
  new Set(["templates", "prompts", "checks", "agents", "mcp", "providers", "policies"]),
  rustRuntimeSurfaces,
  "reviewed runtime surfaces",
);
assertExactSet(
  scaffolderSupportedLocales,
  rustSupportedLocales,
  "supported locale catalog",
);

if (!supportedKinds.has(defaultKind)) {
  fail(`DEFAULT_KIND must stay inside SUPPORTED_KINDS: ${defaultKind}`);
}

if (scaffolderDefaultLocale !== rustDefaultLocale) {
  fail(
    `default locale drift detected.\nrust: ${rustDefaultLocale}\nscaffolder: ${scaffolderDefaultLocale}`,
  );
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

assertOfficialManifestsStayLocalizedAndVersioned(
  manifestContract.parseManifestDocument,
  rustSupportedLocales,
  workspaceVersion,
);
assertOfficialPluginLocaleModules(rustSupportedLocales);
assertOfficialPluginOwnedTests();

process.stdout.write("Plugin contracts verified.\n");
