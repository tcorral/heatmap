import type { TrackingGate } from "../capture/tracking-gate.js";
export type WasmScrollRecorder = {
    recordScroll(): void;
};
export type WireDomScrollOptions = {
    gate?: TrackingGate;
};
export declare function wireDomScrollToWasm(recorder: WasmScrollRecorder, options?: WireDomScrollOptions): () => void;
//# sourceMappingURL=wasm-scroll-bridge.d.ts.map