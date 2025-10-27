use anyhow::Result;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::{fs, path::Path};

fn main() -> Result<()> {
    let out_dir = Path::new("keys/maintainers");
    fs::create_dir_all(out_dir)?;

    let mut rng = OsRng;

    for name in ["admin1", "admin2"] {
        let signing_key = SigningKey::generate(&mut rng);
        let verifying_key: VerifyingKey = signing_key.verifying_key();

        // Save private key (Base64)
        fs::write(
            out_dir.join(format!("{}.key", name)),
            STANDARD.encode(signing_key.to_bytes()),
        )?;

        // Save public key (Base64)
        fs::write(
            out_dir.join(format!("{}.pub", name)),
            STANDARD.encode(verifying_key.to_bytes()),
        )?;

        println!("✅ Generated keypair for {}", name);
    }

    Ok(())
}
