import { useMemo, useState } from "react";

import { createFetchDashboardClient } from "./api/createFetchDashboardClient";
import type { DashboardApiClient, ElementsFilter } from "./api/types";
import { FilterForm } from "./components/FilterForm";
import { HeatmapOverlayPanel } from "./components/HeatmapOverlayPanel";
import { QueryResultsPanel } from "./components/QueryResultsPanel";
import { createDefaultFilters } from "./defaultFilters";
import { useElementsQuery, type ElementsQueryRequest } from "./hooks/useElementsQuery";
import { useOverlayQuery } from "./hooks/useOverlayQuery";

type DashboardAppProps = {
  client?: DashboardApiClient;
  autoLoad?: boolean;
};

export function DashboardApp({ client, autoLoad = false }: DashboardAppProps) {
  const apiClient = useMemo(
    () => client ?? createFetchDashboardClient(readApiBaseUrl()),
    [client],
  );
  const [draftFilters, setDraftFilters] = useState<ElementsFilter>(createDefaultFilters);
  const [request, setRequest] = useState<ElementsQueryRequest | null>(() =>
    autoLoad ? { id: 1, filters: createDefaultFilters() } : null,
  );
  const queryState = useElementsQuery(apiClient, request);
  const overlayState = useOverlayQuery(apiClient, request);

  return (
    <main>
      <h1>Heatmap dashboard</h1>
      <FilterForm
        filters={draftFilters}
        onChange={setDraftFilters}
        onSubmit={() =>
          setRequest((current) => ({
            id: (current?.id ?? 0) + 1,
            filters: draftFilters,
          }))
        }
      />
      <QueryResultsPanel queryState={queryState} />
      <HeatmapOverlayPanel overlayState={overlayState} />
    </main>
  );
}

function readApiBaseUrl(): string {
  return import.meta.env.VITE_API_BASE_URL ?? "";
}
