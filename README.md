# Heatmap

Cross-platform heatmap SDK. One Rust core, thin adapters for **React (web)**, **iOS**, **Android**, **macOS**, and **Windows**. Capture goes to a shared ingest service; the dashboard compares the same controls across platforms.

V1 records sessions, screen views, clicks/taps, and scrolls. It does not record text, typed values, or session replay. Packages are pre-release: install from this repo, not a public registry.

If you only need one platform, jump to that section. The public API is the same everywhere: **init, screen, identify, sign-out, disable tracking, flush**. There is no manual `track` call. Adapters own click and scroll capture.

## Shared behaviour

You need a **project key** and an **ingest endpoint**. Capture starts as soon as init succeeds. Default session timeout is 30 minutes (`1_800_000` ms); backgrounding does not start a new session.

Annotate the controls you want in element totals. Unmarked, non-secret controls still paint the coordinate heatmap. Use the **same business id** on every platform (`checkout.pay`, not a generated view path).

| Platform | How you mark a control |
| -------- | ---------------------- |
| Web / React | `data-heatmap-id="checkout.pay"` |
| iOS | `accessibilityIdentifier = "checkout.pay"` |
| Android View | `HeatmapViewIdentity.setHeatmapId(view, "checkout.pay")` |
| Android Compose | `Modifier.heatmapId("checkout.pay")` (falls back to `testTag`) |
| macOS | `accessibilityIdentifier` (then `NSView.identifier` if empty) |
| Windows | `AutomationProperties.AutomationId="checkout.pay"` |

Password fields and other platform-known secrets are dropped even when marked. Adapters never read labels or typed values.

Native SDKs (`flush()`) upload inside the Rust core. **Web is different:** WASM defers network, so the host must POST the batch with `fetch`. Details in the web section.

Full wire contract: [`docs/event-and-batch-schema-v1.md`](docs/event-and-batch-schema-v1.md).

## Local ingest and dashboard

Point every SDK at a running backend. Default project key is `test-project-key`.

```bash
# Ingest (http://127.0.0.1:8080)
cargo run -p heatmap-backend

# Dashboard (http://localhost:5173) — copy dashboard/.env.example first
cd dashboard
cp .env.example .env
npm install
npm run dev
```

`HEATMAP_PROJECT_KEY` and `PORT` override backend defaults. Android emulator ingest is `http://10.0.2.2:8080` (host loopback). Physical Android: `adb reverse tcp:8080 tcp:8080` and `http://127.0.0.1:8080`.

---

## Web / React (`@heatmap/sdk-web`)

DOM adapter over the WASM core. React is the supported web host; the package itself is plain TypeScript.

### Install

```bash
cd sdk/web
npm run build
npm pack
```

In the app:

```bash
npm install /path/to/heatmap-sdk-web-0.1.0.tgz
# or
npm install file:../heatmap-tool/sdk/web
```

Build and load the matching `heatmap-wasm` bundle before calling WASM `init`. From this repo:

```bash
./scripts/build-wasm-for-poc.sh
```

That writes bindings under `poc/react/src/wasm/pkg`. Copy the same `heatmap_wasm.js` / `.wasm` pair into your app (or point Vite at that folder).

### Init and capture

```typescript
import initWasm, { HeatmapSdkJs } from "./heatmap_wasm.js";
import { createHeatmapWebClient } from "@heatmap/sdk-web";

await initWasm();
const sdk = HeatmapSdkJs.init(
  JSON.stringify({
    project_key: "your-project-key",
    endpoint: "https://heatmap.example.com",
    app_version: "2.4.0", // optional, default 1.0.0
  }),
);

async function flushHeatmapBatch() {
  const batchJson = sdk.beginFlushBatchJson();
  if (!batchJson) return;
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
    if (!uploadAccepted) sdk.completeFlush(false);
    throw err;
  }
}

const client = createHeatmapWebClient({
  wasm: { ...sdk, flushBatch: () => flushHeatmapBatch() },
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
```

`createHeatmapWebClient` starts a 10s async flush timer. Also flush on route change, when the tab backgrounds, and before teardown. Do not use `sdk.flush()` for normal uploads — that path is sync XHR and can block the main thread.

Call `sdk.screen("Checkout", "2024-01")` on navigation. Layout version is optional; omit it (`null`) when you do not version the layout.

```html
<button type="button" data-heatmap-id="checkout.pay">Pay</button>
```

```typescript
sdk.identify("user-123");
client.setTrackingEnabled(false); // adapter + core
sdk.signOut();
```

Integrator reference: [`docs/web-sdk.md`](docs/web-sdk.md). Sample: [`poc/react`](poc/react).

---

## iOS (`HeatmapAppleShared` + `HeatmapAppleIOS`)

UIKit capture over UniFFI. The Swift package already ships `HeatmapFFI.xcframework` (device, simulator, macOS). A clean consumer does **not** build Rust.

### Install

Xcode: **File → Add Package Dependencies → Add Local…** → `sdk/apple`.

