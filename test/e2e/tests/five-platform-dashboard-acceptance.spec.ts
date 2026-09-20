import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

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

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "../../..");

test.describe("five-platform dashboard acceptance", () => {
  test("android Compose checkout does not report scroll from the host", () => {
    const session = readFileSync(
      join(
        repoRoot,
        "poc/android/app/src/main/kotlin/com/heatmap/checkout/heatmap/HeatmapSession.kt",
      ),
      "utf8",
    );
    const composeScreen = readFileSync(
      join(
        repoRoot,
        "poc/android/app/src/main/kotlin/com/heatmap/checkout/ui/screens/ComposeCheckoutScreen.kt",
      ),
      "utf8",
    );
    const adapter = readFileSync(
      join(
        repoRoot,
        "sdk/android/heatmap-android/src/main/kotlin/com/heatmap/sdk/compose/HeatmapScrollCapture.kt",
      ),
      "utf8",
    );
    expect(session).not.toMatch(/recordViewportScroll/);
    expect(composeScreen).not.toMatch(/recordViewportScroll/);
    expect(composeScreen).not.toMatch(/snapshotFlow/);
    expect(composeScreen).toMatch(/heatmapScrollCapture/);
    expect(adapter).toMatch(/ComposeScrollObserver/);
  });

  test("web live journey plus seeded native ingests show Pay counts together and overlay per platform", async ({
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
    await dashboard.getByRole("button", { name: "Load totals" }).click();

    const comparison = dashboard.getByRole("table", {
      name: /checkout pay by platform/i,
    });
    await expect(comparison).toBeVisible();

    for (const header of ["web", "iOS", "Android", "macOS", "Windows"] as const) {
      const row = comparison.getByRole("row").filter({
        has: dashboard.getByRole("rowheader", { name: header, exact: true }),
      });
      await expect(row.locator("td")).not.toHaveText("0");
    }

    for (const platform of ACCEPTANCE_PLATFORMS) {
      await dashboard.getByRole("combobox", { name: /platform/i }).selectOption(platform);
      await dashboard.getByRole("button", { name: "Load totals" }).click();

      const heatmapResponse = await request.get(heatmapQueryUrl(platform));
      expect(heatmapResponse.ok()).toBeTruthy();
      const heatmap = (await heatmapResponse.json()) as { bins: unknown[] };
      expect(heatmap.bins.length).toBeGreaterThan(0);
      await expect(dashboard.getByTestId("heatmap-overlay")).toBeVisible();
    }
  });
});
