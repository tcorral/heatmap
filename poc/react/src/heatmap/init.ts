import {
  startDomClickCapture,
  wireDomClicksToWasm,
  type ClickCaptureSink,
  type WasmActivationRecorder,
} from "@heatmap/sdk-web";

import type { HeatmapSdkJs } from "../wasm/pkg/heatmap_wasm.js";

export type HeatmapClientConfig = {
  projectKey: string;
  endpoint: string;
};

const DEFAULT_PROJECT_KEY = "test-project-key";
const DEFAULT_ENDPOINT = "http://127.0.0.1:8080";

let activeConfig: HeatmapClientConfig | null = null;
let stopClickCapture: (() => void) | null = null;
let wasmSdk: HeatmapSdkJs | null = null;
let bootPromise: Promise<void> | null = null;

export function readDefaultHeatmapConfig(): HeatmapClientConfig {
  return {
    projectKey: import.meta.env.VITE_HEATMAP_PROJECT_KEY ?? DEFAULT_PROJECT_KEY,
    endpoint: import.meta.env.VITE_HEATMAP_ENDPOINT ?? DEFAULT_ENDPOINT,
  };
}

export function getHeatmapConfig(): HeatmapClientConfig | null {
  return activeConfig;
}

export function getHeatmapSdk(): HeatmapSdkJs | null {
  return wasmSdk;
}

export async function flushHeatmapBatch(): Promise<void> {
  if (!wasmSdk || !activeConfig) {
    return;
  }
  const batchJson = wasmSdk.drainBatchJson();
  if (!batchJson) {
    return;
  }
  const endpoint = activeConfig.endpoint.replace(/\/$/, "");
  const response = await fetch(`${endpoint}/v1/batches`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "X-Project-Key": activeConfig.projectKey,
    },
    body: batchJson,
  });
  if (!response.ok) {
    throw new Error(`batch upload failed with status ${response.status}`);
  }
}

export async function bootHeatmap(config: HeatmapClientConfig): Promise<void> {
  if (!bootPromise) {
    bootPromise = loadWasmSdk(config);
  }
  await bootPromise;
}

export function init(projectKey: string, endpoint: string): void {
  void initWithConfig({ projectKey, endpoint });
}

export async function initWithConfig(config: HeatmapClientConfig): Promise<void> {
  await bootHeatmap(config);
  activeConfig = {
    projectKey: config.projectKey.trim(),
    endpoint: config.endpoint.trim(),
  };
  restartClickCapture(activeConfig);
}

async function loadWasmSdk(config: HeatmapClientConfig): Promise<void> {
  const wasmModule = await import("../wasm/pkg/heatmap_wasm.js");
  await wasmModule.default();
  wasmSdk = wasmModule.HeatmapSdkJs.init(
    JSON.stringify({
      project_key: config.projectKey.trim(),
      endpoint: config.endpoint.trim(),
    }),
  );
}

function restartClickCapture(config: HeatmapClientConfig): void {
  stopClickCapture?.();
  const recorder = createRecorder(config);
  stopClickCapture = recorder
    ? wireDomClicksToWasm(recorder)
    : startDomClickCapture(createConsoleSafeSink());
}

function createRecorder(_config: HeatmapClientConfig): WasmActivationRecorder | null {
  if (!wasmSdk) {
    return null;
  }
  return {
    recordActivationClick(x, y, elementId, viewportWidth, viewportHeight, timestampMs) {
      try {
        wasmSdk?.recordActivationClick(
          x,
          y,
          elementId,
          viewportWidth,
          viewportHeight,
          timestampMs,
        );
      } catch (error: unknown) {
        console.error("[heatmap] click capture failed", error);
      }
    },
  };
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
