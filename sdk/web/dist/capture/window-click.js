export function readHeatmapElementId(target) {
    if (!(target instanceof Element)) {
        return undefined;
    }
    const marked = target.closest("[data-heatmap-id]");
    if (!marked) {
        return undefined;
    }
    if (marked instanceof HTMLElement) {
        const fromDataset = marked.dataset.heatmapId;
        if (fromDataset && fromDataset.trim().length > 0) {
            return fromDataset.trim();
        }
    }
    const fromAttribute = marked.getAttribute("data-heatmap-id");
    if (fromAttribute && fromAttribute.trim().length > 0) {
        return fromAttribute.trim();
    }
    return undefined;
}
export function readVisibleWindowViewport() {
    const width = window.innerWidth;
    const height = window.innerHeight;
    if (width <= 0 || height <= 0) {
        return null;
    }
    return { width, height };
}
export function normalizeWindowPosition(clientX, clientY, viewport = readVisibleWindowViewport() ?? { width: 0, height: 0 }) {
    if (viewport.width <= 0 || viewport.height <= 0) {
        return null;
    }
    return {
        x: clampFraction(clientX / viewport.width),
        y: clampFraction(clientY / viewport.height),
    };
}
function clampFraction(value) {
    if (value < 0) {
        return 0;
    }
    if (value > 1) {
        return 1;
    }
    return value;
}
