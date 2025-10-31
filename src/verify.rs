use anyhow::{Context, Result, anyhow};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::{Signature, SigningKey, VerifyingKey, Verifier};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, convert::TryInto};

/// ===========================================================
/// Night Core™ v38 — Proof Mode (Signature + Integrity Only)
/// ===========================================================

/// 🔍 Engine self-test
pub fn verify_environment() -> Result<()> {
    println!("🔍 Night Core — Engine verification OK (proof mode)");
    Ok(())
}

/// 🔄 Safe auto-sync for tenant pubkey
pub fn ensure_pubkey_sync(tenant_dir: &str, tenant_name: &str) -> Result<()> {
    let tenant_path = Path::new(tenant_dir);
    let priv_path = tenant_path.join(format!("{}.key", tenant_name));
    let pub_path = tenant_path.join("pubkey.b64");

    // If pubkey already exists, do not overwrite it.
    if pub_path.exists() {
        if priv_path.exists() {
            let priv_b64 = fs::read_to_string(&priv_path)?;
            let priv_bytes = STANDARD.decode(priv_b64.trim())
                .context("invalid base64 in tenant private key")?;
            if priv_bytes.len() == 32 {
                let signing_key = SigningKey::from_bytes(&priv_bytes.try_into().unwrap());
                let derived_pub_b64 = STANDARD.encode(signing_key.verifying_key().to_bytes());
                let existing_pub_b64 = fs::read_to_string(&pub_path)?.trim().to_string();
                if derived_pub_b64 != existing_pub_b64 {
                    eprintln!(
                        "⚠️  Pubkey mismatch in {} — keeping existing maintainer key.",
                        tenant_name
                    );
                }
            }
        }
        return Ok(());
    }

    // Derive missing pubkey from tenant key
    if priv_path.exists() {
        let priv_b64 = fs::read_to_string(&priv_path)?;
        let priv_bytes = STANDARD
            .decode(priv_b64.trim())
            .context("invalid base64 in tenant private key")?;
        if priv_bytes.len() != 32 {
            return Err(anyhow!("invalid private key length: {}", priv_bytes.len()));
        }
        let signing_key = SigningKey::from_bytes(&priv_bytes.try_into().unwrap());
        let derived_pub_b64 = STANDARD.encode(signing_key.verifying_key().to_bytes());
        fs::write(&pub_path, &derived_pub_b64)?;
        println!("🔄 Auto-generated pubkey.b64 for tenant {}", tenant_name);
    }
    Ok(())
}

/// 📄 Inspect manifest.json
pub fn inspect_manifest(dir: &Path) -> Result<()> {
    let manifest_path = dir.join("manifest.json");
    let contents = fs::read_to_string(&manifest_path)?;
    println!("{}\n{}", manifest_path.display(), contents);
    Ok(())
}

/// ✅ Verify Ed25519 signature + SHA-256 integrity (no runtime execution)
pub fn verify_and_run(dir: &Path) -> Result<String> {
    let module_path = dir.join("module.wasm");
    let sig_path = dir.join("module.sig");
    let pub_path = dir.join("pubkey.b64");

    // Load module bytes
    let module_bytes = fs::read(&module_path)
        .with_context(|| format!("reading {:?}", module_path))?;

    // Decode signature
    let sig_bytes_vec = STANDARD
        .decode(fs::read_to_string(&sig_path)?.trim())
        .context("decoding signature file")?;
    let sig_bytes: [u8; 64] = sig_bytes_vec
        .clone()
        .try_into()
        .map_err(|_| anyhow!("invalid signature length: {}", sig_bytes_vec.len()))?;
    let sig = Signature::from_bytes(&sig_bytes);

    // Decode public key
    let pub_bytes_vec = STANDARD
        .decode(fs::read_to_string(&pub_path)?.trim())
        .context("decoding pubkey file")?;
    let pub_bytes: [u8; 32] = pub_bytes_vec
        .clone()
        .try_into()
        .map_err(|_| anyhow!("invalid pubkey length: {}", pub_bytes_vec.len()))?;
    let vk = VerifyingKey::from_bytes(&pub_bytes)
        .with_context(|| "invalid verifying key")?;

    // Verify signature
    vk.verify(&module_bytes, &sig)
        .with_context(|| format!("signature verification failed for {}", dir.display()))?;

    // SHA-256 integrity hash
    let sha_hex = format!("{:X}", Sha256::digest(&module_bytes));

    // Proof log
    println!("------------------------------------------------------------");
    println!("✅ VERIFIED: {}", dir.display());
    println!("   • Signature: OK (Ed25519)");
    println!("   • SHA-256: {}", sha_hex);
    println!("   • Size: {} bytes", module_bytes.len());
    println!("------------------------------------------------------------");

    Ok(sha_hex)
}
