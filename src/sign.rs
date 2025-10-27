use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::{Signature, SigningKey, Signer};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

/// Sign a tenant's module.wasm with the given Ed25519 private key.
/// Creates module.sig, pubkey.b64, and module.sha256 alongside the module.
pub fn sign_tenant(dir: &Path, key_path: &Path) -> Result<()> {
    let module_path = dir.join("module.wasm");

    let module_bytes = fs::read(&module_path)
        .with_context(|| format!("Failed to read {}", module_path.display()))?;

    // Load and decode Base64 private key
    let key_b64 = fs::read_to_string(key_path)
        .with_context(|| format!("Failed to read key {}", key_path.display()))?;
    let sk_bytes = STANDARD
        .decode(key_b64.trim())
        .context("Private key is not valid base64")?;

    if sk_bytes.len() != 32 {
        anyhow::bail!(
            "Invalid private key length: expected 32 bytes after base64 decode, got {}",
            sk_bytes.len()
        );
    }

    let signing_key =
        SigningKey::from_bytes(sk_bytes.as_slice().try_into().expect("slice to [u8; 32]"));
    let verifying_key = signing_key.verifying_key();

    // Sign module bytes
    let signature: Signature = signing_key
        .try_sign(&module_bytes)
        .context("Failed to sign module bytes")?;

    // Output artifacts
    let sig_path = dir.join("module.sig");
    let pub_path = dir.join("pubkey.b64");
    let sha_path = dir.join("module.sha256");

    fs::write(&sig_path, STANDARD.encode(signature.to_bytes()))?;
    fs::write(&pub_path, STANDARD.encode(verifying_key.to_bytes()))?;

    // Record module SHA256 for audit
    let mut h = Sha256::new();
    h.update(&module_bytes);
    let sha = hex::encode(h.finalize());
    fs::write(&sha_path, format!("{sha}\n"))?;

    println!("✅ Signed module:");
    println!("   - {}", sig_path.display());
    println!("   - {}", pub_path.display());
    println!("   - {}", sha_path.display());
    Ok(())
}
