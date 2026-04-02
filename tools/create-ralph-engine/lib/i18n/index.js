const { env } = require("node:process");

const { LOCALE: EN } = require("./en.js");
const { LOCALE: PT_BR } = require("./pt-br.js");

const LOCALE_CATALOGS = {
  en: EN,
  "pt-br": PT_BR,
};
const SUPPORTED_LOCALES = new Set(Object.keys(LOCALE_CATALOGS));
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
  return LOCALE_CATALOGS[locale] || EN;
}

module.exports = {
  resolveLocaleCatalog,
};
