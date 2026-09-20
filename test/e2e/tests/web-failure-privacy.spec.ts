import { expect, test } from "@playwright/test";

import {
  API_BASE,
  completeCheckoutJourney,
  countPayClickEvents,
} from "./e2e-env.js";

test("offline upload restores once without double-counting pay clicks", async ({
  page,
  request,
}) => {
  const payBefore = await countPayClickEvents(request);

  await page.route("**/v1/batches", (route) => route.abort("failed"));
  await page.goto("/");
  await page.getByRole("link", { name: "Browse products" }).click();
  await page.getByRole("button", { name: "Activate product" }).click();
  await page.getByRole("heading", { name: "Checkout" }).waitFor();
  await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
  await page.getByRole("button", { name: "Apply coupon" }).click();

  const queueWhileOffline = await page.evaluate(() =>
    localStorage.getItem("heatmap.queue"),
  );
  expect(queueWhileOffline).toBeTruthy();

  await page.unroute("**/v1/batches");
  const flushOk = page.waitForResponse(
    (response) =>
      response.url().includes("/v1/batches") &&
      response.request().method() === "POST" &&
      response.status() >= 200 &&
      response.status() < 300,
  );
  const payButton = page.locator('[data-heatmap-id="checkout.pay"]');
  await payButton.scrollIntoViewIfNeeded();
  await payButton.click();
  await page.getByRole("heading", { name: "Confirmation" }).waitFor();
  await flushOk;

  const payAfter = await countPayClickEvents(request);
  expect(payAfter - payBefore).toBe(1);
});

test("invalid project key keeps host usable without batch retry loop", async ({
  page,
}) => {
  const batchPosts: number[] = [];

  page.on("request", (request) => {
    if (request.url().includes("/v1/batches") && request.method() === "POST") {
      batchPosts.push(Date.now());
    }
  });

  const failedBatch = page.waitForResponse(
    (response) =>
      response.url().includes("/v1/batches") &&
      response.request().method() === "POST" &&
      (response.status() === 401 || response.status() === 403),
  );

  await page.goto("/?heatmapProjectKey=invalid-e2e-project-key");
  await completeCheckoutJourney(page);
  await failedBatch;
  await expect(page.getByRole("heading", { name: "Confirmation" })).toBeVisible();

  expect(batchPosts.length).toBe(1);
  await page.waitForTimeout(11_000);
  expect(batchPosts.length).toBe(1);
});

test("buffer at capacity drops oldest events and host stays interactive", async ({
  page,
}) => {
  await page.goto("/");
  await page.getByRole("heading", { name: "Home" }).waitFor();

  const capped = await page.evaluate(() => {
    const sdk = window.__heatmapE2E?.getHeatmapSdk();
    if (!sdk?.setQueueLimitsForTesting) {
      return { ok: false as const, reason: "missing setQueueLimitsForTesting" };
    }
    sdk.setQueueLimitsForTesting(2, 5 * 1024 * 1024);
    sdk.screen("Home", null);
    for (let index = 0; index < 4; index += 1) {
      const fraction = 0.1 + index * 0.05;
      sdk.recordActivationClick(
        fraction,
        fraction,
        `e2e.cap.${index}`,
        800,
        600,
        1_000 + index,
      );
    }
    return {
      ok: true as const,
      queued: sdk.queuedEventCountForTesting(),
    };
  });

  expect(capped.ok, capped.ok ? "" : capped.reason).toBe(true);
  if (capped.ok) {
    expect(capped.queued).toBeLessThanOrEqual(2);
  }

  await page.getByRole("link", { name: "Browse products" }).click();
  await expect(page.getByRole("button", { name: "Activate product" })).toBeVisible();
});

test("password and typed field values never appear in ingest or queue storage", async ({
  page,
  request,
}) => {
  const secretPassword = "E2E_SECRET_PASSWORD_9f3a";
  const secretCoupon = "E2E_SECRET_COUPON_z81k";
  const consoleLines: string[] = [];

  page.on("console", (message) => {
    consoleLines.push(message.text());
  });

  const flushOk = page.waitForResponse(
    (response) =>
      response.url().includes("/v1/batches") &&
      response.request().method() === "POST" &&
      response.status() >= 200 &&
      response.status() < 300,
  );

  await page.goto("/");
  await completeCheckoutJourney(page, {
    couponText: secretCoupon,
    passwordText: secretPassword,
  });
  await flushOk;

  const eventsBody = await (await request.get(`${API_BASE}/v1/events`)).text();
  const queueRaw = await page.evaluate(() => localStorage.getItem("heatmap.queue") ?? "");

  for (const secret of [secretPassword, secretCoupon]) {
    expect(eventsBody).not.toContain(secret);
    expect(queueRaw).not.toContain(secret);
    expect(consoleLines.join("\n")).not.toContain(secret);
  }
});
