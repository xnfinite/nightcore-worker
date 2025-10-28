use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
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

    // --- Step 0: Normalize repo root ---
    let repo_root = env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from("."));

    // --- Step 1: Locate manifest file ---
    let manifest_candidate = repo_root.join(manifest_path);
    let manifest_path = if manifest_candidate.exists() {
        manifest_candidate
    } else {
        let fallback = repo_root.join("upgrades/manifests/upgrade_manifest.json");
        if fallback.exists() {
            println!("📄 Manifest not found at {:?}, using fallback {:?}", manifest_path, fallback);
            fallback
        } else {
            return Err(anyhow!(
                "Manifest not found in either {:?} or fallback {:?}",
                manifest_candidate,
                fallback
            ));
        }
    };

    // --- Step 2: Load and parse manifest ---
    let manifest_data = fs::read_to_string(&manifest_path)
        .with_context(|| format!("Failed to read manifest file at {}", manifest_path.display()))?;
    let manifest: UpgradeManifest = serde_json::from_str(&manifest_data)
        .context("Failed to parse upgrade manifest JSON")?;

    // --- Step 3: Verify file hashes ---
    for (file, expected_hash) in &manifest.sha256 {
        let file_path = repo_root.join(file);
        let actual = compute_sha256(&file_path)
            .with_context(|| format!("Failed to read file {}", file_path.display()))?;
        if &actual != expected_hash {
            return Err(anyhow!(
                "SHA-256 mismatch for '{}': expected {}, got {}",
                file,
                expected_hash,
                actual
            ));
        }
        println!("✅ Hash verified for {}", file);
    }

    // --- Step 4: Load maintainer keys ---
    let keys_dir = repo_root.join("keys/maintainers");
    let key_files: Vec<_> = fs::read_dir(&keys_dir)
        .context("Reading maintainer key directory")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("pub"))
        .collect();

    if key_files.is_empty() {
        return Err(anyhow!("No maintainer keys found in {:?}", keys_dir));
    }

    // --- Step 5: Load signatures (.sig or .b64) ---
    let sig_dir = repo_root.join("upgrades/signatures");
    let sig_files: Vec<_> = fs::read_dir(&sig_dir)
