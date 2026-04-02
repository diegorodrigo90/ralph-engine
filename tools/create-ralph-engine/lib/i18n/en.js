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
};

module.exports = { LOCALE };
