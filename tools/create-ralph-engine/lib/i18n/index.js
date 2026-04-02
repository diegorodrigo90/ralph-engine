const { env } = require("node:process");

const { LOCALE: EN } = require("./en.js");
const { LOCALE: PT_BR } = require("./pt-br.js");

const LOCALE_CATALOGS = {
  en: EN,
  "pt-br": PT_BR,
};
const SUPPORTED_LOCALES = new Set(Object.keys(LOCALE_CATALOGS));
const LOCALE_ENV_KEY = "RALPH_ENGINE_LOCALE";
const DEFAULT_LOCALE = "en";

function normalizeLocale(value) {
  const normalized = String(value || "")
    .trim()
    .toLowerCase()
    .replaceAll("_", "-");
  return SUPPORTED_LOCALES.has(normalized) ? normalized : "en";
}

function resolveLocaleCatalog() {
  const locale = normalizeLocale(env[LOCALE_ENV_KEY] || DEFAULT_LOCALE);
  return LOCALE_CATALOGS[locale] || EN;
}

module.exports = {
  DEFAULT_LOCALE,
  LOCALE_CATALOGS,
  SUPPORTED_LOCALES,
  resolveLocaleCatalog,
};
