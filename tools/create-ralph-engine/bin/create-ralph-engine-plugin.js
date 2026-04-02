#!/usr/bin/env node

const fs = require("node:fs");
const path = require("node:path");
const readline = require("node:readline/promises");
const { stdin, stdout, stderr, exit } = require("node:process");
const { validateManifestDocument } = require("../lib/manifest-contract.js");
const { resolveLocaleCatalog } = require("../lib/i18n/index.js");
const {
  SUPPORTED_CAPABILITIES,
  SUPPORTED_KINDS,
  capabilityImportName,
  defaultCapabilitiesForKind,
  pluginKindVariant,
  runtimeHooksForCapabilities,
} = require("../lib/runtime-surfaces.js");

const DEFAULT_KIND = "mcp_contribution";
const DEFAULT_ENGINE_VERSION = ">=0.1.0";
const DEFAULT_PLUGIN_API_VERSION = "1.0.0";
const RESERVED_PUBLISHERS = new Set(["official"]);
const t = resolveLocaleCatalog();

async function main() {
  const args = process.argv.slice(2);
  if (args.includes("--help") || args.includes("-h")) {
    printHelp();
    return;
  }

  const parsed = parseArgs(args);
  if (parsed.error) {
    fail(parsed.error);
  }

  const interactive = stdin.isTTY && stdout.isTTY && !parsed.flags.yes;
  const scaffold = interactive
    ? await resolveInteractive(parsed)
    : resolveNonInteractive(parsed);

  validateScaffold(scaffold);
  createScaffold(scaffold);

  stdout.write(`${t.createdAt} ${scaffold.targetDir}\n`);
  stdout.write(`${t.pluginId}: ${scaffold.id}\n`);
}

function printHelp() {
  stdout.write(`${t.helpTitle}\n\n`);
  stdout.write(`${t.usageHeading}\n`);
  stdout.write(`  create-ralph-engine-plugin plugin <name> [options]\n`);
  stdout.write(`  create-ralph-engine-plugin --name <name> --publisher <publisher> [options]\n\n`);
  stdout.write(`${t.optionsHeading}\n`);
  stdout.write(`  --name <name>                 ${t.optionName}\n`);
  stdout.write(`  --publisher <publisher>       ${t.optionPublisher}\n`);
  stdout.write(`  --kind <kind>                 ${t.optionKind} (default: ${DEFAULT_KIND})\n`);
  stdout.write(`  --capability <cap>            ${t.optionCapability}\n`);
  stdout.write(`  --capabilities <a,b,c>        ${t.optionCapabilities}\n`);
  stdout.write(`  --dir <path>                  ${t.optionDir} (default: ./<name>)\n`);
  stdout.write(`  --yes                         ${t.optionYes}\n`);
  stdout.write(`  -h, --help                    ${t.optionHelp}\n`);
}

function parseArgs(argv) {
  const flags = {
    capability: [],
    yes: false,
  };
  const positionals = [];

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (!arg.startsWith("--")) {
      positionals.push(arg);
      continue;
    }

    if (arg === "--yes") {
      flags.yes = true;
      continue;
    }

    const next = argv[index + 1];
    if (next == null || next.startsWith("--")) {
      return { error: t.missingValue(arg) };
    }

    switch (arg) {
      case "--name":
        flags.name = next;
        break;
      case "--publisher":
        flags.publisher = next;
        break;
      case "--kind":
        flags.kind = next;
        break;
      case "--dir":
        flags.dir = next;
        break;
      case "--capability":
        flags.capability.push(next);
        break;
      case "--capabilities":
        flags.capability.push(...splitCSV(next));
        break;
      default:
        return { error: `Unknown option: ${arg}` };
    }
    index += 1;
  }

  if (positionals[0] === "plugin") {
    positionals.shift();
  }

  return { flags, positionals };
}

