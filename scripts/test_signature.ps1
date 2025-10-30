param(
    [string]$Pub = "keys/maintainers/admin1.pub",
    [string]$Sig = "upgrades/signatures/v251029_admin1.sig.b64",
    [string]$Manifest = "upgrades/manifests/upgrade_manifest.json"
)

Write-Host "`n🔍 Testing signature verification manually..." -ForegroundColor Cyan
$pubKey = [System.Convert]::FromBase64String((Get-Content $Pub).Trim())
$sigRaw = [System.Convert]::FromBase64String((Get-Content $Sig).Trim())
$manifest = [System.IO.File]::ReadAllBytes($Manifest)

try {
    Add-Type -AssemblyName System.Security
    $ok = [System.Security.Cryptography.Ed25519]::Verify($pubKey, $manifest, $sigRaw)
    if ($ok) {
        Write-Host "✅ Signature verified successfully for $Pub" -ForegroundColor Green
    } else {
        Write-Host "❌ Signature invalid for $Pub" -ForegroundColor Red
    }
} catch {
    Write-Host "⚠️ Verification error: $_" -ForegroundColor Yellow
}
