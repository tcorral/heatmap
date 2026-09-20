# Android Heatmap SDK (`heatmap-android`)

Pre-release View and Compose capture adapters over the UniFFI Rust core. They record taps, resolve stable control ids from dedicated heatmap tags (View) or semantics (Compose), and drop password fields before events reach core.

## Install

1. Publish the pre-release AAR to Maven Local (from `sdk/android/`):

```bash
./gradlew :heatmap-android:publishToMavenLocal
```

The AAR ships `libheatmap_ffi.so` for `arm64-v8a`, `armeabi-v7a`, and `x86_64`. A clean consumer does not clone the SDK as a source module or build Rust.

2. Depend on the artifact:

```kotlin
// settings.gradle.kts
dependencyResolutionManagement {
    repositories {
        mavenLocal()
        google()
        mavenCentral()
    }
}
```

```kotlin
// app/build.gradle.kts
dependencies {
    implementation("com.heatmap:heatmap-android:0.1.0-pre1")
}
```

In-repo samples may still `include(":heatmap-android")`. Integrators should use the Maven coordinate above.

Regenerate Kotlin bindings after Rust API changes:

```bash
./sdk/android/scripts/generate-uniffi.sh \
  --android-so target/aarch64-linux-android/release/libheatmap_ffi.so
```

## Init

Use one `HeatmapSdk` instance for both the app client and capture backends.

```kotlin
import com.heatmap.sdk.HeatmapClient
import com.heatmap.sdk.HeatmapSdk
import com.heatmap.sdk.InitConfig
import com.heatmap.sdk.view.HeatmapAndroidRustCaptureBackend

val initConfig = InitConfig(
    projectKey = "your-project-key",
    endpoint = "https://heatmap.example.com",
    sessionTimeoutMs = 1_800_000u,
    sampleRate = 1.0,
    excludeElements = emptyList(),
    excludeScreens = emptyList(),
    maskElements = emptyList(),
)
val sdk = HeatmapSdk.init(initConfig)
val client = HeatmapClient.fromSdk(sdk)
val backend = HeatmapAndroidRustCaptureBackend(sdk = sdk)
```

`HeatmapClient(projectKey, endpoint)` is fine when capture is not wired yet; with capture, prefer `HeatmapClient.fromSdk(sdk)` so session and queue state stay unified.

POC defaults: project key `test-project-key`, endpoint `http://10.0.2.2:8080` on the emulator (host loopback). On a physical device with ingest on your machine, use `http://127.0.0.1:8080` with `adb reverse tcp:8080 tcp:8080`, or point `HEATMAP_ENDPOINT` at your LAN IP.

## Screen

Call when navigation changes:

```kotlin
client.screen("Checkout", "2024-09")
client.screen("Home", null)
```

## Annotate

**View (XML or code)**

```kotlin
import com.heatmap.sdk.view.HeatmapViewIdentity

HeatmapViewIdentity.setHeatmapId(payButton, "checkout.pay")
```

**Compose**

```kotlin
import com.heatmap.sdk.compose.heatmapId

Button(onClick = { /* ... */ }, modifier = Modifier.heatmapId("checkout.pay")) {
    Text("Pay")
}
```

When the dedicated heatmap id is absent, the Compose adapter can fall back to `Modifier.testTag`. Unmarked non-secret controls still contribute window-normalized click coordinates only.

## Capture wiring

**View-based screens (`Activity` / `Fragment`)**

```kotlin
import com.heatmap.sdk.view.HeatmapViewCapture

HeatmapViewCapture.install(activity, backend)
```

**Compose**

```kotlin
import com.heatmap.sdk.compose.HeatmapComposeCapture
import androidx.compose.ui.platform.AbstractComposeView

HeatmapComposeCapture.install(composeView, backend)
```

Install on the root `AbstractComposeView` after `setContent` (see `poc/android` `ComposeCaptureInstaller`).

## Disable

```kotlin
client.setTrackingEnabled(false)
```

Core stops enqueueing; uninstall capture if you need to stop touch listeners entirely.

## Identify

```kotlin
client.identify("user-123")
```

## Sign-out

```kotlin
client.signOut()
```

## Flush

Upload the in-memory batch via Rust `HttpBatchTransport` (configured `endpoint` + project key):

```kotlin
client.flush()
```

Unlike the web WASM path, you do **not** drain JSON and POST from Kotlin — `flush()` performs the HTTP upload inside the native core.

## Privacy

Password `inputType` fields and secure text entry are dropped even when unmarked. Adapters do not read labels or typed values to classify controls.

Sample journey: [`poc/android/README.md`](../poc/android/README.md).
