import { afterEach, describe, expect, it, vi } from "vitest";

import {
  startDomClickCapture,
  type RecordedWebClick,
} from "../src/adapter/dom-click-capture.js";

function setWindowSize(width: number, height: number): void {
  Object.defineProperty(window, "innerWidth", { configurable: true, value: width });
  Object.defineProperty(window, "innerHeight", { configurable: true, value: height });
}

function clickElement(element: Element, clientX: number, clientY: number): void {
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

describe("secret field click dropping", () => {
  afterEach(() => {
    document.body.innerHTML = "";
    setWindowSize(800, 600);
  });

  it("drops clicks on unmarked password inputs", () => {
    const recordings: RecordedWebClick[] = [];
    const stop = startDomClickCapture({ recordClick: (click) => recordings.push(click) });

    const password = document.createElement("input");
    password.type = "password";
    trapSensitiveReads(password);
    document.body.append(password);

    clickElement(password, 40, 40);

    stop();
    expect(recordings).toHaveLength(0);
  });

  it("drops clicks on unmarked payment and password autocomplete fields", () => {
    const recordings: RecordedWebClick[] = [];
    const stop = startDomClickCapture({ recordClick: (click) => recordings.push(click) });

    const cases = [
      { autocomplete: "cc-number" },
      { autocomplete: "cc-exp" },
      { autocomplete: "cc-csc" },
      { autocomplete: "new-password" },
      { autocomplete: "password" },
    ];

    for (const { autocomplete } of cases) {
      const input = document.createElement("input");
      input.type = "text";
      input.setAttribute("autocomplete", autocomplete);
      trapSensitiveReads(input);
      document.body.append(input);
      clickElement(input, 50, 50);
    }

    stop();
    expect(recordings).toHaveLength(0);
  });

  it("drops clicks on child elements inside a secret field", () => {
    const recordings: RecordedWebClick[] = [];
    const stop = startDomClickCapture({ recordClick: (click) => recordings.push(click) });

    const password = document.createElement("input");
    password.type = "password";
    trapSensitiveReads(password);
    const icon = document.createElement("span");
    password.append(icon);
    document.body.append(password);

    clickElement(icon, 30, 30);

    stop();
    expect(recordings).toHaveLength(0);
  });

  it("does not read value or text when deciding to drop a secret field", () => {
    const recordings: RecordedWebClick[] = [];
    const stop = startDomClickCapture({ recordClick: (click) => recordings.push(click) });

    const password = document.createElement("input");
    password.type = "password";
    trapSensitiveReads(password);
    document.body.append(password);

    const valueSpy = vi.spyOn(HTMLInputElement.prototype, "value", "get");
    const textSpy = vi.spyOn(Element.prototype, "textContent", "get");

    clickElement(password, 40, 40);

    stop();
    expect(recordings).toHaveLength(0);
    expect(valueSpy).not.toHaveBeenCalled();
    expect(textSpy).not.toHaveBeenCalled();

    valueSpy.mockRestore();
    textSpy.mockRestore();
  });
});
