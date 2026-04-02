const fs = require("node:fs");
const path = require("node:path");
const YAML = require("yaml");

const SCHEMA_PATH = path.join(__dirname, "..", "schema", "plugin-manifest.schema.json");
const REQUIRED_TEMPLATE_FILES = [
  ".ralph-engine/config.yaml",
  ".ralph-engine/prompt.md",
];
const KIND_CAPABILITY_REQUIREMENTS = new Map([
  ["agent_runtime", "agent_runtime"],
  ["forge_provider", "forge_provider"],
  ["context_provider", "context_provider"],
  ["data_source", "data_source"],
  ["template", "template"],
  ["remote_control", "remote_control"],
  ["mcp_contribution", "mcp_contribution"],
  ["policy", "policy"],
]);

function loadManifestSchema() {
  return JSON.parse(fs.readFileSync(SCHEMA_PATH, "utf8"));
}

function readManifestDocument(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

function parseManifestDocument(document, sourceLabel = "manifest.yaml") {
  try {
    return YAML.parse(document);
  } catch (error) {
    throw new Error(`${sourceLabel} is not valid YAML: ${error.message}`);
  }
}

function validatePattern(value, pattern) {
  return new RegExp(pattern).test(String(value));
}

function requireArray(value, fieldName, errors) {
  if (!Array.isArray(value)) {
    errors.push(`${fieldName} must be an array`);
    return [];
  }

  return value;
}

function requireObject(value, fieldName, errors) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    errors.push(`${fieldName} must be a mapping object`);
    return {};
  }

  return value;
}

