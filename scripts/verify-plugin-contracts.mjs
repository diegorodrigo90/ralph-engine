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

/**
 * Parse string values from a Rust enum — supports both `impl Foo { match self { Self::X => "x" } }`
 * and `define_plugin_enum! { pub enum Foo => ALL { X => "x" } }` macro formats.
 * Strips doc comments (///) before matching to avoid hitting examples in rustdoc.
 */
function parseRustEnumValues(source, enumName) {
  // Try direct impl block first
  const implRegex = new RegExp(`impl ${enumName} \\{([\\s\\S]*?)\\n\\}`);
  const implMatch = source.match(implRegex);
  if (implMatch) {
    return new Set(
      [...implMatch[1].matchAll(/Self::[A-Za-z]+ => "([^"]+)"/g)].map((m) => m[1]),
    );
  }

  // Try define_plugin_enum! macro format (strip doc comments first to avoid examples)
  const clean = source.replace(/\/\/\/[^\n]*/g, "");
  const macroRegex = new RegExp(`pub enum ${enumName} => \\w+ \\{([\\s\\S]*?)\\}`);
  const macroMatch = clean.match(macroRegex);
  if (macroMatch) {
    return new Set(
      [...macroMatch[1].matchAll(/[A-Za-z]+ => "([^"]+)"/g)].map((m) => m[1]),
    );
  }

  fail(`could not find ${enumName} in Rust plugin contract`);
}

function parseRustPluginKinds(source) {
  return parseRustEnumValues(source, "PluginKind");
}

function parseRustPluginTrustLevels(source) {
  return parseRustEnumValues(source, "PluginTrustLevel");
}

function parseRustRuntimeSurfaces(source) {
  return parseRustEnumValues(source, "PluginRuntimeSurface");
}

function parseRustRuntimeHooks(source) {
  return parseRustEnumValues(source, "PluginRuntimeHook");
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

function assertOfficialManifestContributionLocalization(
  parseManifestDocument,
  supportedLocales,
) {
  const contributionFields = ["templates", "prompts", "agents", "checks", "providers", "policies"];
  const manifestFiles = fs.readdirSync(OFFICIAL_PLUGIN_DIR)
    .map((pluginDir) => path.join(OFFICIAL_PLUGIN_DIR, pluginDir, "manifest.yaml"));

  for (const manifestFile of manifestFiles) {
    const manifest = parseManifestDocument(readUtf8(manifestFile), manifestFile);
    const pluginIdPrefix = `${manifest.id}.`;

    for (const fieldName of contributionFields) {
      const entries = manifest[fieldName];

      if (entries === undefined) {
        continue;
      }

      for (const entry of entries) {
        if (!entry.id.startsWith(pluginIdPrefix)) {
          fail(
            `official manifest contribution ${manifestFile} must keep ${fieldName} ids namespaced by ${manifest.id}`,
          );
        }

        if (!entry.display_name_locales || typeof entry.display_name_locales !== "object") {
          fail(
            `official manifest contribution ${manifestFile} must declare ${fieldName}.${entry.id}.display_name_locales`,
          );
        }

        if (!entry.summary_locales || typeof entry.summary_locales !== "object") {
          fail(
            `official manifest contribution ${manifestFile} must declare ${fieldName}.${entry.id}.summary_locales`,
          );
        }

        for (const locale of supportedLocales) {
          if (locale === "en") {
            continue;
          }

          const localizedName = entry.display_name_locales[locale];
          const localizedSummary = entry.summary_locales[locale];

          if (typeof localizedName !== "string" || localizedName.trim().length === 0) {
            fail(
              `official manifest contribution ${manifestFile} must declare ${fieldName}.${entry.id}.display_name_locales.${locale}`,
            );
          }

          if (typeof localizedSummary !== "string" || localizedSummary.trim().length === 0) {
            fail(
              `official manifest contribution ${manifestFile} must declare ${fieldName}.${entry.id}.summary_locales.${locale}`,
            );
          }
        }
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
    const libPath = path.join(pluginDir, "src", "lib.rs");
    const i18nModPath = path.join(pluginDir, "src", "i18n", "mod.rs");
    const localesDir = path.join(pluginDir, "locales");

    if (!fs.existsSync(libPath)) {
      fail(`official plugin ${pluginName} is missing src/lib.rs`);
    }

    const libSource = readUtf8(libPath);

    if (!libSource.includes("mod i18n;")) {
      fail(`official plugin ${pluginName} must declare mod i18n; in src/lib.rs`);
    }

    if (!fs.existsSync(i18nModPath)) {
      fail(`official plugin ${pluginName} is missing src/i18n/mod.rs`);
    }

    // Plugins use build.rs + include!() — locale TOML files must exist
    // TOML files use the locale ID directly (pt-br.toml), not the Rust module name (pt_br)
    for (const locale of supportedLocales) {
      const tomlFile = `${locale}.toml`;
      const tomlPath = path.join(localesDir, tomlFile);

      if (!fs.existsSync(tomlPath)) {
        fail(`official plugin ${pluginName} is missing locales/${tomlFile}`);
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

function assertNoRawRatatuiStyles() {
  const FORBIDDEN_PATTERNS = [
    { pattern: /use ratatui::style::Style;/, label: "use ratatui::style::Style" },
    { pattern: /use ratatui::style::Modifier;/, label: "use ratatui::style::Modifier" },
    { pattern: /use ratatui::style::Stylize;/, label: "use ratatui::style::Stylize" },
    { pattern: /Style::default\(\)/, label: "Style::default()" },
    { pattern: /Style::new\(\)/, label: "Style::new()" },
    { pattern: /Color::Rgb\(/, label: "Color::Rgb(...)" },
  ];

  const pluginDirs = fs.readdirSync(OFFICIAL_PLUGIN_DIR, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => path.join(OFFICIAL_PLUGIN_DIR, entry.name));

  for (const pluginDir of pluginDirs) {
    const pluginName = path.basename(pluginDir);
    const srcDir = path.join(pluginDir, "src");

    if (!fs.existsSync(srcDir)) {
      continue;
    }

    const rsFiles = fs.readdirSync(srcDir, { recursive: true })
      .filter((f) => f.endsWith(".rs"))
      .map((f) => path.join(srcDir, f));

    for (const rsFile of rsFiles) {
      const source = readUtf8(rsFile);

      // Skip test blocks
      const prodSource = source.replace(/#\[cfg\(test\)\][\s\S]*?^}/gm, "");

      for (const { pattern, label } of FORBIDDEN_PATTERNS) {
        if (pattern.test(prodSource)) {
          fail(
            `official plugin ${pluginName} uses raw ratatui style (${label}) in ${path.basename(rsFile)}. Use ratatui-themekit instead. See AGENTS.md Rule 48-50.`,
          );
        }
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
assertOfficialManifestContributionLocalization(
  manifestContract.parseManifestDocument,
  rustSupportedLocales,
);
assertOfficialPluginLocaleModules(rustSupportedLocales);
assertOfficialPluginOwnedTests();
assertNoRawRatatuiStyles();

process.stdout.write("Plugin contracts verified.\n");
