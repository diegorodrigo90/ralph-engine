/**
 * Shared plugin utilities for the Astro site.
 *
 * Centralizes slug generation and classification so that page routes,
 * catalog grid, and detail pages all use the same logic.
 */

/**
 * Generates a URL-safe slug from a plugin entry.
 *
 * Official plugins strip the `official.` prefix (e.g. `official.claude` → `claude`).
 * Community plugins use the full ID with dots replaced by hyphens
 * (e.g. `diegorodrigo90.hello-world` → `diegorodrigo90-hello-world`).
 */
export function pluginSlug(plugin: { id: string; source?: string }): string {
  if (plugin.id.startsWith("official.")) {
    return plugin.id.replace("official.", "");
  }
  // Community: prefix + dots→hyphens to avoid collision with official slugs
  return `community-${plugin.id.replace(/\./g, "-")}`;
}

/** Whether a plugin is from the official catalog (compiled into the binary). */
export function isOfficial(plugin: { source?: string; trust_level?: string }): boolean {
  return !plugin.source || plugin.source === "official" || plugin.trust_level === "official";
}
