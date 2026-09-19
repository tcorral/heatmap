import type { TrackingGate } from "../capture/tracking-gate.js";
export type WasmActivationRecorder = {
    recordActivationClick(x: number, y: number, elementId: string | undefined, viewportWidth: number, viewportHeight: number, timestampMs: number): void;
};
export type WireDomClickOptions = {
    gate?: TrackingGate;
};
export declare function wireDomClicksToWasm(recorder: WasmActivationRecorder, options?: WireDomClickOptions): () => void;
//# sourceMappingURL=wasm-click-bridge.d.ts.map