use anyhow::{anyhow, Context, Result};
use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

/// AUFS upgrade manifest schema.
#[derive(Debug, Deserialize)]
pub struct AufsManifest {
    pub version: String,
    pub timestamp: String,
    pub description: Option<String>,
    pub files: Vec<String>,
    pub sha256: BTreeMap<String, String>,
    pub previous_version: String,
    pub signatures_required: usize,
}

/// Verify the manifest’s file hashes and threshold signatures.
/// Returns number of verified maintainer signatures.
pub fn verify_manifest_threshold(repo_root: &Path, manifest_path: &Path) -> Result<usize> {
    let manifest_bytes = fs::read(manifest_path)
        .with_context(|| format!("Failed to read manifest: {}", manifest_path.display()))?;

    let manifest: AufsManifest =
        serde_json::from_slice(&manifest_bytes).context("Manifest JSON parse failed")?;

    verify_all_file_hashes(repo_root, &manifest)?;
    let verified =
        verify_threshold_signatures(repo_root, &manifest.version, &manifest_bytes, manifest.signatures_required)?;
    Ok(verified)
}

fn verify_all_file_hashes(repo_root: &Path, manifest: &AufsManifest) -> Result<()> {
    if manifest.files.is_empty() {
        return Err(anyhow!("Manifest lists no files"));
    }
    for file in &manifest.files {
        let expected = manifest
            .sha256
            .get(file)
            .ok_or_else(|| anyhow!("Missing sha256 entry for {}", file))?;
        let path = repo_root.join(file);
        let bytes = fs::read(&path)
            .with_context(|| format!("Failed to read '{}'", path.display()))?;
        let actual = sha256_hex(&bytes);
        if &actual != expected {
            return Err(anyhow!(
                "SHA-256 mismatch for '{}': expected {}, got {}",
                file,
                expected,
                actual
            ));
        }
    }
    Ok(())
}

fn verify_threshold_signatures(
    repo_root: &Path,
    version: &str,
    payload: &[u8],
    required: usize,
) -> Result<usize> {
    if required == 0 {
        return Err(anyhow!("signatures_required must be >= 1"));
    }

    let pub_dir = repo_root.join("keys").join("maintainers");
    let sig_dir = repo_root.join("upgrades").join("signatures");

    let mut pubkeys: Vec<VerifyingKey> = Vec::new();
    if pub_dir.is_dir() {
        for e in fs::read_dir(&pub_dir)? {
            let e = e?;
            if !e.file_type()?.is_file() {
                continue;
            }
            let p = e.path();
            if p.extension().and_then(|x| x.to_str()) != Some("pub") {
                continue;
            }
            let bytes = parse_key_or_sig(&fs::read(&p)?)?;
            if bytes.len() == 32 {
                let vk = VerifyingKey::from_bytes(&bytes.try_into().unwrap())?;
                pubkeys.push(vk);
            }
        }
    } else {
        return Err(anyhow!("Missing maintainer keys: {}", pub_dir.display()));
    }

    if pubkeys.is_empty() {
        return Err(anyhow!("No maintainer keys found in {}", pub_dir.display()));
    }

    let mut sig_files = Vec::new();
    if sig_dir.is_dir() {
        for e in fs::read_dir(&sig_dir)? {
            let e = e?;
            if !e.file_type()?.is_file() {
                continue;
            }
            let p = e.path();
            if p.extension().and_then(|x| x.to_str()) != Some("sig") {
                continue;
            }
            if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
                if stem.starts_with(version) {
                    sig_files.push(p);
                }
            }
        }
    }
    if sig_files.is_empty() {
        return Err(anyhow!(
            "No signatures found for version '{}' in {}",
            version,
            sig_dir.display()
        ));
    }

    let mut unique = HashSet::new();
    for sig_path in sig_files {
        let sig_bytes = parse_key_or_sig(&fs::read(&sig_path)?)?;
        if sig_bytes.len() != 64 {
            return Err(anyhow!("Bad signature size: {}", sig_path.display()));
        }
        let sig = Signature::from_bytes(&sig_bytes.try_into().unwrap());
        for (i, key) in pubkeys.iter().enumerate() {
            if key.verify(payload, &sig).is_ok() {
                unique.insert(i);
                break;
            }
        }
    }

    let verified = unique.len();
    if verified < required {
        return Err(anyhow!("Threshold not met: required {}, got {}", required, verified));
    }
    Ok(verified)
}

fn sha256_hex(b: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(b);
    hex::encode(h.finalize())
}

fn parse_key_or_sig(raw: &[u8]) -> Result<Vec<u8>> {
    if raw.len() == 32 || raw.len() == 64 {
        return Ok(raw.to_vec());
    }
    let s = String::from_utf8_lossy(raw).trim().to_string();
    if let Ok(b) = base64::decode(&s) {
        return Ok(b);
    }
    if let Ok(b) = hex::decode(&s) {
        return Ok(b);
    }
    Err(anyhow!("Unsupported key/signature encoding"))
}

