$ErrorActionPreference = "Stop"

$RootDir = Split-Path -Parent $PSScriptRoot
$AppDir = Join-Path $RootDir "apps/desktop"

pnpm --dir $AppDir tauri build --bundles msi,nsis
