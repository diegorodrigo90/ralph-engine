import en from './en.json';
import ptBr from './pt-br.json';

export type Locale = 'en' | 'pt-br';

const translations: Record<Locale, Record<string, string>> = {
  en,
  'pt-br': ptBr,
};

/** Extract locale from the current URL pathname. */
export function getLangFromUrl(url: URL): Locale {
  const [, segment] = url.pathname.split('/');
  return segment === 'pt-br' ? 'pt-br' : 'en';
}

/** Look up a translation key for the given locale. Falls back to key itself. */
export function t(lang: Locale, key: string): string {
  return translations[lang]?.[key] ?? translations.en[key] ?? key;
}

/** Build locale-aware path. EN has no prefix; pt-br gets /pt-br/. */
export function localePath(lang: Locale, path: string): string {
  const clean = path.startsWith('/') ? path : `/${path}`;
  return lang === 'en' ? clean : `/pt-br${clean}`;
}

/** Get the alternate locale. */
export function alternateLang(lang: Locale): Locale {
  return lang === 'en' ? 'pt-br' : 'en';
}
