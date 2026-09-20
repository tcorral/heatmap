# iOS E2E

XCUITest checkout journey against the iOS POC with a dedicated ingest port (`18081`) so Playwright web E2E can keep using `18080`.

## Prerequisites

- Xcode with an iOS Simulator runtime
- Rust toolchain; `aarch64-apple-ios-sim` target for UniFFI
- `curl` and Python 3 (simulator selection in the runner)

## Run

From repo root:

```bash
chmod +x test/e2e/scripts/run-ios-e2e.sh
npm run test:ios --prefix test/e2e
```

Or directly:

```bash
./test/e2e/scripts/run-ios-e2e.sh
```

Override simulator destination:

```bash
IOS_SIMULATOR_DESTINATION='platform=iOS Simulator,name=iPhone 16' ./test/e2e/scripts/run-ios-e2e.sh
```

## What the runner does

1. Builds `heatmap-ffi` for `aarch64-apple-ios-sim`
2. Starts `heatmap-backend` on port **18081** with `HEATMAP_EXPOSE_TEST_EVENTS=1` and a fresh SQLite file (`heatmap-ios-e2e.db` at repo root)
3. Runs `xcodebuild test` for `HeatmapCheckoutUITests` with `poc/ios/Config/HeatmapE2E.xcconfig`
4. Writes an `.xcresult` bundle under `test/e2e/ios/results/` (gitignored)

## UI tests (`poc/ios/HeatmapCheckoutUITests/`)

| Test | Asserts |
|------|---------|
| `testCheckoutJourneyIngestsSharedSequenceWithNormalizedCoordinates` | `session_start`, screen views, scroll, product/coupon/pay clicks, normalized click positions, single session |
| `testBackgroundResumeKeepsSingleSession` | Home → background → resume → finish checkout; one `session_id` in `/v1/events` |
| `testHostRemainsUsableWhenIngestUnreachable` | Journey completes to Confirmation when `HEATMAP_ENDPOINT` points at a closed port |

Event assertions use `GET /v1/events` from the test runner process (Mac host), same as web Playwright E2E.

## Web E2E (unchanged)

```bash
cd test/e2e && npm test
```

Uses port `18080` via `playwright.config.ts`.
