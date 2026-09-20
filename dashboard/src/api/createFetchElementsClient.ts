import { buildElementsUrl } from "./buildElementsUrl";
import type { ElementsApiClient, ElementsResponse } from "./types";

export function createFetchElementsClient(
  baseUrl: string,
  fetchFn: typeof fetch = fetch,
): ElementsApiClient {
  return {
    getElements: (filters) => fetchElements(baseUrl, filters, fetchFn),
  };
}

async function fetchElements(
  baseUrl: string,
  filters: Parameters<ElementsApiClient["getElements"]>[0],
  fetchFn: typeof fetch,
): Promise<ElementsResponse> {
  const filteredUrl = buildElementsUrl(baseUrl, filters);
  const comparisonUrl = buildElementsUrl(baseUrl, filters, { omitPlatform: true });
  const [filtered, comparison] = await Promise.all([
    readElements(filteredUrl, fetchFn),
    readElements(comparisonUrl, fetchFn),
  ]);
  return {
    unique_sessions: filtered.unique_sessions,
    elements: filtered.elements,
    comparison_elements: comparison.elements,
  };
}

async function readElements(
  url: string,
  fetchFn: typeof fetch,
): Promise<ElementsResponse> {
  const response = await fetchFn(url);
  if (!response.ok) {
    throw new Error(`Query failed (${response.status})`);
  }
  return response.json() as Promise<ElementsResponse>;
}
