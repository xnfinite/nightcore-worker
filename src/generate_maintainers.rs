use anyhow::Result;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("keys/maintainers")?;
    let mut rng = OsRng;

    // === Admin1 ===
    let signing_key1 = SigningKey::generate(&mut rng);
    let verifying_key1: VerifyingKey = signing_key1.verifying_key();
    fs::write("keys/maintainers/admin1.key", STANDARD.encode(signing_key1.to_bytes()))?;
    fs::write("keys/maintainers/admin1.pub", STANDARD.encode(verifying_key1.to_bytes()))?;

    // === Admin2 ===
    let signing_key2 = SigningKey::generate(&mut rng);
    let verifying_key2: VerifyingKey = signing_key2.verifying_key();
    fs::write("keys/maintainers/admin2.key", STANDARD.encode(signing_key2.to_bytes()))?;
    fs::write("keys/maintainers/admin2.pub", STANDARD.encode(verifying_key2.to_bytes()))?;

    println!("✅ Maintainer keys generated successfully in keys/maintainers/");
    Ok(())
}
