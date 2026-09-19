import { readVisibleWindowViewport } from "../capture/window-click.js";
export function startDomScrollCapture(sink, options) {
    const recordIfEnabled = () => {
        if (options?.gate && !options.gate.isTrackingEnabled()) {
            return;
        }
        const viewport = readVisibleWindowViewport();
        if (!viewport) {
            return;
        }
        sink.recordScroll({ type: "scroll", viewport });
    };
    window.addEventListener("scroll", recordIfEnabled, { passive: true });
    document.addEventListener("wheel", recordIfEnabled, { passive: true });
    return () => {
        window.removeEventListener("scroll", recordIfEnabled);
        document.removeEventListener("wheel", recordIfEnabled);
    };
}
