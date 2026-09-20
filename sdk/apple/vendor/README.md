# Pre-release Heatmap FFI XCFramework

`HeatmapFFI.xcframework` is the pre-release native core for SwiftPM consumers.

Slices:

- `ios-arm64` — iOS device
- `ios-arm64_x86_64-simulator` — iOS Simulator
- `macos-arm64_x86_64` — macOS (Apple Silicon and Intel)

A clean consumer adds `sdk/apple` as a Swift package. They do not build Rust or set linker search paths.

Rebuild after Rust FFI changes:

```bash
./sdk/apple/scripts/build-xcframework.sh
```
