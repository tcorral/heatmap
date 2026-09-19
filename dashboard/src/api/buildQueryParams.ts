import type { ElementsFilter } from "./types";

export function appendFilterParams(
  params: URLSearchParams,
  filters: ElementsFilter,
): void {
  params.set("platform", filters.platform);
  params.set("screen", filters.screen);
  params.set("app_version", filters.appVersion);
  if (filters.layoutVersion.trim()) {
    params.set("layout_version", filters.layoutVersion.trim());
  }
  if (filters.fromMs !== undefined) {
    params.set("from", String(filters.fromMs));
  }
  if (filters.toMs !== undefined) {
    params.set("to", String(filters.toMs));
  }
}
