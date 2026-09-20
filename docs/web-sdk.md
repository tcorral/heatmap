# Web Heatmap SDK (`@heatmap/sdk-web`)

Pre-release DOM adapter for the Heatmap WASM core. It captures clicks and scrolls, reads stable element ids from markup, and drops platform-known secret fields before events reach core.

## Install

Build the package and install from a tarball or local path:

```bash
cd sdk/web
npm run build
npm pack
```

In your app:

```bash
npm install /path/to/heatmap-sdk-web-0.1.0.tgz
# or
npm install file:../heatmap-tool/sdk/web
```

You also need the `heatmap-wasm` bundle from `core/heatmap-wasm` (same version as your core release). Load it once before calling WASM `init`.

## Init

WASM owns session lifecycle, queueing, and batching. Typical bootstrap:

```typescript
import initWasm, { HeatmapSdkJs } from "./heatmap_wasm.js";

await initWasm();
const sdk = HeatmapSdkJs.init(
  JSON.stringify({
    project_key: "your-project-key",
    endpoint: "https://heatmap.example.com",
    app_version: "2.4.0",
  }),
);
```

`app_version` is optional and defaults to `1.0.0`. It is sent on every event for dashboard filtering by release.

## Flush and upload

The WASM transport uploads batches with a **synchronous** HTTP call during `flush` (including auto-flush and background flush). That keeps the Rust core API uniform but can block the main thread for the round-trip. Prefer relying on auto-flush thresholds; call `flush()` explicitly when the app backgrounds or before teardown.

Optional: persist identity keys via `@heatmap/sdk-web` localStorage helpers (`readIdentityValue` / `writeIdentityValue`) if you mirror core identity in the browser.

## Screen

Call WASM `screen` when the user navigates. It records a screen view on the current session:

```typescript
sdk.screen("Checkout", "2024-01");
```

## Annotate (`data-heatmap-id`)

Mark controls you want in element totals:

```html
<button type="button" data-heatmap-id="checkout.pay">Pay</button>
```

The adapter walks up from the click target to the nearest `[data-heatmap-id]`. Unmarked non-secret controls still contribute window-normalized click coordinates only.

## Scroll (automatic)

Wire DOM scroll capture into WASM `recordScroll`:

```typescript
import { wireDomCaptureToWasm } from "@heatmap/sdk-web";

const stop = wireDomCaptureToWasm({
  recordActivationClick(x, y, elementId, viewportWidth, viewportHeight, timestampMs) {
    sdk.recordActivationClick(
      x,
      y,
      elementId,
      viewportWidth,
      viewportHeight,
      timestampMs,
    );
  },
  recordScroll() {
    sdk.recordScroll();
  },
});
```

Scroll events are type `scroll` with viewport only (no scroll-depth map). Listeners use passive `scroll` / `wheel` on `window` / `document`.

## Disable

Turn off capture in the adapter and in core:

```typescript
import { createHeatmapWebClient } from "@heatmap/sdk-web";

const client = createHeatmapWebClient({
  wasm: sdk,
  recordClick: (click) => {
    sdk.recordActivationClick(
      click.position.x,
      click.position.y,
      click.elementId,
      click.viewport.width,
      click.viewport.height,
      Date.now(),
    );
  },
  recordScroll: () => sdk.recordScroll(),
});

client.startDomCapture();
client.setTrackingEnabled(false); // stops DOM enqueue + sdk.setTrackingEnabled(false)
```

You can also call `sdk.setTrackingEnabled(false)` directly; use the client when you want DOM listeners to respect the same gate.

## Identify

```typescript
sdk.identify("user-123");
```

## Sign-out

```typescript
sdk.signOut();
```

## Flush

Same-origin uploads can call `sdk.flush()` (WASM transport). Cross-origin POC uploads use async `fetch`; take the batch only when upload starts and confirm after the server accepts it so failed uploads restore the queue:

```typescript
const batchJson = sdk.beginFlushBatchJson();
if (!batchJson) {
  return;
}
let uploadAccepted = false;
try {
  const response = await fetch(`${endpoint}/v1/batches`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "X-Project-Key": projectKey,
    },
    body: batchJson,
  });
  uploadAccepted = response.ok;
  sdk.completeFlush(uploadAccepted);
  if (!response.ok) {
    throw new Error(`batch upload failed with status ${response.status}`);
  }
} catch (err) {
  if (!uploadAccepted) {
    sdk.completeFlush(false);
  }
  throw err;
}
```

`drainBatchJson()` remains for tests that inspect batch JSON without HTTP; do not drain then fetch in production.

## Privacy

Password inputs and common payment/password `autocomplete` values are dropped in the adapter even when unmarked. The adapter does not read `.value`, `textContent`, or `innerText` to classify fields.
