import type { ElementTotalRow } from "../api/types";

type TopElementsListProps = {
  rows: ElementTotalRow[];
  limit?: number;
};

export function TopElementsList({ rows, limit = 5 }: TopElementsListProps) {
  const topRows = [...rows]
    .sort((left, right) => right.click_count - left.click_count)
    .slice(0, limit);

  return (
    <section aria-labelledby="top-elements-heading">
      <h2 id="top-elements-heading">Top elements</h2>
      <ol aria-label="Top elements">
        {topRows.length === 0 ? (
          <li>No clicks recorded.</li>
        ) : (
          topRows.map((row) => (
            <li key={`${row.platform ?? ""}:${row.element_id}`}>
              {row.element_id} ({row.click_count} clicks)
            </li>
          ))
        )}
      </ol>
    </section>
  );
}
