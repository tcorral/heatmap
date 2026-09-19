import { afterEach, describe, expect, it } from "vitest";

import {
  startDomScrollCapture,
  type RecordedWebScroll,
} from "../src/adapter/dom-scroll-capture.js";

function setWindowSize(width: number, height: number): void {
  Object.defineProperty(window, "innerWidth", { configurable: true, value: width });
  Object.defineProperty(window, "innerHeight", { configurable: true, value: height });
}

describe("dom scroll capture", () => {
  afterEach(() => {
    setWindowSize(800, 600);
  });

  it("emits scroll with viewport and no position map", () => {
    setWindowSize(800, 600);
    const recordings: RecordedWebScroll[] = [];
    const stop = startDomScrollCapture({
      recordScroll: (scroll) => recordings.push(scroll),
    });

    window.dispatchEvent(new Event("scroll"));

    stop();
    expect(recordings).toHaveLength(1);
    expect(recordings[0]).toEqual({
      type: "scroll",
      viewport: { width: 800, height: 600 },
    });
    expect(recordings[0]).not.toHaveProperty("position");
  });

  it("records scroll after wheel on document", () => {
    const recordings: RecordedWebScroll[] = [];
    const stop = startDomScrollCapture({
      recordScroll: (scroll) => recordings.push(scroll),
    });

    document.dispatchEvent(new WheelEvent("wheel", { bubbles: true }));

    stop();
    expect(recordings).toHaveLength(1);
    expect(recordings[0]?.type).toBe("scroll");
  });
});
