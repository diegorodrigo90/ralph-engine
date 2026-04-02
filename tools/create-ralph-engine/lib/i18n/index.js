const { env } = require("node:process");

const { LOCALE: EN } = require("./en.js");
const { LOCALE: PT_BR } = require("./pt-br.js");

const SUPPORTED_LOCALES = new Set(["en", "pt-br"]);
const LOCALE_ENV_KEY = "RALPH_ENGINE_LOCALE";

function normalizeLocale(value) {
  const normalized = String(value || "")
    .trim()
    .toLowerCase()
    .replaceAll("_", "-");
  return SUPPORTED_LOCALES.has(normalized) ? normalized : "en";
}

function resolveLocaleCatalog() {
  const locale = normalizeLocale(env[LOCALE_ENV_KEY] || "en");
  return locale === "pt-br" ? PT_BR : EN;
}

module.exports = {
  resolveLocaleCatalog,
};