async function resolveInteractive(parsed) {
  const rl = readline.createInterface({ input: stdin, output: stdout });
  try {
    const name = normalizeSlug(parsed.flags.name || parsed.positionals[0] || await ask(rl, t.promptPluginName, ""));
    const publisher = normalizeSlug(parsed.flags.publisher || await ask(rl, t.promptPublisher, ""));
    const kind = normalizeKind(parsed.flags.kind || await ask(rl, t.promptPrimaryKind, DEFAULT_KIND) || DEFAULT_KIND);
    const capabilityInput = parsed.flags.capability.length > 0
      ? parsed.flags.capability.join(",")
      : await ask(rl, t.promptCapabilities, defaultCapabilitiesForKind(kind).join(","));

    const capabilities = normalizeCapabilities(splitCSV(capabilityInput), kind);
    const targetDir = path.resolve(parsed.flags.dir || path.join(process.cwd(), name));

    return buildScaffold({
      name,
      publisher,
      kind,
      capabilities,
      targetDir,
    });
  } finally {
    rl.close();
  }
}

function resolveNonInteractive(parsed) {
  const name = normalizeSlug(parsed.flags.name || parsed.positionals[0]);
  const publisher = normalizeSlug(parsed.flags.publisher);
  const kind = normalizeKind(parsed.flags.kind || DEFAULT_KIND);
  const capabilities = normalizeCapabilities(parsed.flags.capability, kind);
  const targetDir = path.resolve(parsed.flags.dir || path.join(process.cwd(), name || ""));

  return buildScaffold({
    name,
    publisher,
    kind,
    capabilities,
    targetDir,
  });
}

function buildScaffold(input) {
  return {
    name: input.name,
    publisher: input.publisher,
    kind: input.kind,
    capabilities: input.capabilities,
    id: `${input.publisher}.${input.name}`,
    targetDir: input.targetDir,
  };
}

function validateScaffold(scaffold) {
  if (!scaffold.name) {
    fail(t.pluginNameRequired);
  }
  if (!scaffold.publisher) {
    fail(t.publisherRequired);
  }
  if (RESERVED_PUBLISHERS.has(scaffold.publisher)) {
    fail(t.reservedPublisher(scaffold.publisher));
  }
  if (!SUPPORTED_KINDS.has(scaffold.kind)) {
    fail(t.unsupportedKind(scaffold.kind));
  }
  for (const capability of scaffold.capabilities) {
    if (!SUPPORTED_CAPABILITIES.has(capability)) {
      fail(t.unsupportedCapability(capability));
    }
  }
  if (fs.existsSync(scaffold.targetDir)) {
    fail(t.targetDirectoryExists(scaffold.targetDir));
  }
}

function createScaffold(scaffold) {
  fs.mkdirSync(scaffold.targetDir, { recursive: true });
  const manifest = renderManifest(scaffold);
  validateManifestDocument(manifest, "manifest.yaml");
  writeFile(scaffold.targetDir, "manifest.yaml", manifest);
  writeFile(scaffold.targetDir, "Cargo.toml", renderCargoToml(scaffold));
  writeFile(scaffold.targetDir, "README.md", renderREADME(scaffold));
  writeFile(scaffold.targetDir, path.join("src", "lib.rs"), renderRustPluginLib(scaffold));
  writeFile(scaffold.targetDir, path.join("src", "i18n", "mod.rs"), renderRustPluginI18nMod());
  writeFile(scaffold.targetDir, path.join("src", "i18n", "en.rs"), renderRustPluginI18nEn(scaffold));
  writeFile(
    scaffold.targetDir,
    path.join("src", "i18n", "pt_br.rs"),
    renderRustPluginI18nPtBr(scaffold),
  );

  if (scaffold.capabilities.includes("template")) {
    writeFile(scaffold.targetDir, path.join("template", "config.yaml"), renderTemplateConfig(scaffold));
    writeFile(scaffold.targetDir, path.join("template", "hooks.yaml"), renderTemplateHooks());
    writeFile(scaffold.targetDir, path.join("template", "prompt.md"), renderTemplatePrompt(scaffold));
  }
}

function writeFile(baseDir, relativePath, content) {
  const fullPath = path.join(baseDir, relativePath);
  fs.mkdirSync(path.dirname(fullPath), { recursive: true });
  fs.writeFileSync(fullPath, content, "utf8");
}

