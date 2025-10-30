use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: verify_sig <pubkey.b64> <sig.b64> <manifest.json>");
        std::process::exit(1);
    }

    let pub_path = Path::new(&args[1]);
    let sig_path = Path::new(&args[2]);
    let manifest_path = Path::new(&args[3]);

    let pub_b64 = fs::read_to_string(pub_path).expect("Failed to read pubkey");
    let pub_bytes = STANDARD.decode(pub_b64.trim()).expect("Invalid base64 pubkey");
    let verifying_key =
        VerifyingKey::from_bytes(&pub_bytes.try_into().expect("Invalid pubkey length"))
            .expect("Failed to load verifying key");

    let sig_b64 = fs::read_to_string(sig_path).expect("Failed to read signature");
    let sig_bytes = STANDARD.decode(sig_b64.trim()).expect("Invalid base64 signature");
    let signature =
        Signature::from_bytes(&sig_bytes.try_into().expect("Invalid signature length"));

    let payload = fs::read(manifest_path).expect("Failed to read manifest");

    match verifying_key.verify(&payload, &signature) {
        Ok(_) => println!("✅ Signature verified successfully for {}", pub_path.display()),
        Err(_) => {
            eprintln!("❌ Signature invalid for {}", pub_path.display());
            std::process::exit(1);
        }
    }
}
