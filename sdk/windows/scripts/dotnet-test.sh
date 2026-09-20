#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export DOTNET_ROOT="${DOTNET_ROOT:-/opt/homebrew/opt/dotnet@8/libexec}"
export PATH="$DOTNET_ROOT:$PATH"

cd "$ROOT"

mkdir -p artifacts
dotnet pack src/Heatmap.Sdk/Heatmap.Sdk.csproj -c Release -o artifacts

dotnet test tests/Heatmap.Sdk.Tests/Heatmap.Sdk.Tests.csproj -c Release

dotnet restore consumer-smoke-test/Heatmap.Sdk.ConsumerSmokeTest.csproj
dotnet test consumer-smoke-test/Heatmap.Sdk.ConsumerSmokeTest.csproj -c Release --no-restore
