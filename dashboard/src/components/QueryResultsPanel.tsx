import type { ElementsQueryState } from "../api/types";
import { ElementTotalsTable } from "./ElementTotalsTable";
import { PlatformPayCountsTable } from "./PlatformPayCountsTable";
import { TopElementsList } from "./TopElementsList";

type QueryResultsPanelProps = {
  queryState: ElementsQueryState;
};

export function QueryResultsPanel({ queryState }: QueryResultsPanelProps) {
  if (queryState.status === "idle") {
    return <p>Set filters and load totals.</p>;
  }
  if (queryState.status === "loading") {
    return (
      <p role="status" aria-label="Loading element totals">
        Loading element totals…
      </p>
    );
  }
  if (queryState.status === "error") {
    return (
      <p role="alert">
        {queryState.message}
      </p>
    );
  }

  const { data } = queryState;
  return (
    <>
      <p role="status" aria-label="Unique sessions">
        Unique sessions: {data.unique_sessions}
      </p>
      <PlatformPayCountsTable rows={data.comparison_elements ?? data.elements} />
      <ElementTotalsTable rows={data.elements} />
      <TopElementsList rows={data.elements} />
    </>
  );
}
