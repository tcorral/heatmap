# Android checkout POC (`Heatmap Checkout`)

Kotlin sample app for the shared checkout journey. It links the local `:heatmap-android` module and exercises `HeatmapClient`, `HeatmapViewCapture`, and `HeatmapComposeCapture`.

## Prerequisites

- Android SDK (API 34) and JDK 17
- Android emulator or device (minSdk 24)
- Rust toolchain + Android NDK for native `libheatmap_ffi.so`
- Ingest API reachable from the emulator (default `http://10.0.2.2:8080` → host `127.0.0.1:8080`)

Build native libraries:

```bash
# Repo root
export ANDROID_NDK_HOME=/path/to/ndk
./sdk/android/scripts/build-rust-android.sh
```

## Configuration

Defaults are set in `app/build.gradle.kts` `manifestPlaceholders` and copied into `AndroidManifest.xml` meta-data:

| Key | Default |
|-----|---------|
| `HEATMAP_PROJECT_KEY` | `test-project-key` |
| `HEATMAP_ENDPOINT` | `http://10.0.2.2:8080` |

**Emulator:** `10.0.2.2` is the emulator’s alias for the host loopback.

**Physical device:** run `adb reverse tcp:8080 tcp:8080` and set `heatmapEndpoint` to `http://127.0.0.1:8080`, or use your machine’s LAN IP in `manifestPlaceholders` / `HEATMAP_ENDPOINT` env var.

Cleartext HTTP to localhost is allowed via `res/xml/network_security_config.xml`.

## Build

```bash
cd poc/android
./gradlew :app:assembleDebug
```

Unit test (Pay contract id, no device):

```bash
./gradlew :app:testDebugUnitTest --tests com.heatmap.checkout.CheckoutPayIdentifierTest
```

Install on a connected emulator:

```bash
./gradlew :app:installDebug
adb shell am start -n com.heatmap.checkout/.MainActivity
```

Set `sdk.dir` in `local.properties` or export `ANDROID_HOME` when the Android SDK is installed.

## Shared journey

Matches `poc/react` and `poc/ios`:

1. `session_start` — first capture on install
2. `screen_view(Home)` — Home appears
3. Tap **Activate product** (`product`) — choose Compose or View checkout
4. `screen_view(Checkout)` — layout version `2024-09`
5. Scroll checkout content
6. Tap **Apply coupon** (`checkout.coupon`)
7. Tap **Pay** (`checkout.pay`) — same business id on View and Compose checkout screens
8. `screen_view(Confirmation)` then `HeatmapClient.flush()`

The checkout UI includes a password field (events from that control are dropped by the SDK).

## Capture surfaces

| Path | Checkout UI | Capture adapter |
|------|-------------|-----------------|
| Product → **Compose checkout** | Compose (`ComposeCheckoutScreen`) | `HeatmapComposeCapture` on `MainActivity` |
| Product → **View checkout** | XML + Views (`ViewCheckoutActivity`) | `HeatmapViewCapture` on that activity |

Both Pay buttons use heatmap id `checkout.pay`.

## Docs

Integrator reference: [`docs/android-sdk.md`](../../docs/android-sdk.md).

## Gaps

- Android E2E (Step 35) is not part of this POC folder yet.
- `jniLibs/` must be populated by `build-rust-android.sh` before on-device init succeeds.
- Compose scroll events are not forwarded separately (same as current Android adapter scope); scroll the checkout column manually to reach Pay.
