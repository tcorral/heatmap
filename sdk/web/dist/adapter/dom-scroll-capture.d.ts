import { type WindowViewport } from "../capture/window-click.js";
import type { TrackingGate } from "../capture/tracking-gate.js";
export type RecordedWebScroll = {
    type: "scroll";
    viewport: WindowViewport;
};
export type ScrollCaptureSink = {
    recordScroll(scroll: RecordedWebScroll): void;
};
export type DomScrollCaptureOptions = {
    gate?: TrackingGate;
};
export declare function startDomScrollCapture(sink: ScrollCaptureSink, options?: DomScrollCaptureOptions): () => void;
//# sourceMappingURL=dom-scroll-capture.d.ts.map