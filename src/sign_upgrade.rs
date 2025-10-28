use anyhow::{anyhow, Result};
use ed25519_dalek::{Signer, SigningKey};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::{fs, convert::TryInto, path::{Path, PathBuf}};

/// ✍️ Sign an upgrade manifest using Base64 Ed25519 private keys.
/// Produces `.sig.b64` files for AUFS verification.
pub fn sign_upgrade(manifest_path: &Path) -> Result<()> {
    println!("🔏 Signing AUFS manifest: {}", manifest_path.display());

    let data = fs::read(manifest_path)
        .map_err(|e| anyhow!("Failed to read manifest: {e}"))?;

    // Maintainer keys directory
    let maintainers_dir = PathBuf::from("upgrades/signatures/maintainers");
    let output_dir = PathBuf::from("upgrades/signatures");

    // Keys to sign with
    let admins = ["admin1", "admin2"];

    for admin in admins {
        let key_path = maintainers_dir.join(format!("{admin}.key"));
        if !key_path.exists() {
            println!("⚠️  Skipping {} — key not found at {}", admin, key_path.display());
            continue;
        }

        // Read key and decode
        let key_b64 = fs::read_to_string(&key_path)
            .map_err(|e| anyhow!("Failed to read private key {admin}: {e}"))?;
        let key_bytes = STANDARD
            .decode(key_b64.trim())
            .map_err(|e| anyhow!("Invalid base64 in {admin} key: {e}"))?;

        if key_bytes.len() != 32 {
            return Err(anyhow!("Invalid key length for {admin} — expected 32 bytes, got {}", key_bytes.len()));
        }

        // Sign data
        let signing_key = SigningKey::from_bytes(&key_bytes.try_into().unwrap());
        let signature = signing_key.sign(&data);
        let sig_b64 = STANDARD.encode(signature.to_bytes());

        // Output to versioned file
        let out_path = output_dir.join(format!("v38_{admin}.sig"));
        fs::write(&out_path, sig_b64)
            .map_err(|e| anyhow!("Failed to write signature: {e}"))?;

        println!("✅ Signed manifest → {}", out_path.display());
    }

    println!("🎯 All available maintainer signatures written successfully.");
    Ok(())
}
