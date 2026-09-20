import {
  COMPARISON_PLATFORMS,
  payCountForPlatform,
} from "../api/platformPayCounts";
import type { ElementTotalRow } from "../api/types";

type PlatformPayCountsTableProps = {
  rows: ElementTotalRow[];
};

export function PlatformPayCountsTable({ rows }: PlatformPayCountsTableProps) {
  return (
    <table aria-label="Checkout Pay by platform">
      <thead>
        <tr>
          <th scope="col">Platform</th>
          <th scope="col">Pay clicks</th>
        </tr>
      </thead>
      <tbody>
        {COMPARISON_PLATFORMS.map((platform) => {
          const count = payCountForPlatform(rows, platform.id);
          return (
            <tr key={platform.id}>
              <th scope="row">{platform.label}</th>
              <td>{count}</td>
            </tr>
          );
        })}
      </tbody>
    </table>
  );
}
