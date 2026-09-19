import type { ElementsFilter } from "./types";

export function buildElementsUrl(
  baseUrl: string,
  filters: ElementsFilter,
): string {
  const params = new URLSearchParams({
    platform: filters.platform,
    screen: filters.screen,
    app_version: filters.appVersion,
  });
  if (filters.layoutVersion.trim()) {
    params.set("layout_version", filters.layoutVersion.trim());
  }
  if (filters.fromMs !== undefined) {
    params.set("from", String(filters.fromMs));
  }
  if (filters.toMs !== undefined) {
    params.set("to", String(filters.toMs));
  }
  const root = baseUrl.replace(/\/$/, "");
  return `${root}/v1/elements?${params.toString()}`;
}
