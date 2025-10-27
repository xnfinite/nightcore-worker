use anyhow::{anyhow, Result};
use ed25519_dalek::{Signer, SigningKey};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::{fs, convert::TryInto, path::Path};

/// ✍️ Sign an upgrade manifest using a Base64 Ed25519 private key.
/// Produces a `.sig.b64` file for AUFS verification.
pub fn sign_upgrade(manifest_path: &Path, key_path: &Path, out_path: &Path) -> Result<()> {
    let key_b64 = fs::read_to_string(key_path)
        .map_err(|e| anyhow!("Failed to read private key: {e}"))?;
    let key_bytes = STANDARD
        .decode(key_b64.trim())
        .map_err(|e| anyhow!("Invalid base64 in private key: {e}"))?;

    if key_bytes.len() != 32 {
        return Err(anyhow!("Invalid key length — expected 32 bytes, got {}", key_bytes.len()));
    }

    let data = fs::read(manifest_path)
        .map_err(|e| anyhow!("Failed to read manifest: {e}"))?;
    let signing_key = SigningKey::from_bytes(&key_bytes.try_into().unwrap());
    let signature = signing_key.sign(&data);

    let sig_b64 = STANDARD.encode(signature.to_bytes());
    fs::write(out_path, sig_b64)
        .map_err(|e| anyhow!("Failed to write signature: {e}"))?;

    println!("✅ Signed manifest → {}", out_path.display());
    Ok(())
}
