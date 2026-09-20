# iOS checkout POC (`HeatmapCheckout`)

SwiftUI sample app for the shared checkout journey. It links the local Swift packages under `sdk/apple` and exercises `HeatmapClient`, `HeatmapIosRustCaptureBackend`, and `HeatmapIosCapture`.

## Prerequisites

- Xcode 16+ with an iOS Simulator runtime (app deployment target iOS 16+ for `NavigationStack`)
- Rust toolchain (1.85+)
- Ingest API reachable from the simulator (default `http://127.0.0.1:8080`)

Build the UniFFI static library for the **iOS Simulator** (Apple Silicon):

```bash
# From repo root
rustup target add aarch64-apple-ios-sim
cargo build -p heatmap-ffi --target aarch64-apple-ios-sim
```

For macOS SDK development and `swift test`, keep the host dylib:

```bash
cargo build -p heatmap-ffi
```

## Configuration

Defaults live in `Config/Heatmap.xcconfig` and are copied into `Info.plist`:

| Key | Default |
|-----|---------|
| `HEATMAP_PROJECT_KEY` | `test-project-key` |
| `HEATMAP_ENDPOINT` | `http://127.0.0.1:8080` |

On the simulator, `127.0.0.1` reaches the Mac host. Use a LAN IP if ingest runs elsewhere.

Override in Xcode: edit `Config/Heatmap.xcconfig` or set build settings on the `HeatmapCheckout` target.

## Build (simulator)

```bash
cd poc/ios
xcodebuild \
  -project HeatmapCheckout.xcodeproj \
  -scheme HeatmapCheckout \
  -destination 'platform=iOS Simulator,name=iPhone 16' \
  -configuration Debug \
  build
```

Run from Xcode: open `HeatmapCheckout.xcodeproj`, select an iPhone simulator, Run.

## Shared journey

Matches `poc/react`:

1. `session_start` — first capture on install
2. `screen_view(Home)` — Home appears
3. Tap **Activate product** (`accessibilityIdentifier`: `product`)
4. `screen_view(Checkout)` — layout version `2024-09`
5. Scroll checkout content (automatic via `HeatmapIosCapture`)
6. Tap **Apply coupon** (`checkout.coupon`)
7. Tap **Pay** (`checkout.pay`) — see [Pay identifier test](#tests)
8. `screen_view(Confirmation)` then `HeatmapClient.flush()` (HTTP upload via Rust `HttpBatchTransport`)

The checkout screen includes a secure password field (events from that control are dropped by the SDK).

## Tests

Unit test (no simulator required for logic-only build on macOS is **not** supported — target is iOS):

```bash
xcodebuild \
  -project HeatmapCheckout.xcodeproj \
  -scheme HeatmapCheckout \
  -destination 'platform=iOS Simulator,name=iPhone 16' \
  -only-testing:HeatmapCheckoutTests/CheckoutPayIdentifierTests \
  test
```

`CheckoutPayIdentifierTests` asserts `CheckoutIdentifiers.payButton == "checkout.pay"`.

## Docs

Integrator reference: [`docs/ios-sdk.md`](../../docs/ios-sdk.md).
