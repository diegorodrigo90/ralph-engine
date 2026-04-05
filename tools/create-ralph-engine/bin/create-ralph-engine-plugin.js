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
  stdout.write(`GitHub repo name: ralph-engine-plugin-${scaffold.name}\n`);
  stdout.write(`Install command:  ralph-engine install ${scaffold.publisher}/${scaffold.name}\n`);
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
  writeFile(scaffold.targetDir, path.join("src", "i18n", "mod.rs"), renderRustPluginI18nMod(scaffold));
  writeFile(scaffold.targetDir, path.join("locales", "en.toml"), renderLocalesToml(scaffold, "en"));
  writeFile(scaffold.targetDir, path.join("locales", "pt-br.toml"), renderLocalesToml(scaffold, "pt-br"));
  writeFile(scaffold.targetDir, "build.rs", renderBuildRs(scaffold));

  if (scaffold.capabilities.includes("template")) {
    writeFile(scaffold.targetDir, path.join("template", "config.yaml"), renderTemplateConfig(scaffold));
    writeFile(scaffold.targetDir, path.join("template", "hooks.yaml"), renderTemplateHooks());
    writeFile(scaffold.targetDir, path.join("template", "prompt.md"), renderTemplatePrompt(scaffold));
  }

  writeFile(scaffold.targetDir, ".gitignore", renderGitignore());
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

  lines.push(`plugin_api_version: ${DEFAULT_PLUGIN_API_VERSION}`);
  lines.push(`engine_version: "${DEFAULT_ENGINE_VERSION}"`);
  lines.push("runtime: true");

  if (scaffold.kind !== "template") {
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

function buildRuntimeContributionDefinitions(scaffold) {
  const definitions = {
    templates: [],
    prompts: [],
    agents: [],
    checks: [],
    providers: [],
    policies: [],
  };

  if (scaffold.capabilities.includes("template")) {
    definitions.templates.push({
      id: `${scaffold.id}.starter`,
      displayName: `${humanize(scaffold.name)} Starter`,
      displayNamePtBr: `Starter ${humanize(scaffold.name)}`,
      summary: `Starter template for ${humanize(scaffold.name)} workflows.`,
      summaryPtBr: `Template inicial para workflows ${humanize(scaffold.name)}.`,
    });
  }

  if (scaffold.capabilities.includes("prompt_fragments")) {
    definitions.prompts.push({
      id: `${scaffold.id}.workflow`,
      displayName: `${humanize(scaffold.name)} workflow prompt`,
      displayNamePtBr: `Prompt de workflow ${humanize(scaffold.name)}`,
      summary: `Prompt bundle for ${humanize(scaffold.name)} workflow assembly.`,
      summaryPtBr: `Pacote de prompts para montar workflows ${humanize(scaffold.name)}.`,
    });
  }

  if (scaffold.capabilities.includes("agent_runtime")) {
    definitions.agents.push({
      id: `${scaffold.id}.session`,
      displayName: `${humanize(scaffold.name)} session`,
      displayNamePtBr: `Sessão ${humanize(scaffold.name)}`,
      summary: `${humanize(scaffold.name)} runtime session for Ralph Engine.`,
      summaryPtBr: `Sessão de runtime do ${humanize(scaffold.name)} para o Ralph Engine.`,
    });
  }

  if (scaffold.capabilities.includes("prepare_checks")) {
    definitions.checks.push({
      id: `${scaffold.id}.prepare`,
      kind: "prepare",
      checkKindVariant: "PluginCheckKind::Prepare",
      displayName: `${humanize(scaffold.name)} prepare check`,
      displayNamePtBr: `Verificação de preparo ${humanize(scaffold.name)}`,
      summary: `Runs typed prepare-time validation for ${humanize(scaffold.name)} workflows.`,
      summaryPtBr: `Executa validação tipada de preparo para workflows ${humanize(scaffold.name)}.`,
    });
  }

  if (scaffold.capabilities.includes("doctor_checks")) {
    definitions.checks.push({
      id: `${scaffold.id}.doctor`,
      kind: "doctor",
      checkKindVariant: "PluginCheckKind::Doctor",
      displayName: `${humanize(scaffold.name)} doctor check`,
      displayNamePtBr: `Verificação de diagnóstico ${humanize(scaffold.name)}`,
      summary: `Runs typed doctor-time validation for ${humanize(scaffold.name)} workflows.`,
      summaryPtBr: `Executa validação tipada de diagnóstico para workflows ${humanize(scaffold.name)}.`,
    });
  }

  if (scaffold.capabilities.includes("data_source")) {
    definitions.providers.push({
      id: `${scaffold.id}.data`,
      kind: "data_source",
      providerKindVariant: "PluginProviderKind::DataSource",
      displayName: `${humanize(scaffold.name)} data source`,
      displayNamePtBr: `Fonte de dados ${humanize(scaffold.name)}`,
      summary: `Exposes typed data-source capabilities for ${humanize(scaffold.name)} workflows.`,
      summaryPtBr: `Expõe capacidades tipadas de fonte de dados para workflows ${humanize(scaffold.name)}.`,
    });
  }

  if (scaffold.capabilities.includes("context_provider")) {
    definitions.providers.push({
      id: `${scaffold.id}.context`,
      kind: "context_provider",
      providerKindVariant: "PluginProviderKind::ContextProvider",
      displayName: `${humanize(scaffold.name)} context provider`,
      displayNamePtBr: `Provedor de contexto ${humanize(scaffold.name)}`,
      summary: `Exposes typed context-provider capabilities for ${humanize(scaffold.name)} workflows.`,
      summaryPtBr: `Expõe capacidades tipadas de provedor de contexto para workflows ${humanize(scaffold.name)}.`,
    });
  }

  if (scaffold.capabilities.includes("forge_provider")) {
    definitions.providers.push({
      id: `${scaffold.id}.forge`,
      kind: "forge_provider",
      providerKindVariant: "PluginProviderKind::ForgeProvider",
      displayName: `${humanize(scaffold.name)} forge provider`,
      displayNamePtBr: `Provedor forge ${humanize(scaffold.name)}`,
      summary: `Exposes typed forge-provider capabilities for ${humanize(scaffold.name)} workflows.`,
      summaryPtBr: `Expõe capacidades tipadas de provedor forge para workflows ${humanize(scaffold.name)}.`,
    });
  }

  if (scaffold.capabilities.includes("remote_control")) {
    definitions.providers.push({
      id: `${scaffold.id}.remote`,
      kind: "remote_control",
      providerKindVariant: "PluginProviderKind::RemoteControl",
      displayName: `${humanize(scaffold.name)} remote control`,
      displayNamePtBr: `Controle remoto ${humanize(scaffold.name)}`,
      summary: `Exposes typed remote-control capabilities for ${humanize(scaffold.name)} workflows.`,
      summaryPtBr: `Expõe capacidades tipadas de controle remoto para workflows ${humanize(scaffold.name)}.`,
    });
  }

  if (scaffold.capabilities.includes("policy")) {
    definitions.policies.push({
      id: `${scaffold.id}.guardrails`,
      displayName: `${humanize(scaffold.name)} guardrails`,
      displayNamePtBr: `Guardrails ${humanize(scaffold.name)}`,
      summary: `Policy guardrails shipped by ${humanize(scaffold.name)}.`,
      summaryPtBr: `Guardrails de política distribuídos por ${humanize(scaffold.name)}.`,
    });
  }

  return definitions;
}

function rustContributionIdent(sectionName, entryId) {
  return `${sectionName}_${entryId}`
    .replace(/[^a-zA-Z0-9]+/g, "_")
    .replace(/^_+|_+$/g, "")
    .replace(/_+/g, "_")
    .toUpperCase();
}

function rustContributionHelperBase(sectionName, entryId) {
  return rustContributionIdent(sectionName, entryId).toLowerCase();
}

function renderREADME(scaffold) {
  const repoName = `ralph-engine-plugin-${scaffold.name}`;
  const lines = [
    `# ${scaffold.id}`,
    "",
    `A [Ralph Engine](https://ralphengine.com) community plugin.`,
    "",
    "## Summary",
    "",
    `- **Plugin ID:** \`${scaffold.id}\``,
    `- **Kind:** \`${scaffold.kind}\``,
    `- **Capabilities:** ${scaffold.capabilities.length > 0 ? scaffold.capabilities.map((value) => `\`${value}\``).join(", ") : "none"}`,
    `- **Publisher:** \`${scaffold.publisher}\``,
    "",
    "## Installation",
    "",
    "```bash",
    `ralph-engine install ${scaffold.publisher}/${scaffold.name}`,
    "```",
    "",
    "## Publishing",
    "",
    `Your GitHub repo **must** be named \`${repoName}\` for auto-discovery:`,
    "",
    "```bash",
    `gh repo create ${scaffold.publisher}/${repoName} --public --source . --push`,
    "```",
    "",
    "| Convention | Value |",
    "| --- | --- |",
    `| GitHub repo | \`${scaffold.publisher}/${repoName}\` |`,
    `| Install command | \`ralph-engine install ${scaffold.publisher}/${scaffold.name}\` |`,
    `| Plugin ID | \`${scaffold.id}\` |`,
    `| Local path | \`.ralph-engine/plugins/${scaffold.id}/\` |`,
    "",
    "## Development",
    "",
    "1. Edit `manifest.yaml` — capabilities, compatibility, metadata.",
    "2. Implement runtime in `src/lib.rs` — fill in trait methods.",
    "3. Add locales in `locales/` — EN + PT-BR minimum.",
    "4. Add tests and release metadata before publishing.",
  ];

  if (scaffold.capabilities.includes("template")) {
    lines.push("5. Refine the files under `template/` for the starter experience.");
  }

  lines.push("");
  lines.push("---");
  lines.push("");
  lines.push("Generated by `create-ralph-engine-plugin`.");

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
re-mcp = { git = "https://github.com/diegorodrigo90/ralph-engine.git", tag = "v0.2.0-alpha.1", package = "re-mcp" }

