use std::{fs, path::PathBuf};
use anyhow::{Context, Result};
use clap::Parser;
use sha2::{Digest, Sha256};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::{SigningKey, Signer};

/// Generate a Night Core™ Pro-style license_unlock.key with a real Ed25519 signature.
/// Signs the message:  LicenseID || DeviceHash || UnlockToken
#[derive(Parser, Debug)]
#[command(name="make_unlock", about="Generate a signed license_unlock.key")]
struct Args {
    /// License ID (e.g., NC-PRO-007)
    #[arg(long)]
    license_id: String,

    /// Short unlock token (any string, e.g., auto-increment or GUID)
    #[arg(long)]
    unlock_token: String,

    /// Path to Base64 private key (32 bytes)
    #[arg(long)]
    key: PathBuf,

    /// Path to device id file (default: device.id)
    #[arg(long, default_value = "device.id")]
    device_id: PathBuf,

    /// Output path (default: license_unlock.key)
    #[arg(long, default_value = "license_unlock.key")]
    out: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // --- Load and hash device id (sha256 of trimmed, lower-cased contents)
    let device_raw = fs::read_to_string(&args.device_id)
        .with_context(|| format!("reading {}", args.device_id.display()))?;
    let device_norm = device_raw.trim().to_lowercase();
    let device_hash = format!("{:x}", Sha256::digest(device_norm.as_bytes()));

    // --- Load Base64 private key (32 bytes)
    let key_b64 = fs::read_to_string(&args.key)
        .with_context(|| format!("reading {}", args.key.display()))?;
    let sk_bytes = STANDARD
        .decode(key_b64.trim())
        .context("private key is not valid base64")?;
    if sk_bytes.len() != 32 {
        anyhow::bail!("private key must be 32 bytes after base64-decoding");
    }
    let sk = SigningKey::from_bytes(
        &sk_bytes
            .as_slice()
            .try_into()
            .expect("length checked above (32)"),
    );

    // --- Build message and sign (LicenseID || DeviceHash || UnlockToken)
    let message = format!("{}{}{}", args.license_id, device_hash, args.unlock_token);
    let sig = sk.sign(message.as_bytes());
    let sig_b64 = STANDARD.encode(sig.to_bytes());

    // --- Write license_unlock.key
    let out = format!(
        "LicenseID: {lid}\nDeviceHash: {dh}\nUnlockToken: {tok}\nSignature: {sig}\n",
        lid = args.license_id,
        dh  = device_hash,
        tok = args.unlock_token,
        sig = sig_b64
    );
    fs::write(&args.out, out).with_context(|| format!("writing {}", args.out.display()))?;

    println!("✅ Wrote {}", args.out.display());
    println!("   LicenseID  : {}", args.license_id);
    println!("   DeviceHash : {}", device_hash);
    println!("   UnlockToken: {}", args.unlock_token);
    Ok(())
}
