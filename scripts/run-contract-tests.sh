#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "${ROOT}/test"

if [[ ! -d node_modules/ajv ]]; then
  npm ci --omit=dev 2>/dev/null || npm install --omit=dev
fi

npm run contract
