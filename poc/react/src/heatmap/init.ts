import {
  startDomClickCapture,
  wireDomClicksToWasm,
  type ClickCaptureSink,
  type WasmActivationRecorder,
} from "@heatmap/sdk-web";

export type HeatmapClientConfig = {
  projectKey: string;
  endpoint: string;
};

const DEFAULT_PROJECT_KEY = "pk_poc_dev";
const DEFAULT_ENDPOINT = "https://127.0.0.1:8080";

let activeConfig: HeatmapClientConfig | null = null;
let stopClickCapture: (() => void) | null = null;

export function readDefaultHeatmapConfig(): HeatmapClientConfig {
  return {
    projectKey: import.meta.env.VITE_HEATMAP_PROJECT_KEY ?? DEFAULT_PROJECT_KEY,
    endpoint: import.meta.env.VITE_HEATMAP_ENDPOINT ?? DEFAULT_ENDPOINT,
  };
}

export function getHeatmapConfig(): HeatmapClientConfig | null {
  return activeConfig;
}

export function init(projectKey: string, endpoint: string): void {
  initWithConfig({ projectKey, endpoint });
}

export function initWithConfig(config: HeatmapClientConfig): void {
  activeConfig = {
    projectKey: config.projectKey.trim(),
    endpoint: config.endpoint.trim(),
  };
  restartClickCapture(activeConfig);
}

function restartClickCapture(config: HeatmapClientConfig): void {
  stopClickCapture?.();
  const recorder = createRecorder(config);
  stopClickCapture = recorder
    ? wireDomClicksToWasm(recorder)
    : startDomClickCapture(createConsoleSafeSink());
}

function createRecorder(config: HeatmapClientConfig): WasmActivationRecorder | null {
  const wasmSdk = loadWasmSdk(config);
  if (!wasmSdk) {
    return null;
  }
  return {
    recordActivationClick(x, y, elementId, viewportWidth, viewportHeight, timestampMs) {
      wasmSdk.recordActivationClick(
        x,
        y,
        elementId,
        viewportWidth,
        viewportHeight,
        timestampMs,
      );
    },
  };
}

type WasmSdkHandle = WasmActivationRecorder & {
  recordActivationClick: WasmActivationRecorder["recordActivationClick"];
};

function loadWasmSdk(_config: HeatmapClientConfig): WasmSdkHandle | null {
  return null;
}

function createConsoleSafeSink(): ClickCaptureSink {
  return {
    recordClick(click) {
      if (!import.meta.env.DEV) {
        return;
      }
      console.debug("[heatmap] click", {
        elementId: click.elementId ?? null,
        x: click.position.x,
        y: click.position.y,
      });
    },
  };
}
