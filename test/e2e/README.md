# End-to-end tests

## Web (Playwright)

```bash
cd test/e2e
npm install
npm test
```

Backend default port: `18080`. See `playwright.config.ts`.

### Cross-platform dashboard acceptance

`tests/five-platform-dashboard-acceptance.spec.ts` is the macOS CI-friendly acceptance proof:

- Fresh backend DB with `HEATMAP_EXPOSE_TEST_EVENTS=1` (see `playwright.config.ts` webServer).
- Live **web** checkout journey (same flow as `web-checkout-journey.spec.ts`).
- **Seeded ingest** for iOS, Android, macOS, and Windows via `POST /v1/batches` using `test/fixtures/click-checkout-pay-*.json` (native SDK E2E is not required in the same job).
- Optional checkout still at `poc/stills/checkout-layout-2024-09.png`, registered with `POST /v1/stills` when the overlay panel is asserted.

```bash
npm run test:acceptance
```

Per-platform install → journey → upload privacy and failure coverage remains in `test:ios`, `test:android`, `test:macos`, and `test:windows` (or platform README runners).

## Privacy gate

Unified runner (web Playwright privacy E2E + aggregated fixture scan):

```bash
../../scripts/run-privacy-gate.sh
```

Or run pieces separately:

```bash
npx playwright test tests/web-failure-privacy.spec.ts
cd ../test && npm run test:privacy
```

`test/privacy/cross-platform-secrets.test.mjs` scans Pay goldens, other checked-in fixtures, and synthesized per-platform Pay payloads for forbidden secret literals and excluded element ids. iOS, Android, macOS, and Windows native secret-field behaviour is additionally covered by platform E2E runners and adapter unit tests (XCTest, Gradle, dotnet).

## iOS (XCUITest)

See [`ios/README.md`](./ios/README.md).

```bash
npm run test:ios
```

(from this directory)

## macOS (XCUITest)

See [`macos/README.md`](./macos/README.md).

```bash
npm run test:macos
```

## Android (instrumentation)

See [`android/README.md`](./android/README.md).

```bash
npm run test:android
```

## Windows (FlaUI)

See [`windows/README.md`](./windows/README.md).

```bash
npm run test:windows
```

(from this directory; executes on Windows only)
