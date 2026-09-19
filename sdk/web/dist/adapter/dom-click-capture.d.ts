import type { TrackingGate } from "../capture/tracking-gate.js";
import { type WindowPosition, type WindowViewport } from "../capture/window-click.js";
export type RecordedWebClick = {
    type: "click";
    elementId?: string;
    position: WindowPosition;
    viewport: WindowViewport;
};
export type ClickCaptureSink = {
    recordClick(click: RecordedWebClick): void;
};
export type DomClickCaptureOptions = {
    gate?: TrackingGate;
};
export declare function startDomClickCapture(sink: ClickCaptureSink, options?: DomClickCaptureOptions): () => void;
//# sourceMappingURL=dom-click-capture.d.ts.map