[build-dependencies]
re-build-utils = { git = "https://github.com/diegorodrigo90/ralph-engine.git", tag = "v0.2.0-alpha.1", package = "re-build-utils" }

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

function toPascalCase(slug) {
  return slug.split("-").map((s) => s.charAt(0).toUpperCase() + s.slice(1)).join("");
}

function renderRustPluginLib(scaffold) {
  const structName = toPascalCase(scaffold.name);
  const contributions = buildRuntimeContributionDefinitions(scaffold);
  const capabilityImports = [...new Set(scaffold.capabilities.map(capabilityImportName))].sort();
  const descriptorImports = [];
  if (contributions.templates.length > 0) descriptorImports.push("PluginTemplateAsset", "PluginTemplateDescriptor");
  if (contributions.prompts.length > 0) descriptorImports.push("PluginPromptAsset", "PluginPromptDescriptor");
  if (contributions.agents.length > 0) descriptorImports.push("PluginAgentDescriptor");
  if (contributions.checks.length > 0) descriptorImports.push("PluginCheckDescriptor", "PluginCheckKind");
  if (contributions.providers.length > 0) descriptorImports.push("PluginProviderDescriptor", "PluginProviderKind");
  if (contributions.policies.length > 0) descriptorImports.push("PluginPolicyDescriptor");
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
      ...descriptorImports,
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
const PLUGIN_NAME: &str = i18n::plugin_name();
const LOCALIZED_NAMES: &[PluginLocalizedText] = i18n::localized_plugin_names();
const PLUGIN_SUMMARY: &str = i18n::plugin_summary();
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = i18n::localized_plugin_summaries();
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
    re_plugin::CURRENT_PLUGIN_API_VERSION,
    CAPABILITIES,
    LIFECYCLE,
    PluginLoadBoundary::InProcess,
    RUNTIME_HOOKS,
);
${contributions.templates.length > 0 ? `const TEMPLATE_ASSETS: &[PluginTemplateAsset] = &[
    PluginTemplateAsset::new(".ralph-engine/config.yaml", include_str!("../template/config.yaml")),
    PluginTemplateAsset::new(".ralph-engine/hooks.yaml", include_str!("../template/hooks.yaml")),
    PluginTemplateAsset::new(".ralph-engine/prompt.md", include_str!("../template/prompt.md")),
];
const TEMPLATES: &[PluginTemplateDescriptor] = &[
${contributions.templates.map((entry) => {
  return `    PluginTemplateDescriptor::new(
        "${entry.id}",
        PLUGIN_ID,
        i18n::template_name(),
        i18n::localized_template_names(),
        i18n::template_summary(),
        i18n::localized_template_summaries(),
        TEMPLATE_ASSETS,
    ),`;
}).join("\n")}
];` : ""}
${contributions.prompts.length > 0 ? `const PROMPT_ASSETS: &[PluginPromptAsset] = &[];
const PROMPTS: &[PluginPromptDescriptor] = &[
${contributions.prompts.map((entry) => {
  return `    PluginPromptDescriptor::new(
        "${entry.id}",
        PLUGIN_ID,
        i18n::prompt_name(),
        i18n::localized_prompt_names(),
        i18n::prompt_summary(),
        i18n::localized_prompt_summaries(),
        PROMPT_ASSETS,
    ),`;
}).join("\n")}
];` : ""}
${contributions.agents.length > 0 ? `const AGENTS: &[PluginAgentDescriptor] = &[
${contributions.agents.map((entry) => {
  return `    PluginAgentDescriptor::new(
        "${entry.id}",
        PLUGIN_ID,
        i18n::agent_name(),
        i18n::localized_agent_names(),
        i18n::agent_summary(),
        i18n::localized_agent_summaries(),
    ),`;
}).join("\n")}
];` : ""}
${contributions.checks.length > 0 ? `const CHECKS: &[PluginCheckDescriptor] = &[
${contributions.checks.map((entry) => {
  return `    PluginCheckDescriptor::new(
        "${entry.id}",
        PLUGIN_ID,
        ${entry.checkKindVariant},
        i18n::check_name(),
        i18n::localized_check_names(),
        i18n::check_summary(),
        i18n::localized_check_summaries(),
    ),`;
}).join("\n")}
];` : ""}
${contributions.providers.length > 0 ? `const PROVIDERS: &[PluginProviderDescriptor] = &[
${contributions.providers.map((entry) => {
  return `    PluginProviderDescriptor::new(
        "${entry.id}",
        PLUGIN_ID,
        ${entry.providerKindVariant},
        i18n::provider_name(),
        i18n::localized_provider_names(),
        i18n::provider_summary(),
        i18n::localized_provider_summaries(),
    ),`;
}).join("\n")}
];` : ""}
${contributions.policies.length > 0 ? `const POLICIES: &[PluginPolicyDescriptor] = &[
${contributions.policies.map((entry) => {
  return `    PluginPolicyDescriptor::new(
        "${entry.id}",
        PLUGIN_ID,
        i18n::policy_name(),
        i18n::localized_policy_names(),
        i18n::policy_summary(),
        i18n::localized_policy_summaries(),
    ),`;
}).join("\n")}
];` : ""}

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

