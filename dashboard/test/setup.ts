import "@testing-library/jest-dom/vitest";
import { cleanup } from "@testing-library/react";
import { afterEach, vi } from "vitest";

HTMLCanvasElement.prototype.getContext = vi.fn(() => null);

afterEach(() => {
  cleanup();
});
