# macOS Heatmap SDK (`HeatmapAppleShared` + `HeatmapAppleMacOS`)

Pre-release AppKit capture adapter over the UniFFI Rust core. It records clicks and scrolls, resolves stable control ids from `accessibilityIdentifier`, attaches an opaque `window_id` per `NSWindow`, and drops secure text fields before events reach core.

## Install

1. Add the local Swift package in Xcode: **File → Add Package Dependencies → Add Local…** → `sdk/apple`.

   Or in another `Package.swift`:

```swift
.package(path: "../heatmap-tool/sdk/apple")
```

2. Link products:

- `HeatmapAppleShared` — `HeatmapClient`
- `HeatmapAppleMacOS` — `HeatmapMacosCapture`, `HeatmapMacosRustCaptureBackend`

The package ships `libheatmap_ffi` under `sdk/apple/vendor/macos`. A clean consumer does not build Rust or point at `target/debug`.

Regenerate Swift bindings after Rust API changes: `./sdk/apple/scripts/generate-uniffi.sh`. Rebuild vendor dylibs as in `sdk/apple/vendor/README.md`.

## Init

Use one `HeatmapSdk` instance for both the app client and the capture backend.

```swift
import HeatmapAppleMacOS
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
let backend = HeatmapMacosRustCaptureBackend(sdk: sdk)

// After the target NSWindow exists:
_ = HeatmapMacosCapture.install(on: window, backend: backend)
```

`HeatmapClient(projectKey:endpoint:)` is fine when you are not wiring capture yet; with capture, prefer `HeatmapClient(sdk:)` so session and queue state stay unified.

## Screen

Call when navigation changes:

```swift
try client.screen("Checkout", layoutVersion: "2024-09")
try client.screen("Home", layoutVersion: nil)
```

Pair screen calls with viewport lifecycle on the backend when you track dimensions explicitly (see sample `HeatmapSession` in `poc/macos`).

## Annotate (`accessibilityIdentifier`)

Mark controls you want in element totals (SwiftUI or AppKit):

```swift
Button("Pay") { /* ... */ }
    .accessibilityIdentifier("checkout.pay")
```

```swift
payButton.setAccessibilityIdentifier("checkout.pay")
```

The adapter walks from the hit view to ancestors for the nearest non-empty identifier. Unmarked non-secret controls still contribute window-normalized click coordinates only.

## Scroll (automatic)

`HeatmapMacosCapture` observes `NSScrollView` content view bounds under the window content view and forwards scroll events. No extra API in the app for checkout-style scrolling.

## Disable

```swift
client.setTrackingEnabled(false)
```

Core stops enqueueing; remove or reinstall capture if you need to stop AppKit listeners entirely.

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

Secure text entry (`NSSecureTextField` and related) is dropped even when unmarked. The adapter does not read window titles, labels, or typed values to classify controls.

Sample journey: [`poc/macos/README.md`](../poc/macos/README.md).
