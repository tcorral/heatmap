#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
KOTLIN_OUT="$ROOT/sdk/android/heatmap-android/src/main/kotlin"
BINDGEN_DIR="$ROOT/sdk/apple/scripts/bindgen"

usage() {
  cat <<'EOF'
Generate UniFFI Kotlin bindings for Android from a built libheatmap_ffi.

Usage:
  generate-uniffi.sh [--host-dylib | --android-so DIR]

  --host-dylib     Use target/debug/libheatmap_ffi.dylib (macOS dev bindgen only).
  --android-so DIR Use libheatmap_ffi.so from an Android cargo build (CI / release).

After Kotlin generation, copy .so files into:
  sdk/android/heatmap-android/src/main/jniLibs/<abi>/

See scripts/build-rust-android.sh for Android target builds.
EOF
}

MODE="host"
ANDROID_LIB=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --host-dylib) MODE="host"; shift ;;
    --android-so)
      MODE="android"
      ANDROID_LIB="${2:?missing path after --android-so}"
      shift 2
      ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown arg: $1" >&2; usage; exit 1 ;;
  esac
done

cd "$ROOT"

case "$MODE" in
  host)
    cargo build -p heatmap-ffi
    LIB="$ROOT/target/debug/libheatmap_ffi.dylib"
    ;;
  android)
    LIB="$ANDROID_LIB"
    ;;
esac

if [[ ! -f "$LIB" ]]; then
  echo "error: library not found: $LIB" >&2
  exit 1
fi

mkdir -p "$KOTLIN_OUT"

cargo run --manifest-path "$BINDGEN_DIR/Cargo.toml" -- \
  generate --library \
  --config "$ROOT/core/heatmap-ffi/uniffi.toml" \
  --crate heatmap_ffi \
  --language kotlin \
  --out-dir "$KOTLIN_OUT" \
  "$LIB"

echo "Wrote Kotlin bindings under $KOTLIN_OUT/com/heatmap/sdk/heatmap_ffi.kt"
