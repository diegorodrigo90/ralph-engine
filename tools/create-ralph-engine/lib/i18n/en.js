const LOCALE = {
  helpTitle: "create-ralph-engine-plugin",
  usageHeading: "Usage:",
  optionsHeading: "Options:",
  optionName: "Plugin name slug",
  optionPublisher: "Publisher slug (GitHub owner style)",
  optionKind: "Primary kind",
  optionCapability: "Repeatable capability flag",
  optionCapabilities: "Comma-separated capabilities",
  optionDir: "Target directory",
  optionYes: "Accept defaults in non-interactive mode",
  optionHelp: "Show this help",
  promptPluginName: "Plugin name",
  promptPublisher: "Publisher slug",
  promptPrimaryKind: "Primary kind",
  promptCapabilities: "Capabilities (comma separated)",
  createdAt: "Created Ralph Engine plugin scaffold at",
  pluginId: "Plugin ID",
  missingValue: (arg) => `Missing value for ${arg}`,
  pluginNameRequired:
    "Plugin name is required. Use --name or pass it as the first positional argument.",
  publisherRequired: "Publisher is required. Use --publisher.",
  reservedPublisher: (publisher) => `Publisher \"${publisher}\" is reserved.`,
  unsupportedKind: (kind) => `Unsupported kind \"${kind}\".`,
  unsupportedCapability: (capability) => `Unsupported capability \"${capability}\".`,
  targetDirectoryExists: (targetDir) => `Target directory already exists: ${targetDir}`,
  manifestNotValidYaml: (sourceLabel, error) => `${sourceLabel} is not valid YAML: ${error}`,
  manifestMustDecodeToMappingObject: (sourceLabel) => `${sourceLabel} must decode to a mapping object`,
  manifestUnsupportedField: (key) => `unsupported field "${key}"`,
  manifestMissingRequiredField: (field) => `missing required field "${field}"`,
  manifestIdPattern: "id must follow the dotted namespace contract publisher.name",
  manifestPublisherPattern: "publisher must stay a lowercase slug",
  manifestIdPrefix: "id must start with the publisher slug followed by a dot",
  manifestNonEmptyString: (field) => `${field} must be a non-empty string`,
  manifestMappingObject: (field) => `${field} must be a mapping object`,
  manifestArray: (field) => `${field} must be an array`,
  manifestLocaleKeyPattern: (field, locale) =>
    `${field} key "${locale}" must be a stable locale identifier`,
  manifestLocaleValueNonEmpty: (field, locale) =>
    `${field}.${locale} must be a non-empty string`,
  manifestKindEnum: "kind must stay inside the reviewed plugin kinds",
  manifestTrustLevelEnum: "trust_level must stay inside the reviewed trust levels",
  manifestSemver: (field) => `${field} must be a semver string`,
  manifestCapabilitiesNonEmpty: "capabilities must contain only non-empty strings",
  manifestUnsupportedCapabilityEntry: (capability) => `unsupported capability "${capability}"`,
  manifestRepeatedCapability: (capability) => `capabilities must not repeat "${capability}"`,
  manifestKindRequiresCapability: (kind, capability) =>
    `kind "${kind}" must declare capability "${capability}"`,
  manifestRuntimeFacingRequiresPluginApi:
    "plugin_api_version is required for runtime-facing plugin capabilities",
  manifestRuntimeFacingRequiresEngineVersion:
    "engine_version is required for runtime-facing plugin capabilities",
  manifestProjectRequiresTemplate:
    "project metadata is only valid when the template capability is declared",
  manifestRequiredFilesNonEmpty:
    "project.required_files must contain only non-empty strings",
  manifestRepeatedRequiredFile: (requiredFile) =>
    `project.required_files must not repeat "${requiredFile}"`,
  manifestTemplateMustRequire: (requiredFile) =>
    `template manifests must require "${requiredFile}"`,
  manifestTemplateMustDeclareProjectFiles:
    "template manifests must declare project.required_files",
  manifestInvalid: (sourceLabel, errors) => `${sourceLabel} is invalid:\n- ${errors.join("\n- ")}`,
};

module.exports = { LOCALE };
