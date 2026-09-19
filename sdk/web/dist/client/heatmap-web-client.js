import { startDomClickCapture } from "../adapter/dom-click-capture.js";
import { startDomScrollCapture, } from "../adapter/dom-scroll-capture.js";
import { createTrackingGate } from "../capture/tracking-gate.js";
export function createHeatmapWebClient(options) {
    const gate = options.gate ?? createTrackingGate(true);
    let stopListeners = null;
    return {
        setTrackingEnabled(enabled) {
            gate.setTrackingEnabled(enabled);
            options.wasm?.setTrackingEnabled(enabled);
        },
        startDomCapture() {
            this.stopDomCapture();
            const stopClicks = startDomClickCapture({ recordClick: options.recordClick }, { gate });
            const stopScroll = startDomScrollCapture({ recordScroll: options.recordScroll }, { gate });
            stopListeners = () => {
                stopClicks();
                stopScroll();
            };
        },
        stopDomCapture() {
            stopListeners?.();
            stopListeners = null;
        },
    };
}
