// src/aufs.rs
use anyhow::{Context, Result};
use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::{collections::BTreeMap, fs, path::{Path, PathBuf}};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeManifest {
    pub version: String,
    pub previous_version: String,
    pub timestamp: String,
    pub description: Option<String>,
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub sha256: BTreeMap<String, String>,
    #[serde(default)]
    pub keys: Vec<String>,
    #[serde(default)]
    pub signature: Option<String>,
    #[serde(default)]
    pub signatures_required: usize,
}

/// Run AUFS verification for an upgrade manifest.
pub fn verify_upgrade(manifest_path: &Path) -> Result<()> {
    println!("🔄 Running AUFS verification...");

    let manifest_data = fs::read_to_string(manifest_path)
        .context("Failed to read manifest file")?;
    let manifest: UpgradeManifest = serde_json::from_str(&manifest_data)
        .context("Failed to parse upgrade manifest JSON")?;

    // 1️⃣ Verify file hashes
    for (file, expected_hash) in &manifest.sha256 {
        let actual = compute_sha256(Path::new(file))?;
        if &actual != expected_hash {
            return Err(anyhow::anyhow!(
                "SHA-256 mismatch for '{}': expected {}, got {}",
                file,
                expected_hash,
                actual
            ));
        }
    }

    // 2️⃣ Load all maintainer keys
    let keys_dir = PathBuf::from("keys/maintainers");
    let key_files: Vec<_> = fs::read_dir(&keys_dir)
        .context("Reading maintainer key directory")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("pub"))
        .collect();

    if key_files.is_empty() {
        return Err(anyhow::anyhow!("No maintainer keys found in {:?}", keys_dir));
    }

    // 3️⃣ Load signatures
    let sig_dir = PathBuf::from("upgrades/signatures");
    let sig_files: Vec<_> = fs::read_dir(&sig_dir)
        .context("Reading signatures directory")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("sig"))
        .collect();

    if sig_files.is_empty() {
        return Err(anyhow::anyhow!("No signatures found in {:?}", sig_dir));
    }

    // 4️⃣ Compute manifest digest
    let payload = serde_json::to_vec(&manifest)?;
    let mut hasher = Sha256::new();
    hasher.update(&payload);
    let digest = hasher.finalize();

    let mut valid_count = 0;

    // 5️⃣ Verify signatures
    for sig_entry in &sig_files {
        let sig_path = sig_entry.path();
        let sig_b64 = fs::read_to_string(&sig_path)
            .context("Reading signature file")?;
        let sig_bytes = match STANDARD.decode(sig_b64.trim()) {
            Ok(bytes) => bytes,
            Err(_) => continue,
        };

        if sig_bytes.len() != 64 {
            continue;
        }

        let signature = Signature::from_bytes(
            &sig_bytes.try_into().expect("Invalid signature length (need 64 bytes)"),
        );

        // Verify against each maintainer key
        for key_entry in &key_files {
            let key_path = key_entry.path();
            let key_raw = fs::read_to_string(&key_path)
                .context("Reading maintainer public key")?;
            let key_bytes = match STANDARD.decode(key_raw.trim()) {
                Ok(bytes) => bytes,
                Err(_) => key_raw.as_bytes().to_vec(),
            };

            if key_bytes.len() != 32 {
                continue;
            }

            let verifying_key = VerifyingKey::from_bytes(
                &key_bytes.try_into().expect("Invalid public key length (need 32 bytes)"),
            )?;

            if verifying_key.verify(&digest, &signature).is_ok() {
                valid_count += 1;
                println!(
                    "✅ Valid signature from {}",
                    key_path.file_name().unwrap().to_string_lossy()
                );
                break;
            }
        }
    }

    if valid_count < manifest.signatures_required {
        return Err(anyhow::anyhow!(
            "AUFS threshold verification failed: only {} valid, need {}",
            valid_count,
            manifest.signatures_required
        ));
    }

    // 6️⃣ Log success
    println!("✅ AUFS verification passed — hash chain updated");
    println!("🔗 Audit hash: {}", hex::encode(digest));

    Ok(())
}

/// Compute SHA-256 for a file.
pub fn compute_sha256(file_path: &Path) -> Result<String> {
    let data = fs::read(file_path)
        .with_context(|| format!("Failed to read file {}", file_path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    Ok(hex::encode(hasher.finalize()))
}