Or in `Package.swift`:

```swift
.package(path: "../heatmap-tool/sdk/apple")
```

Link **HeatmapAppleShared** (`HeatmapClient`) and **HeatmapAppleIOS** (`HeatmapIosCapture`, `HeatmapIosRustCaptureBackend`). Minimum: iOS 15.

### Init and capture

Share one `HeatmapSdk` between the client and the capture backend.

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
    maskElements: [],
    storageDirectory: HeatmapClient.defaultStoragePath,
    appVersion: "1.0.0"
)
let sdk = try HeatmapSdk.init(config: config)
let client = HeatmapClient(sdk: sdk)
let backend = HeatmapIosRustCaptureBackend(sdk: sdk, runOffUi: { client.enqueueNative($0) })

// After the root view controller and window exist:
_ = HeatmapIosCapture.install(on: viewController, window: window, backend: backend)
```

`HeatmapClient(projectKey:endpoint:)` is fine before capture is wired. With capture, use `HeatmapClient(sdk:)`.

```swift
try client.screen("Checkout", layoutVersion: "2024-09")
payButton.accessibilityIdentifier = "checkout.pay"
// SwiftUI:
Button("Pay") { }.accessibilityIdentifier("checkout.pay")

try client.identify(userId: "user-123")
client.setTrackingEnabled(false)
client.signOut()
try client.flush()
```

Scroll on `UIScrollView` is automatic. `flush()` performs HTTP inside the native core.

Integrator reference: [`docs/ios-sdk.md`](docs/ios-sdk.md). Sample: [`poc/ios`](poc/ios).

---

## Android (`heatmap-android`)

View **and** Compose adapters over UniFFI. The AAR ships `libheatmap_ffi.so` for `arm64-v8a`, `armeabi-v7a`, and `x86_64`. Integrators should take the Maven artifact, not include the SDK as a source module.

### Install

```bash
cd sdk/android
./gradlew :heatmap-android:publishToMavenLocal
```

```kotlin
// settings.gradle.kts
dependencyResolutionManagement {
    repositories {
        mavenLocal()
        google()
        mavenCentral()
    }
}

// app/build.gradle.kts
dependencies {
    implementation("com.heatmap:heatmap-android:0.1.0-pre1")
}
```

minSdk 24, JDK 17, Compose compiler 1.5.x if you use Compose capture.

### Init and capture

```kotlin
import com.heatmap.sdk.HeatmapClient
import com.heatmap.sdk.HeatmapSdk
import com.heatmap.sdk.InitConfig
import com.heatmap.sdk.view.HeatmapAndroidRustCaptureBackend
import com.heatmap.sdk.view.HeatmapViewCapture
import com.heatmap.sdk.view.HeatmapViewIdentity
import com.heatmap.sdk.compose.HeatmapComposeCapture
import com.heatmap.sdk.compose.heatmapId
import java.io.File
import java.io.File

val sdk = HeatmapSdk.init(
    InitConfig(
        projectKey = "your-project-key",
        endpoint = "https://heatmap.example.com",
        sessionTimeoutMs = 1_800_000u,
        sampleRate = 1.0,
        excludeElements = emptyList(),
        excludeScreens = emptyList(),
        maskElements = emptyList(),
        storageDirectory = File(context.filesDir, "heatmap").absolutePath,
        appVersion = "1.0.0",
    ),
)
val client = HeatmapClient.fromSdk(sdk)
val backend = HeatmapAndroidRustCaptureBackend(
    sdk = sdk,
    appVersion = "1.0.0",
    runOffUi = client::enqueueNative,
)

HeatmapViewCapture.install(activity, backend)
HeatmapViewIdentity.setHeatmapId(payButton, "checkout.pay")

// Compose: install on the root AbstractComposeView after setContent
HeatmapComposeCapture.install(composeView, backend)
Button(onClick = { }, modifier = Modifier.heatmapId("checkout.pay")) {
    Text("Pay")
}

client.screen("Checkout", "2024-09")
client.identify("user-123")
client.setTrackingEnabled(false)
client.signOut()
client.flush()
```

`HeatmapClient(projectKey, endpoint)` works without capture. With capture, `fromSdk` keeps session and queue state on one `HeatmapSdk`.

POC emulator endpoint: `http://10.0.2.2:8080`.

Integrator reference: [`docs/android-sdk.md`](docs/android-sdk.md). Sample: [`poc/android`](poc/android).

---

## macOS (`HeatmapAppleShared` + `HeatmapAppleMacOS`)

Same Swift package as iOS, AppKit capture instead of UIKit. Link **HeatmapAppleMacOS**. Minimum: macOS 12.

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
    maskElements: [],
    storageDirectory: HeatmapClient.defaultStoragePath,
    appVersion: "1.0.0"
)
let sdk = try HeatmapSdk.init(config: config)
let client = HeatmapClient(sdk: sdk)
let backend = HeatmapMacosRustCaptureBackend(sdk: sdk, runOffUi: { client.enqueueNative($0) })

