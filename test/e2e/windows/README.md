# Windows E2E

FlaUI checkout journey against the WPF POC with ingest port **18084** (web **18080**, iOS **18081**, macOS **18082**, Android **18083**).

## Run (Windows only)

```powershell
npm run test:windows --prefix test/e2e
```

Or:

```powershell
pwsh test/e2e/scripts/run-windows-e2e.ps1
```

## Runner

1. Builds `heatmap_ffi.dll` via `sdk/windows/scripts/build-rust-windows.ps1`
2. Starts `heatmap-backend` on **18084** with `HEATMAP_EXPOSE_TEST_EVENTS=1` and `heatmap-windows-e2e.db`
3. Builds `poc/windows/HeatmapCheckout` (Debug)
4. Runs `HeatmapCheckout.E2ETests` (FlaUI + `GET /v1/events`)

## macOS / Linux host

Execution requires Windows (WPF app, FlaUI, native DLL). Cross-host CI can compile tests only:

```bash
dotnet build test/e2e/windows/HeatmapCheckout.E2ETests/HeatmapCheckout.E2ETests.csproj \
  -p:EnableWindowsTargeting=true
```

`npm run test:windows` from `test/e2e/scripts/run-windows-e2e.sh` exits with instructions (does not run FlaUI).

## Coverage

| Test | Asserts |
|------|---------|
| Checkout journey | Shared sequence via ingest; 0–1 click coords; single session |
| Resize | Post-resize click normalized; viewport reflects smaller window |
| UI scale (`HEATMAP_E2E_UI_SCALE`) | Clicks stay in 0–1 window space |
| Auxiliary window | `window_id` differs from main window |
| Ingest down | Journey completes; confirmation UI remains |

Optional POC hooks: `HEATMAP_E2E_WINDOW_CAPTURE=1` records identified activations (macOS/iOS parity). Default E2E uses WPF capture + FlaUI.

E2E endpoint default: `poc/windows/Config/HeatmapE2E.heatmap.json` (**18084**); runner sets `HEATMAP_ENDPOINT` at launch.
