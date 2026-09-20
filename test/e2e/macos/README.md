# macOS E2E

XCUITest checkout journey against the macOS POC with ingest port **18082** (web **18080**, iOS **18081**).

## Run

```bash
chmod +x test/e2e/scripts/run-macos-e2e.sh
npm run test:macos --prefix test/e2e
```

## Runner

1. Builds `heatmap-ffi` for host macOS (`target/debug`)
2. Starts `heatmap-backend` on **18082** with `HEATMAP_EXPOSE_TEST_EVENTS=1` and `heatmap-macos-e2e.db`
3. Runs `HeatmapCheckoutUITests` with `poc/macos/Config/HeatmapE2E.xcconfig`
4. Writes `.xcresult` under `test/e2e/macos/results/` (gitignored)
