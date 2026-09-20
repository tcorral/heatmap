#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
JNI_OUT="$ROOT/sdk/android/heatmap-android/src/main/jniLibs"

abi_for_triple() {
  case "$1" in
    aarch64-linux-android) echo "arm64-v8a" ;;
    armv7-linux-androideabi) echo "armeabi-v7a" ;;
    x86_64-linux-android) echo "x86_64" ;;
    *)
      echo "error: unknown rust android triple $1" >&2
      exit 1
      ;;
  esac
}

TARGETS=(
  "aarch64-linux-android"
  "armv7-linux-androideabi"
  "x86_64-linux-android"
)

if [[ -z "${ANDROID_NDK_HOME:-}" ]]; then
  echo "error: set ANDROID_NDK_HOME to your NDK root" >&2
  exit 1
fi

PREBUILT_DIR=""
for candidate in "$ANDROID_NDK_HOME"/toolchains/llvm/prebuilt/*; do
  if [[ -d "$candidate/bin" ]]; then
    PREBUILT_DIR="$candidate"
    break
  fi
done
if [[ -z "$PREBUILT_DIR" ]]; then
  echo "error: no NDK llvm prebuilt toolchain under $ANDROID_NDK_HOME" >&2
  exit 1
fi

LLVM_BIN="$PREBUILT_DIR/bin"
export PATH="$LLVM_BIN:$PATH"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$LLVM_BIN/aarch64-linux-android24-clang"
export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="$LLVM_BIN/armv7a-linux-androideabi24-clang"
export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="$LLVM_BIN/x86_64-linux-android24-clang"
export CC_aarch64_linux_android="$LLVM_BIN/aarch64-linux-android24-clang"
export CC_armv7_linux_androideabi="$LLVM_BIN/armv7a-linux-androideabi24-clang"
export CC_x86_64_linux_android="$LLVM_BIN/x86_64-linux-android24-clang"
export AR_aarch64_linux_android="$LLVM_BIN/llvm-ar"
export AR_armv7_linux_androideabi="$LLVM_BIN/llvm-ar"
export AR_x86_64_linux_android="$LLVM_BIN/llvm-ar"
export AR="$LLVM_BIN/llvm-ar"

cd "$ROOT"

for triple in "${TARGETS[@]}"; do
  rustup target add "$triple" 2>/dev/null || true
  cargo build -p heatmap-ffi --target "$triple" --release
  abi="$(abi_for_triple "$triple")"
  dest="$JNI_OUT/$abi"
  mkdir -p "$dest"
  cp "$ROOT/target/$triple/release/libheatmap_ffi.so" "$dest/"
  echo "Installed libheatmap_ffi.so → $dest"
done

echo "Run scripts/generate-uniffi.sh --android-so target/aarch64-linux-android/release/libheatmap_ffi.so to refresh Kotlin if the FFI surface changed."
