import { readVisibleWindowViewport, type WindowViewport } from "../capture/window-click.js";
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

export function startDomScrollCapture(
  sink: ScrollCaptureSink,
  options?: DomScrollCaptureOptions,
): () => void {
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
