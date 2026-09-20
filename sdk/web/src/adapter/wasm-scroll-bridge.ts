import { startDomScrollCapture } from "./dom-scroll-capture.js";
import type { TrackingGate } from "../capture/tracking-gate.js";

export type WasmScrollRecorder = {
  recordScroll(): void;
};

export type WireDomScrollOptions = {
  gate?: TrackingGate;
};

export function wireDomScrollToWasm(
  recorder: WasmScrollRecorder,
  options?: WireDomScrollOptions,
): () => void {
  return startDomScrollCapture(
    {
      recordScroll: () => {
        recorder.recordScroll();
      },
    },
    options,
  );
}
