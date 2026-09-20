#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
JNI_OUT="$ROOT/sdk/android/heatmap-android/src/main/jniLibs"

# Map Rust triple → Android ABI folder name.
declare -A ABI_MAP=(
  ["aarch64-linux-android"]="arm64-v8a"
  ["armv7-linux-androideabi"]="armeabi-v7a"
  ["x86_64-linux-android"]="x86_64"
)

TARGETS=(
  aarch64-linux-android
  armv7-linux-androideabi
  x86_64-linux-android
)

if [[ -z "${ANDROID_NDK_HOME:-}" ]]; then
  echo "error: set ANDROID_NDK_HOME to your NDK root" >&2
  exit 1
fi

export PATH="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)/bin:$PATH"

cd "$ROOT"

for triple in "${TARGETS[@]}"; do
  rustup target add "$triple" 2>/dev/null || true
  cargo build -p heatmap-ffi --target "$triple" --release
  abi="${ABI_MAP[$triple]}"
  dest="$JNI_OUT/$abi"
  mkdir -p "$dest"
  cp "$ROOT/target/$triple/release/libheatmap_ffi.so" "$dest/"
  echo "Installed libheatmap_ffi.so → $dest"
done

echo "Run scripts/generate-uniffi.sh --android-so target/aarch64-linux-android/release/libheatmap_ffi.so to refresh Kotlin if the FFI surface changed."
