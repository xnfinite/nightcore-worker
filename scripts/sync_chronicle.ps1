Write-Host "`n🔁 Night Core™ Chronicle Sync Utility — Worker → Pro`n" -ForegroundColor Cyan

# --- CONFIG ---
$workerRepo = "C:\Users\gabeg\source\repos\nightcore-worker"
$proRepo    = "C:\Users\gabeg\source\repos\nightcore-pro\nightcore-pro"

$chronicleWorker = Join-Path $workerRepo "docs\legacy\Night_Core_Chronicle.md"
$proofWorker     = Join-Path $workerRepo "logs\nightcore_proof.html"

$chroniclePro = Join-Path $proRepo "docs\legacy\Night_Core_Chronicle.md"
$proofPro     = Join-Path $proRepo "logs\nightcore_proof.html"

# --- Step 1: Validate paths ---
if (!(Test-Path $chronicleWorker)) {
    Write-Host "❌ Chronicle not found in Worker repo: $chronicleWorker" -ForegroundColor Red
    exit 1
}

if (!(Test-Path $proofWorker)) {
    Write-Host "❌ Proof log not found in Worker repo: $proofWorker" -ForegroundColor Red
    exit 1
}

# --- Step 2: Copy to Pro repo ---
Write-Host "📦 Copying Chronicle and Proof Log to Pro repo..." -ForegroundColor Yellow
Copy-Item -Force $chronicleWorker $chroniclePro
Copy-Item -Force $proofWorker $proofPro

Write-Host "✅ Copied:" -ForegroundColor Green
Write-Host " → $chroniclePro"
Write-Host " → $proofPro"

# --- Step 3: Commit and push from Pro repo ---
Set-Location $proRepo
Write-Host "`n🪶 Committing and pushing Chronicle sync in Pro repo..." -ForegroundColor Yellow

git add docs\legacy\Night_Core_Chronicle.md logs\nightcore_proof.html
git commit -S -m "🔁 Synced Night Core Chronicle and Proof Log from Worker"
git push origin main

Write-Host "`n✅ Chronicle successfully synced to Pro repository!" -ForegroundColor Green
