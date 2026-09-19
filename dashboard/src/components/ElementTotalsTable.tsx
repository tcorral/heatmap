import { formatElementLabel } from "../api/elementLabel";
import type { ElementTotalRow } from "../api/types";

type ElementTotalsTableProps = {
  rows: ElementTotalRow[];
};

export function ElementTotalsTable({ rows }: ElementTotalsTableProps) {
  return (
    <table aria-label="Element totals">
      <thead>
        <tr>
          <th scope="col">Element</th>
          <th scope="col">Clicks</th>
          <th scope="col">Unique sessions</th>
        </tr>
      </thead>
      <tbody>
        {rows.length === 0 ? (
          <tr>
            <td colSpan={3}>No element totals for this slice.</td>
          </tr>
        ) : (
          rows.map((row) => (
            <tr key={row.element_id}>
              <td>{formatElementLabel(row.element_id)}</td>
              <td aria-label={String(row.click_count)}>{row.click_count}</td>
              <td>{row.unique_sessions}</td>
            </tr>
          ))
        )}
      </tbody>
    </table>
  );
}
