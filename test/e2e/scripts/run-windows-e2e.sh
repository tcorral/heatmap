#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

case "$(uname -s)" in
  MINGW* | MSYS* | CYGWIN*)
    exec pwsh -NoProfile -File "${SCRIPT_DIR}/run-windows-e2e.ps1"
    ;;
  Darwin | Linux)
    echo "Windows E2E runs only on Windows (FlaUI + WPF + heatmap_ffi.dll)." >&2
    echo "Compile test project on this host:" >&2
    echo "  dotnet build test/e2e/windows/HeatmapCheckout.E2ETests/HeatmapCheckout.E2ETests.csproj -p:EnableWindowsTargeting=true" >&2
    exit 1
    ;;
  *)
    if command -v pwsh >/dev/null 2>&1 && [[ "${OS:-}" == "Windows_NT" ]]; then
      exec pwsh -NoProfile -File "${SCRIPT_DIR}/run-windows-e2e.ps1"
    fi
    echo "Windows E2E is not supported on this platform." >&2
    exit 1
    ;;
esac
