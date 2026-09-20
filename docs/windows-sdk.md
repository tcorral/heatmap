# Windows Heatmap SDK (`Heatmap.Sdk` + `Heatmap.Sdk.Wpf`)

Pre-release WPF capture adapter over the stable C ABI Rust core (`heatmap_ffi`). It records clicks and scrolls, resolves stable control ids from `AutomationProperties.AutomationId`, attaches an opaque `window_id` per `Window`, and drops `PasswordBox` interactions before events reach core.

## Install

1. Build `heatmap_ffi` for Windows:

```powershell
# Repo root
.\sdk\windows\scripts\build-rust-windows.ps1
```

2. Reference the local projects (POC layout) or the pre-release NuGet after packing:

```powershell
cd sdk\windows
dotnet pack src\Heatmap.Sdk\Heatmap.Sdk.csproj -c Release -o artifacts
```

```xml
<!-- App csproj -->
<ProjectReference Include="path\to\sdk\windows\src\Heatmap.Sdk\Heatmap.Sdk.csproj" />
<ProjectReference Include="path\to\sdk\windows\src\Heatmap.Sdk.Wpf\Heatmap.Sdk.Wpf.csproj" />
```

NuGet install (when using the package):

```powershell
dotnet add package Heatmap.Sdk --version 0.1.0-pre1 --source .\artifacts
```

The SDK copies `heatmap_ffi.dll` from `runtimes/win-x64/native/` (or `win-arm64`) into the app output. DLLs are produced only after `build-rust-windows` runs on Windows.

## Init

Use one `HeatmapClient` instance for both the app client and the capture backend.

```csharp
using Heatmap.Sdk;
using Heatmap.Sdk.Wpf;

using var client = new HeatmapClient("your-project-key", "https://heatmap.example.com");
var backend = new HeatmapWpfRustCaptureBackend(client);

// After the target Window exists:
_ = HeatmapWpfCapture.Install(mainWindow, backend);
```

`HeatmapClient(projectKey, endpoint)` is fine when capture is not wired yet; with capture, share the same `HeatmapClient` with `HeatmapWpfRustCaptureBackend` so session and queue state stay unified.

POC defaults: project key `test-project-key`, endpoint `http://127.0.0.1:8080` (host loopback on Windows desktop).

## Screen

Call when navigation changes:

```csharp
client.Screen("Checkout", "2024-09");
client.Screen("Home", null);
```

Pair screen calls with viewport lifecycle on the backend when you track dimensions explicitly (see sample `HeatmapSession` in `poc/windows`).

## Annotate (`AutomationId`)

Mark controls you want in element totals (XAML or code):

```xml
<Button Content="Pay"
        AutomationProperties.AutomationId="checkout.pay" />
```

```csharp
AutomationProperties.SetAutomationId(payButton, "checkout.pay");
```

The adapter walks from the hit element to ancestors for the nearest non-empty automation id. Unmarked non-secret controls still contribute window-normalized click coordinates only.

## Scroll (automatic)

`HeatmapWpfCapture` attaches to `ScrollViewer` controls under the window content and forwards scroll events. No extra API in the app for checkout-style scrolling.

## Disable

```csharp
client.SetTrackingEnabled(false);
```

Core stops enqueueing; uninstall capture if you need to stop WPF listeners entirely.

## Identify

```csharp
client.Identify("user-123");
```

## Sign-out

```csharp
client.SignOut();
```

## Flush

Upload the in-memory batch via Rust `HttpBatchTransport` (configured `endpoint` + project key):

```csharp
client.Flush();
```

Unlike the web WASM path, you do **not** drain JSON and POST from C# — `Flush()` performs the HTTP upload inside the native core.

## Privacy

`PasswordBox` hits are dropped even when unmarked. The adapter does not read window titles, labels, or typed values to classify controls.

Sample journey: [`poc/windows/README.md`](../poc/windows/README.md).
