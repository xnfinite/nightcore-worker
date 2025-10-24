use anyhow::Result;
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::PathBuf};

#[derive(Serialize, Deserialize)]
struct AuditEntry {
    v: u8,
    ts: String,
    event: String,
    details: serde_json::Value,
    prev_hash_hex: String,
    this_hash_hex: String,
}

/// Append an event to the hash-chained audit log.
pub fn append(event: &str, details: serde_json::Value) -> Result<()> {
    let path = PathBuf::from("logs/audit.jsonl");
    let tailp = PathBuf::from("logs/audit.tail");

    let prev_hash_hex = fs::read_to_string(&tailp).unwrap_or_else(|_| "0".repeat(64));

    let mut entry = AuditEntry {
        v: 1,
        ts: chrono::Utc::now().to_rfc3339(),
        event: event.to_string(),
        details,
        prev_hash_hex: prev_hash_hex.trim().to_string(),
        this_hash_hex: String::new(),
    };

    let payload = serde_json::to_vec(&entry)?;
    let mut hasher = Sha256::new();
    hasher.update(&payload);
    entry.this_hash_hex = hex::encode(hasher.finalize());

    let line = format!("{}\n", serde_json::to_string(&entry)?);

    if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
    if let Some(parent) = tailp.parent() { fs::create_dir_all(parent)?; }

    let mut f = fs::OpenOptions::new().create(true).append(true).open(&path)?;
    f.write_all(line.as_bytes())?;
    fs::write(&tailp, &entry.this_hash_hex)?;
    Ok(())
}
