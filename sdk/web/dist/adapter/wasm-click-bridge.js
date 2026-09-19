import { startDomClickCapture } from "./dom-click-capture.js";
export function wireDomClicksToWasm(recorder, options) {
    return startDomClickCapture({
        recordClick: (click) => {
            recorder.recordActivationClick(click.position.x, click.position.y, click.elementId, click.viewport.width, click.viewport.height, Date.now());
        },
    }, options);
}
