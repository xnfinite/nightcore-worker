#!/usr/bin/env pwsh
<#
  Night Core – Audit Transparency Hasher
  Creates a base64 SHA-256 hash chain over logs/audit.log
  Output: logs/transparency.log.jsonl
#>

$ErrorActionPreference = "Stop"
$logPath   = "logs/audit.log"
$chainPath = "logs/transparency.log.jsonl"

if (-not (Test-Path $logPath)) {
  Write-Host "[ERROR] audit.log not found at $logPath" -ForegroundColor Red
  exit 1
}

# Read every JSON line in audit.log
$lines = Get-Content $logPath | ForEach-Object { 
  try { $_ | ConvertFrom-Json -ErrorAction Stop } catch { $null }
} | Where-Object { $_ -ne $null }

if ($lines.Count -eq 0) {
  Write-Host "[WARN] No valid audit entries found." -ForegroundColor Yellow
  exit 0
}

$prevHash = ""
$entries  = @()

foreach ($entry in $lines) {
  $json = ($entry | ConvertTo-Json -Compress)
  $bytes = [Text.Encoding]::UTF8.GetBytes($json)
  $sha = [System.Security.Cryptography.SHA256]::Create()
  $hash = [Convert]::ToBase64String($sha.ComputeHash($bytes))

  $chainEntry = [ordered]@{
    ts_utc         = (Get-Date).ToUniversalTime().ToString("o")
    commit         = $entry.commit
    actor          = $entry.by
    prev_hash_b64  = $prevHash
    entry_hash_b64 = $hash
  }

  $entries += ($chainEntry | ConvertTo-Json -Compress)
  $prevHash = $hash
}

Add-Content -Path $chainPath -Value ($entries -join "`n")
Write-Host "[OK] Transparency chain extended to $($entries.Count) entries." -ForegroundColor Green
