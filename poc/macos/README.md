# macOS checkout POC (`HeatmapCheckout`)

SwiftUI sample app for the shared checkout journey. It links the local Swift packages under `sdk/apple` and exercises `HeatmapClient`, `HeatmapMacosRustCaptureBackend`, and `HeatmapMacosCapture`.

## Prerequisites

- Xcode 16+ on macOS 13+
- Rust toolchain (1.85+)
- Ingest API reachable at the configured endpoint (default `http://127.0.0.1:8080`)

Build the UniFFI library for the Mac host:

```bash
# From repo root
cargo build -p heatmap-ffi
```

## Configuration

Defaults live in `Config/Heatmap.xcconfig` and are copied into `Info.plist`:

| Key | Default |
|-----|---------|
| `HEATMAP_PROJECT_KEY` | `test-project-key` |
| `HEATMAP_ENDPOINT` | `http://127.0.0.1:8080` |

Override in Xcode: edit `Config/Heatmap.xcconfig` or set build settings on the `HeatmapCheckout` target. Process environment variables `HEATMAP_PROJECT_KEY` and `HEATMAP_ENDPOINT` override bundled values at runtime.

## Build

```bash
cd poc/macos
xcodebuild \
  -project HeatmapCheckout.xcodeproj \
  -scheme HeatmapCheckout \
  -destination 'platform=macOS' \
  -configuration Debug \
  build
```

Run from Xcode: open `HeatmapCheckout.xcodeproj`, select **My Mac**, Run.

## Shared journey

Matches `poc/react` and `poc/ios`:

1. `session_start` — first capture on launch
2. `screen_view(Home)` — Home appears
3. Tap **Activate product** (`accessibilityIdentifier`: `product`)
4. `screen_view(Checkout)` — layout version `2024-09`
5. Scroll checkout content (automatic via `HeatmapMacosCapture` on `NSScrollView`)
6. Tap **Apply coupon** (`checkout.coupon`)
7. Tap **Pay** (`checkout.pay`)
8. `screen_view(Confirmation)` then `HeatmapClient.flush()` (HTTP upload via Rust `HttpBatchTransport`)

The checkout screen includes a secure password field (events from that control are dropped by the SDK).

## Tests

```bash
xcodebuild \
  -project HeatmapCheckout.xcodeproj \
  -scheme HeatmapCheckout \
  -destination 'platform=macOS' \
  test
```

`CheckoutPayIdentifierTests` asserts `CheckoutIdentifiers.payButton == "checkout.pay"`.

## Docs

Integrator reference: [`docs/macos-sdk.md`](../../docs/macos-sdk.md).