function validateManifestObject(manifest, sourceLabel = "manifest.yaml") {
  const schema = loadManifestSchema();
  const errors = [];

  if (!manifest || typeof manifest !== "object" || Array.isArray(manifest)) {
    throw new Error(`${sourceLabel} must decode to a mapping object`);
  }

  const allowedKeys = new Set(Object.keys(schema.properties));
  for (const key of Object.keys(manifest)) {
    if (!allowedKeys.has(key)) {
      errors.push(`unsupported field "${key}"`);
    }
  }

  for (const requiredField of schema.required) {
    if (!(requiredField in manifest)) {
      errors.push(`missing required field "${requiredField}"`);
    }
  }

  if ("id" in manifest && !validatePattern(manifest.id, schema.properties.id.pattern)) {
    errors.push(`id must follow the dotted namespace contract publisher.name`);
  }

  if ("publisher" in manifest && !validatePattern(manifest.publisher, schema.properties.publisher.pattern)) {
    errors.push(`publisher must stay a lowercase slug`);
  }

  if (
    typeof manifest.id === "string" &&
    typeof manifest.publisher === "string" &&
    !manifest.id.startsWith(`${manifest.publisher}.`)
  ) {
    errors.push(`id must start with the publisher slug followed by a dot`);
  }

  if (
    "display_name" in manifest &&
    (typeof manifest.display_name !== "string" || manifest.display_name.trim().length === 0)
  ) {
    errors.push(`display_name must be a non-empty string`);
  }

  if ("summary" in manifest && (typeof manifest.summary !== "string" || manifest.summary.trim().length === 0)) {
    errors.push(`summary must be a non-empty string`);
  }

  if ("display_name_locales" in manifest) {
    const displayNameLocales = requireObject(
      manifest.display_name_locales,
      "display_name_locales",
      errors,
    );
    const localePattern = schema.properties.display_name_locales.propertyNames.pattern;

    for (const [locale, value] of Object.entries(displayNameLocales)) {
      if (!validatePattern(locale, localePattern)) {
        errors.push(`display_name_locales key "${locale}" must be a stable locale identifier`);
      }
      if (typeof value !== "string" || value.trim().length === 0) {
        errors.push(`display_name_locales.${locale} must be a non-empty string`);
      }
    }
  }

  if ("summary_locales" in manifest) {
    const summaryLocales = requireObject(
      manifest.summary_locales,
      "summary_locales",
      errors,
    );
    const localePattern = schema.properties.summary_locales.propertyNames.pattern;

    for (const [locale, value] of Object.entries(summaryLocales)) {
      if (!validatePattern(locale, localePattern)) {
        errors.push(`summary_locales key "${locale}" must be a stable locale identifier`);
      }
      if (typeof value !== "string" || value.trim().length === 0) {
        errors.push(`summary_locales.${locale} must be a non-empty string`);
      }
    }
  }

  if ("kind" in manifest && !schema.properties.kind.enum.includes(manifest.kind)) {
    errors.push(`kind must stay inside the reviewed plugin kinds`);
  }

  if (
    "trust_level" in manifest &&
    !schema.properties.trust_level.enum.includes(manifest.trust_level)
  ) {
    errors.push(`trust_level must stay inside the reviewed trust levels`);
  }

  if (
    "plugin_version" in manifest &&
    !validatePattern(manifest.plugin_version, schema.properties.plugin_version.pattern)
  ) {
    errors.push(`plugin_version must be a semver string`);
  }

  if (
    "plugin_api_version" in manifest &&
    !validatePattern(manifest.plugin_api_version, schema.properties.plugin_api_version.pattern)
  ) {
    errors.push(`plugin_api_version must be a semver string`);
  }

  if (
    "engine_version" in manifest &&
    (typeof manifest.engine_version !== "string" || manifest.engine_version.trim().length === 0)
  ) {
    errors.push(`engine_version must be a non-empty string`);
  }

  const capabilities = "capabilities" in manifest
    ? requireArray(manifest.capabilities, "capabilities", errors)
    : [];

  const supportedCapabilities = new Set(schema.properties.capabilities.items.enum);
  const seenCapabilities = new Set();
  for (const capability of capabilities) {
    if (typeof capability !== "string" || capability.trim().length === 0) {
      errors.push(`capabilities must contain only non-empty strings`);
      continue;
    }
    if (!supportedCapabilities.has(capability)) {
      errors.push(`unsupported capability "${capability}"`);
    }
    if (seenCapabilities.has(capability)) {
      errors.push(`capabilities must not repeat "${capability}"`);
    }
    seenCapabilities.add(capability);
  }

  const requiredCapability = KIND_CAPABILITY_REQUIREMENTS.get(manifest.kind);
  if (requiredCapability && !seenCapabilities.has(requiredCapability)) {
    errors.push(`kind "${manifest.kind}" must declare capability "${requiredCapability}"`);
  }

  const nonTemplateCapabilities = [...seenCapabilities].filter((capability) => capability !== "template");
  if (nonTemplateCapabilities.length > 0) {
    if (!("plugin_api_version" in manifest)) {
      errors.push(`plugin_api_version is required for runtime-facing plugin capabilities`);
    }
    if (!("engine_version" in manifest)) {
      errors.push(`engine_version is required for runtime-facing plugin capabilities`);
    }
  }

  if ("project" in manifest) {
    if (!seenCapabilities.has("template")) {
      errors.push(`project metadata is only valid when the template capability is declared`);
    }

    if (!manifest.project || typeof manifest.project !== "object" || Array.isArray(manifest.project)) {
      errors.push(`project must be a mapping object`);
    } else {
      const requiredFiles = requireArray(
        manifest.project.required_files,
        "project.required_files",
        errors,
      );
      const seenRequiredFiles = new Set();
      for (const requiredFile of requiredFiles) {
        if (typeof requiredFile !== "string" || requiredFile.trim().length === 0) {
          errors.push(`project.required_files must contain only non-empty strings`);
          continue;
        }
        if (seenRequiredFiles.has(requiredFile)) {
          errors.push(`project.required_files must not repeat "${requiredFile}"`);
        }
        seenRequiredFiles.add(requiredFile);
      }

      for (const requiredFile of REQUIRED_TEMPLATE_FILES) {
        if (seenCapabilities.has("template") && !seenRequiredFiles.has(requiredFile)) {
          errors.push(`template manifests must require "${requiredFile}"`);
        }
      }
    }
  } else if (seenCapabilities.has("template")) {
    errors.push(`template manifests must declare project.required_files`);
  }

  if (errors.length > 0) {
    throw new Error(`${sourceLabel} is invalid:\n- ${errors.join("\n- ")}`);
  }

  return manifest;
}

function validateManifestDocument(document, sourceLabel = "manifest.yaml") {
  return validateManifestObject(parseManifestDocument(document, sourceLabel), sourceLabel);
}

module.exports = {
  KIND_CAPABILITY_REQUIREMENTS,
  REQUIRED_TEMPLATE_FILES,
  loadManifestSchema,
  parseManifestDocument,
  readManifestDocument,
  validateManifestDocument,
  validateManifestObject,
};
