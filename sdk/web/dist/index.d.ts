export { HEATMAP_STORE_KEY_ANONYMOUS_ID, HEATMAP_STORE_KEY_USER_ID, readIdentityValue, writeIdentityValue, } from "./identity/local-storage-store.js";
export { normalizeWindowPosition, readHeatmapElementId, readVisibleWindowViewport, } from "./capture/window-click.js";
export { isSecretInteractionTarget } from "./capture/secret-field.js";
export { createTrackingGate, type TrackingGate, } from "./capture/tracking-gate.js";
export { startDomClickCapture, type ClickCaptureSink, type DomClickCaptureOptions, type RecordedWebClick, } from "./adapter/dom-click-capture.js";
export { startDomScrollCapture, type DomScrollCaptureOptions, type RecordedWebScroll, type ScrollCaptureSink, } from "./adapter/dom-scroll-capture.js";
export { wireDomClicksToWasm, type WasmActivationRecorder, type WireDomClickOptions, } from "./adapter/wasm-click-bridge.js";
export { wireDomScrollToWasm, type WasmScrollRecorder, type WireDomScrollOptions, } from "./adapter/wasm-scroll-bridge.js";
export { wireDomCaptureToWasm, type WasmDomRecorder, type WireDomCaptureOptions, } from "./adapter/wasm-capture-bridge.js";
export { createHeatmapWebClient, type HeatmapWebClient, type HeatmapWebClientOptions, type HeatmapWebWasm, } from "./client/heatmap-web-client.js";
//# sourceMappingURL=index.d.ts.map