/// Returns a new instance of the plugin runtime.
#[must_use]
pub fn runtime() -> ${structName}Runtime {
    ${structName}Runtime
}

/// Plugin runtime — probes for the plugin binary on the system PATH.
pub struct ${structName}Runtime;

impl re_plugin::PluginRuntime for ${structName}Runtime {
    fn plugin_id(&self) -> &str { PLUGIN_ID }

    fn run_check(&self, check_id: &str, kind: re_plugin::PluginCheckKind, project_root: &std::path::Path) -> Result<re_plugin::CheckExecutionResult, re_plugin::PluginRuntimeError> {
        let mut findings = Vec::new();
        if !project_root.join(".ralph-engine/config.yaml").exists() {
            findings.push("missing: .ralph-engine/config.yaml".to_owned());
        }
        Ok(re_plugin::CheckExecutionResult { check_id: check_id.to_owned(), passed: findings.is_empty(), findings })
    }

    fn bootstrap_agent(&self, agent_id: &str) -> Result<re_plugin::AgentBootstrapResult, re_plugin::PluginRuntimeError> {
        Err(re_plugin::PluginRuntimeError::new("not_an_agent_plugin", format!("Plugin does not provide agent '{agent_id}'")))
    }

    fn register_mcp_server(&self, server_id: &str) -> Result<re_plugin::McpRegistrationResult, re_plugin::PluginRuntimeError> {
        Err(re_plugin::PluginRuntimeError::new("not_an_mcp_plugin", format!("Plugin does not provide MCP server '{server_id}'")))
    }
}
${contributions.templates.length > 0 ? `\n/// Returns the immutable template contributions declared by the plugin.\n#[must_use]\npub const fn templates() -> &'static [PluginTemplateDescriptor] {\n    TEMPLATES\n}\n` : ""}
${contributions.prompts.length > 0 ? `\n/// Returns the immutable prompt contributions declared by the plugin.\n#[must_use]\npub const fn prompts() -> &'static [PluginPromptDescriptor] {\n    PROMPTS\n}\n` : ""}
${contributions.agents.length > 0 ? `\n/// Returns the immutable agent contributions declared by the plugin.\n#[must_use]\npub const fn agents() -> &'static [PluginAgentDescriptor] {\n    AGENTS\n}\n` : ""}
${contributions.checks.length > 0 ? `\n/// Returns the immutable check contributions declared by the plugin.\n#[must_use]\npub const fn checks() -> &'static [PluginCheckDescriptor] {\n    CHECKS\n}\n` : ""}
${contributions.providers.length > 0 ? `\n/// Returns the immutable provider contributions declared by the plugin.\n#[must_use]\npub const fn providers() -> &'static [PluginProviderDescriptor] {\n    PROVIDERS\n}\n` : ""}
${contributions.policies.length > 0 ? `\n/// Returns the immutable policy contributions declared by the plugin.\n#[must_use]\npub const fn policies() -> &'static [PluginPolicyDescriptor] {\n    POLICIES\n}\n` : ""}

