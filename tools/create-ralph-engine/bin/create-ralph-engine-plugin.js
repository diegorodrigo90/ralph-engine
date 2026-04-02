#!/usr/bin/env node

const fs = require("node:fs");
const path = require("node:path");
const readline = require("node:readline/promises");
const { stdin, stdout, stderr, exit } = require("node:process");
const { validateManifestDocument } = require("../lib/manifest-contract.js");
const { resolveLocaleCatalog } = require("../lib/i18n/index.js");

const DEFAULT_KIND = "mcp_contribution";
const DEFAULT_ENGINE_VERSION = ">=0.1.0";
const DEFAULT_PLUGIN_API_VERSION = "1.0.0";
const RESERVED_PUBLISHERS = new Set(["official"]);
const SUPPORTED_KINDS = new Set([
  "agent_runtime",
  "forge_provider",
  "context_provider",
  "data_source",
  "template",
  "remote_control",
  "mcp_contribution",
  "policy",
]);
const SUPPORTED_CAPABILITIES = new Set([
  "agent_runtime",
  "data_source",
  "context_provider",
  "forge_provider",
  "doctor_checks",
  "prepare_checks",
  "prompt_fragments",
  "template",
  "remote_control",
  "mcp_contribution",
  "policy",
]);
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
  writeFile(scaffold.targetDir, "README.md", renderREADME(scaffold));

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

  return `${lines.join("\n")}\n`;
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
    "2. Implement the runtime, MCP bridge, or assets that your manifest declares.",
    "3. Add tests and release metadata before publishing.",
  ];

  if (scaffold.capabilities.includes("template")) {
    lines.push("4. Refine the files under `template/` so the starter experience is non-destructive and useful.");
  }

  return `${lines.join("\n")}\n`;
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

function defaultCapabilitiesForKind(kind) {
  switch (kind) {
    case "template":
      return ["template"];
    case "agent_runtime":
      return ["agent_runtime"];
    case "remote_control":
      return ["remote_control"];
    case "mcp_contribution":
      return ["mcp_contribution"];
    case "forge_provider":
      return ["forge_provider"];
    case "context_provider":
      return ["context_provider"];
    case "data_source":
      return ["data_source"];
    case "policy":
      return ["policy"];
    default:
      return [];
  }
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
