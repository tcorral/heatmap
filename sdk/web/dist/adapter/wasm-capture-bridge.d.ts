import type { TrackingGate } from "../capture/tracking-gate.js";
import { type WasmActivationRecorder } from "./wasm-click-bridge.js";
import { type WasmScrollRecorder } from "./wasm-scroll-bridge.js";
export type WasmDomRecorder = WasmActivationRecorder & WasmScrollRecorder;
export type WireDomCaptureOptions = {
    gate?: TrackingGate;
};
export declare function wireDomCaptureToWasm(recorder: WasmDomRecorder, options?: WireDomCaptureOptions): () => void;
//# sourceMappingURL=wasm-capture-bridge.d.ts.map