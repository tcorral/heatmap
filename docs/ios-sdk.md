# iOS Heatmap SDK (`HeatmapAppleShared` + `HeatmapAppleIOS`)

Pre-release UIKit capture adapter over the UniFFI Rust core. It records taps and scrolls, resolves stable control ids from `accessibilityIdentifier`, and drops secure text fields before events reach core.

## Install

1. Add the local Swift package in Xcode: **File → Add Package Dependencies → Add Local…** → `sdk/apple`.

   Or in another `Package.swift`:

```swift
.package(path: "../heatmap-tool/sdk/apple")
```

2. Link products:

- `HeatmapAppleShared` — `HeatmapClient`
- `HeatmapAppleIOS` — `HeatmapIosCapture`, `HeatmapIosRustCaptureBackend`

The package ships `HeatmapFFI.xcframework` (iOS device, iOS Simulator, and macOS). A clean consumer does not build Rust or point at `target/debug`.

Regenerate Swift bindings after Rust API changes: `./sdk/apple/scripts/generate-uniffi.sh`. Rebuild the XCFramework with `./sdk/apple/scripts/build-xcframework.sh`.

## Init

Use one `HeatmapSdk` instance for both the app client and the capture backend.

```swift
import HeatmapAppleIOS
import HeatmapAppleShared
import HeatmapFFI

let config = InitConfig(
    projectKey: "your-project-key",
    endpoint: "https://heatmap.example.com",
    sessionTimeoutMs: 1_800_000,
    sampleRate: 1.0,
    excludeElements: [],
    excludeScreens: [],
    maskElements: []
)
let sdk = try HeatmapSdk.init(config: config)
let client = HeatmapClient(sdk: sdk)
let backend = HeatmapIosRustCaptureBackend(sdk: sdk)

// After root view controller + window exist:
_ = HeatmapIosCapture.install(on: viewController, window: window, backend: backend)
```

`HeatmapClient(projectKey:endpoint:)` is fine when you are not wiring capture yet; with capture, prefer `HeatmapClient(sdk:)` so session and queue state stay unified.

## Screen

Call when navigation changes:

```swift
try client.screen("Checkout", layoutVersion: "2024-09")
try client.screen("Home", layoutVersion: nil)
```

## Annotate (`accessibilityIdentifier`)

Mark controls you want in element totals (SwiftUI or UIKit):

```swift
Button("Pay") { /* ... */ }
    .accessibilityIdentifier("checkout.pay")
```

```swift
payButton.accessibilityIdentifier = "checkout.pay"
```

The adapter walks from the hit view to ancestors for the nearest non-empty identifier. Unmarked non-secret controls still contribute window-normalized click coordinates only.

## Scroll (automatic)

`HeatmapIosCapture` observes `UIScrollView` `contentOffset` under the installed host view and forwards scroll events. No extra API in the app for checkout-style scrolling.

## Disable

```swift
client.setTrackingEnabled(false)
```

Core stops enqueueing; remove or reinstall capture if you need to stop UIKit listeners entirely.

## Identify

```swift
try client.identify(userId: "user-123")
```

## Sign-out

```swift
client.signOut()
```

## Flush

Upload the in-memory batch via Rust `HttpBatchTransport` (configured `endpoint` + project key):

```swift
try client.flush()
```

Unlike the web WASM path, you do **not** drain JSON and POST from Swift — `flush()` performs the HTTP upload inside the native core.

## Privacy

Secure text entry (`UITextField` / `UITextView` with secure content) and password-style fields are dropped even when unmarked. The adapter does not read labels or typed values to classify controls.

Sample journey: [`poc/ios/README.md`](../poc/ios/README.md).
