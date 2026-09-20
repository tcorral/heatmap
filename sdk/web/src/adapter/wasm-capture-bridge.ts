import type { TrackingGate } from "../capture/tracking-gate.js";
import { wireDomClicksToWasm, type WasmActivationRecorder } from "./wasm-click-bridge.js";
import { wireDomScrollToWasm, type WasmScrollRecorder } from "./wasm-scroll-bridge.js";

export type WasmDomRecorder = WasmActivationRecorder & WasmScrollRecorder;

export type WireDomCaptureOptions = {
  gate?: TrackingGate;
};

export function wireDomCaptureToWasm(
  recorder: WasmDomRecorder,
  options?: WireDomCaptureOptions,
): () => void {
  const stopClicks = wireDomClicksToWasm(recorder, options);
  const stopScroll = wireDomScrollToWasm(recorder, options);
  return () => {
    stopClicks();
    stopScroll();
  };
}
