Param(
  [string]$TargetTriple = "x86_64-pc-windows-msvc",
  [string]$DestinationDir = "src-tauri/bin"
)

$ErrorActionPreference = "Stop"

function Ensure-Dir([string]$Path) {
  if (-not (Test-Path -LiteralPath $Path)) {
    New-Item -ItemType Directory -Force -Path $Path | Out-Null
  }
}

if ($env:MEDIAFORGE_SKIP_FFMPEG -eq "1") {
  Write-Host "MEDIAFORGE_SKIP_FFMPEG=1 set; skipping ffmpeg download."
  exit 0
}

$destDir = Join-Path -Path (Get-Location) -ChildPath $DestinationDir
Ensure-Dir $destDir

$destExe = Join-Path $destDir ("ffmpeg-$TargetTriple.exe")
if (Test-Path -LiteralPath $destExe) {
  Write-Host "ffmpeg already present: $destExe"
  exit 0
}

if ($TargetTriple -ne "x86_64-pc-windows-msvc") {
  throw "Unsupported TargetTriple '$TargetTriple' for this script."
}

$url = "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip"
$tmpRoot = Join-Path -Path (Get-Location) -ChildPath ".tmp_ffmpeg"
$zipPath = Join-Path -Path (Get-Location) -ChildPath ".tmp_ffmpeg.zip"

try {
  if (Test-Path -LiteralPath $tmpRoot) { Remove-Item -Recurse -Force -LiteralPath $tmpRoot }
  if (Test-Path -LiteralPath $zipPath) { Remove-Item -Force -LiteralPath $zipPath }

  Write-Host "Downloading ffmpeg zip..."
  Invoke-WebRequest -Uri $url -OutFile $zipPath -UseBasicParsing

  Write-Host "Extracting..."
  Expand-Archive -Path $zipPath -DestinationPath $tmpRoot

  $folder = Get-ChildItem -LiteralPath $tmpRoot -Directory | Select-Object -First 1
  if (-not $folder) { throw "Could not find extracted ffmpeg folder in $tmpRoot" }

  $ffmpegExe = Join-Path $folder.FullName "bin/ffmpeg.exe"
  if (-not (Test-Path -LiteralPath $ffmpegExe)) { throw "ffmpeg.exe not found at $ffmpegExe" }

  Copy-Item -Force -LiteralPath $ffmpegExe -Destination $destExe
  Write-Host "Installed ffmpeg sidecar: $destExe"
}
finally {
  if (Test-Path -LiteralPath $tmpRoot) { Remove-Item -Recurse -Force -LiteralPath $tmpRoot }
  if (Test-Path -LiteralPath $zipPath) { Remove-Item -Force -LiteralPath $zipPath }
}
