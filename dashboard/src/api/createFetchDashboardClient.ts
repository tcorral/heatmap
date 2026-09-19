import { buildElementsUrl } from "./buildElementsUrl";
import { buildHeatmapUrl } from "./buildHeatmapUrl";
import { buildStillsUrl } from "./buildStillsUrl";
import type {
  DashboardApiClient,
  ElementsFilter,
  ElementsResponse,
  HeatmapResponse,
  StillResponse,
} from "./types";

export function createFetchDashboardClient(
  baseUrl: string,
  fetchFn: typeof fetch = fetch,
): DashboardApiClient {
  return {
    getElements: (filters) => fetchElements(baseUrl, filters, fetchFn),
    getHeatmap: (filters) => fetchHeatmap(baseUrl, filters, fetchFn),
    getStill: (filters) => fetchStill(baseUrl, filters, fetchFn),
  };
}

async function fetchElements(
  baseUrl: string,
  filters: ElementsFilter,
  fetchFn: typeof fetch,
): Promise<ElementsResponse> {
  const response = await fetchFn(buildElementsUrl(baseUrl, filters));
  if (!response.ok) {
    throw new Error(`Query failed (${response.status})`);
  }
  return response.json() as Promise<ElementsResponse>;
}

async function fetchHeatmap(
  baseUrl: string,
  filters: ElementsFilter,
  fetchFn: typeof fetch,
): Promise<HeatmapResponse> {
  const response = await fetchFn(buildHeatmapUrl(baseUrl, filters));
  if (!response.ok) {
    throw new Error(`Heatmap query failed (${response.status})`);
  }
  return response.json() as Promise<HeatmapResponse>;
}

async function fetchStill(
  baseUrl: string,
  filters: ElementsFilter,
  fetchFn: typeof fetch,
): Promise<StillResponse | null> {
  const response = await fetchFn(buildStillsUrl(baseUrl, filters));
  if (response.status === 404) {
    return null;
  }
  if (!response.ok) {
    throw new Error(`Still query failed (${response.status})`);
  }
  return response.json() as Promise<StillResponse>;
}
