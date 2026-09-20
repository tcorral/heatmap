import { useEffect, useState } from "react";

import type {
  ElementsApiClient,
  ElementsFilter,
  ElementsQueryState,
} from "../api/types";

export type ElementsQueryRequest = {
  id: number;
  filters: ElementsFilter;
};

export function useElementsQuery(
  client: ElementsApiClient,
  request: ElementsQueryRequest | null,
): ElementsQueryState {
  const [state, setState] = useState<ElementsQueryState>({ status: "idle" });
  const filterKey = request ? serializeFilters(request.filters) : "";

  useEffect(() => {
    if (!request) {
      return;
    }
    let cancelled = false;
    setState({ status: "loading" });
    client
      .getElements(request.filters)
      .then((data) => {
        if (!cancelled) {
          setState({ status: "success", data });
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

function serializeFilters(filters: ElementsFilter): string {
  return JSON.stringify(filters);
}

function readErrorMessage(error: unknown): string {
  if (error instanceof Error && error.message) {
    return error.message;
  }
  return "Query failed";
}
