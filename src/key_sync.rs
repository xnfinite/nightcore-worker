use anyhow::{Result, anyhow};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::SigningKey;
use std::{fs, convert::TryInto};
use std::array::TryFromSliceError; // ✅ added for correct error type

/// Auto-sync the public key for a tenant based on its private key.
/// Called before verification so pubkey.b64 is always current.
pub fn ensure_pubkey_sync(tenant_dir: &str, tenant_name: &str) -> Result<()> {
    let priv_path = format!("{}/{}.key", tenant_dir, tenant_name);
    let pub_path = format!("{}/pubkey.b64", tenant_dir);

    // Skip if private key missing
    if !std::path::Path::new(&priv_path).exists() {
        return Ok(());
    }

    // Read private key (base64 → bytes)
    let priv_b64 = fs::read_to_string(&priv_path)?;
    let priv_bytes = STANDARD.decode(priv_b64.trim())?;

    // ✅ FIXED: correct error type (TryFromSliceError)
    let priv_array: [u8; 32] = priv_bytes
        .as_slice()
        .try_into()
        .map_err(|_e: TryFromSliceError| anyhow!("invalid private key length: {}", priv_bytes.len()))?;

    let signing_key = SigningKey::from_bytes(&priv_array);

    // Derive and encode pubkey
    let derived_pub_b64 = STANDARD.encode(signing_key.verifying_key().to_bytes());
    let current_pub_b64 = fs::read_to_string(&pub_path).unwrap_or_default();

    // Update only if different
    if current_pub_b64.trim() != derived_pub_b64 {
        fs::write(&pub_path, &derived_pub_b64)?;
        println!("🔄 Auto-synced pubkey for tenant {}", tenant_name);
    }

    Ok(())
}
