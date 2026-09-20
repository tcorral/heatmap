import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import type { APIRequestContext } from "@playwright/test";

import { API_BASE } from "./e2e-env.js";

export const E2E_PROJECT_KEY = "test-project-key";

const NATIVE_PLATFORMS = ["ios", "android", "macos", "windows"] as const;

const here = dirname(fileURLToPath(import.meta.url));
const fixturesDir = join(here, "../../fixtures");
const repoRoot = join(here, "../../..");
const checkoutStillPath = join(repoRoot, "poc/stills/checkout-layout-2024-09.png");

const TINY_PNG_BASE64 =
  "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";

type NestedEvent = Record<string, unknown>;

function loadPayFixture(platform: string): NestedEvent {
  const raw = readFileSync(
    join(fixturesDir, `click-checkout-pay-${platform}.json`),
    "utf8",
  );
  return JSON.parse(raw) as NestedEvent;
}

export async function seedNativePayClicks(
  request: APIRequestContext,
  runId: string,
): Promise<void> {
  for (const platform of NATIVE_PLATFORMS) {
    const batchId = `acceptance-seed-${platform}-${runId}`;
    const event = loadPayFixture(platform);
    const response = await request.post(`${API_BASE}/v1/batches`, {
      headers: {
        "X-Project-Key": E2E_PROJECT_KEY,
        "content-type": "application/json",
      },
      data: {
        batch_id: batchId,
        events: [event],
      },
    });
    if (!response.ok()) {
      throw new Error(
        `seed ${platform} failed: ${response.status()} ${await response.text()}`,
      );
    }
  }
}

export async function registerCheckoutStillsForAllPlatforms(
  request: APIRequestContext,
): Promise<void> {
  let imageBase64 = TINY_PNG_BASE64;
  try {
    imageBase64 = readFileSync(checkoutStillPath).toString("base64");
  } catch {
    // Optional still on disk; fall back to minimal PNG bytes.
  }

  for (const platform of ["web", ...NATIVE_PLATFORMS]) {
    const response = await request.post(`${API_BASE}/v1/stills`, {
      headers: { "content-type": "application/json" },
      data: {
        platform,
        screen: "Checkout",
        layout_version: "2024-09",
        content_type: "image/png",
        image_base64: imageBase64,
      },
    });
    if (!response.ok()) {
      throw new Error(
        `register still ${platform} failed: ${response.status()} ${await response.text()}`,
      );
    }
  }
}

export function heatmapQueryUrl(platform: string): string {
  const params = new URLSearchParams({
    platform,
    screen: "Checkout",
    layout_version: "2024-09",
    app_version: "1.0.0",
  });
  return `${API_BASE}/v1/heatmap?${params.toString()}`;
}

export const ACCEPTANCE_PLATFORMS = ["web", "ios", "android", "macos", "windows"] as const;
