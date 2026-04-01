$ErrorActionPreference = 'Stop'

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
  throw 'cargo is required'
}

$CargoBin = Join-Path $HOME '.cargo/bin'
New-Item -ItemType Directory -Force -Path $CargoBin | Out-Null

cargo install cargo-llvm-cov --version 0.8.5 --locked
cargo install cargo-audit --version 0.22.1 --locked
cargo install cargo-deny --version 0.19.0 --locked
cargo install cargo-dist --version 0.31.0 --locked

$gitleaksVersion = 'v8.30.1'
$gitleaksAsset = 'gitleaks_8.30.1_windows_x64.zip'
$gitleaksZip = Join-Path $env:TEMP $gitleaksAsset
Invoke-WebRequest -Uri "https://github.com/gitleaks/gitleaks/releases/download/$gitleaksVersion/$gitleaksAsset" -OutFile $gitleaksZip
Expand-Archive -Path $gitleaksZip -DestinationPath $env:TEMP -Force
Copy-Item (Join-Path $env:TEMP 'gitleaks.exe') (Join-Path $CargoBin 'gitleaks.exe') -Force

$trivyVersion = 'v0.69.3'
$trivyAsset = 'trivy_0.69.3_windows-64bit.zip'
$trivyZip = Join-Path $env:TEMP $trivyAsset
Invoke-WebRequest -Uri "https://github.com/aquasecurity/trivy/releases/download/$trivyVersion/$trivyAsset" -OutFile $trivyZip
Expand-Archive -Path $trivyZip -DestinationPath $env:TEMP -Force
Copy-Item (Join-Path $env:TEMP 'trivy.exe') (Join-Path $CargoBin 'trivy.exe') -Force
