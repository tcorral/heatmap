# End-to-end tests

## Web (Playwright)

```bash
cd test/e2e
npm install
npm test
```

Backend default port: `18080`. See `playwright.config.ts`.

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
