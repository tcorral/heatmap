#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

cd "${ROOT}/test/e2e"
if [[ ! -d node_modules/@playwright/test ]]; then
  npm install
fi
npx playwright test tests/web-failure-privacy.spec.ts

cd "${ROOT}/test"
if [[ ! -d node_modules/ajv ]]; then
  npm ci --omit=dev 2>/dev/null || npm install --omit=dev
fi
node --test privacy/cross-platform-secrets.test.mjs
