#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export DOTNET_ROOT="${DOTNET_ROOT:-/opt/homebrew/opt/dotnet@8/libexec}"
export PATH="$DOTNET_ROOT:$PATH"

cd "$ROOT"

mkdir -p artifacts
dotnet pack src/Heatmap.Sdk/Heatmap.Sdk.csproj -c Release -o artifacts
dotnet pack src/Heatmap.Sdk.Capture/Heatmap.Sdk.Capture.csproj -c Release -o artifacts

# WPF targets net8.0-windows; EnableWindowsTargeting allows restore on non-Windows hosts.
# Pack/build still requires Microsoft.NET.Sdk.WindowsDesktop (Windows SDK or full Windows .NET SDK).
WPF_PACKED=0
if dotnet pack src/Heatmap.Sdk.Wpf/Heatmap.Sdk.Wpf.csproj -c Release -o artifacts -p:EnableWindowsTargeting=true; then
  WPF_PACKED=1
else
  case "$(uname -s)" in
    MINGW* | MSYS* | CYGWIN* | *_NT*)
      echo "Heatmap.Sdk.Wpf pack failed on Windows host." >&2
      exit 1
      ;;
    *)
      echo "Heatmap.Sdk.Wpf pack skipped: Microsoft.NET.Sdk.WindowsDesktop not available on this host (expected on macOS/Linux). Build/pack Wpf on Windows CI or a Windows dev machine." >&2
      ;;
  esac
fi

for nupkg in \
  "Heatmap.Sdk.0.1.0-pre1.nupkg" \
  "Heatmap.Sdk.Capture.0.1.0-pre1.nupkg"; do
  if [[ ! -f "artifacts/${nupkg}" ]]; then
    echo "Expected NuGet package missing: artifacts/${nupkg}" >&2
    exit 1
  fi
done

if [[ "$WPF_PACKED" -eq 1 ]]; then
  if [[ ! -f "artifacts/Heatmap.Sdk.Wpf.0.1.0-pre1.nupkg" ]]; then
    echo "Expected NuGet package missing: artifacts/Heatmap.Sdk.Wpf.0.1.0-pre1.nupkg" >&2
    exit 1
  fi
fi

dotnet test tests/Heatmap.Sdk.Tests/Heatmap.Sdk.Tests.csproj -c Release

dotnet restore consumer-smoke-test/Heatmap.Sdk.ConsumerSmokeTest.csproj
dotnet test consumer-smoke-test/Heatmap.Sdk.ConsumerSmokeTest.csproj -c Release --no-restore
