use anyhow::Result;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer};
use rand::rngs::OsRng;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs;

fn main() -> Result<()> {
    // 1️⃣ Read the wasm module you want to sign
    let wasm = fs::read("module.wasm")?;

    // 2️⃣ Generate keypair using the ed25519-dalek 2.2 API
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key: VerifyingKey = signing_key.verifying_key();

    // 3️⃣ Sign the wasm
    let signature: Signature = signing_key.sign(&wasm);

    // 4️⃣ Save outputs
    fs::write("pubkey.b64", STANDARD.encode(verifying_key.to_bytes()))?;
    fs::write("module.sig", STANDARD.encode(signature.to_bytes()))?;

    println!("✅ pubkey.b64 and module.sig created for module.wasm!");
    Ok(())
}



