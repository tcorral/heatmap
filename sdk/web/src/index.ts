export {
  HEATMAP_STORE_KEY_ANONYMOUS_ID,
  HEATMAP_STORE_KEY_USER_ID,
  readIdentityValue,
  writeIdentityValue,
} from "./identity/local-storage-store.js";
export {
  normalizeWindowPosition,
  readHeatmapElementId,
  readVisibleWindowViewport,
} from "./capture/window-click.js";
export {
  startDomClickCapture,
  type ClickCaptureSink,
  type RecordedWebClick,
} from "./adapter/dom-click-capture.js";
export {
  wireDomClicksToWasm,
  type WasmActivationRecorder,
} from "./adapter/wasm-click-bridge.js";
