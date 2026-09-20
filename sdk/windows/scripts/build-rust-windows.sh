#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
RUNTIMES="$ROOT/sdk/windows/runtimes"
HOST_DYLIB=0

usage() {
  cat <<'EOF'
Build heatmap_ffi native libraries for the Windows SDK package layout.

Usage:
  build-rust-windows.sh                 # Windows CI: MSVC x64 + arm64 → runtimes/
  build-rust-windows.sh --host-dylib    # macOS/Linux dev: host release dylib/so for smoke tests

Environment:
  HEATMAP_FFI_COPY_ONLY=1  Skip cargo build; only copy from existing target/ outputs.

Windows (GitHub Actions / dev machine):
  rustup target add x86_64-pc-windows-msvc aarch64-pc-windows-msvc
  cargo build -p heatmap-ffi --target x86_64-pc-windows-msvc --release
  cargo build -p heatmap-ffi --target aarch64-pc-windows-msvc --release

macOS host cannot cross-compile MSVC DLLs without a Windows cross toolchain; use --host-dylib
for dotnet smoke tests against libheatmap_ffi.dylib via HEATMAP_FFI_LIBRARY_PATH.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --host-dylib) HOST_DYLIB=1; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown arg: $1" >&2; usage >&2; exit 1 ;;
  esac
done

cd "$ROOT"

if [[ "$HOST_DYLIB" -eq 1 ]]; then
  if [[ "${HEATMAP_FFI_COPY_ONLY:-}" != "1" ]]; then
    cargo build -p heatmap-ffi --release
  fi
  case "$(uname -s)" in
    Darwin)
      echo "Host dylib: $ROOT/target/release/libheatmap_ffi.dylib"
      echo "Export HEATMAP_FFI_LIBRARY_PATH=$ROOT/target/release/libheatmap_ffi.dylib for dotnet test"
      ;;
    Linux)
      echo "Host so: $ROOT/target/release/libheatmap_ffi.so"
      echo "Export HEATMAP_FFI_LIBRARY_PATH=$ROOT/target/release/libheatmap_ffi.so for dotnet test"
      ;;
    *)
      echo "Use default Windows targets without --host-dylib on this host." >&2
      exit 1
      ;;
  esac
  exit 0
fi

if [[ "$(uname -s)" != MINGW* ]] && [[ "$(uname -s)" != CYGWIN* ]] && [[ "${OS:-}" != Windows_NT ]]; then
  echo "error: MSVC Windows DLL build requires a Windows host or use --host-dylib for local smoke tests" >&2
  exit 1
fi

declare -A TARGET_MAP=(
  ["x86_64-pc-windows-msvc"]="win-x64"
  ["aarch64-pc-windows-msvc"]="win-arm64"
)

for triple in "${!TARGET_MAP[@]}"; do
  rid="${TARGET_MAP[$triple]}"
  if [[ "${HEATMAP_FFI_COPY_ONLY:-}" != "1" ]]; then
    rustup target add "$triple" 2>/dev/null || true
    cargo build -p heatmap-ffi --target "$triple" --release
  fi
  dest="$RUNTIMES/$rid/native"
  mkdir -p "$dest"
  cp "$ROOT/target/$triple/release/heatmap_ffi.dll" "$dest/"
  echo "Installed heatmap_ffi.dll → $dest"
done
