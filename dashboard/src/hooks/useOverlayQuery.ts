import { useEffect, useState } from "react";

import type {
  DashboardApiClient,
  ElementsFilter,
  OverlayQueryState,
} from "../api/types";
import type { ElementsQueryRequest } from "./useElementsQuery";

export function useOverlayQuery(
  client: DashboardApiClient,
  request: ElementsQueryRequest | null,
): OverlayQueryState {
  const [state, setState] = useState<OverlayQueryState>({ status: "idle" });
  const filterKey = request ? JSON.stringify(request.filters) : "";

  useEffect(() => {
    if (!request) {
      return;
    }
    let cancelled = false;
    setState({ status: "loading" });
    loadOverlay(client, request.filters)
      .then((next) => {
        if (!cancelled) {
          setState(next);
        }
      })
      .catch((error: unknown) => {
        if (!cancelled) {
          setState({ status: "error", message: readErrorMessage(error) });
        }
      });
    return () => {
      cancelled = true;
    };
  }, [client, request?.id, filterKey]);

  return state;
}

async function loadOverlay(
  client: DashboardApiClient,
  filters: ElementsFilter,
): Promise<OverlayQueryState> {
  const still = await client.getStill(filters);
  if (!still) {
    return { status: "no_still" };
  }
  const heatmap = await client.getHeatmap(filters);
  return { status: "success", still, heatmap };
}

function readErrorMessage(error: unknown): string {
  if (error instanceof Error && error.message) {
    return error.message;
  }
  return "Overlay query failed";
}
