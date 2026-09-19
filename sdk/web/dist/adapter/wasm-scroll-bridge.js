import { startDomScrollCapture } from "./dom-scroll-capture.js";
export function wireDomScrollToWasm(recorder, options) {
    return startDomScrollCapture({
        recordScroll: () => {
            recorder.recordScroll();
        },
    }, options);
}
