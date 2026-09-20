import type { ElementsFilter } from "./api/types";

export function createDefaultFilters(): ElementsFilter {
  return {
    platform: "web",
    screen: "Checkout",
    layoutVersion: "2024-09",
    appVersion: "1.0.0",
  };
}
