import { isSecretInteractionTarget } from "../capture/secret-field.js";
import type { TrackingGate } from "../capture/tracking-gate.js";
import {
  normalizeWindowPosition,
  readHeatmapElementId,
  readVisibleWindowViewport,
  type WindowPosition,
  type WindowViewport,
} from "../capture/window-click.js";

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

export function startDomClickCapture(
  sink: ClickCaptureSink,
  options?: DomClickCaptureOptions,
): () => void {
  const onClick = (event: MouseEvent) => {
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
