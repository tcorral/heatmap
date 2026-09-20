# Heatmap Apple shared SDK

Swift package (`sdk/apple/`) exposing `HeatmapClient` over UniFFI-generated `HeatmapSdk`.

## Regenerate UniFFI Swift bindings

Requires Rust **1.85+**, `uniffi` **0.28.3** (via the local bindgen helper), and a built `heatmap-ffi` cdylib.

```bash
# From repo root
./sdk/apple/scripts/generate-uniffi.sh
```

Manual equivalent:

```bash
cargo build -p heatmap-ffi
cargo run --manifest-path sdk/apple/scripts/bindgen/Cargo.toml -- \
  generate --library \
  --config core/heatmap-ffi/uniffi.toml \
  --crate heatmap_ffi \
  --language swift \
  --out-dir sdk/apple/shared/Sources/HeatmapFFI \
  target/debug/libheatmap_ffi.dylib
```

Generated output is split for SwiftPM:

- `Sources/HeatmapFFI/heatmap_ffi.swift`
- `Sources/heatmap_ffiFFI/include/heatmap_ffiFFI.h` and `module.modulemap`

Check them in after API changes on the Rust side.

The bindgen helper pins `textwrap` for Rust 1.85; see `sdk/apple/scripts/bindgen/Cargo.lock`.

## Build and test (macOS)

Link against the debug dylib at repo `target/debug/`:

```bash
cargo build -p heatmap-ffi
cd sdk/apple && swift test
```

iOS Simulator (Apple Silicon):

```bash
rustup target add aarch64-apple-ios-sim
cargo build -p heatmap-ffi --target aarch64-apple-ios-sim
```

The Swift package links `target/aarch64-apple-ios-sim/debug` when building for iOS. See [`poc/ios/README.md`](../../../poc/ios/README.md).
