use anyhow::Result;
use ed25519_dalek::{Signer, SigningKey};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::{fs, env, convert::TryInto};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: sign_upgrade <path-to-key> <manifest>");
        std::process::exit(1);
    }

    let key_path = &args[1];
    let manifest_path = &args[2];

    // Read the private key and manifest
    let key_b64 = fs::read_to_string(key_path)?;
let key_bytes = base64::engine::general_purpose::STANDARD.decode(key_b64.trim())?;
let signing_key = SigningKey::from_bytes(&key_bytes.try_into().unwrap());

    let data = fs::read(manifest_path)?;

    // Sign and output .sig file
    let signature = signing_key.sign(&data);
    let sig_b64 = STANDARD.encode(signature.to_bytes());
    fs::write(format!("{}.sig", manifest_path), sig_b64)?;

    println!("✅ Signed: {}.sig", manifest_path);
    Ok(())
}
