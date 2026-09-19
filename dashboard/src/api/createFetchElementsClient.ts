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
  const url = buildElementsUrl(baseUrl, filters);
  const response = await fetchFn(url);
  if (!response.ok) {
    throw new Error(`Query failed (${response.status})`);
  }
  return response.json() as Promise<ElementsResponse>;
}
