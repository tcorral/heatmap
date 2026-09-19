import type { ElementsFilter } from "../api/types";

type FieldProps = {
  filters: ElementsFilter;
  onChange: (filters: ElementsFilter) => void;
};

const PLATFORMS = ["web", "android", "ios", "macos", "windows"];

export function PlatformField({ filters, onChange }: FieldProps) {
  return (
    <label>
      Platform
      <select
        aria-label="Platform"
        value={filters.platform}
        onChange={(event) => patch(filters, onChange, { platform: event.target.value })}
      >
        {PLATFORMS.map((platform) => (
          <option key={platform} value={platform}>
            {platform}
          </option>
        ))}
      </select>
    </label>
  );
}

export function ScreenField({ filters, onChange }: FieldProps) {
  return (
    <label>
      Screen
      <input
        aria-label="Screen"
        value={filters.screen}
        onChange={(event) => patch(filters, onChange, { screen: event.target.value })}
      />
    </label>
  );
}

export function LayoutVersionField({ filters, onChange }: FieldProps) {
  return (
    <label>
      Layout version
      <input
        aria-label="Layout version"
        value={filters.layoutVersion}
        onChange={(event) =>
          patch(filters, onChange, { layoutVersion: event.target.value })
        }
      />
    </label>
  );
}

export function AppVersionField({ filters, onChange }: FieldProps) {
  return (
    <label>
      App version
      <input
        aria-label="App version"
        value={filters.appVersion}
        onChange={(event) =>
          patch(filters, onChange, { appVersion: event.target.value })
        }
      />
    </label>
  );
}

export function DateRangeFields({ filters, onChange }: FieldProps) {
  return (
    <>
      <label>
        From date
        <input
          aria-label="From date"
          type="datetime-local"
          value={toLocalInput(filters.fromMs)}
          onChange={(event) =>
            patch(filters, onChange, { fromMs: fromLocalInput(event.target.value) })
          }
        />
      </label>
      <label>
        To date
        <input
          aria-label="To date"
          type="datetime-local"
          value={toLocalInput(filters.toMs)}
          onChange={(event) =>
            patch(filters, onChange, { toMs: fromLocalInput(event.target.value) })
          }
        />
      </label>
    </>
  );
}

function patch(
  current: ElementsFilter,
  onChange: (filters: ElementsFilter) => void,
  partial: Partial<ElementsFilter>,
): void {
  onChange({ ...current, ...partial });
}

function toLocalInput(ms: number | undefined): string {
  if (ms === undefined) {
    return "";
  }
  return new Date(ms).toISOString().slice(0, 16);
}

function fromLocalInput(value: string): number | undefined {
  if (!value) {
    return undefined;
  }
  const parsed = Date.parse(value);
  return Number.isNaN(parsed) ? undefined : parsed;
}
