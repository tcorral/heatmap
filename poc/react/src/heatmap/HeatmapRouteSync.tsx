import { useEffect } from "react";
import { useLocation } from "react-router-dom";

import { flushHeatmapBatch, getHeatmapSdk } from "./init";

export function HeatmapRouteSync() {
  const location = useLocation();

  useEffect(() => {
    const sdk = getHeatmapSdk();
    if (!sdk) {
      return;
    }
    const path = location.pathname;
    if (path === "/") {
      sdk.screen("Home", null);
      return;
    }
    if (path === "/checkout") {
      sdk.screen("Checkout", "2024-09");
      return;
    }
    if (path === "/confirmation") {
      sdk.screen("Confirmation", null);
      void flushHeatmapBatch().catch((error: unknown) => {
        console.error("[heatmap] flush failed", error);
      });
    }
  }, [location.pathname]);

  return null;
}
