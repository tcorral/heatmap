import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./App";
import { initWithConfig, readDefaultHeatmapConfig } from "./heatmap/init";
import "./styles/app.css";

const root = document.getElementById("root")!;

void (async () => {
  try {
    await initWithConfig(readDefaultHeatmapConfig());
    createRoot(root).render(
      <StrictMode>
        <App />
      </StrictMode>,
    );
  } catch (error: unknown) {
    console.error("[heatmap] failed to boot", error);
  }
})();
