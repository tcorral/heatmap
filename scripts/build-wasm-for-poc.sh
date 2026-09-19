#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/poc/react/src/wasm/pkg"
WASM="$ROOT/target/wasm32-unknown-unknown/release/heatmap_wasm.wasm"

mkdir -p "$OUT"
cd "$ROOT"
cargo build -p heatmap-wasm --release --target wasm32-unknown-unknown
wasm-bindgen "$WASM" --out-dir "$OUT" --target web --no-typescript
