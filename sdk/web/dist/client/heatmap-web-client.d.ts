import { type ClickCaptureSink } from "../adapter/dom-click-capture.js";
import { type ScrollCaptureSink } from "../adapter/dom-scroll-capture.js";
import { type TrackingGate } from "../capture/tracking-gate.js";
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
export declare function createHeatmapWebClient(options: HeatmapWebClientOptions): HeatmapWebClient;
//# sourceMappingURL=heatmap-web-client.d.ts.map