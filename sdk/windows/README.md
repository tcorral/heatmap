# Heatmap Windows SDK (Step 36)

C# public API over the stable C ABI in `core/heatmap-ffi` (`heatmap_sdk_*`), packaged as pre-release NuGet packages **Heatmap.Sdk**, **Heatmap.Sdk.Capture**, and **Heatmap.Sdk.Wpf**.

## Public API

Use `Heatmap.Sdk.HeatmapClient` only:

- `HeatmapClient(configJson)` / `HeatmapClient(projectKey, endpoint)`
- `Identify`, `SignOut`, `Screen`, `SetTrackingEnabled`, `Flush`

There is no public `Track` API. V1 wire activations use type `click` at the adapter boundary (`WireActivationType.Click` in `Heatmap.Sdk.Internal`); WPF capture ships in **Heatmap.Sdk.Wpf**.

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

# Pack pre-release NuGet (all three packages; script verifies artifacts/*.nupkg)
./scripts/dotnet-test.sh
# Or pack individually:
dotnet pack src/Heatmap.Sdk/Heatmap.Sdk.csproj -c Release -o artifacts
dotnet pack src/Heatmap.Sdk.Capture/Heatmap.Sdk.Capture.csproj -c Release -o artifacts
dotnet pack src/Heatmap.Sdk.Wpf/Heatmap.Sdk.Wpf.csproj -c Release -o artifacts -p:EnableWindowsTargeting=true

# Consumer smoke (restore uses ./artifacts via nuget.config)
dotnet restore consumer-smoke-test/Heatmap.Sdk.ConsumerSmokeTest.csproj
dotnet test consumer-smoke-test/Heatmap.Sdk.ConsumerSmokeTest.csproj

# All of the above
./scripts/dotnet-test.sh
```

Install packages locally:

```bash
# WPF apps (capture + session + native runtimes)
dotnet add package Heatmap.Sdk.Wpf --version 0.1.0-pre1 --source ./artifacts

# Session API only (no WPF capture types)
dotnet add package Heatmap.Sdk --version 0.1.0-pre1 --source ./artifacts
```

## Module layout

| Project / package | Role |
|-------------------|------|
| `src/Heatmap.Sdk` → **Heatmap.Sdk** | NuGet: `HeatmapClient`, P/Invoke, packaged `runtimes/` |
| `src/Heatmap.Sdk.Capture` → **Heatmap.Sdk.Capture** | NuGet: shared capture contracts (dependency of Wpf) |
| `src/Heatmap.Sdk.Wpf` → **Heatmap.Sdk.Wpf** | NuGet: WPF capture (`HeatmapWpfCapture`, `HeatmapWpfRustCaptureBackend`) |
| `tests/Heatmap.Sdk.Tests` | Public API contract tests (no native lib) |
| `consumer-smoke-test` | PackageReference to packed `Heatmap.Sdk`; FFI smoke when native lib present |

## Gaps / CI follow-ups

- **Heatmap.Sdk.Wpf** `dotnet pack` requires **Microsoft.NET.Sdk.WindowsDesktop** (full Windows .NET SDK). Homebrew/macOS/Linux `dotnet-test.sh` packs **Heatmap.Sdk** and **Heatmap.Sdk.Capture** and skips Wpf with a message; Windows CI should assert all three `.nupkg` files.
- `runtimes/**/heatmap_ffi.dll` is empty until `build-rust-windows` runs on Windows.
- macOS/Linux dev runs consumer smoke against host `libheatmap_ffi.dylib` via `HEATMAP_FFI_LIBRARY_PATH`; production integrators load the DLL from the NuGet RID folder on Windows.
- NuGet.org publish and package signing are out of scope for this step.
