import { afterEach, describe, expect, it, vi } from "vitest";

import { createHeatmapWebClient } from "../src/client/heatmap-web-client.js";

describe("scheduled flush", () => {
  afterEach(() => {
    vi.useRealTimers();
  });

  it("prefers async flushBatch over pollFlush", () => {
    vi.useFakeTimers();
    const flushBatch = vi.fn();
    const pollFlush = vi.fn();
    const client = createHeatmapWebClient({
      wasm: {
        setTrackingEnabled: vi.fn(),
        flushBatch,
        pollFlush,
      },
      recordClick: () => {},
      recordScroll: () => {},
    });

    client.startDomCapture();
    vi.advanceTimersByTime(10_000);
    client.stopDomCapture();

    expect(flushBatch).toHaveBeenCalledTimes(1);
    expect(pollFlush).not.toHaveBeenCalled();
  });
});
