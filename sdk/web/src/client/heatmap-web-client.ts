import { startDomClickCapture, type ClickCaptureSink } from "../adapter/dom-click-capture.js";
import {
  startDomScrollCapture,
  type ScrollCaptureSink,
} from "../adapter/dom-scroll-capture.js";
import { createTrackingGate, type TrackingGate } from "../capture/tracking-gate.js";

export type HeatmapWebWasm = {
  setTrackingEnabled(enabled: boolean): void;
};

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
      stopListeners = () => {
        stopClicks();
        stopScroll();
      };
    },
    stopDomCapture() {
      stopListeners?.();
      stopListeners = null;
    },
  };
}
