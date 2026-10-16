<#
.SYNOPSIS
  Submits a release to the Microsoft Store: the .msix, plus the "What's new" text from
  whats-new.txt.

.DESCRIPTION
  Runs in the `publish` job of .github/workflows/store.yml, after `msstore reconfigure` has
  signed in to Partner Center. Steps:
   1. Read whats-new.txt (before anything is uploaded, so a bad file stops it early).
   2. `msstore publish --noCommit`: a new submission, copied from the last one, with the
      new package, kept as a draft.
   3. `msstore submission get`, change "What's new" in its JSON, `submission updateMetadata`.
   4. `msstore submission publish`: send the draft to certification. (In the CLI's source,
      this is the commit call: it doesn't make anything live by itself.)
#>
param(
    [Parameter(Mandatory)] [string] $Msix,
    [Parameter(Mandatory)] [string] $ProductId
)
$ErrorActionPreference = 'Stop'
Set-Location (Resolve-Path "$PSScriptRoot/../..")

function Assert-Success([string] $step) {
    if ($LASTEXITCODE -ne 0) { throw "$step failed (exit code $LASTEXITCODE)" }
}

# --- 1. The text ------------------------------------------------------------------------

$version = (Select-String -Path Cargo.toml -Pattern '^version = "(\d+\.\d+\.\d+)"').Matches[0].Groups[1].Value

$lines = @(Get-Content packaging/msix/whats-new.txt | Where-Object { $_ -notmatch '^\s*#' })
$first = [array]::FindIndex($lines, [Predicate[string]] { param($line) $line.Trim() -ne '' })
$textVersion = if ($first -ge 0) { $lines[$first].Trim() } else { '' }
$text = if ($first -ge 0 -and $first + 1 -lt $lines.Count) {
    ($lines[($first + 1)..($lines.Count - 1)] -join "`n").Trim()
} else { '' }

if ($textVersion -ne $version) {
    Write-Output "::warning::whats-new.txt is for version '$textVersion', not ${version}: this release goes to the Store without a ""What's new"" text."
    $text = ''
}
if ($text.Length -gt 1500) { throw "whats-new.txt: $($text.Length) characters; the Store allows 1500" }
Write-Output "What's new for ${version}:`n$text`n"

# --- 2. The draft with the package --------------------------------------------------------

msstore publish $Msix -id $ProductId --noCommit
Assert-Success 'msstore publish --noCommit'

# From here on, a failure leaves a draft with the new package in Partner Center. It was
# made through the API, so the next run can replace it; or submit it there by hand.
try {

# --- 3. "What's new" ----------------------------------------------------------------------

$json = msstore submission get $ProductId | Out-String
Assert-Success 'msstore submission get'

# Keep dates exactly as text: converting them to dates and back could change their format.
$keepDates = @{}
if ((Get-Command ConvertFrom-Json).Parameters.ContainsKey('DateKind')) { $keepDates.DateKind = 'String' }
$submission = $json | ConvertFrom-Json @keepDates

# PowerShell finds properties whatever their capitals (Listings or listings).
$listing = $submission.Listings.'en-us'
if (-not $listing) {
    $languages = ($submission.Listings.PSObject.Properties.Name) -join ', '
    throw "No en-us listing in the submission (found: $languages)"
}
$base = $listing.BaseListing
$notes = $base.PSObject.Properties | Where-Object Name -EQ 'ReleaseNotes' # -eq ignores case
if ($notes) {
    $notes.Value = $text
} else {
    # Not there when it was empty: add it, in the same style as its neighbours' names.
    $camelCase = ($base.PSObject.Properties | Select-Object -First 1).Name -cmatch '^[a-z]'
    $name = if ($camelCase) { 'releaseNotes' } else { 'ReleaseNotes' }
    $base | Add-Member -NotePropertyName $name -NotePropertyValue $text
}

$updated = $submission | ConvertTo-Json -Depth 100 -Compress
# It goes on the command line, which Windows limits to 32767 characters.
if ($updated.Length -gt 30000) { throw "The submission JSON is $($updated.Length) characters: too long for the command line" }
msstore submission updateMetadata $ProductId $updated
Assert-Success 'msstore submission updateMetadata'

# --- 4. Certification ---------------------------------------------------------------------

msstore submission publish $ProductId
Assert-Success 'msstore submission publish'

} catch {
    Write-Output "::error::The package is uploaded, but the submission wasn't sent to certification. Re-run this job, or open the draft in Partner Center and submit it there."
    throw
}
Write-Output "Submitted $version to the Microsoft Store."
