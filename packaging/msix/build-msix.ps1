<#
.SYNOPSIS
  Builds the Microsoft Store package: target/msix/minesweeper-<version>-x64.msix

.DESCRIPTION
  Needs Windows with Rust and the Windows SDK (MakePri, MakeAppx): the GitHub Actions
  runner has both. The .msix is not signed; the Store signs it after you upload it in
  Partner Center.

  The identity comes from store-identity.json. While it still says TODO, the package is
  built with a test identity, which checks that packaging works but can't be uploaded.

  Steps:
   1. Build the game without the update check (the Store updates apps itself), with the
      C runtime built into the .exe, so the package doesn't depend on the VC++ runtime.
   2. Put the .exe, the licenses and the logos (examples/store_logos.rs) in one folder.
   3. Fill in AppxManifest.xml.
   4. MakePri: index the logos, so Windows can pick the right size for each screen.
   5. MakeAppx: pack the folder into the .msix.
#>
$ErrorActionPreference = 'Stop'
Set-Location (Resolve-Path "$PSScriptRoot/../..")

# Stops the script if a native tool (cargo, MakeAppx, ...) failed.
function Assert-Success([string] $step) {
    if ($LASTEXITCODE -ne 0) { throw "$step failed (exit code $LASTEXITCODE)" }
}

# --- Version and identity ---------------------------------------------------------------

$cargoVersion = (Select-String -Path Cargo.toml -Pattern '^version = "(\d+\.\d+\.\d+)"').Matches[0].Groups[1].Value
# Store packages have four numbers, and the Store needs the last one to be 0.
$packageVersion = "$cargoVersion.0"

$identity = Get-Content packaging/msix/store-identity.json -Raw | ConvertFrom-Json
$placeholder = $identity.PSObject.Properties | Where-Object { "$($_.Value)" -like 'TODO*' }
if ($placeholder) {
    Write-Output '::warning::store-identity.json still has TODO values: building with a test identity. This package cannot be uploaded to the Store.'
    $identity = [pscustomobject]@{
        IdentityName         = 'MinesweeperRust.Test'
        Publisher            = 'CN=Test'
        PublisherDisplayName = 'Test'
        DisplayName          = 'Minesweeper (test)'
    }
}

# --- 1. Build --------------------------------------------------------------------------

$env:RUSTFLAGS = '-C target-feature=+crt-static'
cargo build --release --locked
Assert-Success 'cargo build'
Remove-Item Env:RUSTFLAGS

# --- 2. The package folder -------------------------------------------------------------

$out = 'target/msix'
$package = "$out/package"
if (Test-Path $package) { Remove-Item $package -Recurse -Force }
New-Item -ItemType Directory -Path "$package/licenses" | Out-Null

Copy-Item target/release/minesweeper.exe $package/
Copy-Item LICENSE-MIT, LICENSE-APACHE "$package/licenses/"
Copy-Item assets/fonts/*LICENSE*.txt "$package/licenses/"

cargo run --locked --example store_logos -- "$package/Assets"
Assert-Success 'store_logos'

# --- 3. The manifest -------------------------------------------------------------------

# XML-escape the values: a name with & or < would otherwise break the manifest.
function Escape([string] $text) { [System.Security.SecurityElement]::Escape($text) }

$manifest = Get-Content packaging/msix/AppxManifest.xml -Raw
$manifest = $manifest.Replace('{{IdentityName}}', (Escape $identity.IdentityName)).
    Replace('{{Publisher}}', (Escape $identity.Publisher)).
    Replace('{{PublisherDisplayName}}', (Escape $identity.PublisherDisplayName)).
    Replace('{{DisplayName}}', (Escape $identity.DisplayName)).
    Replace('{{Version}}', $packageVersion)
Set-Content -Path "$package/AppxManifest.xml" -Value $manifest -Encoding utf8NoBOM

# --- 4 and 5. MakePri and MakeAppx, from the newest Windows SDK installed ----------------

$sdkBin = Get-ChildItem "${env:ProgramFiles(x86)}\Windows Kits\10\bin\10.*\x64\makeappx.exe" |
    Sort-Object { [version]$_.Directory.Parent.Name } |
    Select-Object -Last 1 -ExpandProperty DirectoryName
if (-not $sdkBin) { throw 'Windows SDK not found (MakeAppx.exe)' }
Write-Output "Windows SDK tools: $sdkBin"

$priConfig = "$out/priconfig.xml"
& "$sdkBin\makepri.exe" createconfig /cf $priConfig /dq en-US /pv 10.0.0 /o
Assert-Success 'MakePri createconfig'
& "$sdkBin\makepri.exe" new /pr $package /cf $priConfig /mn "$package/AppxManifest.xml" /of "$package/resources.pri" /o
Assert-Success 'MakePri new'

$msix = "$out/minesweeper-$cargoVersion-x64.msix"
& "$sdkBin\makeappx.exe" pack /d $package /p $msix /o
Assert-Success 'MakeAppx pack'

Write-Output "Built $msix (version $packageVersion, identity $($identity.IdentityName))"
