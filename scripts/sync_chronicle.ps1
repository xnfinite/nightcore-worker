<#
.SYNOPSIS
🔁 Night Core™ Chronicle Sync Utility — Worker → Pro (Hash-Aware + Fingerprint Display)

Synchronizes the Chronicle and Proof Log from Worker → Pro only when file hashes differ.
Displays and logs SHA-256 fingerprints under Night Core™ v38 Foundation Lock.
#>

$ErrorActionPreference = "Stop"
Write-Host "`n🔁 Night Core™ Chronicle Sync Utility — Worker → Pro (Hash-Aware + Fingerprint Display)`n" -ForegroundColor Cyan

# --- CONFIG ---
$workerRepo = "C:\Users\gabeg\source\repos\nightcore-worker"
$proRepo    = "C:\Users\gabeg\source\repos\nightcore-pro"

$chronicleWorker = Join-Path $workerRepo "docs\legacy\Night_Core_Chronicle.md"
$proofWorker     = Join-Path $workerRepo "logs\nightcore_proof.html"

$chronicleProDir = Join-Path $proRepo "docs\legacy"
$proofProDir     = Join-Path $proRepo "logs"

$chroniclePro = Join-Path $chronicleProDir "Night_Core_Chronicle.md"
$proofPro     = Join-Path $proofProDir "nightcore_proof.html"
$auditLog     = Join-Path $workerRepo "logs\audit.log"

# --- UTILITY: Compute SHA-256 hash ---
function Get-FileHashHex([string]$path) {
    if (Test-Path $path) {
        return (Get-FileHash -Algorithm SHA256 -Path $path).Hash.ToLower()
    } else {
        return $null
    }
}

# --- Step 1: Validate Worker sources ---
foreach ($f in @($chronicleWorker, $proofWorker)) {
    if (!(Test-Path $f)) {
        Write-Host "❌ Missing file in Worker repo: $f" -ForegroundColor Red
        exit 1
    }
}

# --- Step 2: Ensure Pro directories exist ---
foreach ($dir in @($chronicleProDir, $proofProDir)) {
    if (!(Test-Path $dir)) {
        Write-Host "📁 Creating missing directory: $dir" -ForegroundColor Yellow
        New-Item -ItemType Directory -Force -Path $dir | Out-Null
    }
}

# --- Step 3: Compute hashes ---
$chronicleWorkerHash = Get-FileHashHex $chronicleWorker
$chronicleProHash    = Get-FileHashHex $chroniclePro
$proofWorkerHash     = Get-FileHashHex $proofWorker
$proofProHash        = Get-FileHashHex $proofPro
$timestamp = (Get-Date).ToUniversalTime().ToString("yyyy-MM-dd HH:mm:ss 'UTC'")

# --- Step 4: Display hash fingerprints ---
Write-Host "🧾 Hash Fingerprints (SHA-256):" -ForegroundColor Yellow
Write-Host "Chronicle (Worker): $chronicleWorkerHash"
Write-Host "Chronicle (Pro)    : $chronicleProHash"
Write-Host "Proof Log (Worker) : $proofWorkerHash"
Write-Host "Proof Log (Pro)    : $proofProHash"
Write-Host ""

# --- Step 5: Compare & sync if different ---
$changed = $false
if ($chronicleWorkerHash -ne $chronicleProHash -or $proofWorkerHash -ne $proofProHash) {
    Write-Host "📦 Differences detected — syncing updated files to Pro repo..." -ForegroundColor Yellow
    Copy-Item -Force $chronicleWorker $chroniclePro
    Copy-Item -Force $proofWorker $proofPro
    $changed = $true
} else {
    Write-Host "✅ No changes detected — Chronicle and Proof already identical." -ForegroundColor Green
}

# --- Step 6: Commit and push if changed ---
if ($changed) {
    Set-Location $proRepo
    git add docs\legacy\Night_Core_Chronicle.md logs\nightcore_proof.html
    git commit -S -m "🔁 Synced Night Core™ Chronicle & Proof Log (hash update @ $timestamp)" 2>$null
    git push origin main
    Write-Host "`n✅ Chronicle and Proof successfully synced & pushed to Pro repository." -ForegroundColor Green
} else {
    Write-Host "ℹ️ No commit necessary; state already up to date." -ForegroundColor Gray
}

# --- Step 7: Log to audit trail ---
if (!(Test-Path $auditLog)) { New-Item -ItemType File -Force -Path $auditLog | Out-Null }

@"
[$timestamp] Night Core™ Chronicle Sync — Hash Verification
------------------------------------------------------------
Chronicle (Worker): $chronicleWorkerHash
Chronicle (Pro)    : $chronicleProHash
Proof (Worker)     : $proofWorkerHash
Proof (Pro)        : $proofProHash
Result             : $(if ($changed) { "Files updated and pushed" } else { "No differences" })
------------------------------------------------------------
"@ | Out-File -FilePath $auditLog -Append -Encoding utf8

Write-Host "`n🔒 Foundation Lock Verified — Audit log updated with hash fingerprints.`n" -ForegroundColor Cyan
