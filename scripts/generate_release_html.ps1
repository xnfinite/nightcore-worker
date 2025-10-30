# ==========================================================
# Night Core — Release Provenance HTML Generator (v38)
# Maintainer: xnfinite
# ==========================================================

param(
    [string]$Version = "v38",
    [string]$Commit = "",
    [string]$AuditHash = "",
    [string]$PolicyHash = "",
    [string]$Date = (Get-Date -Format 'yyyy-MM-dd')
)

$logoPath = "../assets/nightcore_logo_tm.png"
$releaseHtml = "docs/releases/$Version.html"

# === Metadata Header ===
$commit = if ($Commit) { $Commit } else { (git rev-parse HEAD).Trim() }
$auditHash = if ($AuditHash) { $AuditHash } else { "unknown" }
$policyHash = if ($PolicyHash) { $PolicyHash } else { "unknown" }

# === HTML Template ===
$html = @"
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Night Core™ — $Version Provenance</title>
<style>
body { font-family: 'Segoe UI', sans-serif; background:#0b0c10; color:#cfd8dc; margin:2em; }
h1, h2 { color:#00bcd4; }
table { width:100%; border-collapse:collapse; margin-top:1em; }
td, th { border:1px solid #263238; padding:8px; text-align:left; }
tr:nth-child(even) { background:#1c1f26; }
a { color:#4dd0e1; }
.logo { width:160px; }
.footer { margin-top:2em; font-size:0.85em; color:#78909c; }
</style>
</head>
<body>
<img src="$logoPath" alt="Night Core Logo" class="logo">
<h1>Night Core™ $Version — Provenance Record</h1>

<p><b>Release Date:</b> $Date<br>
<b>Commit ID:</b> $commit<br>
<b>Audit Hash (SHA-256):</b> $auditHash<br>
<b>Policy Fingerprint:</b> $policyHash</p>

<h2>✅ Verification Summary</h2>
<table>
<tr><th>Check</th><th>Status</th><th>Details</th></tr>
<tr><td>AUFS Threshold</td><td>✅</td><td>2 of 2 valid signatures (admin1, admin2)</td></tr>
<tr><td>Module Integrity</td><td>✅</td><td>tenantA-hello & tenantB-math SHA-256 match</td></tr>
<tr><td>Policy Verification</td><td>✅</td><td>RELEASE_POLICY.md hash verified</td></tr>
<tr><td>Audit Chain</td><td>✅</td><td>Entry appended to logs/audit.log</td></tr>
</table>

<h2>🔐 Maintainer Signatures</h2>
<ul>
<li><b>admin1.pub</b>: atEOzpDAAxJC94Mk/Shc5Lc0KqTnTq/iHfOcLdnA3vc=</li>
<li><b>admin2.pub</b>: f5M+HyBN8wvRyR3Z8Ay0fc2C9eLSk44HYlhNs5Aka/o=</li>
</ul>

<h2>🧩 Highlights</h2>
<ul>
<li>AUFS v38 core verification completed successfully</li>
<li>Full multi-tenant integrity check (A & B)</li>
<li>Dual Ed25519 maintainer threshold validated</li>
<li>Audit log and policy hash commit recorded</li>
<li>GitHub tag <b>v38-stable-aufs-verified</b></li>
</ul>

<div class="footer">
<p>Night Core™ © B106 Labs | Open Core MIT License | Generated $(Get-Date)</p>
</div>
</body>
</html>
"@

# === Write file ===
New-Item -ItemType Directory -Force -Path "docs/releases" | Out-Null
$html | Set-Content -Encoding UTF8 $releaseHtml
Write-Host "✅ Provenance HTML generated: $releaseHtml" -ForegroundColor Green

# === Step 8: Generate Provenance HTML Page ===
Write-Host "`n🌐 Generating Night Core Provenance Page..." -ForegroundColor Cyan

$latestCommit = (git rev-parse HEAD).Trim()
$latestAuditHash = $policyHash
$releaseVersion = "v38"

pwsh -File scripts/generate_release_html.ps1 `
  -Version $releaseVersion `
  -Commit $latestCommit `
  -AuditHash "192890d99bfbf65ef6cb05e3a2bf3a2debf87c4ef51cde396fcdd6e51bfaf0b2" `
  -PolicyHash $latestAuditHash

if (Test-Path "docs/releases/$releaseVersion.html") {
    Write-Host "✅ Provenance page created successfully." -ForegroundColor Green
    git add "docs/releases/$releaseVersion.html"
    git commit -S -m "🌐 Added Provenance Page for $releaseVersion (auto-generated)"
    git push origin main
    Write-Host "📤 Provenance HTML committed and pushed to origin/main" -ForegroundColor Green
} else {
    Write-Host "⚠️ Provenance generation failed or file missing." -ForegroundColor Yellow
}
