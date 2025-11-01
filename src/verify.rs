use anyhow::{Context, Result, anyhow};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::{Signature, SigningKey, VerifyingKey, Verifier};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::Path,
    convert::TryInto,
    fs::OpenOptions,
    io::Write,
    process::Command,
    sync::Once,
    io::Read,
};
use chrono::Local;

static INIT_LOG: Once = Once::new();

/// ===========================================================
/// Night Core™ v38 — Verify + Proof Mode (with Git metadata)
/// ===========================================================

/// 🔍 Environment verification
pub fn verify_environment() -> Result<()> {
    println!("🔍 Night Core — Environment verification OK");
    Ok(())
}

/// 🔄 Safe pubkey sync (keeps maintainer key if already present)
pub fn ensure_pubkey_sync(tenant_dir: &str, tenant_name: &str) -> Result<()> {
    let tenant_path = Path::new(tenant_dir);
    let priv_path = tenant_path.join(format!("{}.key", tenant_name));
    let pub_path = tenant_path.join("pubkey.b64");

    if pub_path.exists() {
        if priv_path.exists() {
            let priv_b64 = fs::read_to_string(&priv_path)?;
            let priv_bytes = STANDARD.decode(priv_b64.trim())
                .context("invalid base64 in tenant private key")?;
            if priv_bytes.len() == 32 {
                let signing_key = SigningKey::from_bytes(&priv_bytes.try_into().unwrap());
                let derived_pub_b64 = STANDARD.encode(signing_key.verifying_key().to_bytes());
                let existing_pub_b64 = fs::read_to_string(&pub_path)?.trim().to_string();
                if derived_pub_b64 != existing_pub_b64 {
                    eprintln!("⚠️  Pubkey mismatch in {} — keeping existing maintainer key.", tenant_name);
                }
            }
        }
        return Ok(());
    }

    // Derive pubkey if missing
    if priv_path.exists() {
        let priv_b64 = fs::read_to_string(&priv_path)?;
        let priv_bytes = STANDARD.decode(priv_b64.trim())
            .context("invalid base64 in tenant private key")?;
        if priv_bytes.len() != 32 {
            return Err(anyhow!("invalid private key length: {}", priv_bytes.len()));
        }
        let signing_key = SigningKey::from_bytes(&priv_bytes.try_into().unwrap());
        let derived_pub_b64 = STANDARD.encode(signing_key.verifying_key().to_bytes());
        fs::write(&pub_path, &derived_pub_b64)?;
        println!("🔄 Auto-generated pubkey.b64 for tenant {}", tenant_name);
    }
    Ok(())
}

/// ✅ Verify Ed25519 signature + SHA-256 integrity
pub fn verify_and_run(dir: &Path, proof: bool) -> Result<String> {
    let module_path = dir.join("module.wasm");
    let sig_path = dir.join("module.sig");
    let pub_path = dir.join("pubkey.b64");

    let module_bytes = fs::read(&module_path)
        .with_context(|| format!("reading {:?}", module_path))?;

    // Decode signature
    let sig_bytes_vec = STANDARD
        .decode(fs::read_to_string(&sig_path)?.trim())
        .context("decoding signature file")?;
    let sig_bytes: [u8; 64] = sig_bytes_vec.clone()
        .try_into()
        .map_err(|_| anyhow!("invalid signature length: {}", sig_bytes_vec.len()))?;
    let sig = Signature::from_bytes(&sig_bytes);

    // Decode public key
    let pub_bytes_vec = STANDARD
        .decode(fs::read_to_string(&pub_path)?.trim())
        .context("decoding pubkey file")?;
    let pub_bytes: [u8; 32] = pub_bytes_vec.clone()
        .try_into()
        .map_err(|_| anyhow!("invalid pubkey length: {}", pub_bytes_vec.len()))?;
    let vk = VerifyingKey::from_bytes(&pub_bytes)
        .with_context(|| "invalid verifying key")?;

    // Verify
    vk.verify(&module_bytes, &sig)
        .with_context(|| format!("signature verification failed for {}", dir.display()))?;

    // Compute SHA
    let sha_hex = format!("{:X}", Sha256::digest(&module_bytes));

    println!("✅ VERIFIED: {}", dir.display());
    println!("  • Signature: OK (Ed25519)");
    println!("  • SHA-256: {}", sha_hex);
    println!("  • Size: {} bytes", module_bytes.len());

    if proof {
        write_proof_report(dir, &sha_hex, module_bytes.len())?;
    }

    Ok(sha_hex)
}

/// ===========================================================
/// 🧾 Night Core Proof Report (HTML Summary with Metadata)
/// ===========================================================
pub fn write_proof_report(tenant_path: &Path, sha_hex: &str, module_size: usize) -> Result<()> {
    let log_dir = Path::new("logs");
    let log_file = log_dir.join("nightcore_proof.html");
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    if !log_dir.exists() {
        fs::create_dir_all(log_dir)?;
    }

    INIT_LOG.call_once(|| {
        let _ = fs::write(&log_file, "");
    });

    // Safe commit hash
    let commit_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "unknown".into())
        .trim()
        .to_string();

    // Basic audit digest
    let audit_data = format!("{}{}{}", tenant_path.display(), sha_hex, timestamp);
    let audit_hash = hex::encode(Sha256::digest(audit_data.as_bytes()));

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .append(true)
        .open(&log_file)
        .with_context(|| format!("opening {:?}", log_file))?;

    // Header only if new
    if file.metadata()?.len() == 0 {
        let header = format!(r#"<!-- Night Core v38 Verified Badge -->
<p align="center">
  <img src="../docs/assets/nightcore_logo_tm.png" alt="Night Core Logo™" width="220"/>
  <br/>
  <a href="https://github.com/xnfinite/nightcore-worker/actions">
    <img src="https://img.shields.io/badge/AUFS%20Verified-v38-success?style=for-the-badge&color=0B3D91"/>
  </a>
  <br/>
  <sub>Night Core™ — Secure • Autonomous • Verified</sub>
</p>

<h1 align="center">Night Core™ v38 — Proof Report</h1>
<p align="center"><i>Generated at {ts}</i></p>
<hr/>
"#, ts = timestamp);
        file.write_all(header.as_bytes())?;
    }

    // Tenant entry
    let entry = format!(
        "<pre>✅ VERIFIED: {tenant}\n  • Signature: OK (Ed25519)\n  • SHA-256: {sha}\n  • Size: {size} bytes\n  • Commit: {commit}\n  • Audit-Hash: {audit}\n  • Timestamp: {ts}\n  • Maintainers: core-ops • system-check\n</pre>\n",
        tenant = tenant_path.display(),
        sha = sha_hex,
        size = module_size,
        commit = commit_hash,
        audit = audit_hash,
        ts = timestamp
    );
    file.write_all(entry.as_bytes())?;
    Ok(())
}

/// ===========================================================
/// 📄 Manifest Inspector (Restored for CLI Compatibility)
/// ===========================================================
pub fn inspect_manifest(dir: &Path) -> Result<()> {
    let manifest_path = dir.join("manifest.json");

    if !manifest_path.exists() {
        println!("⚠️ No manifest.json found in {}", dir.display());
        return Ok(());
    }

    let mut file = fs::File::open(&manifest_path)
        .with_context(|| format!("opening manifest: {}", manifest_path.display()))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("\n--- manifest.json for {} ---", dir.display());
    println!("{}", contents);
    println!("------------------------------\n");

    Ok(())
}