// After the target NSWindow exists:
_ = HeatmapMacosCapture.install(on: window, backend: backend)
```

Identity is `accessibilityIdentifier` first, then `identifier`. Desktop events carry an opaque `window_id`; coordinates are still that window’s visible bounds.

```swift
payButton.setAccessibilityIdentifier("checkout.pay")
try client.screen("Checkout", layoutVersion: "2024-09")
try client.flush()
```

`HeatmapMacosCapture` observes `NSScrollView` under the window content view. Pair screen calls with viewport lifecycle if you track dimensions yourself — the macOS sample’s `HeatmapSession` shows that.

Integrator reference: [`docs/macos-sdk.md`](docs/macos-sdk.md). Sample: [`poc/macos`](poc/macos).

---

## Windows (`Heatmap.Sdk` + `Heatmap.Sdk.Wpf`)

WPF capture over the C ABI (`heatmap_ffi`). Session-only apps can take **Heatmap.Sdk** alone. Capture-to-dashboard apps should take **Heatmap.Sdk.Wpf** (pulls Sdk + Capture).

Native DLLs are produced on Windows. Pack from `sdk/windows` after that build.

```powershell
.\sdk\windows\scripts\build-rust-windows.ps1
cd sdk\windows
dotnet pack src\Heatmap.Sdk\Heatmap.Sdk.csproj -c Release -o artifacts
dotnet pack src\Heatmap.Sdk.Capture\Heatmap.Sdk.Capture.csproj -c Release -o artifacts
dotnet pack src\Heatmap.Sdk.Wpf\Heatmap.Sdk.Wpf.csproj -c Release -o artifacts -p:EnableWindowsTargeting=true
```

```powershell
dotnet add package Heatmap.Sdk.Wpf --version 0.1.0-pre1 --source .\artifacts
```

Requires .NET 8 and Microsoft.NET.Sdk.WindowsDesktop for the WPF package. The SDK copies `heatmap_ffi.dll` from `runtimes/win-x64/native/` (or `win-arm64`) into app output.

```csharp
using Heatmap.Sdk;
using Heatmap.Sdk.Wpf;

using var client = new HeatmapClient("your-project-key", "https://heatmap.example.com");
var backend = new HeatmapWpfRustCaptureBackend(client);

// After the target Window exists:
_ = HeatmapWpfCapture.Install(mainWindow, backend);

client.Screen("Checkout", "2024-09");
AutomationProperties.SetAutomationId(payButton, "checkout.pay");
client.Identify("user-123");
client.SetTrackingEnabled(false);
client.SignOut();
client.Flush();
```

XAML equivalent: `AutomationProperties.AutomationId="checkout.pay"`. Scroll on `ScrollViewer` is automatic. `PasswordBox` hits are dropped. POC endpoint on the same machine: `http://127.0.0.1:8080`.

In-repo samples use `ProjectReference` instead of NuGet; that is not the integrator path.

Integrator reference: [`docs/windows-sdk.md`](docs/windows-sdk.md). Sample: [`poc/windows`](poc/windows).

---

## Rust core

Not a fifth GUI SDK. `heatmap-core` owns session, identity, privacy, sampling, queue, batching, and retry. Hosts bind through:

- `heatmap-wasm` — wasm-bindgen for the browser
- `heatmap-ffi` — UniFFI (Apple, Android) and C ABI (Windows)

If you are wrapping a new host, depend on those crates and keep capture, secret detection, and HTTPS in the adapter. Do not reimplement the queue.

```toml
heatmap-core = { path = "../heatmap-tool/core/heatmap-core" }
heatmap-ffi = { path = "../heatmap-tool/core/heatmap-ffi" }
```

Rebuild native artifacts after FFI changes: Apple `./sdk/apple/scripts/build-xcframework.sh`, Android `./sdk/android/scripts/build-rust-android.sh`, Windows `.\sdk\windows\scripts\build-rust-windows.ps1`.

---

## Samples

Five checkout apps walk the same journey (Home → product → Checkout → coupon → Pay → Confirmation) with Pay annotated as `checkout.pay`:

- [`poc/react`](poc/react)
- [`poc/ios`](poc/ios)
- [`poc/android`](poc/android)
- [`poc/macos`](poc/macos)
- [`poc/windows`](poc/windows)

## Docs

| Doc | Contents |
| --- | -------- |
| [`docs/web-sdk.md`](docs/web-sdk.md) | Web install, WASM flush, DOM capture |
| [`docs/ios-sdk.md`](docs/ios-sdk.md) | SwiftPM, UIKit capture |
| [`docs/android-sdk.md`](docs/android-sdk.md) | AAR, View + Compose |
| [`docs/macos-sdk.md`](docs/macos-sdk.md) | AppKit capture, window id |
| [`docs/windows-sdk.md`](docs/windows-sdk.md) | NuGet, WPF capture |
| [`docs/event-and-batch-schema-v1.md`](docs/event-and-batch-schema-v1.md) | Event and batch JSON |
