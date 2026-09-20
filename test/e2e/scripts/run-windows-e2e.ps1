# Windows checkout E2E — backend on 18084, FlaUI journey, GET /v1/events assertions.
$ErrorActionPreference = "Stop"

$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..")).Path
$E2eBackendPort = if ($env:E2E_WINDOWS_BACKEND_PORT) { $env:E2E_WINDOWS_BACKEND_PORT } else { "18084" }
$E2eDb = Join-Path $RepoRoot "heatmap-windows-e2e.db"
$AppProject = Join-Path $RepoRoot "poc\windows\HeatmapCheckout\HeatmapCheckout.csproj"
$TestProject = Join-Path $RepoRoot "test\e2e\windows\HeatmapCheckout.E2ETests\HeatmapCheckout.E2ETests.csproj"
$BackendJob = $null

function Stop-Backend {
    if ($null -ne $BackendJob) {
        Stop-Job -Job $BackendJob -ErrorAction SilentlyContinue
        Remove-Job -Job $BackendJob -Force -ErrorAction SilentlyContinue
    }
}

Register-EngineEvent -SourceIdentifier PowerShell.Exiting -Action { Stop-Backend } | Out-Null

function Wait-Backend {
    param([string]$Port)
    $url = "http://127.0.0.1:$Port/v1/elements?platform=web&screen=Checkout&app_version=1.0.0"
    for ($i = 0; $i -lt 120; $i++) {
        try {
            Invoke-WebRequest -Uri $url -UseBasicParsing -TimeoutSec 2 | Out-Null
            return
        } catch {
            Start-Sleep -Milliseconds 500
        }
    }
    throw "Backend did not become ready on port $Port"
}

Write-Host "Building heatmap_ffi for Windows..."
& (Join-Path $RepoRoot "sdk\windows\scripts\build-rust-windows.ps1")

if (Test-Path $E2eDb) { Remove-Item -Force $E2eDb }

Write-Host "Starting backend on port $E2eBackendPort..."
$BackendJob = Start-Job -ScriptBlock {
    param($Root, $Port, $Db)
    Set-Location $Root
    $env:PORT = $Port
    $env:HEATMAP_EXPOSE_TEST_EVENTS = "1"
    $env:DATABASE_PATH = $Db
    cargo run -p heatmap-backend
} -ArgumentList $RepoRoot, $E2eBackendPort, $E2eDb

Wait-Backend -Port $E2eBackendPort

Write-Host "Building HeatmapCheckout (Debug)..."
dotnet build $AppProject -c Debug

Write-Host "Running Windows E2E tests..."
$env:E2E_WINDOWS_BACKEND_PORT = $E2eBackendPort
$env:HEATMAP_E2E_BACKEND_PORT = $E2eBackendPort
dotnet test $TestProject -c Debug

Stop-Backend
Write-Host "Windows E2E finished."