function renderManifest(scaffold) {
  const lines = [
    `id: ${scaffold.id}`,
    `kind: ${scaffold.kind}`,
    `display_name: ${humanize(scaffold.name)}`,
    "display_name_locales:",
    `  pt-br: ${humanize(scaffold.name)}`,
    `summary: ${humanize(scaffold.name)} plugin for Ralph Engine.`,
    "summary_locales:",
    `  pt-br: Plugin ${humanize(scaffold.name)} para o Ralph Engine.`,
    `publisher: ${scaffold.publisher}`,
    `trust_level: community`,
    `plugin_version: 0.1.0`,
  ];

  if (scaffold.kind !== "template") {
    lines.push(`plugin_api_version: ${DEFAULT_PLUGIN_API_VERSION}`);
    lines.push(`engine_version: "${DEFAULT_ENGINE_VERSION}"`);
  }

  if (scaffold.capabilities.length > 0) {
    lines.push("capabilities:");
    for (const capability of scaffold.capabilities) {
      lines.push(`  - ${capability}`);
    }
  }

  if (scaffold.capabilities.includes("template")) {
    lines.push("project:");
    lines.push("  required_files:");
    lines.push("    - .ralph-engine/config.yaml");
    lines.push("    - .ralph-engine/prompt.md");
  }

  for (const section of buildManifestContributions(scaffold)) {
    lines.push(...section);
  }

  return `${lines.join("\n")}\n`;
}

function buildManifestContributions(scaffold) {
  const sections = [];

  if (scaffold.capabilities.includes("template")) {
    sections.push(renderContributionSection("templates", [{
      id: `${scaffold.id}.starter`,
      displayName: `${humanize(scaffold.name)} Starter`,
      displayNamePtBr: `Starter ${humanize(scaffold.name)}`,
      summary: `Starter template for ${humanize(scaffold.name)} workflows.`,
      summaryPtBr: `Template inicial para workflows ${humanize(scaffold.name)}.`,
    }]));
  }

  if (scaffold.capabilities.includes("prompt_fragments")) {
    sections.push(renderContributionSection("prompts", [{
      id: `${scaffold.id}.workflow`,
      displayName: `${humanize(scaffold.name)} workflow prompt`,
      displayNamePtBr: `Prompt de workflow ${humanize(scaffold.name)}`,
      summary: `Prompt bundle for ${humanize(scaffold.name)} workflow assembly.`,
      summaryPtBr: `Pacote de prompts para montar workflows ${humanize(scaffold.name)}.`,
    }]));
  }

  if (scaffold.capabilities.includes("agent_runtime")) {
    sections.push(renderContributionSection("agents", [{
      id: `${scaffold.id}.session`,
      displayName: `${humanize(scaffold.name)} session`,
      displayNamePtBr: `Sessão ${humanize(scaffold.name)}`,
      summary: `${humanize(scaffold.name)} runtime session for Ralph Engine.`,
      summaryPtBr: `Sessão de runtime do ${humanize(scaffold.name)} para o Ralph Engine.`,
    }]));
  }

  const checks = [];
  if (scaffold.capabilities.includes("prepare_checks")) {
    checks.push({
      id: `${scaffold.id}.prepare`,
      kind: "prepare",
      displayName: `${humanize(scaffold.name)} prepare check`,
      displayNamePtBr: `Verificação de preparo ${humanize(scaffold.name)}`,
      summary: `Runs typed prepare-time validation for ${humanize(scaffold.name)} workflows.`,
      summaryPtBr: `Executa validação tipada de preparo para workflows ${humanize(scaffold.name)}.`,
    });
  }
  if (scaffold.capabilities.includes("doctor_checks")) {
    checks.push({
      id: `${scaffold.id}.doctor`,
      kind: "doctor",
      displayName: `${humanize(scaffold.name)} doctor check`,
      displayNamePtBr: `Verificação de diagnóstico ${humanize(scaffold.name)}`,
      summary: `Runs typed doctor-time validation for ${humanize(scaffold.name)} workflows.`,
      summaryPtBr: `Executa validação tipada de diagnóstico para workflows ${humanize(scaffold.name)}.`,
    });
  }
  if (checks.length > 0) {
    sections.push(renderContributionSection("checks", checks));
  }

  const providers = [];
  if (scaffold.capabilities.includes("data_source")) {
    providers.push({
      id: `${scaffold.id}.data`,
      kind: "data_source",
      displayName: `${humanize(scaffold.name)} data source`,
      displayNamePtBr: `Fonte de dados ${humanize(scaffold.name)}`,
      summary: `Exposes typed data-source capabilities for ${humanize(scaffold.name)} workflows.`,
      summaryPtBr: `Expõe capacidades tipadas de fonte de dados para workflows ${humanize(scaffold.name)}.`,
    });
  }
  if (scaffold.capabilities.includes("context_provider")) {
    providers.push({
      id: `${scaffold.id}.context`,
      kind: "context_provider",
      displayName: `${humanize(scaffold.name)} context provider`,
      displayNamePtBr: `Provedor de contexto ${humanize(scaffold.name)}`,
      summary: `Exposes typed context-provider capabilities for ${humanize(scaffold.name)} workflows.`,
      summaryPtBr: `Expõe capacidades tipadas de provedor de contexto para workflows ${humanize(scaffold.name)}.`,
    });
  }
  if (scaffold.capabilities.includes("forge_provider")) {
    providers.push({
      id: `${scaffold.id}.forge`,
      kind: "forge_provider",
      displayName: `${humanize(scaffold.name)} forge provider`,
      displayNamePtBr: `Provedor forge ${humanize(scaffold.name)}`,
      summary: `Exposes typed forge-provider capabilities for ${humanize(scaffold.name)} workflows.`,
      summaryPtBr: `Expõe capacidades tipadas de provedor forge para workflows ${humanize(scaffold.name)}.`,
    });
  }
  if (scaffold.capabilities.includes("remote_control")) {
    providers.push({
      id: `${scaffold.id}.remote`,
      kind: "remote_control",
      displayName: `${humanize(scaffold.name)} remote control`,
      displayNamePtBr: `Controle remoto ${humanize(scaffold.name)}`,
      summary: `Exposes typed remote-control capabilities for ${humanize(scaffold.name)} workflows.`,
      summaryPtBr: `Expõe capacidades tipadas de controle remoto para workflows ${humanize(scaffold.name)}.`,
    });
  }
  if (providers.length > 0) {
    sections.push(renderContributionSection("providers", providers));
  }

  if (scaffold.capabilities.includes("policy")) {
    sections.push(renderContributionSection("policies", [{
      id: `${scaffold.id}.guardrails`,
      displayName: `${humanize(scaffold.name)} guardrails`,
      displayNamePtBr: `Guardrails ${humanize(scaffold.name)}`,
      summary: `Policy guardrails shipped by ${humanize(scaffold.name)}.`,
      summaryPtBr: `Guardrails de política distribuídos por ${humanize(scaffold.name)}.`,
    }]));
  }

  return sections;
}

