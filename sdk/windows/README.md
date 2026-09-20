# Heatmap Windows SDK (Step 36)

C# public API over the stable C ABI in `core/heatmap-ffi` (`heatmap_sdk_*`), packaged as pre-release NuGet **Heatmap.Sdk**.

## Public API

Use `Heatmap.Sdk.HeatmapClient` only:

- `HeatmapClient(configJson)` / `HeatmapClient(projectKey, endpoint)`
- `Identify`, `SignOut`, `Screen`, `SetTrackingEnabled`, `Flush`

There is no public `Track` API. V1 wire activations use type `click` at the adapter boundary (`WireActivationType.Click` in `Heatmap.Sdk.Internal`); WPF capture arrives in Step 37.

P/Invoke entry points: `heatmap_sdk_init`, `heatmap_sdk_free`, `heatmap_sdk_identify`, `heatmap_sdk_sign_out`, `heatmap_sdk_screen`, `heatmap_sdk_set_tracking_enabled`, `heatmap_sdk_flush` (see `core/heatmap-ffi/src/c_abi.rs`).

## Native library

| RID | Path (after build) |
|-----|-------------------|
| win-x64 | `runtimes/win-x64/native/heatmap_ffi.dll` |
| win-arm64 | `runtimes/win-arm64/native/heatmap_ffi.dll` |

**Windows CI / dev machine:**

```powershell
.\sdk\windows\scripts\build-rust-windows.ps1
```

```bash
./sdk/windows/scripts/build-rust-windows.sh
```

Requires Rust MSVC targets: `x86_64-pc-windows-msvc`, `aarch64-pc-windows-msvc`.

**macOS host (smoke tests only, no DLL cross-compile):**

```bash
./sdk/windows/scripts/build-rust-windows.sh --host-dylib
export HEATMAP_FFI_LIBRARY_PATH="$PWD/target/release/libheatmap_ffi.dylib"
```

## dotnet commands

From `sdk/windows/` ( [.NET 8 SDK](https://dotnet.microsoft.com/download/dotnet/8.0) ):

```bash
# Unit tests (no native lib required)
dotnet test tests/Heatmap.Sdk.Tests/Heatmap.Sdk.Tests.csproj

# Pack pre-release NuGet
dotnet pack src/Heatmap.Sdk/Heatmap.Sdk.csproj -c Release -o artifacts

# Consumer smoke (restore uses ./artifacts via nuget.config)
dotnet restore consumer-smoke-test/Heatmap.Sdk.ConsumerSmokeTest.csproj
dotnet test consumer-smoke-test/Heatmap.Sdk.ConsumerSmokeTest.csproj

# All of the above
./scripts/dotnet-test.sh
```

Install the package locally:

```bash
dotnet add package Heatmap.Sdk --version 0.1.0-pre1 --source ./artifacts
```

## Module layout

| Project | Role |
|---------|------|
| `src/Heatmap.Sdk` | NuGet: `HeatmapClient`, P/Invoke, packaged `runtimes/` |
| `tests/Heatmap.Sdk.Tests` | Public API contract tests (no native lib) |
| `consumer-smoke-test` | PackageReference to packed `Heatmap.Sdk`; FFI smoke when native lib present |

## Gaps / CI follow-ups

- `runtimes/**/heatmap_ffi.dll` is empty until `build-rust-windows` runs on Windows.
- macOS/Linux dev runs consumer smoke against host `libheatmap_ffi.dylib` via `HEATMAP_FFI_LIBRARY_PATH`; production integrators load the DLL from the NuGet RID folder on Windows.
- NuGet.org publish, signing, and WPF capture (Step 37) are out of scope for this step.
