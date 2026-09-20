import { afterEach, describe, expect, it, vi } from "vitest";

import { createHeatmapWebClient } from "../src/client/heatmap-web-client.js";
import type { RecordedWebClick } from "../src/adapter/dom-click-capture.js";
import type { RecordedWebScroll } from "../src/adapter/dom-scroll-capture.js";

function setWindowSize(width: number, height: number): void {
  Object.defineProperty(window, "innerWidth", { configurable: true, value: width });
  Object.defineProperty(window, "innerHeight", { configurable: true, value: height });
}

function clickDocument(clientX: number, clientY: number): void {
  document.dispatchEvent(
    new MouseEvent("click", {
      bubbles: true,
      cancelable: true,
      clientX,
      clientY,
    }),
  );
}

describe("tracking disabled", () => {
  afterEach(() => {
    document.body.innerHTML = "";
    setWindowSize(800, 600);
  });

  it("captures nothing further after setTrackingEnabled(false)", () => {
    const clicks: RecordedWebClick[] = [];
    const scrolls: RecordedWebScroll[] = [];
    const wasm = { setTrackingEnabled: vi.fn() };

    const client = createHeatmapWebClient({
      wasm,
      recordClick: (click) => clicks.push(click),
      recordScroll: (scroll) => scrolls.push(scroll),
    });

    client.startDomCapture();
    clickDocument(100, 100);
    window.dispatchEvent(new Event("scroll"));

    client.setTrackingEnabled(false);
    clickDocument(200, 200);
    window.dispatchEvent(new Event("scroll"));

    client.stopDomCapture();

    expect(clicks).toHaveLength(1);
    expect(scrolls).toHaveLength(1);
    expect(wasm.setTrackingEnabled).toHaveBeenCalledWith(false);
  });
});
