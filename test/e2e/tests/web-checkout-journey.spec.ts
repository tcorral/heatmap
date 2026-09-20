import { expect, test } from "@playwright/test";

const API_BASE =
  process.env.E2E_BACKEND_PORT != null
    ? `http://127.0.0.1:${process.env.E2E_BACKEND_PORT}`
    : "http://127.0.0.1:18080";
const DASHBOARD_BASE =
  process.env.E2E_DASHBOARD_PORT != null
    ? `http://127.0.0.1:${process.env.E2E_DASHBOARD_PORT}`
    : "http://127.0.0.1:15174";

test("web checkout journey ingests lifecycle sequence and dashboard shows Pay totals", async ({
  page,
  browser,
}) => {
  const flushResponse = page.waitForResponse(
    (response) =>
      response.url().includes("/v1/batches") &&
      response.request().method() === "POST" &&
      response.status() >= 200 &&
      response.status() < 300,
  );

  await page.goto("/");
  await page.getByRole("link", { name: "Browse products" }).click();
  await page.getByRole("button", { name: "Activate product" }).click();
  await page.getByRole("heading", { name: "Checkout" }).waitFor();
  await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
  await page.getByRole("button", { name: "Apply coupon" }).click();
  const payButton = page.locator('[data-heatmap-id="checkout.pay"]');
  await payButton.scrollIntoViewIfNeeded();
  await payButton.click();
  await page.getByRole("heading", { name: "Confirmation" }).waitFor();
  await flushResponse;

  const eventsResponse = await page.request.get(`${API_BASE}/v1/events`);
  expect(eventsResponse.ok()).toBeTruthy();
  const payload = (await eventsResponse.json()) as {
    events: Array<{
      event: { type: string; element_id?: string };
      screen?: { id: string };
    }>;
  };

  const eventTypes = payload.events.map((event) => event.event.type);
  expect(eventTypes).toContain("session_start");
  expect(eventTypes).toContain("scroll");
  expect(eventTypes.filter((type) => type === "click").length).toBeGreaterThanOrEqual(3);

  const screenViews = payload.events
    .filter((event) => event.event.type === "screen_view")
    .map((event) => event.screen?.id.toLowerCase());
  expect(screenViews).toEqual(
    expect.arrayContaining(["home", "checkout", "confirmation"]),
  );

  const elementIds = payload.events
    .filter((event) => event.event.type === "click")
    .map((event) => event.event.element_id);
  expect(elementIds).toEqual(
    expect.arrayContaining(["product", "checkout.coupon", "checkout.pay"]),
  );

  const dashboard = await browser.newPage();
  await dashboard.goto(DASHBOARD_BASE);
  await dashboard.getByRole("button", { name: "Load totals" }).click();
  const payRow = dashboard.locator("tr", { hasText: "Checkout → Pay" });
  await expect(payRow.locator("td").nth(1)).not.toHaveText("0");
});
