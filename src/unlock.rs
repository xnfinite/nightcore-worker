use std::fs;
use sha2::{Digest, Sha256};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::{Signature, VerifyingKey, Verifier};

/// ===========================================================
/// 🔐 Night Core™ Pro — License Unlock Verifier (Public-safe)
/// ===========================================================
/// Runs inside the open-core Night Core Worker.
/// Validates `license_unlock.key` issued by Night Core™ Pro.
/// No private key material is stored or embedded here.
/// ===========================================================
pub fn check_unlock() -> bool {
    let path = "license_unlock.key";
    let data = match fs::read_to_string(path) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("⚠️ No license_unlock.key found — running in open-core mode");
            return false;
        }
    };

    // --- Parse license fields ---
    let mut license_id = String::new();
    let mut device_hash = String::new();
    let mut unlock_token = String::new();
    let mut signature_b64 = String::new();

    for line in data.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("LicenseID:") {
            license_id = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("DeviceHash:") {
            device_hash = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("UnlockToken:") {
            unlock_token = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("Signature:") {
            signature_b64 = rest.trim().to_string();
        }
    }

    if license_id.is_empty() || unlock_token.is_empty() || signature_b64.is_empty() {
        eprintln!("⚠️ Invalid or incomplete license_unlock.key");
        return false;
    }

    // --- Load embedded Pro public key (from Night Core™ Pro) ---
    const PUBKEY_B64: &str = "atEOzpDAAxJC94Mk/Shc5Lc0KqTnTq/iHfOcLdnA3vc=";
    let pub_bytes = match STANDARD.decode(PUBKEY_B64) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("⚠️ Failed to decode embedded Pro public key.");
            return false;
        }
    };

    let verify_key = match VerifyingKey::from_bytes(&pub_bytes.try_into().unwrap()) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("⚠️ Invalid Pro public key format.");
            return false;
        }
    };

    // --- Reconstruct signed message ---
    let message = format!("{}{}{}", license_id, device_hash, unlock_token);
    let sig_bytes = match STANDARD.decode(&signature_b64) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("⚠️ Failed to decode signature.");
            return false;
        }
    };

    // --- Safely construct signature object ---
    let sig_bytes_arr: [u8; 64] = match sig_bytes.as_slice().try_into() {
        Ok(arr) => arr,
        Err(_) => {
            eprintln!("⚠️ Signature length invalid (expected 64 bytes).");
            return false;
        }
    };
    let sig_obj = Signature::from_bytes(&sig_bytes_arr);

    // --- Optional local integrity check (device binding, normalized) ---
    if !device_hash.is_empty() {
        let local_id = fs::read_to_string("device.id").unwrap_or_default();
        // normalize for cross-platform consistency
        let normalized = local_id.replace("\r", "").trim().to_lowercase();
        let calc_hash = format!("{:x}", Sha256::digest(normalized.as_bytes()));
        if calc_hash != device_hash {
            eprintln!("⚠️ Device hash mismatch — license not bound to this node");
            return false;
        }
    }

    // --- Verify signature ---
    match verify_key.verify(message.as_bytes(), &sig_obj) {
        Ok(_) => {
            println!("🔓 Night Core™ Pro license verified — advanced features unlocked");
            true
        }
        Err(_) => {
            eprintln!("🔒 License verification failed — running in open-core mode");
            false
        }
    }
}
