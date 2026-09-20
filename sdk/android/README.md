# Heatmap Android SDK (Step 31)

Kotlin public API over UniFFI from `core/heatmap-ffi`, packaged as an AAR from `:heatmap-android`.

## Public API

Use `com.heatmap.sdk.HeatmapClient` only:

- `HeatmapClient(configJson)` / `HeatmapClient(projectKey, endpoint)`
- `identify`, `signOut`, `screen`, `setTrackingEnabled`, `flush`

There is no public `track` API. V1 wire activations use type `click` at the adapter boundary (`WireActivationType.CLICK` in `com.heatmap.sdk.internal`); View/Compose capture arrives in later steps.

Generated UniFFI types (`HeatmapSdk`, `InitConfig`, …) live in the same module for packaging but are not part of the integrator contract.

## Bindgen (Kotlin)

Bindings are generated with the same UniFFI 0.28.3 bindgen as Apple (`sdk/apple/scripts/bindgen`).

**macOS dev (refresh Kotlin only, uses host dylib checksums):**

```bash
./sdk/android/scripts/generate-uniffi.sh --host-dylib
```

**After building Android `.so` libraries (checksums must match shipped native libs):**

```bash
export ANDROID_NDK_HOME=/path/to/ndk
./sdk/android/scripts/build-rust-android.sh
./sdk/android/scripts/generate-uniffi.sh \
  --android-so target/aarch64-linux-android/release/libheatmap_ffi.so
```

Config: `core/heatmap-ffi/uniffi.toml` (`package_name = "com.heatmap.sdk"`, `cdylib_name = "heatmap_ffi"`).

Install bindgen locally (optional; the repo runs it via `cargo run` in `sdk/apple/scripts/bindgen`):

```text
cargo install uniffi-bindgen --version 0.28.3 --locked
```

## Gradle commands

From `sdk/android/`:

```bash
./gradlew :heatmap-android:assembleRelease          # AAR under heatmap-android/build/outputs/aar/
./gradlew :heatmap-android:publishToMavenLocal      # pre-release Maven Local artifact
./gradlew :heatmap-android:testDebugUnitTest        # public API and packaging tests
./gradlew :consumer-smoke-test:testDebugUnitTest    # in-tree project dependency smoke (host JVM cannot load Android .so)
./gradlew -p consumer-aar-smoke testDebugUnitTest   # clean AAR consumer (after publishToMavenLocal)
./gradlew test                                      # all module unit tests
```

Set `sdk.dir` via `local.properties` or `ANDROID_HOME` when the Android SDK is installed.

## Module layout

| Module | Role |
|--------|------|
| `:heatmap-android` | AAR: `HeatmapClient`, UniFFI Kotlin, `jniLibs/libheatmap_ffi.so`; publishes `com.heatmap:heatmap-android:0.1.0-pre1` |
| `:consumer-smoke-test` | In-tree `project(":heatmap-android")` smoke |
| `consumer-aar-smoke/` | Clean consumer: Maven Local AAR only, no source-module include |

## Gaps / CI follow-ups

- Host JVM unit tests skip FFI init when the Android `.so` cannot load on the host; emulator/device tests are not part of Step 31.
- Rebuild native libraries after Rust FFI changes: `ANDROID_NDK_HOME=... ./sdk/android/scripts/build-rust-android.sh`.
