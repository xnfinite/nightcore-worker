#!/usr/bin/env pwsh
<#
  Night Core – Transparency Chain Verifier
  Verifies that each entry in logs/transparency.log.jsonl correctly
  matches the SHA-256 hash of its audit.log JSON and the previous link.
#>

$ErrorActionPreference = "Stop"
$logPath   = "logs/audit.log"
$chainPath = "logs/transparency.log.jsonl"

if (-not (Test-Path $logPath) -or -not (Test-Path $chainPath)) {
    Write-Host "[ERROR] Required log files missing." -ForegroundColor Red
    exit 1
}

# Load audit entries
$audit = Get-Content $logPath | ForEach-Object {
    try { $_ | ConvertFrom-Json -ErrorAction Stop } catch { $null }
} | Where-Object { $_ -ne $null }

# Load transparency entries
$chain = Get-Content $chainPath | ForEach-Object {
    try { $_ | ConvertFrom-Json -ErrorAction Stop } catch { $null }
} | Where-Object { $_ -ne $null }

if ($audit.Count -ne $chain.Count) {
    Write-Host "[WARN] Entry counts differ: audit=$($audit.Count)  chain=$($chain.Count)" -ForegroundColor Yellow
}

$prevHash = ""
$ok = $true

for ($i = 0; $i -lt $chain.Count; $i++) {
    $entry     = $audit[$i]
    $chainItem = $chain[$i]

    # recompute current audit hash
    $json  = ($entry | ConvertTo-Json -Compress)
    $bytes = [Text.Encoding]::UTF8.GetBytes($json)
    $sha   = [System.Security.Cryptography.SHA256]::Create()
    $calc  = [Convert]::ToBase64String($sha.ComputeHash($bytes))

    if ($calc -ne $chainItem.entry_hash_b64) {
        Write-Host ("[FAIL] hash mismatch at entry #{0} commit {1}" -f $i, $entry.commit) -ForegroundColor Red
        $ok = $false
        break
    }

    if ($chainItem.prev_hash_b64 -ne $prevHash) {
        Write-Host ("[FAIL] chain broken before entry #{0}" -f $i) -ForegroundColor Red
        $ok = $false
        break
    }

    $prevHash = $calc
}

if ($ok) {
    Write-Host "[OK] Transparency chain verified: all hashes intact." -ForegroundColor Green
    exit 0
} else {
    Write-Host "[ERROR] Transparency chain integrity check failed." -ForegroundColor Red
    exit 2
}
