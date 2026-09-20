# Pre-release Heatmap FFI binaries

These dylibs are the pre-release native core for SwiftPM consumers.

- `macos/libheatmap_ffi.dylib` — macOS host
- `ios-sim/libheatmap_ffi.dylib` — iOS Simulator (Apple Silicon)

Rebuild and replace after Rust FFI changes:

```bash
# Repo root
cargo build -p heatmap-ffi
cargo build -p heatmap-ffi --target aarch64-apple-ios-sim
cp target/debug/libheatmap_ffi.dylib sdk/apple/vendor/macos/
cp target/aarch64-apple-ios-sim/debug/libheatmap_ffi.dylib sdk/apple/vendor/ios-sim/
install_name_tool -id @rpath/libheatmap_ffi.dylib sdk/apple/vendor/macos/libheatmap_ffi.dylib
install_name_tool -id @rpath/libheatmap_ffi.dylib sdk/apple/vendor/ios-sim/libheatmap_ffi.dylib
codesign -s - --force sdk/apple/vendor/macos/libheatmap_ffi.dylib
codesign -s - --force sdk/apple/vendor/ios-sim/libheatmap_ffi.dylib
```

A clean consumer adds `sdk/apple` as a Swift package and does not build Rust or set `target/debug` search paths.
