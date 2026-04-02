const LOCALE = {
  helpTitle: "create-ralph-engine-plugin",
  usageHeading: "Uso:",
  optionsHeading: "Opções:",
  optionName: "Slug do nome do plugin",
  optionPublisher: "Slug do publicador (estilo owner do GitHub)",
  optionKind: "Kind principal",
  optionCapability: "Flag repetível de capability",
  optionCapabilities: "Capabilities separadas por vírgula",
  optionDir: "Diretório de destino",
  optionYes: "Aceita os defaults no modo não interativo",
  optionHelp: "Mostra esta ajuda",
  promptPluginName: "Nome do plugin",
  promptPublisher: "Slug do publicador",
  promptPrimaryKind: "Kind principal",
  promptCapabilities: "Capabilities (separadas por vírgula)",
  createdAt: "Scaffold de plugin do Ralph Engine criado em",
  pluginId: "ID do plugin",
  missingValue: (arg) => `Falta um valor para ${arg}`,
  pluginNameRequired:
    "O nome do plugin é obrigatório. Use --name ou passe-o como primeiro argumento posicional.",
  publisherRequired: "O publicador é obrigatório. Use --publisher.",
  reservedPublisher: (publisher) => `O publicador \"${publisher}\" é reservado.`,
  unsupportedKind: (kind) => `Kind não suportado: \"${kind}\".`,
  unsupportedCapability: (capability) => `Capability não suportada: \"${capability}\".`,
  targetDirectoryExists: (targetDir) => `O diretório de destino já existe: ${targetDir}`,
};

module.exports = { LOCALE };
