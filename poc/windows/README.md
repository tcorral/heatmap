# Windows checkout POC (`HeatmapCheckout`)

WPF sample app for the shared checkout journey. It links the local projects under `sdk/windows` and exercises `HeatmapClient`, `HeatmapWpfRustCaptureBackend`, and `HeatmapWpfCapture`.

## Prerequisites

- Windows 10/11
- [.NET 8 SDK](https://dotnet.microsoft.com/download/dotnet/8.0)
- Rust toolchain with MSVC targets (`x86_64-pc-windows-msvc`, `aarch64-pc-windows-msvc`)
- Ingest API reachable at the configured endpoint (default `http://127.0.0.1:8080` on the same machine)

Build the native library:

```powershell
# From repo root
.\sdk\windows\scripts\build-rust-windows.ps1
```

## Configuration

Defaults live in `Config/heatmap.json` and are copied next to the app at build time:

| Key | Default |
|-----|---------|
| `ProjectKey` | `test-project-key` |
| `Endpoint` | `http://127.0.0.1:8080` |

On Windows desktop, loopback ingest on the host uses `127.0.0.1:8080` directly (unlike the Android emulator, which uses `10.0.2.2`).

Override at runtime with environment variables `HEATMAP_PROJECT_KEY` and `HEATMAP_ENDPOINT`, or edit `Config/heatmap.json` before building.

## Build

```powershell
cd poc\windows
dotnet build HeatmapCheckout.sln -c Debug
```

Run:

```powershell
dotnet run --project HeatmapCheckout\HeatmapCheckout.csproj -c Debug
```

Or open `HeatmapCheckout.sln` in Visual Studio 2022 and run **HeatmapCheckout**.

## Shared journey

Matches `poc/react`, `poc/ios`, `poc/macos`, and `poc/android`:

1. `session_start` — first capture when the main window loads
2. `screen_view(Home)` — Home appears
3. Click **Activate product** (`AutomationId`: `product`)
4. `screen_view(Checkout)` — layout version `2024-09`
5. Scroll checkout content (automatic via `HeatmapWpfCapture` on `ScrollViewer`)
6. Click **Apply coupon** (`checkout.coupon`)
7. Click **Pay** (`checkout.pay`)
8. `screen_view(Confirmation)` then `HeatmapClient.Flush()` (HTTP upload via Rust `HttpBatchTransport`)

The checkout screen includes a `PasswordBox` (events from that control are dropped by the SDK).

## Tests

```powershell
dotnet test HeatmapCheckout.sln -c Debug
```

`CheckoutPayIdentifierTests` asserts `CheckoutIdentifiers.PayButton == "checkout.pay"`.

Windows checkout E2E (FlaUI + ingest **18084**): [`test/e2e/windows/README.md`](../../test/e2e/windows/README.md).

## Docs

Integrator reference: [`docs/windows-sdk.md`](../../docs/windows-sdk.md).
