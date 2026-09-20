import { afterEach, describe, expect, it, vi } from "vitest";

import {
  normalizeWindowPosition,
  readHeatmapElementId,
} from "../src/capture/window-click.js";
import {
  startDomClickCapture,
  type RecordedWebClick,
} from "../src/adapter/dom-click-capture.js";

function setWindowSize(width: number, height: number): void {
  Object.defineProperty(window, "innerWidth", {
    configurable: true,
    value: width,
  });
  Object.defineProperty(window, "innerHeight", {
    configurable: true,
    value: height,
  });
}

function clickElement(
  element: Element,
  clientX: number,
  clientY: number,
): void {
  element.dispatchEvent(
    new MouseEvent("click", {
      bubbles: true,
      cancelable: true,
      clientX,
      clientY,
    }),
  );
}

function trapSensitiveReads(element: HTMLElement): void {
  for (const key of ["textContent", "innerText", "value"] as const) {
    Object.defineProperty(element, key, {
      configurable: true,
      get() {
        throw new Error(`forbidden read: ${key}`);
      },
    });
  }
}

describe("readHeatmapElementId", () => {
  it("reads checkout.pay from data-heatmap-id on Pay", () => {
    const pay = document.createElement("button");
    pay.setAttribute("data-heatmap-id", "checkout.pay");
    pay.textContent = "Pay";

    expect(readHeatmapElementId(pay)).toBe("checkout.pay");
  });
});

describe("normalizeWindowPosition", () => {
  it("stores about 0.5, 0.5 for the visible window center", () => {
    setWindowSize(800, 600);
    const position = normalizeWindowPosition(400, 300);

    expect(position).toEqual({ x: 0.5, y: 0.5 });
  });
});

describe("dom click capture", () => {
  afterEach(() => {
    document.body.innerHTML = "";
    setWindowSize(800, 600);
  });

  it("emits click with checkout.pay element_id when Pay is marked", () => {
    const recordings: RecordedWebClick[] = [];
    const stop = startDomClickCapture({ recordClick: (click) => recordings.push(click) });

    const pay = document.createElement("button");
    pay.setAttribute("data-heatmap-id", "checkout.pay");
    pay.textContent = "Pay";
    document.body.append(pay);

    clickElement(pay, 120, 80);

    stop();
    expect(recordings).toHaveLength(1);
    expect(recordings[0]?.type).toBe("click");
    expect(recordings[0]?.elementId).toBe("checkout.pay");
    expect(recordings[0]?.position).toEqual({ x: 0.15, y: 80 / 600 });
  });

  it("emits click with position and no element_id for an unmarked control", () => {
    const recordings: RecordedWebClick[] = [];
    const stop = startDomClickCapture({ recordClick: (click) => recordings.push(click) });

    const cancel = document.createElement("button");
    cancel.type = "button";
    cancel.textContent = "Cancel";
    document.body.append(cancel);

    clickElement(cancel, 200, 300);

    stop();
    expect(recordings).toHaveLength(1);
    expect(recordings[0]?.elementId).toBeUndefined();
    expect(recordings[0]?.position).toEqual({ x: 0.25, y: 0.5 });
  });

  it("uses the current window size after resize for new events", () => {
    setWindowSize(800, 600);
    const recordings: RecordedWebClick[] = [];
    const stop = startDomClickCapture({ recordClick: (click) => recordings.push(click) });

    const target = document.createElement("div");
    document.body.append(target);
    clickElement(target, 400, 300);

    setWindowSize(400, 200);
    clickElement(target, 200, 100);

    stop();
    expect(recordings[0]?.position).toEqual({ x: 0.5, y: 0.5 });
    expect(recordings[0]?.viewport).toEqual({ width: 800, height: 600 });
    expect(recordings[1]?.position).toEqual({ x: 0.5, y: 0.5 });
    expect(recordings[1]?.viewport).toEqual({ width: 400, height: 200 });
  });

  it("does not read control text or input values while capturing", () => {
    const recordings: RecordedWebClick[] = [];
    const stop = startDomClickCapture({ recordClick: (click) => recordings.push(click) });

    const input = document.createElement("input");
    input.setAttribute("data-heatmap-id", "checkout.coupon");
    trapSensitiveReads(input);
    document.body.append(input);

    const textContentSpy = vi.spyOn(Element.prototype, "textContent", "get");
    const valueSpy = vi.spyOn(HTMLInputElement.prototype, "value", "get");

    clickElement(input, 40, 40);

    stop();
    expect(recordings).toHaveLength(1);
    expect(recordings[0]?.elementId).toBe("checkout.coupon");
    expect(recordings[0]?.position).toEqual({ x: 0.05, y: 40 / 600 });
    expect(textContentSpy).not.toHaveBeenCalled();
    expect(valueSpy).not.toHaveBeenCalled();

    textContentSpy.mockRestore();
    valueSpy.mockRestore();
  });
});
