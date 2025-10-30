use std::{fs, path::Path};
use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json::Value;

/// ===========================================================
/// 🔐 Night Core — Key-Seal Integrity System
/// ===========================================================
/// Verifies or automatically repairs each tenant’s public-key
/// seal stored in its manifest.json. If the pubkey hash in the
/// manifest doesn’t match the current pubkey.b64, the system:
///   1️⃣ Re-hashes pubkey.b64
///   2️⃣ Updates the manifest’s `pubkey_hash`
///   3️⃣ Attempts to re-sign module.wasm (if private key exists)
///
/// This ensures each tenant’s cryptographic identity is always
/// consistent and self-healing between re-runs.
pub fn verify_pubkey_seal(dir: &Path) -> Result<()> {
    let manifest_path = dir.join("manifest.json");
    let pubkey_path = dir.join("pubkey.b64");
    let tenant_name = dir.file_name().unwrap().to_string_lossy();
    let privkey_path = dir.join(format!("{}.key", tenant_name));

    // Skip quietly if essential files missing
    if !manifest_path.exists() || !pubkey_path.exists() {
        return Ok(());
    }

    // Load manifest
    let manifest_str =
        fs::read_to_string(&manifest_path).with_context(|| format!("reading {:?}", manifest_path))?;
    let mut manifest: Value =
        serde_json::from_str(&manifest_str).with_context(|| "parsing manifest JSON")?;
    let stored_hash = manifest
        .get("pubkey_hash")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Compute current pubkey hash (base64 content)
    let pub_b64 = fs::read_to_string(&pubkey_path)?;
    let mut hasher = Sha256::new();
    hasher.update(pub_b64.as_bytes());
    let current_hash = format!("SHA256:{:X}", hasher.finalize());

    if stored_hash != current_hash {
        println!(
            "⚠️  Pubkey hash mismatch for {}, regenerating seal...",
            tenant_name
        );

        // Attempt re-sign only if private key exists
        if privkey_path.exists() {
            crate::verify::sign_module(&dir.to_path_buf(), &privkey_path)
                .with_context(|| format!("re-signing module for {}", tenant_name))?;
            manifest["pubkey_hash"] = Value::String(current_hash.clone());
            fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;
            println!("🔄 Updated manifest seal for {}", tenant_name);
        } else {
            println!(
                "⚠️  Missing private key — cannot auto-repair {}",
                tenant_name
            );
        }
    } else {
        println!("✅ Key seal verified for {}", tenant_name);
    }

    Ok(())
}

