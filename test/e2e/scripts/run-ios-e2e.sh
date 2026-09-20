#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
E2E_BACKEND_PORT="${E2E_IOS_BACKEND_PORT:-18081}"
IOS_PROJECT="${REPO_ROOT}/poc/ios/HeatmapCheckout.xcodeproj"
IOS_SCHEME="HeatmapCheckout"
E2E_XCCONFIG="${REPO_ROOT}/poc/ios/Config/HeatmapE2E.xcconfig"
E2E_DB="${REPO_ROOT}/heatmap-ios-e2e.db"
RESULT_BUNDLE="${REPO_ROOT}/test/e2e/ios/results/HeatmapCheckoutUITests.xcresult"
BACKEND_PID=""

cleanup() {
  if [[ -n "${BACKEND_PID}" ]] && kill -0 "${BACKEND_PID}" 2>/dev/null; then
    kill "${BACKEND_PID}" 2>/dev/null || true
    wait "${BACKEND_PID}" 2>/dev/null || true
  fi
}
trap cleanup EXIT

pick_simulator() {
  if [[ -n "${IOS_SIMULATOR_DESTINATION:-}" ]]; then
    echo "${IOS_SIMULATOR_DESTINATION}"
    return
  fi
  local runtime_id device_id
  runtime_id="$(xcrun simctl list runtimes available -j | python3 -c "
import json,sys
r=json.load(sys.stdin).get('runtimes',[])
ios=[x for x in r if x.get('isAvailable') and 'iOS' in x.get('name','')]
ios.sort(key=lambda x: x.get('version',''), reverse=True)
print(ios[0]['identifier'] if ios else '')
")"
  if [[ -z "${runtime_id}" ]]; then
    echo "No available iOS simulator runtime." >&2
    exit 1
  fi
  device_id="$(xcrun simctl list devices available -j | python3 -c "
import json,sys
runtime=sys.argv[1]
data=json.load(sys.stdin)
for devs in data.get('devices',{}).get(runtime,[]):
    if devs.get('isAvailable') and 'iPhone' in devs.get('name',''):
        print(devs['udid'])
        break
" "${runtime_id}")"
  if [[ -z "${device_id}" ]]; then
    echo "No available iPhone simulator." >&2
    exit 1
  fi
  echo "platform=iOS Simulator,id=${device_id}"
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

echo "Building heatmap-ffi for iOS Simulator…"
(
  cd "${REPO_ROOT}"
  rustup target add aarch64-apple-ios-sim 2>/dev/null || true
  cargo build -p heatmap-ffi --target aarch64-apple-ios-sim
)

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

DESTINATION="$(pick_simulator)"
mkdir -p "$(dirname "${RESULT_BUNDLE}")"
rm -rf "${RESULT_BUNDLE}"

echo "Running HeatmapCheckoutUITests on ${DESTINATION}…"
xcodebuild test \
  -project "${IOS_PROJECT}" \
  -scheme "${IOS_SCHEME}" \
  -destination "${DESTINATION}" \
  -configuration Debug \
  -xcconfig "${E2E_XCCONFIG}" \
  -only-testing:HeatmapCheckoutUITests \
  -resultBundlePath "${RESULT_BUNDLE}"

echo "iOS E2E finished. Result bundle: ${RESULT_BUNDLE}"
