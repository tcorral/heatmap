#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SWIFT_OUT="$ROOT/sdk/apple/shared/Sources/HeatmapFFI"
C_OUT="$ROOT/sdk/apple/shared/Sources/heatmap_ffiFFI/include"
TMP_OUT="$(mktemp -d)"
BINDGEN_DIR="$ROOT/sdk/apple/scripts/bindgen"
LIB="$ROOT/target/debug/libheatmap_ffi.dylib"

cd "$ROOT"
cargo build -p heatmap-ffi

if [[ ! -f "$LIB" ]]; then
  echo "error: expected $LIB after cargo build" >&2
  exit 1
fi

mkdir -p "$SWIFT_OUT" "$C_OUT"
cargo run --manifest-path "$BINDGEN_DIR/Cargo.toml" -- \
  generate --library \
  --config "$ROOT/core/heatmap-ffi/uniffi.toml" \
  --crate heatmap_ffi \
  --language swift \
  --out-dir "$TMP_OUT" \
  "$LIB"

mv "$TMP_OUT/heatmap_ffi.swift" "$SWIFT_OUT/"
mv "$TMP_OUT/heatmap_ffiFFI.h" "$C_OUT/"
mv "$TMP_OUT/heatmap_ffiFFI.modulemap" "$C_OUT/module.modulemap"
rm -rf "$TMP_OUT"

echo "Wrote Swift bindings to $SWIFT_OUT and C headers to $C_OUT"
