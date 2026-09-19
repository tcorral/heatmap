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
