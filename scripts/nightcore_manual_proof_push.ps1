Write-Host "`n🚀 Night Core™ v38 — Manual Proof Verification & Chronicle Push`n" -ForegroundColor Cyan

# --- CONFIG ---
$chronicle = "docs\legacy\Night_Core_Chronicle.md"
$proofLog  = "logs\nightcore_proof.html"
$timestamp = (Get-Date).ToUniversalTime().ToString("yyyy-MM-dd HH:mm:ss 'UTC'")

# --- Step 1: Run Proof Verification ---
Write-Host "🔍 Running Night Core proof verification..." -ForegroundColor Yellow
cargo +nightly run -- run --all --proof

if (!(Test-Path $proofLog)) {
    Write-Host "❌ Proof log not found! Exiting." -ForegroundColor Red
    exit 1
}

# --- Step 2: Extract SHA-256 hashes from proof log ---
$proofData = Get-Content $proofLog -Raw
$tenantA = ($proofData -split "`n" | Select-String "tenantA" -Context 0,5 | Out-String).Trim()
$tenantB = ($proofData -split "`n" | Select-String "tenantB" -Context 0,5 | Out-String).Trim()

# --- Step 3: Append to Chronicle ---
$entry = @"
### 🔒 Proof Entry — $timestamp
Tenant A Proof:
$tenantA

Tenant B Proof:
$tenantB

Source Log: [$proofLog](../../logs/nightcore_proof.html)
---
"@
Add-Content -Path $chronicle -Value $entry
Write-Host "🪶 Chronicle updated: $chronicle" -ForegroundColor Green

# --- Step 4: Commit and Push ---
Write-Host "📤 Committing and pushing Chronicle update..." -ForegroundColor Yellow
git add -f $chronicle $proofLog
git commit -S -m "📜 Night Core Proof Chronicle update — $timestamp"
git push origin main

Write-Host "✅ Proof + Chronicle push complete." -ForegroundColor Green
