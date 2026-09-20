import { expect, test } from "@playwright/test";

import {
  completeCheckoutJourney,
  DASHBOARD_BASE,
} from "./e2e-env.js";
import {
  ACCEPTANCE_PLATFORMS,
  heatmapQueryUrl,
  registerCheckoutStillsForAllPlatforms,
  seedNativePayClicks,
} from "./five-platform-seed.js";

test.describe("five-platform dashboard acceptance", () => {
  test("web live journey plus seeded native ingests show Pay totals and heatmap bins per platform", async ({
    page,
    browser,
    request,
  }) => {
    const runId = `${Date.now()}`;

    await seedNativePayClicks(request, runId);
    await registerCheckoutStillsForAllPlatforms(request);

    const flushResponse = page.waitForResponse(
      (response) =>
        response.url().includes("/v1/batches") &&
        response.request().method() === "POST" &&
        response.status() >= 200 &&
        response.status() < 300,
    );

    await page.goto("/");
    await completeCheckoutJourney(page);
    await flushResponse;

    const dashboard = await browser.newPage();
    await dashboard.goto(DASHBOARD_BASE);

    for (const platform of ACCEPTANCE_PLATFORMS) {
      await dashboard.getByRole("combobox", { name: /platform/i }).selectOption(platform);
      await dashboard.getByRole("button", { name: "Load totals" }).click();

      const payRow = dashboard.locator("tr", { hasText: "Checkout → Pay" });
      await expect(payRow.locator("td").nth(1)).not.toHaveText("0");

      const heatmapResponse = await request.get(heatmapQueryUrl(platform));
      expect(heatmapResponse.ok()).toBeTruthy();
      const heatmap = (await heatmapResponse.json()) as { bins: unknown[] };
      expect(heatmap.bins.length).toBeGreaterThan(0);
      await expect(dashboard.getByTestId("heatmap-overlay")).toBeVisible();
    }
  });
});
