export type WindowPosition = {
  x: number;
  y: number;
};

export type WindowViewport = {
  width: number;
  height: number;
};

export function readHeatmapElementId(target: EventTarget | null): string | undefined {
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

export function readVisibleWindowViewport(): WindowViewport | null {
  const width = window.innerWidth;
  const height = window.innerHeight;
  if (width <= 0 || height <= 0) {
    return null;
  }
  return { width, height };
}

export function normalizeWindowPosition(
  clientX: number,
  clientY: number,
  viewport: WindowViewport = readVisibleWindowViewport() ?? { width: 0, height: 0 },
): WindowPosition | null {
  if (viewport.width <= 0 || viewport.height <= 0) {
    return null;
  }
  return {
    x: clampFraction(clientX / viewport.width),
    y: clampFraction(clientY / viewport.height),
  };
}

function clampFraction(value: number): number {
  if (value < 0) {
    return 0;
  }
  if (value > 1) {
    return 1;
  }
  return value;
}