#[cfg(test)]
mod tests {
    use super::{PLUGIN_ID, PLUGIN_SUMMARY, capabilities, descriptor, lifecycle, runtime_hooks${contributions.templates.length > 0 ? ", templates" : ""}${contributions.prompts.length > 0 ? ", prompts" : ""}${contributions.agents.length > 0 ? ", agents" : ""}${contributions.checks.length > 0 ? ", checks" : ""}${contributions.providers.length > 0 ? ", providers" : ""}${contributions.policies.length > 0 ? ", policies" : ""}};

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
            && plugin.name == i18n::plugin_name()
            && plugin.display_name_for_locale("es") == PLUGIN_SUMMARY;

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
${contributions.templates.length > 0 ? `\n    #[test]\n    fn plugin_declares_template_contributions() {\n        assert_eq!(templates()[0].id, "${contributions.templates[0].id}");\n    }\n` : ""}
${contributions.prompts.length > 0 ? `\n    #[test]\n    fn plugin_declares_prompt_contributions() {\n        assert_eq!(prompts()[0].id, "${contributions.prompts[0].id}");\n    }\n` : ""}
${contributions.agents.length > 0 ? `\n    #[test]\n    fn plugin_declares_agent_contributions() {\n        assert_eq!(agents()[0].id, "${contributions.agents[0].id}");\n    }\n` : ""}
${contributions.checks.length > 0 ? `\n    #[test]\n    fn plugin_declares_check_contributions() {\n        assert_eq!(checks()[0].id, "${contributions.checks[0].id}");\n    }\n` : ""}
${contributions.providers.length > 0 ? `\n    #[test]\n    fn plugin_declares_provider_contributions() {\n        assert_eq!(providers()[0].id, "${contributions.providers[0].id}");\n    }\n` : ""}
${contributions.policies.length > 0 ? `\n    #[test]\n    fn plugin_declares_policy_contributions() {\n        assert_eq!(policies()[0].id, "${contributions.policies[0].id}");\n    }\n` : ""}

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();

