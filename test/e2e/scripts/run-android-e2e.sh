#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
E2E_BACKEND_PORT="${E2E_ANDROID_BACKEND_PORT:-18083}"
E2E_DB="${REPO_ROOT}/heatmap-android-e2e.db"
ANDROID_DIR="${REPO_ROOT}/poc/android"
BACKEND_PID=""

cleanup() {
  if [[ -n "${BACKEND_PID}" ]] && kill -0 "${BACKEND_PID}" 2>/dev/null; then
    kill "${BACKEND_PID}" 2>/dev/null || true
    wait "${BACKEND_PID}" 2>/dev/null || true
  fi
}
trap cleanup EXIT

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

wait_for_backend() {
  local url="http://127.0.0.1:${E2E_BACKEND_PORT}/v1/elements?platform=web&screen=Checkout&app_version=1.0.0"
  for _ in $(seq 1 120); do
    if curl -sf "${url}" >/dev/null; then
      return 0
    fi
    sleep 0.5
  done
  echo "Backend did not become ready on port ${E2E_BACKEND_PORT}" >&2
  exit 1
}

require_cmd curl
require_cmd adb

if [[ -z "${ANDROID_NDK_HOME:-}" ]]; then
  echo "error: set ANDROID_NDK_HOME to your NDK root (needed for libheatmap_ffi.so)" >&2
  exit 1
fi

if [[ -z "${ANDROID_HOME:-}" && ! -f "${ANDROID_DIR}/local.properties" ]]; then
  echo "error: set ANDROID_HOME or sdk.dir in poc/android/local.properties" >&2
  exit 1
fi

device_line="$(adb devices | awk 'NR>1 && $2=="device" { print $1; exit }')"
if [[ -z "${device_line}" ]]; then
  echo "No connected Android device/emulator (adb devices)." >&2
  exit 1
fi

echo "Building heatmap-ffi for Android (x86_64 + arm64-v8a)…"
(
  cd "${REPO_ROOT}"
  export ANDROID_NDK_HOME
  rustup target add x86_64-linux-android aarch64-linux-android 2>/dev/null || true
  cargo build -p heatmap-ffi --target x86_64-linux-android --release
  cargo build -p heatmap-ffi --target aarch64-linux-android --release
  mkdir -p sdk/android/heatmap-android/src/main/jniLibs/x86_64
  mkdir -p sdk/android/heatmap-android/src/main/jniLibs/arm64-v8a
  cp target/x86_64-linux-android/release/libheatmap_ffi.so \
    sdk/android/heatmap-android/src/main/jniLibs/x86_64/
  cp target/aarch64-linux-android/release/libheatmap_ffi.so \
    sdk/android/heatmap-android/src/main/jniLibs/arm64-v8a/
)

echo "adb reverse tcp:${E2E_BACKEND_PORT} tcp:${E2E_BACKEND_PORT}"
adb reverse "tcp:${E2E_BACKEND_PORT}" "tcp:${E2E_BACKEND_PORT}"

rm -f "${E2E_DB}"
echo "Starting backend on port ${E2E_BACKEND_PORT}…"
(
  cd "${REPO_ROOT}"
  PORT="${E2E_BACKEND_PORT}" \
    HEATMAP_EXPOSE_TEST_EVENTS=1 \
    DATABASE_PATH="${E2E_DB}" \
    cargo run -p heatmap-backend
) &
BACKEND_PID=$!
wait_for_backend

HEATMAP_ENDPOINT="http://127.0.0.1:${E2E_BACKEND_PORT}"
echo "Running connected Android E2E (endpoint ${HEATMAP_ENDPOINT})…"
(
  cd "${ANDROID_DIR}"
  ./gradlew :app:connectedDebugAndroidTest \
    -PheatmapEndpoint="${HEATMAP_ENDPOINT}" \
    -PheatmapE2eBackendPort="${E2E_BACKEND_PORT}"
)

echo "Android E2E finished."
