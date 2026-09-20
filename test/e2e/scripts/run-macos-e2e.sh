#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
E2E_BACKEND_PORT="${E2E_MACOS_BACKEND_PORT:-18082}"
MACOS_PROJECT="${REPO_ROOT}/poc/macos/HeatmapCheckout.xcodeproj"
MACOS_SCHEME="HeatmapCheckout"
E2E_XCCONFIG="${REPO_ROOT}/poc/macos/Config/HeatmapE2E.xcconfig"
E2E_DB="${REPO_ROOT}/heatmap-macos-e2e.db"
RESULT_BUNDLE="${REPO_ROOT}/test/e2e/macos/results/HeatmapCheckoutUITests.xcresult"
BACKEND_PID=""

cleanup() {
  if [[ -n "${BACKEND_PID}" ]] && kill -0 "${BACKEND_PID}" 2>/dev/null; then
    kill "${BACKEND_PID}" 2>/dev/null || true
    wait "${BACKEND_PID}" 2>/dev/null || true
  fi
}
trap cleanup EXIT

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

echo "Building heatmap-ffi for macOS…"
(
  cd "${REPO_ROOT}"
  cargo build -p heatmap-ffi
)

if existing_pid="$(lsof -tiTCP:"${E2E_BACKEND_PORT}" -sTCP:LISTEN 2>/dev/null || true)"; then
  echo "Port ${E2E_BACKEND_PORT} in use (pid ${existing_pid}); stopping existing listener…"
  kill ${existing_pid} 2>/dev/null || true
  sleep 1
fi

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

mkdir -p "$(dirname "${RESULT_BUNDLE}")"
rm -rf "${RESULT_BUNDLE}"

echo "Running HeatmapCheckoutUITests on macOS…"
xcodebuild test \
  -project "${MACOS_PROJECT}" \
  -scheme "${MACOS_SCHEME}" \
  -destination "platform=macOS" \
  -configuration Debug \
  -xcconfig "${E2E_XCCONFIG}" \
  -only-testing:HeatmapCheckoutUITests \
  -resultBundlePath "${RESULT_BUNDLE}"

echo "macOS E2E finished. Result bundle: ${RESULT_BUNDLE}"