        assert!(manifest.contains("id: ${scaffold.id}"));
        assert!(manifest.contains("kind: ${scaffold.kind}"));
        assert!(manifest.contains("trust_level: community"));
${scaffold.capabilities.map((capability) => `        assert!(manifest.contains("- ${capability}"));`).join("\n")}
${[
  ...contributions.templates,
  ...contributions.prompts,
  ...contributions.agents,
  ...contributions.checks,
  ...contributions.providers,
  ...contributions.policies,
].map((entry) => `        assert!(manifest.contains("id: ${entry.id}"));`).join("\n")}
    }
}
`;
}

function renderRustPluginI18nMod() {
  return `include!(concat!(env!("OUT_DIR"), "/i18n_generated.rs"));\n`;
}

function renderLocalesToml(scaffold, locale) {
  const contributions = buildRuntimeContributionDefinitions(scaffold);
  const isEn = locale === "en";
  const lines = [];

  lines.push("[plugin]");
  lines.push(`name = "${humanize(scaffold.name)}"`);
  lines.push(`summary = "${isEn
    ? `${humanize(scaffold.name)} plugin for Ralph Engine.`
    : `Plugin ${humanize(scaffold.name)} para o Ralph Engine.`}"`);

  const sectionMap = [
    ["template", contributions.templates],
    ["prompt", contributions.prompts],
    ["agent", contributions.agents],
    ["check", contributions.checks],
    ["provider", contributions.providers],
    ["policy", contributions.policies],
  ];

  for (const [sectionName, entries] of sectionMap) {
    for (const entry of entries) {
      lines.push("");
      lines.push(`[${sectionName}]`);
      lines.push(`name = "${isEn ? entry.displayName : entry.displayNamePtBr}"`);
      lines.push(`summary = "${isEn ? entry.summary : entry.summaryPtBr}"`);
    }
  }

  return `${lines.join("\n")}\n`;
}

