use anyhow::Result;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs;

/// 🔐 Generate two maintainer keypairs (Admin1 + Admin2)
/// Called from: `nightcore generate-keys --out-dir <path>`
pub fn generate_keys(out_dir: &str) -> Result<()> {
    let target_dir = format!("{}/maintainers", out_dir);
    fs::create_dir_all(&target_dir)?;
    let mut rng = OsRng;

    // === Admin1 ===
    let signing_key1 = SigningKey::generate(&mut rng);
    let verifying_key1: VerifyingKey = signing_key1.verifying_key();
    fs::write(format!("{}/admin1.key", target_dir), STANDARD.encode(signing_key1.to_bytes()))?;
    fs::write(format!("{}/admin1.pub", target_dir), STANDARD.encode(verifying_key1.to_bytes()))?;

    // === Admin2 ===
    let signing_key2 = SigningKey::generate(&mut rng);
    let verifying_key2: VerifyingKey = signing_key2.verifying_key();
    fs::write(format!("{}/admin2.key", target_dir), STANDARD.encode(signing_key2.to_bytes()))?;
    fs::write(format!("{}/admin2.pub", target_dir), STANDARD.encode(verifying_key2.to_bytes()))?;

    println!("✅ Maintainer keys generated successfully in {target_dir}");
    Ok(())
}
