import { describe, expect, it, vi } from "vitest";

import { createFetchDashboardClient } from "../src/api/createFetchDashboardClient";

const emptyJson = { unique_sessions: 0, elements: [] };

describe("createFetchDashboardClient", () => {
  it("should request filtered totals and an unfiltered Pay comparison", async () => {
    const fetchFn = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => emptyJson,
    });
    const client = createFetchDashboardClient("http://localhost:8080", fetchFn);

    await client.getElements({
      platform: "ios",
      screen: "Checkout",
      layoutVersion: "2024-09",
      appVersion: "1.0.0",
    });

    const urls = fetchFn.mock.calls.map((call) => call[0] as string);
    expect(urls).toContain(
      "http://localhost:8080/v1/elements?platform=ios&screen=Checkout&app_version=1.0.0&layout_version=2024-09",
    );
    expect(urls).toContain(
      "http://localhost:8080/v1/elements?screen=Checkout&app_version=1.0.0&layout_version=2024-09",
    );
  });
});
