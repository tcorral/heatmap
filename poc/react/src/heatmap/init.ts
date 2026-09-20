import {
  startDomClickCapture,
  wireDomClicksToWasm,
  type ClickCaptureSink,
  type WasmActivationRecorder,
} from "@heatmap/sdk-web";

import type { HeatmapSdkJs } from "../wasm/pkg/heatmap_wasm.js";

declare global {
  interface Window {
    __heatmapE2E?: {
      flushHeatmapBatch: () => Promise<void>;
      getHeatmapSdk: () => HeatmapSdkJs | null;
    };
  }
}

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

function readQueryHeatmapOverrides(): Partial<HeatmapClientConfig> {
  if (typeof window === "undefined") {
    return {};
  }
  const params = new URLSearchParams(window.location.search);
  const projectKey = params.get("heatmapProjectKey");
  if (!projectKey) {
    return {};
  }
  return { projectKey };
}

export function readDefaultHeatmapConfig(): HeatmapClientConfig {
  const defaults = {
    projectKey: import.meta.env.VITE_HEATMAP_PROJECT_KEY ?? DEFAULT_PROJECT_KEY,
    endpoint: import.meta.env.VITE_HEATMAP_ENDPOINT ?? DEFAULT_ENDPOINT,
  };
  return { ...defaults, ...readQueryHeatmapOverrides() };
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
  const endpoint = activeConfig.endpoint.replace(/\/$/, "");
  while (await uploadOneBatch(wasmSdk, endpoint, activeConfig.projectKey)) {
    // drain remaining batches after each successful upload
  }
}

async function uploadOneBatch(sdk: HeatmapSdkJs, endpoint: string, projectKey: string): Promise<boolean> {
  const batchJson = sdk.beginFlushBatchJson();
  if (!batchJson) {
    return false;
  }
  let response: Response;
  try {
    response = await fetch(`${endpoint}/v1/batches`, {
      method: "POST",
      headers: { "Content-Type": "application/json", "X-Project-Key": projectKey },
      body: batchJson,
    });
  } catch (err) {
    sdk.completeFlush(false);
    throw err;
  }
  sdk.completeFlushHttp(response.status, parseRetryAfterSeconds(response.headers.get("Retry-After")));
  if (!response.ok) {
    throw new Error(`batch upload failed with status ${response.status}`);
  }
  return true;
}

function parseRetryAfterSeconds(header: string | null): number | undefined {
  if (!header) {
    return undefined;
  }
  const delta = Number(header);
  if (!Number.isFinite(delta) || delta < 0) {
    return undefined;
  }
  return Math.floor(delta);
}

function exposeE2EHelpers(): void {
  if (!import.meta.env.DEV || typeof window === "undefined") {
    return;
  }
  window.__heatmapE2E = {
    flushHeatmapBatch,
    getHeatmapSdk: () => wasmSdk,
  };
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
  if (typeof window !== "undefined") {
    window.setInterval(() => {
      void flushHeatmapBatch().catch((error: unknown) => {
        console.error("[heatmap] timed flush failed", error);
      });
    }, 10_000);
  }
  exposeE2EHelpers();
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
        void flushHeatmapBatch().catch((error: unknown) => {
          console.error("[heatmap] flush after click failed", error);
        });
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
