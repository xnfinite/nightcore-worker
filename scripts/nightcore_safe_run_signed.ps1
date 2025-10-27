# ==========================================================
# Night Core — Safe Run & Signed Push Script (v38 Stable)
# Maintainer: xnfinite
# Purpose: Verify AUFS, sign, and safely push verified commits
# ==========================================================

Write-Host "`n🚀 Starting Night Core Safe Run (Signed Mode)" -ForegroundColor Cyan

# === Step 0: Move to script directory safely ===
$scriptPath = $PSCommandPath
if (-not $scriptPath) { $scriptPath = $MyInvocation.MyCommand.Path }
if ($scriptPath) {
    Set-Location (Split-Path -Parent $scriptPath)
} else {
    Write-Host "⚠️ Could not determine script directory; continuing in current location." -ForegroundColor Yellow
}

# === Step 1: Ensure release policy exists ===
if (-not (Test-Path 'docs/internal/RELEASE_POLICY.md')) {
    Write-Host '⚠️ RELEASE_POLICY.md not found — creating default policy...' -ForegroundColor Yellow
    New-Item -ItemType Directory -Force -Path docs/internal | Out-Null
    @'
# 🧩 Night Core — Internal Release Policy (AUFS v38)
Maintainer: xnfinite
Scope: Night Core AUFS Chain / Safe Push Governance
Version: v38
Last Updated: (Get-Date -Format "yyyy-MM-dd HH:mm:ss")

Purpose: Defines the trusted release policy for the Night Core AUFS baseline chain.
'@ | Set-Content -Encoding UTF8 docs/internal/RELEASE_POLICY.md
}

# === Step 1.5: Generate SHA-256 integrity fingerprint for policy ===
Write-Host "`n🔐 Generating SHA-256 hash for RELEASE_POLICY.md..." -ForegroundColor Cyan
$policyHash = (Get-FileHash 'docs/internal/RELEASE_POLICY.md' -Algorithm SHA256).Hash
Add-Content 'docs/internal/RELEASE_POLICY.md' "`n`n---`nIntegrity Hash (SHA-256): $policyHash"
Write-Host "✅ Policy integrity hash appended:`n   $policyHash" -ForegroundColor Green

# === Step 2: Build and verify project ===
Write-Host "`n🔧 Building Night Core..." -ForegroundColor Cyan
cargo build
if ($LASTEXITCODE -ne 0) { Write-Host '❌ Build failed, aborting.' -ForegroundColor Red; exit 1 }

Write-Host "`n🧩 Running AUFS verification..." -ForegroundColor Cyan
cargo run -- upgrade --manifest upgrades/manifests/upgrade_manifest.json
if ($LASTEXITCODE -ne 0) { Write-Host '❌ AUFS verification failed, aborting.' -ForegroundColor Red; exit 1 }

# === Step 3: Safe file allowlist check ===
$safePaths = @(
  'baseline.json',
  'logs/audit.log',
  'logs/orchestration_report.json',
  'logs/nightcore_dashboard.html',
  'upgrades/manifests/upgrade_manifest.json',
  'upgrades/signatures',
  'keys/maintainers',
  'scripts',
  'docs/NOTICE.html',
  'README_NOTICE.txt',
  'docs/internal/RELEASE_POLICY.md',
  'docs/internal',
  'sign_upgrade.rs',
  'src'
)

Write-Host "`n🧠 Checking modified files..." -ForegroundColor Cyan
$modified = git status --porcelain | ForEach-Object { $_.Trim() -split '\s+' | Select-Object -Last 1 }

foreach ($file in $modified) {
    if (-not ($safePaths | Where-Object { $file -like "$_*" })) {
        Write-Host "❌ Unsafe file detected: $file" -ForegroundColor Red
        Write-Host "Push blocked. File not in allowlist." -ForegroundColor Red
        exit 1
    }
}

# === Step 4: Signed commit ===
Write-Host "`n📝 Creating signed commit..." -ForegroundColor Cyan
git add -A
git commit -S -m '🔒 Safe Signed Commit — Verified AUFS Chain (xnfinite)'
if ($LASTEXITCODE -ne 0) { Write-Host '⚠️  No changes to commit or signing failed.' -ForegroundColor Yellow }

# === Step 5: Push to main ===
Write-Host "`n🌐 Pushing to origin/main..." -ForegroundColor Cyan
git push origin main
if ($LASTEXITCODE -ne 0) { Write-Host '❌ Push failed.' -ForegroundColor Red; exit 1 }

# === Step 6: Append audit log entry ===
Write-Host "`n🧾 Appending audit log entry..." -ForegroundColor Cyan
$commitHash = (git rev-parse HEAD).Trim()
$timestamp = (Get-Date -Format 'yyyy-MM-dd HH:mm:ss')

$auditEntry = @"
---
🧩 Verification Pass — AUFS v38
Timestamp: $timestamp
Commit: $commitHash
Audit Hash: $policyHash
Maintainer: xnfinite
Outcome: SUCCESS — Safe signed run completed and verified.
---
"@
Add-Content -Encoding UTF8 "logs/audit.log" $auditEntry
Write-Host "✅ Audit entry appended for commit $commitHash" -ForegroundColor Green

Write-Host "`n✅ Safe signed run completed successfully!" -ForegroundColor Green
