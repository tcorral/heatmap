# Build heatmap_ffi.dll for win-x64 and win-arm64 into sdk/windows/runtimes/.
# Run from repo root on Windows CI or a Windows dev machine with Rust MSVC targets installed.

$ErrorActionPreference = "Stop"
$Root = Resolve-Path (Join-Path $PSScriptRoot "..\..\..")
$Runtimes = Join-Path $Root "sdk/windows/runtimes"

$Targets = @{
    "x86_64-pc-windows-msvc" = "win-x64"
    "aarch64-pc-windows-msvc"  = "win-arm64"
}

Push-Location $Root
try {
    foreach ($triple in $Targets.Keys) {
        $rid = $Targets[$triple]
        rustup target add $triple 2>$null
        cargo build -p heatmap-ffi --target $triple --release
        $dest = Join-Path $Runtimes "$rid/native"
        New-Item -ItemType Directory -Force -Path $dest | Out-Null
        Copy-Item (Join-Path $Root "target/$triple/release/heatmap_ffi.dll") $dest -Force
        Write-Host "Installed heatmap_ffi.dll -> $dest"
    }
}
finally {
    Pop-Location
}
