import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./App";
import { init, readDefaultHeatmapConfig } from "./heatmap/init";
import "./styles/app.css";

const config = readDefaultHeatmapConfig();
init(config.projectKey, config.endpoint);

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
