import { startDomClickCapture, type ClickCaptureSink } from "../adapter/dom-click-capture.js";
import {
  startDomScrollCapture,
  type ScrollCaptureSink,
} from "../adapter/dom-scroll-capture.js";
import { createTrackingGate, type TrackingGate } from "../capture/tracking-gate.js";

export type HeatmapWebWasm = {
  setTrackingEnabled(enabled: boolean): void;
  pollFlush?: () => void;
  flushBatch?: () => void | Promise<void>;
};

const FLUSH_INTERVAL_MS = 10_000;

function startFlushTimer(
  flush: (() => void | Promise<void>) | undefined,
): number | null {
  if (!flush || typeof window === "undefined") {
    return null;
  }
  return window.setInterval(() => {
    try {
      void Promise.resolve(flush()).catch(() => {
        // Fail closed: interval flush must not throw into the host page.
      });
    } catch {
      // Fail closed: interval flush must not throw into the host page.
    }
  }, FLUSH_INTERVAL_MS);
}

export type HeatmapWebClientOptions = {
  wasm?: HeatmapWebWasm;
  recordClick: ClickCaptureSink["recordClick"];
  recordScroll: ScrollCaptureSink["recordScroll"];
  gate?: TrackingGate;
};

export type HeatmapWebClient = {
  setTrackingEnabled(enabled: boolean): void;
  startDomCapture(): void;
  stopDomCapture(): void;
};

export function createHeatmapWebClient(
  options: HeatmapWebClientOptions,
): HeatmapWebClient {
  const gate = options.gate ?? createTrackingGate(true);
  let stopListeners: (() => void) | null = null;
  let flushTimer: number | null = null;

  return {
    setTrackingEnabled(enabled: boolean) {
      gate.setTrackingEnabled(enabled);
      options.wasm?.setTrackingEnabled(enabled);
    },
    startDomCapture() {
      this.stopDomCapture();
      const stopClicks = startDomClickCapture(
        { recordClick: options.recordClick },
        { gate },
      );
      const stopScroll = startDomScrollCapture(
        { recordScroll: options.recordScroll },
        { gate },
      );
      flushTimer = startFlushTimer(
        options.wasm?.flushBatch ?? options.wasm?.pollFlush,
      );
      stopListeners = () => {
        stopClicks();
        stopScroll();
        if (flushTimer !== null) {
          window.clearInterval(flushTimer);
          flushTimer = null;
        }
      };
    },
    stopDomCapture() {
      stopListeners?.();
      stopListeners = null;
    },
  };
}
