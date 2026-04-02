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
  manifestNotValidYaml: (sourceLabel, error) => `${sourceLabel} não é um YAML válido: ${error}`,
  manifestMustDecodeToMappingObject: (sourceLabel) =>
    `${sourceLabel} deve ser decodificado como um objeto de mapeamento`,
  manifestUnsupportedField: (key) => `campo não suportado: "${key}"`,
  manifestMissingRequiredField: (field) => `campo obrigatório ausente: "${field}"`,
  manifestIdPattern: "id deve seguir o contrato publisher.name com namespace pontuado",
  manifestPublisherPattern: "publisher deve permanecer um slug minúsculo",
  manifestIdPrefix: "id deve começar com o slug do publisher seguido de ponto",
  manifestNonEmptyString: (field) => `${field} deve ser uma string não vazia`,
  manifestMappingObject: (field) => `${field} deve ser um objeto de mapeamento`,
  manifestArray: (field) => `${field} deve ser um array`,
  manifestLocaleKeyPattern: (field, locale) =>
    `a chave "${locale}" em ${field} deve ser um identificador de locale estável`,
  manifestLocaleValueNonEmpty: (field, locale) =>
    `${field}.${locale} deve ser uma string não vazia`,
  manifestKindEnum: "kind deve permanecer dentro dos kinds de plugin revisados",
  manifestTrustLevelEnum: "trust_level deve permanecer dentro dos níveis de confiança revisados",
  manifestSemver: (field) => `${field} deve ser uma string semver`,
  manifestCapabilitiesNonEmpty: "capabilities deve conter apenas strings não vazias",
  manifestUnsupportedCapabilityEntry: (capability) => `capability não suportada: "${capability}"`,
  manifestRepeatedCapability: (capability) => `capabilities não deve repetir "${capability}"`,
  manifestKindRequiresCapability: (kind, capability) =>
    `kind "${kind}" deve declarar a capability "${capability}"`,
  manifestRuntimeFacingRequiresPluginApi:
    "plugin_api_version é obrigatório para capabilities de plugin voltadas ao runtime",
  manifestRuntimeFacingRequiresEngineVersion:
    "engine_version é obrigatório para capabilities de plugin voltadas ao runtime",
  manifestProjectRequiresTemplate:
    "metadados de project só são válidos quando a capability template é declarada",
  manifestRequiredFilesNonEmpty:
    "project.required_files deve conter apenas strings não vazias",
  manifestRepeatedRequiredFile: (requiredFile) =>
    `project.required_files não deve repetir "${requiredFile}"`,
  manifestTemplateMustRequire: (requiredFile) =>
    `manifests de template devem exigir "${requiredFile}"`,
  manifestTemplateMustDeclareProjectFiles:
    "manifests de template devem declarar project.required_files",
  manifestInvalid: (sourceLabel, errors) =>
    `${sourceLabel} é inválido:\n- ${errors.join("\n- ")}`,
};

module.exports = { LOCALE };
