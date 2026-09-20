import type { ElementsFilter } from "./types";
import { appendFilterParams } from "./buildQueryParams";

export function buildHeatmapUrl(baseUrl: string, filters: ElementsFilter): string {
  const url = new URL("/v1/heatmap", baseUrl.endsWith("/") ? baseUrl : `${baseUrl}/`);
  appendFilterParams(url.searchParams, filters);
  return url.toString();
}
