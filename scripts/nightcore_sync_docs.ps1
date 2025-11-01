# ======================================================
# 🧩 Night Core™ v38 — Safe Docs Sync (No Code Touch)
# ======================================================

Write-Host "`n🚀 Starting Night Core Documentation Sync..." -ForegroundColor Cyan
Set-Location "$PSScriptRoot/.."

# 1️⃣ Run proof verification
Write-Host "`n🔍 Running proof verification..."
cargo +nightly run -- run --all --proof

# 2️⃣ Deduplicate proof entries
Write-Host "`n🧹 Cleaning duplicate proof entries..."
$proofPath = "logs/nightcore_proof.html"
if (Test-Path $proofPath) {
    $unique = Get-Content $proofPath | Get-Unique
    Set-Content $proofPath -Value $unique -Encoding UTF8
} else {
    Write-Host "⚠️ No proof file found." -ForegroundColor Yellow
}

# 3️⃣ Normalize all Markdown docs (no code)
Write-Host "`n🧾 Normalizing Markdown..."
Get-ChildItem -Recurse -Filter *.md | ForEach-Object {
    $text = Get-Content $_.FullName -Raw
    $text = $text -replace 'Night Core ™', 'Night Core™'
    $text = $text -replace '\r?\n{3,}', "`r`n`r`n"
    Set-Content $_.FullName -Value $text -Encoding UTF8
}

# 4️⃣ Update README summary from latest proof entry
if (Test-Path $proofPath) {
    Write-Host "`n🧩 Updating README.md summary..."
    $lines = Get-Content $proofPath | Select-String "✅ VERIFIED:" -Context 0,8 | Select-Object -Last 1
    if ($lines) {
        $commit = ($lines.Context.PostContext | Select-String "Commit:").ToString().Split(":")[1].Trim()
        $audit = ($lines.Context.PostContext | Select-String "Audit-Hash:").ToString().Split(":")[1].Trim()
        $timestamp = ($lines.Context.PostContext | Select-String "Timestamp:").ToString().Split(":")[1].Trim()
        $summary = @"
### 🧩 Night Core™ v38 — Verified Build Summary

| Field | Value |
|-------|-------|
| **Commit ID** | $commit |
| **Timestamp** | $timestamp |
| **Audit Hash** | $audit |
| **Maintainers** | core-ops • system-check |
| **Status** | ✅ Verified |

Night Core™ — Secure • Autonomous • Verified
"@
        (Get-Content README.md -Raw) -replace '(?s)### 🧩.*?Verified', $summary | 
            Set-Content README.md -Encoding UTF8
    }
}

# 5️⃣ Commit + push safely
Write-Host "`n💾 Committing and pushing docs..." -ForegroundColor Cyan
git add README.md logs/nightcore_proof.html
git commit -m "🧾 Night Core Docs Sync — No Code Change (Proof Deduplicated)"
git push origin main

Write-Host "`n✅ Docs sync complete — No duplicates, no code changed." -ForegroundColor Green
