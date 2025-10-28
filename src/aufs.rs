use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    env,
    fs,
    path::{Path, PathBuf},
};

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

/// 🔒 Verify AUFS upgrade integrity and threshold signatures.
pub fn verify_upgrade(manifest_path: &Path) -> Result<()> {
    println!("🔄 Running AUFS verification...");

    // --- Step 0: Resolve manifest relative to repo root ---
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let repo_root = cwd
        .ancestors()
        .find(|p| p.join("Cargo.toml").exists())
        .unwrap_or(&cwd)
        .to_path_buf();

    // Debug info
    println!("📁 Current working directory: {}", cwd.display());
    println!("📦 Resolved repo root: {}", repo_root.display());

    let mut resolved_path = repo_root.join(manifest_path);
    if !resolved_path.exists() {
        let fallback = repo_root.join("upgrades/manifests/upgrade_manifest.json");
        if fallback.exists() {
            println!(
                "📄 Manifest not found at {:?}, using fallback {:?}",
                manifest_path, fallback
            );
            resolved_path = fallback;
        } else {
            return Err(anyhow!(
                "Failed to locate manifest: {:?} (also checked {:?})",
                manifest_path,
                fallback
            ));
        }
    }

    println!("🗂️  Using manifest file: {}", resolved_path.display());

    // --- Step 1: Load and parse manifest ---
    let manifest_data = fs::read_to_string(&resolved_path)
        .with_context(|| format!("Failed to read manifest file at {}", resolved_path.display()))?;
    let manifest: UpgradeManifest =
        serde_json::from_str(&manifest_data).context("Failed to parse upgrade manifest JSON")?;

    // --- Step 2: Verify file hashes ---
    for (file, expected_hash) in &manifest.sha256 {
        let file_path = repo_root.join(file);
        if !file_path.exists() {
            return Err(anyhow!("Missing referenced file: {}", file_path.display()));
        }
        let actual = compute_sha256(&file_path)
            .with_context(|| format!("Failed to read file {}", file_path.display()))?;
        if actual.to_lowercase() != expected_hash.to_lowercase() {
            return Err(anyhow!(
                "SHA-256 mismatch for '{}': expected {}, got {}",
                file,
                expected_hash,
                actual
            ));
        }
        println!("✅ Hash verified for {}", file);
    }

    // --- Step 3: Load maintainer keys ---
    let keys_dir = repo_root.join("keys/maintainers");
    let key_files: Vec<_> = fs::read_dir(&keys_dir)
        .context("Reading maintainer key directory")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("pub"))
        .collect();
    if key_files.is_empty() {
        return Err(anyhow!("No maintainer keys found in {:?}", keys_dir));
    }

    // --- Step 4: Load signatures (.sig or .b64) ---
    let sig_dir = repo_root.join("upgrades/signatures");
    let sig_files: Vec<_> = fs::read_dir(&sig_dir)
        .context("Reading signatures directory")?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|ext| ext == "sig" || ext == "b64")
                .unwrap_or(false)
        })
        .collect();
    if sig_files.is_empty() {
        return Err(anyhow!("No signatures found in {:?}", sig_dir));
    }

    // --- Step 5: Use raw manifest bytes for signature input ---
    let payload =
        fs::read(&resolved_path).context("Failed to read manifest for digest computation")?;
    let audit_hash = Sha256::digest(&payload); // for log/audit chain
    let mut valid_count = 0;

    // --- Step 6: Verify signatures ---
    for sig_entry in &sig_files {
        let sig_path = sig_entry.path();
        let sig_b64 = fs::read_to_string(&sig_path).context("Reading signature file")?;
        let sig_bytes = match STANDARD.decode(sig_b64.trim()) {
            Ok(bytes) => bytes,
            Err(_) => continue,
        };
        if sig_bytes.len() != 64 {
            continue;
        }
        let signature = Signature::from_bytes(
            &sig_bytes
                .try_into()
                .expect("Invalid signature length (need 64 bytes)"),
        );

        for key_entry in &key_files {
            let key_path = key_entry.path();
            let key_raw = fs::read_to_string(&key_path).context("Reading maintainer public key")?;
            let key_bytes = STANDARD.decode(key_raw.trim()).context("Invalid base64 pubkey")?;
            if key_bytes.len() != 32 {
                continue;
            }
            let verifying_key =
                VerifyingKey::from_bytes(&key_bytes.try_into().expect("Invalid public key length"))?;

            if verifying_key.verify(&payload, &signature).is_ok() {
                valid_count += 1;
                println!(
                    "🔐 Valid signature from {}",
                    key_path.file_name().unwrap().to_string_lossy()
                );
                break;
            }
        }
    }

    // --- Step 7: Enforce threshold ---
    let required = if manifest.signatures_required == 0 {
        2
    } else {
        manifest.signatures_required
    };
    if valid_count < required {
        return Err(anyhow!(
            "AUFS threshold verification failed: only {} valid, need {}",
            valid_count,
            required
        ));
    }

    println!(
        "✅ AUFS verification passed — {} valid of {} required",
        valid_count, required
    );
    println!("🔗 Audit hash: {}", hex::encode(audit_hash));

    Ok(())
}

/// Compute SHA-256 for a file.
fn compute_sha256(file_path: &Path) -> Result<String> {
    let data = fs::read(file_path)
        .with_context(|| format!("Failed to read file {}", file_path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    Ok(hex::encode(hasher.finalize()))
}