function renderContributionSection(fieldName, entries) {
  const lines = [fieldName + ":"];

  for (const entry of entries) {
    lines.push(`  - id: ${entry.id}`);
    if (entry.kind) {
      lines.push(`    kind: ${entry.kind}`);
    }
    lines.push(`    display_name: ${entry.displayName}`);
    lines.push("    display_name_locales:");
    lines.push(`      pt-br: ${entry.displayNamePtBr}`);
    lines.push(`    summary: ${entry.summary}`);
    lines.push("    summary_locales:");
    lines.push(`      pt-br: ${entry.summaryPtBr}`);
  }

  return lines;
}

function renderREADME(scaffold) {
  const lines = [
    `# ${scaffold.id}`,
    "",
    `Generated by create-ralph-engine-plugin.`,
    "",
    "## Summary",
    "",
    `- Kind: \`${scaffold.kind}\``,
    `- Capabilities: ${scaffold.capabilities.length > 0 ? scaffold.capabilities.map((value) => `\`${value}\``).join(", ") : "none"}`,
    "",
    "## Next Steps",
    "",
    "1. Edit `manifest.yaml` to match your real compatibility and capabilities.",
    "2. Refine `Cargo.toml`, `src/lib.rs`, and the files under `src/i18n/` so the crate matches your real runtime behavior and locale coverage.",
    "3. Implement the runtime, MCP bridge, or assets that your manifest declares.",
    "4. Add tests and release metadata before publishing.",
  ];

  if (scaffold.capabilities.includes("template")) {
    lines.push("5. Refine the files under `template/` so the starter experience is non-destructive and useful.");
  }

  return `${lines.join("\n")}\n`;
}

function renderCargoToml(scaffold) {
  return `[package]
