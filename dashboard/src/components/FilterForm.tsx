import type { ElementsFilter } from "../api/types";
import {
  AppVersionField,
  DateRangeFields,
  LayoutVersionField,
  PlatformField,
  ScreenField,
} from "./filterFields";

type FilterFormProps = {
  filters: ElementsFilter;
  onChange: (filters: ElementsFilter) => void;
  onSubmit: () => void;
};

export function FilterForm({ filters, onChange, onSubmit }: FilterFormProps) {
  return (
    <form
      aria-label="Dashboard filters"
      onSubmit={(event) => {
        event.preventDefault();
        onSubmit();
      }}
    >
      <PlatformField filters={filters} onChange={onChange} />
      <ScreenField filters={filters} onChange={onChange} />
      <LayoutVersionField filters={filters} onChange={onChange} />
      <AppVersionField filters={filters} onChange={onChange} />
      <DateRangeFields filters={filters} onChange={onChange} />
      <button type="submit">Load totals</button>
    </form>
  );
}
