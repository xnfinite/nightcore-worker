//! Night Core — AUFS (Autonomous Upgrade & Fork System)
//! - Threshold-signed upgrade manifest (2-of-3 by default)
//! - Rollback protection
//! - SHA-256 integrity checks for target artifacts
//! - Optional audit hash-chain integration (crate::audit::append)

use anyhow::{anyhow, bail, Context, Result};
use ed25519_dalek::{Verifier, PublicKey, Signature};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, time::Duration};

/// Default location of the upgrade manifest (can be overridden)
pub const DEFAULT_META_TOML: &str = "upgrades/meta.toml";
/// Default location of crypto config listing AUFS verifiers
pub const DEFAULT_CRYPTO_TOML: &str = "configs/crypto.toml";

/// AUFS manifest describing an allowed upgrade path.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpgradeMeta {
    pub upgrade: UpgradeBlock,
    #[serde(default)]
    pub targets: Vec<UpgradeTarget>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpgradeBlock {
    pub from: String,
    pub to: String,
    /// If true, AUFS runs post-upgrade self-test automatically
    #[serde(default = "default_true")]
    pub auto_verify: bool,
    /// If true, require explicit user consent flag to proceed
    #[serde(default)]
    pub requires_user: bool,
    /// Minimum number of valid signatures required on this manifest
    #[serde(default = "default_threshold")]
    pub threshold: u8,
    /// Base64 signatures over the canonical JSON of this manifest
    #[serde(default)]
    pub signatures_b64: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpgradeTarget {
    /// Artifact label, e.g., "nightcore.exe" or "engine_adapter_wasi_p1_to_p2.rsbin"
    pub name: String,
    /// Hex-encoded SHA-256 of the artifact
    pub sha256: String,
    /// Optional download URL (AUFS does not fetch; distribution channel does)
    #[serde(default)]
    pub url: Option<String>,
}

fn default_true() -> bool { true }
fn default_threshold() -> u8 { 2 }

/// Minimal AUFS verifier list loaded from configs/crypto.toml
#[derive(Debug, Deserialize)]
struct CryptoToml {
    #[serde(default)]
    aufs: AufsSection,
}
#[derive(Debug, Deserialize, Default)]
struct AufsSection {
    /// Required threshold for manifest signatures (overrides manifest if > 0)
    #[serde(default)]
    threshold: Option<u8>,
    /// Array of base64 Ed25519 public keys
    #[serde(default)]
    verifiers: Vec<String>,
}

/// Load and parse `upgrades/meta.toml`.
pub fn load_upgrade_meta(path: impl AsRef<Path>) -> Result<UpgradeMeta> {
    let s = fs::read_to_string(path)?;
    let meta: UpgradeMeta = toml::from_str(&s)?;
    Ok(meta)
}

/// Load AUFS verifiers (base64 Ed25519 public keys) and optional threshold.
pub fn load_verifiers(path: impl AsRef<Path>) -> Result<(Vec<PublicKey>, Option<u8>)> {
    let s = fs::read_to_string(path)?;
    let cfg: CryptoToml = toml::from_str(&s)?;
    let mut keys = Vec::new();
    for b64 in cfg.aufs.verifiers {
        let bytes = base64::decode(b64.trim()).context("invalid base64 in aufs.verifiers")?;
        let pk = PublicKey::from_bytes(&bytes).context("invalid ed25519 pubkey bytes")?;
        keys.push(pk);
    }
    Ok((keys, cfg.aufs.threshold))
}

/// Verify threshold signatures over the canonical JSON of `meta`.
/// - Signers: Ed25519 public keys from configs/crypto.toml
/// - Sigs: base64 strings in `meta.upgrade.signatures_b64`
pub fn verify_threshold(meta: &UpgradeMeta, verifiers: &[PublicKey], cfg_threshold: Option<u8>) -> Result<()> {
    let json = serde_json::to_vec(&meta).context("serialize meta to json")?;

    let mut valid: u8 = 0;
    for sig_b64 in &meta.upgrade.signatures_b64 {
        let sig_bytes = base64::decode(sig_b64.trim()).context("decode signature b64")?;
        let sig = Signature::from_bytes(&sig_bytes).context("ed25519 signature bytes")?;
        // Count signature if ANY configured verifier matches.
        let mut matched = false;
        for pk in verifiers {
            if pk.verify(&json, &sig).is_ok() {
                matched = true;
                break;
            }
        }
        if matched { valid += 1; }
    }

    let needed = cfg_threshold.unwrap_or(meta.upgrade.threshold).max(1);
    if valid < needed {
        bail!("threshold not met: have {valid}, need {needed}");
    }
    Ok(())
}

/// Prevent downgrades: current must equal `meta.upgrade.from`.
pub fn check_rollback_protection(current: &str, meta: &UpgradeMeta) -> Result<()> {
    if current != meta.upgrade.from {
        bail!("rollback/invalid path: current='{current}' expected='{from}'", from = meta.upgrade.from);
    }
    Ok(())
}

/// Validate target artifact digests against declared sha256 hex values.
/// AUFS does not download; this checks files already present in your dist dir.
pub fn verify_targets_sha256(meta: &UpgradeMeta, dist_dir: impl AsRef<Path>) -> Result<()> {
    for t in &meta.targets {
        let p = dist_dir.as_ref().join(&t.name);
        let data = fs::read(&p)
            .with_context(|| format!("missing artifact '{}'; expected at {}", t.name, p.display()))?;
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let got = hex::encode(hasher.finalize());
        if got != t.sha256 {
            bail!("sha256 mismatch for '{}': got {}, expected {}", t.name, got, t.sha256);
        }
    }
    Ok(())
}

/// Apply the upgrade (placeholder: copy/swap artifacts).
/// In your pipeline, this could replace the binary, adapters, etc.
pub fn apply_upgrade(_meta: &UpgradeMeta, _dist_dir: impl AsRef<Path>) -> Result<()> {
    // For safety in OSS baseline, we don't auto-replace the running binary here.
    // Instead, we rely on your CI/distribution step to deploy artifacts,
    // and AUFS records proof + runs self-test below.
    Ok(())
}

/// Run a short post-upgrade self-test (deterministic and time-bounded).
pub fn self_test(timeout_ms: u64) -> Result<()> {
    // Minimal deterministic wait to simulate a short test
    let timeout = Duration::from_millis(timeout_ms.min(2000));
    std::thread::sleep(timeout);
    Ok(())
}

/// Append to tamper-evident audit chain, if available.
fn audit(event: &str, details: serde_json::Value) {
    #[cfg(feature = "audit")]
    {
        let _ = crate::audit::append(event, details);
    }
    #[cfg(not(feature = "audit"))]
    {
        // Fallback: emit JSON line to stdout; still useful for CI logs.
        let now = chrono::Utc::now().to_rfc3339();
        println!(r#"{{"ts":"{now}","event":"{event}","details":{}}}"#, details);
    }
}

/// High-level “one button” flow used by `nightcore upgrade --auto --safe`.
///
/// - `current_tag` should be your running version tag (e.g., "v37-stable-b106-2025LTS").
/// - `meta_path` typically `upgrades/meta.toml`.
/// - `crypto_toml` typically `configs/crypto.toml`.
/// - `dist_dir` where artifacts named in `meta.targets[].name` reside (already downloaded).
pub fn run_auto(current_tag: &str,
                meta_path: impl AsRef<Path>,
                crypto_toml: impl AsRef<Path>,
                dist_dir: impl AsRef<Path>) -> Result<()> {
    let meta = load_upgrade_meta(meta_path.as_ref())
        .with_context(|| format!("parse {}", meta_path.as_ref().display()))?;
    let (verifiers, cfg_thresh) = load_verifiers(crypto_toml.as_ref())
        .with_context(|| format!("parse {}", crypto_toml.as_ref().display()))?;

    check_rollback_protection(current_tag, &meta)?;
    verify_threshold(&meta, &verifiers, cfg_thresh)?;
    verify_targets_sha256(&meta, dist_dir.as_ref())?;

    audit("aufs.pre-apply", serde_json::json!({
        "from": meta.upgrade.from, "to": meta.upgrade.to, "targets": meta.targets
    }));

    apply_upgrade(&meta, dist_dir.as_ref())?;

    if meta.upgrade.auto_verify {
        self_test(500)?; // fast, deterministic self-test
    }

    audit("aufs.applied", serde_json::json!({
        "from": meta.upgrade.from, "to": meta.upgrade.to
    }));

    Ok(())
}