name = "${cargoPackageName(scaffold)}"
version = "0.1.0"
edition = "2024"
rust-version = "1.91"
license = "MIT"
repository = "https://github.com/your-org/${scaffold.name}"
homepage = "https://ralphengine.com"
authors = ["Your Name <you@example.com>"]

[dependencies]
re-plugin = { git = "https://github.com/diegorodrigo90/ralph-engine.git", tag = "v0.2.0-alpha.1", package = "re-plugin" }

[lints.rust]
missing_docs = "deny"
unsafe_code = "forbid"

[lints.clippy]
panic = "deny"
unwrap_used = "deny"
expect_used = "deny"
todo = "deny"
unimplemented = "deny"
`;
}

function renderRustPluginLib(scaffold) {
  const capabilityImports = [...new Set(scaffold.capabilities.map(capabilityImportName))].sort();
  const runtimeHooks = runtimeHooksForCapabilities(scaffold.capabilities);
  const lifecycle = [
    "PluginLifecycleStage::Discover",
    "PluginLifecycleStage::Configure",
    "PluginLifecycleStage::Load",
  ];

  return `//! Community plugin metadata for ${scaffold.id}.

mod i18n;

use re_plugin::{
    ${[
      ...capabilityImports,
      "PluginDescriptor",
      "PluginKind",
      "PluginLifecycleStage",
      "PluginLoadBoundary",
      "PluginLocalizedText",
      "PluginRuntimeHook",
      "PluginTrustLevel",
    ].join(",\n    ")},
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "${scaffold.id}";
const PLUGIN_NAME: &str = i18n::default_name();
const LOCALIZED_NAMES: &[PluginLocalizedText] = i18n::localized_names();
const PLUGIN_SUMMARY: &str = i18n::default_summary();
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = i18n::localized_summaries();
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const CAPABILITIES: &[re_plugin::PluginCapability] = &[${scaffold.capabilities
    .map(capabilityImportName)
    .join(", ")}];
const LIFECYCLE: &[PluginLifecycleStage] = &[
    ${lifecycle.join(",\n    ")},
];
const RUNTIME_HOOKS: &[PluginRuntimeHook] = &[
    ${runtimeHooks.join(",\n    ")},
];
const DESCRIPTOR: PluginDescriptor = PluginDescriptor::new(
    PLUGIN_ID,
    ${pluginKindVariant(scaffold.kind)},
    PluginTrustLevel::Community,
    PLUGIN_NAME,
    LOCALIZED_NAMES,
    PLUGIN_SUMMARY,
    LOCALIZED_SUMMARIES,
    PLUGIN_VERSION,
    CAPABILITIES,
    LIFECYCLE,
    PluginLoadBoundary::InProcess,
    RUNTIME_HOOKS,
);

/// Declared capabilities for the plugin.
#[must_use]
pub fn capabilities() -> &'static [re_plugin::PluginCapability] {
    DESCRIPTOR.capabilities
}

/// Declared lifecycle stages for the plugin.
#[must_use]
pub fn lifecycle() -> &'static [PluginLifecycleStage] {
    DESCRIPTOR.lifecycle
}

/// Declared runtime hooks for the plugin.
#[must_use]
pub fn runtime_hooks() -> &'static [PluginRuntimeHook] {
    DESCRIPTOR.runtime_hooks
}

/// Returns the immutable plugin descriptor.
#[must_use]
pub const fn descriptor() -> PluginDescriptor {
    DESCRIPTOR
}

#[cfg(test)]
mod tests {
    use super::{PLUGIN_ID, PLUGIN_SUMMARY, capabilities, descriptor, lifecycle, runtime_hooks};

