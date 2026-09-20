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
./gradlew :heatmap-android:testDebugUnitTest        # public API tests (no native lib required)
./gradlew :consumer-smoke-test:testDebugUnitTest    # consumer smoke (skipped if jniLibs empty)
./gradlew test                                      # all module unit tests
```

Set `sdk.dir` via `local.properties` or `ANDROID_HOME` when the Android SDK is installed.

## Module layout

| Module | Role |
|--------|------|
| `:heatmap-android` | AAR: `HeatmapClient`, UniFFI Kotlin, `jniLibs/libheatmap_ffi.so` |
| `:consumer-smoke-test` | Depends on `:heatmap-android`; calls public API in unit tests |

## Gaps / CI follow-ups

- `jniLibs/` is empty until `build-rust-android.sh` runs on a machine with Rust Android targets and NDK.
- Host JVM unit tests skip FFI smoke when `libheatmap_ffi` is missing; emulator/device tests are not part of Step 31.
- Maven publish coordinates and signing are out of scope for this step.