function renderBuildRs(scaffold) {
  const contributions = buildRuntimeContributionDefinitions(scaffold);
  const sections = [];

  sections.push(`            PluginLocaleSection { toml_section: "plugin", const_prefix: "PLUGIN", fn_prefix: "plugin", fields: &["name", "summary"], localized_text_type: "re_plugin::PluginLocalizedText" }`);

  const sectionMap = [
    ["template", "TEMPLATE", "template", contributions.templates, "re_plugin::PluginLocalizedText"],
    ["prompt", "PROMPT", "prompt", contributions.prompts, "re_plugin::PluginLocalizedText"],
    ["agent", "AGENT", "agent", contributions.agents, "re_plugin::PluginLocalizedText"],
    ["check", "CHECK", "check", contributions.checks, "re_plugin::PluginLocalizedText"],
    ["provider", "PROVIDER", "provider", contributions.providers, "re_plugin::PluginLocalizedText"],
    ["policy", "POLICY", "policy", contributions.policies, "re_plugin::PluginLocalizedText"],
    ["mcp_server", "MCP_SERVER", "mcp_server", [], "re_mcp::McpLocalizedText"],
  ];

  for (const [toml, prefix, fnPrefix, entries, textType] of sectionMap) {
    if (entries.length > 0 || (toml === "mcp_server" && scaffold.capabilities.includes("mcp_contribution"))) {
      const fields = toml === "mcp_server" ? `&["name"]` : `&["name", "summary"]`;
      sections.push(`            PluginLocaleSection { toml_section: "${toml}", const_prefix: "${prefix}", fn_prefix: "${fnPrefix}", fields: ${fields}, localized_text_type: "${textType}" }`);
    }
  }

  return `//! Build script for plugin locale generation.
#![allow(missing_docs, clippy::panic)]

use std::fs;
use std::path::Path;

use re_build_utils::PluginLocaleSection;

fn main() {
    let locales_dir = Path::new("locales");
    re_build_utils::rerun_if_locales_changed(locales_dir);
    let locales = re_build_utils::read_locale_dir(locales_dir);

    let code = re_build_utils::generate_plugin_locale_module(
        &locales,
        &[
${sections.join(",\n")},
        ],
    );

    let out = re_build_utils::out_dir().join("i18n_generated.rs");
    fs::write(&out, code).unwrap_or_else(|e| panic!("build.rs: {e}"));
}
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

function renderGitignore() {
  return `/target/
Cargo.lock
*.swp
*.swo
.DS_Store
`;
}

function fail(message) {
  stderr.write(`${message}\n`);
  exit(1);
}

main().catch((error) => {
  fail(error instanceof Error ? error.message : String(error));
});