    fn manifest_document() -> &'static str {
        include_str!("../manifest.yaml")
    }

    #[test]
    fn plugin_id_is_namespaced() {
        let plugin_id = PLUGIN_ID;

        assert!(plugin_id.contains('.'));
    }

    #[test]
    fn plugin_declares_at_least_one_capability() {
        assert!(!capabilities().is_empty());
    }

    #[test]
    fn plugin_descriptor_is_consistent() {
        let plugin = descriptor();

        let descriptor_matches = plugin.id == PLUGIN_ID
            && plugin.name == i18n::default_name()
            && plugin.display_name_for_locale("pt-br") == i18n::pt_br::LOCALE.name
            && plugin.summary_for_locale("pt-br") == i18n::pt_br::LOCALE.summary
            && plugin.summary_for_locale("es") == PLUGIN_SUMMARY;

        assert!(descriptor_matches);
    }

    #[test]
    fn plugin_declares_lifecycle_stages() {
        assert!(!lifecycle().is_empty());
    }

    #[test]
    fn plugin_declares_runtime_hooks() {
        assert!(!runtime_hooks().is_empty());
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();

        assert!(manifest.contains("id: ${scaffold.id}"));
        assert!(manifest.contains("kind: ${scaffold.kind}"));
        assert!(manifest.contains("trust_level: community"));
${scaffold.capabilities.map((capability) => `        assert!(manifest.contains("- ${capability}"));`).join("\n")}
    }
}
`;
}

function renderRustPluginI18nMod() {
  return `pub mod en;
pub mod pt_br;

use re_plugin::PluginLocalizedText;

pub struct PluginLocaleCatalog {
    pub name: &'static str,
    pub summary: &'static str,
}

const LOCALIZED_NAMES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::LOCALE.name,
)];
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::LOCALE.summary,
)];

/// Returns the default English plugin name.
#[must_use]
pub const fn default_name() -> &'static str {
    en::LOCALE.name
}

/// Returns the default English plugin summary.
#[must_use]
pub const fn default_summary() -> &'static str {
    en::LOCALE.summary
}

/// Returns localized plugin names beyond the default English value.
#[must_use]
pub const fn localized_names() -> &'static [PluginLocalizedText] {
    LOCALIZED_NAMES
}

/// Returns localized plugin summaries beyond the default English value.
#[must_use]
pub const fn localized_summaries() -> &'static [PluginLocalizedText] {
    LOCALIZED_SUMMARIES
}
`;
}

function renderRustPluginI18nEn(scaffold) {
  return `use super::PluginLocaleCatalog;

pub const LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    name: "${humanize(scaffold.name)}",
    summary: "${humanize(scaffold.name)} plugin for Ralph Engine.",
};
`;
}

function renderRustPluginI18nPtBr(scaffold) {
  return `use super::PluginLocaleCatalog;

pub const LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    name: "${humanize(scaffold.name)}",
    summary: "Plugin ${humanize(scaffold.name)} para o Ralph Engine.",
};
`;
}

function renderTemplateConfig(scaffold) {
  return `agent:\n  type: claude\nworkflow:\n  instructions: |\n    Follow the ${humanize(scaffold.name)} workflow.\n`;
}

function renderTemplateHooks() {
  return `hooks: {}\n`;
}

function renderTemplatePrompt(scaffold) {
  return `# ${humanize(scaffold.name)}\n\nProject-specific starter prompt content goes here.\n`;
}

function normalizeSlug(value) {
  return String(value || "")
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9-]+/g, "-")
    .replace(/^-+|-+$/g, "");
}

function normalizeKind(value) {
  return String(value || "")
    .trim()
    .toLowerCase()
    .replace(/\s+/g, "_");
}

function normalizeCapabilities(values, kind) {
  const combined = [...defaultCapabilitiesForKind(kind), ...values.map((value) => String(value || "").trim()).filter(Boolean)];
  return [...new Set(combined.map((value) => value.toLowerCase()))];
}

function splitCSV(value) {
  return String(value || "")
    .split(",")
    .map((item) => item.trim())
    .filter(Boolean);
}

function humanize(value) {
  return String(value || "")
    .split("-")
    .filter(Boolean)
    .map((part) => part[0].toUpperCase() + part.slice(1))
    .join(" ");
}

function cargoPackageName(scaffold) {
  return `re-plugin-${scaffold.publisher}-${scaffold.name}`.replace(/[^a-z0-9-]+/g, "-");
}

async function ask(rl, label, fallback) {
  const suffix = fallback ? ` [${fallback}]` : "";
  const answer = await rl.question(`${label}${suffix}: `);
  return answer.trim() || fallback;
}

function fail(message) {
  stderr.write(`${message}\n`);
  exit(1);
}

main().catch((error) => {
  fail(error instanceof Error ? error.message : String(error));
});
