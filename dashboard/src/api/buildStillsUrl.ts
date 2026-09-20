import type { ElementsFilter } from "./types";

export function buildStillsUrl(baseUrl: string, filters: ElementsFilter): string {
  const url = new URL("/v1/stills", baseUrl.endsWith("/") ? baseUrl : `${baseUrl}/`);
  url.searchParams.set("platform", filters.platform);
  url.searchParams.set("screen", filters.screen);
  if (filters.layoutVersion) {
    url.searchParams.set("layout_version", filters.layoutVersion);
  }
  return url.toString();
}
