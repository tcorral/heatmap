import { isSecretInteractionTarget } from "../capture/secret-field.js";
import { normalizeWindowPosition, readHeatmapElementId, readVisibleWindowViewport, } from "../capture/window-click.js";
export function startDomClickCapture(sink, options) {
    const onClick = (event) => {
        if (options?.gate && !options.gate.isTrackingEnabled()) {
            return;
        }
        if (isSecretInteractionTarget(event.target)) {
            return;
        }
        if (event.button !== 0) {
            return;
        }
        const viewport = readVisibleWindowViewport();
        if (!viewport) {
            return;
        }
        const position = normalizeWindowPosition(event.clientX, event.clientY, viewport);
        if (!position) {
            return;
        }
        const elementId = readHeatmapElementId(event.target);
        sink.recordClick({
            type: "click",
            elementId,
            position,
            viewport,
        });
    };
    document.addEventListener("click", onClick, true);
    return () => document.removeEventListener("click", onClick, true);
}
