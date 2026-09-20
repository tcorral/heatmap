import { describe, expect, it } from "vitest";

import { buildElementsUrl } from "../src/api/buildElementsUrl";

describe("buildElementsUrl", () => {
  it("should map dashboard filters to GET /v1/elements query params", () => {
    const url = buildElementsUrl("http://localhost:8080", {
      platform: "web",
      screen: "Checkout",
      layoutVersion: "2024-09",
      appVersion: "1.0.0",
      fromMs: 1_700_000_000_000,
      toMs: 1_800_000_000_000,
    });

    expect(url).toBe(
      "http://localhost:8080/v1/elements?platform=web&screen=Checkout&app_version=1.0.0&layout_version=2024-09&from=1700000000000&to=1800000000000",
    );
  });

  it("should omit platform when building the comparison elements URL", () => {
    const url = buildElementsUrl(
      "http://localhost:8080",
      {
        platform: "ios",
        screen: "Checkout",
        layoutVersion: "2024-09",
        appVersion: "1.0.0",
      },
      { omitPlatform: true },
    );

    expect(url).toBe(
      "http://localhost:8080/v1/elements?screen=Checkout&app_version=1.0.0&layout_version=2024-09",
    );
    expect(url).not.toContain("platform=");
  });
});
