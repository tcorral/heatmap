import type { ElementsFilter } from "./types";
import { appendFilterParams } from "./buildQueryParams";

export type ElementsUrlOptions = {
  omitPlatform?: boolean;
};

export function buildElementsUrl(
  baseUrl: string,
  filters: ElementsFilter,
  options: ElementsUrlOptions = {},
): string {
  const params = new URLSearchParams();
  appendFilterParams(params, filters, options);
  const root = baseUrl.replace(/\/$/, "");
  return `${root}/v1/elements?${params.toString()}`;
}
