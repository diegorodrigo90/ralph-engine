const fs = require("node:fs");
const path = require("node:path");
const YAML = require("yaml");
const { resolveLocaleCatalog } = require("./i18n/index.js");

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
  const t = resolveLocaleCatalog();
  try {
    return YAML.parse(document);
  } catch (error) {
    throw new Error(t.manifestNotValidYaml(sourceLabel, error.message));
  }
}

function validatePattern(value, pattern) {
  return new RegExp(pattern).test(String(value));
}

function requireArray(value, fieldName, errors) {
  const t = resolveLocaleCatalog();
  if (!Array.isArray(value)) {
    errors.push(t.manifestArray(fieldName));
    return [];
  }

  return value;
}

function requireObject(value, fieldName, errors) {
  const t = resolveLocaleCatalog();
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    errors.push(t.manifestMappingObject(fieldName));
    return {};
  }

  return value;
}

function validateManifestObject(manifest, sourceLabel = "manifest.yaml") {
  const t = resolveLocaleCatalog();
  const schema = loadManifestSchema();
  const errors = [];

  if (!manifest || typeof manifest !== "object" || Array.isArray(manifest)) {
    throw new Error(t.manifestMustDecodeToMappingObject(sourceLabel));
  }

  const allowedKeys = new Set(Object.keys(schema.properties));
  for (const key of Object.keys(manifest)) {
    if (!allowedKeys.has(key)) {
      errors.push(t.manifestUnsupportedField(key));
    }
  }

  for (const requiredField of schema.required) {
    if (!(requiredField in manifest)) {
      errors.push(t.manifestMissingRequiredField(requiredField));
    }
  }

  if ("id" in manifest && !validatePattern(manifest.id, schema.properties.id.pattern)) {
    errors.push(t.manifestIdPattern);
  }

  if ("publisher" in manifest && !validatePattern(manifest.publisher, schema.properties.publisher.pattern)) {
    errors.push(t.manifestPublisherPattern);
  }

  if (
    typeof manifest.id === "string" &&
    typeof manifest.publisher === "string" &&
    !manifest.id.startsWith(`${manifest.publisher}.`)
  ) {
    errors.push(t.manifestIdPrefix);
  }

  if (
    "display_name" in manifest &&
    (typeof manifest.display_name !== "string" || manifest.display_name.trim().length === 0)
  ) {
    errors.push(t.manifestNonEmptyString("display_name"));
  }

  if ("summary" in manifest && (typeof manifest.summary !== "string" || manifest.summary.trim().length === 0)) {
    errors.push(t.manifestNonEmptyString("summary"));
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
        errors.push(t.manifestLocaleKeyPattern("display_name_locales", locale));
      }
      if (typeof value !== "string" || value.trim().length === 0) {
        errors.push(t.manifestLocaleValueNonEmpty("display_name_locales", locale));
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
        errors.push(t.manifestLocaleKeyPattern("summary_locales", locale));
      }
      if (typeof value !== "string" || value.trim().length === 0) {
        errors.push(t.manifestLocaleValueNonEmpty("summary_locales", locale));
      }
    }
  }

  if ("kind" in manifest && !schema.properties.kind.enum.includes(manifest.kind)) {
    errors.push(t.manifestKindEnum);
  }

  if (
    "trust_level" in manifest &&
    !schema.properties.trust_level.enum.includes(manifest.trust_level)
  ) {
    errors.push(t.manifestTrustLevelEnum);
  }

  if (
    "plugin_version" in manifest &&
    !validatePattern(manifest.plugin_version, schema.properties.plugin_version.pattern)
  ) {
    errors.push(t.manifestSemver("plugin_version"));
  }

  if (
    "plugin_api_version" in manifest &&
    !validatePattern(manifest.plugin_api_version, schema.properties.plugin_api_version.pattern)
  ) {
    errors.push(t.manifestSemver("plugin_api_version"));
  }

  if (
    "engine_version" in manifest &&
    (typeof manifest.engine_version !== "string" || manifest.engine_version.trim().length === 0)
  ) {
    errors.push(t.manifestNonEmptyString("engine_version"));
  }

  const capabilities = "capabilities" in manifest
    ? requireArray(manifest.capabilities, "capabilities", errors)
    : [];

  const supportedCapabilities = new Set(schema.properties.capabilities.items.enum);
  const seenCapabilities = new Set();
  for (const capability of capabilities) {
    if (typeof capability !== "string" || capability.trim().length === 0) {
      errors.push(t.manifestCapabilitiesNonEmpty);
      continue;
    }
    if (!supportedCapabilities.has(capability)) {
      errors.push(t.manifestUnsupportedCapabilityEntry(capability));
    }
    if (seenCapabilities.has(capability)) {
      errors.push(t.manifestRepeatedCapability(capability));
    }
    seenCapabilities.add(capability);
  }

  const requiredCapability = KIND_CAPABILITY_REQUIREMENTS.get(manifest.kind);
  if (requiredCapability && !seenCapabilities.has(requiredCapability)) {
    errors.push(t.manifestKindRequiresCapability(manifest.kind, requiredCapability));
  }

  const nonTemplateCapabilities = [...seenCapabilities].filter((capability) => capability !== "template");
  if (nonTemplateCapabilities.length > 0) {
    if (!("plugin_api_version" in manifest)) {
      errors.push(t.manifestRuntimeFacingRequiresPluginApi);
    }
    if (!("engine_version" in manifest)) {
      errors.push(t.manifestRuntimeFacingRequiresEngineVersion);
    }
  }

  if ("project" in manifest) {
    if (!seenCapabilities.has("template")) {
      errors.push(t.manifestProjectRequiresTemplate);
    }

    if (!manifest.project || typeof manifest.project !== "object" || Array.isArray(manifest.project)) {
      errors.push(t.manifestMappingObject("project"));
    } else {
      const requiredFiles = requireArray(
        manifest.project.required_files,
        "project.required_files",
        errors,
      );
      const seenRequiredFiles = new Set();
      for (const requiredFile of requiredFiles) {
        if (typeof requiredFile !== "string" || requiredFile.trim().length === 0) {
          errors.push(t.manifestRequiredFilesNonEmpty);
          continue;
        }
        if (seenRequiredFiles.has(requiredFile)) {
          errors.push(t.manifestRepeatedRequiredFile(requiredFile));
        }
        seenRequiredFiles.add(requiredFile);
      }

      for (const requiredFile of REQUIRED_TEMPLATE_FILES) {
        if (seenCapabilities.has("template") && !seenRequiredFiles.has(requiredFile)) {
          errors.push(t.manifestTemplateMustRequire(requiredFile));
        }
      }
    }
  } else if (seenCapabilities.has("template")) {
    errors.push(t.manifestTemplateMustDeclareProjectFiles);
  }

  if (errors.length > 0) {
    throw new Error(t.manifestInvalid(sourceLabel, errors));
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
