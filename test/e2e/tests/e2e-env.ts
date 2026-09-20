export const API_BASE =
  process.env.E2E_BACKEND_PORT != null
    ? `http://127.0.0.1:${process.env.E2E_BACKEND_PORT}`
    : "http://127.0.0.1:18080";

export const DASHBOARD_BASE =
  process.env.E2E_DASHBOARD_PORT != null
    ? `http://127.0.0.1:${process.env.E2E_DASHBOARD_PORT}`
    : "http://127.0.0.1:15174";

export type EventsPayload = {
  events: Array<{
    event: { type: string; element_id?: string };
    screen?: { id: string };
  }>;
};

export async function countPayClickEvents(
  request: { get: (url: string) => Promise<{ json: () => Promise<unknown> }> },
): Promise<number> {
  const payload = (await request.get(`${API_BASE}/v1/events`).then((r) => r.json())) as EventsPayload;
  return payload.events.filter(
    (event) =>
      event.event.type === "click" && event.event.element_id === "checkout.pay",
  ).length;
}

export async function completeCheckoutJourney(
  page: import("@playwright/test").Page,
  options?: { couponText?: string; passwordText?: string },
): Promise<void> {
  await page.getByRole("link", { name: "Browse products" }).click();
  await page.getByRole("button", { name: "Activate product" }).click();
  await page.getByRole("heading", { name: "Checkout" }).waitFor();
  if (options?.couponText) {
    await page.getByLabel("Coupon").fill(options.couponText);
  }
  if (options?.passwordText) {
    await page.getByLabel("Account password").fill(options.passwordText);
  }
  await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
  await page.getByRole("button", { name: "Apply coupon" }).click();
  const payButton = page.locator('[data-heatmap-id="checkout.pay"]');
  await payButton.scrollIntoViewIfNeeded();
  await payButton.click();
  await page.getByRole("heading", { name: "Confirmation" }).waitFor();
}

declare global {
  interface Window {
    __heatmapE2E?: {
      flushHeatmapBatch: () => Promise<void>;
      getHeatmapSdk: () => {
        flush: () => void;
        setQueueLimitsForTesting: (maxEvents: number, maxBytes: number) => void;
        queuedEventCountForTesting: () => number;
      } | null;
    };
  }
}
