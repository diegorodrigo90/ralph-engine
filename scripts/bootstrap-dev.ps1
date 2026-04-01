$ErrorActionPreference = 'Stop'

if (Get-Command asdf -ErrorAction SilentlyContinue) {
  $plugins = asdf plugin list
  if ($plugins -contains 'rust' -and $plugins -contains 'nodejs') {
    asdf install
  } else {
    Write-Host 'asdf found but missing rust and/or nodejs plugins; skipping asdf install'
  }
} else {
  Write-Host 'asdf not found; continuing with the existing local toolchain'
}

npm ci
./scripts/install-dev-tools.ps1
npx lefthook install
