# Android E2E

Instrumentation checkout journey against the Android POC with ingest port **18083** (web **18080**, iOS **18081**, macOS **18082**).

## Prerequisites

- Android SDK (API 34), JDK 17, `adb`
- Running emulator or device (`adb devices`)
- Rust + Android NDK (`ANDROID_NDK_HOME`) for `libheatmap_ffi.so`
- `curl`

## Run

From repo root:

```bash
chmod +x test/e2e/scripts/run-android-e2e.sh
npm run test:android --prefix test/e2e
```

Or directly:

```bash
./test/e2e/scripts/run-android-e2e.sh
```

The runner uses `adb reverse tcp:18083 tcp:18083` so the app can call `http://127.0.0.1:18083` (allowed by core loopback HTTP rules).

## What the runner does

1. Builds `heatmap-ffi` for `x86_64-linux-android` and `aarch64-linux-android`
2. Starts `heatmap-backend` on **18083** with `HEATMAP_EXPOSE_TEST_EVENTS=1` and `heatmap-android-e2e.db`
3. Runs `:app:connectedDebugAndroidTest` with `heatmapEndpoint=http://127.0.0.1:18083`

## Instrumentation tests (`poc/android/app/src/androidTest/.../e2e/`)

| Test | Asserts |
|------|---------|
| `composeCheckoutJourney_capturesPayAsCheckoutPay` | Shared checkout sequence via Compose Pay (`checkout.pay`) |
| `viewCheckoutJourney_capturesPayAsCheckoutPay` | Same sequence via View XML Pay (`checkout.pay`) |
| `offlineThenRestore_uploadsOnceWithoutDoublePay` | Wi‑Fi off through coupon, restore, one Pay in `/v1/events` |
| `processDeathWithPendingEvents_uploadsOnRelaunchWithoutCrash` | Unreachable ingest, journey, force-stop, relaunch + flush uploads Pay |
| `hostRemainsUsableWhenIngestUnreachable` | Journey reaches Confirmation when ingest is down |

Event assertions use `GET /v1/events` from the instrumented test (emulator loopback via `adb reverse`), same pattern as iOS/macOS XCUITest.

## Web / iOS / macOS E2E (unchanged)

```bash
cd test/e2e && npm test          # web — 18080
npm run test:ios                   # 18081
npm run test:macos                 # 18082
```
