use ed25519_dalek::SigningKey;
use base64::{engine::general_purpose::STANDARD, Engine as _};

fn main() {
    let sk_bytes = STANDARD.decode("9+oQ52WFe4m+w2e1kNg+mV0vFpI6v//t0DJeGBJfrtA=").unwrap();
    let sk = SigningKey::from_bytes(&sk_bytes.try_into().unwrap());
    let pk = sk.verifying_key();
    println!("{}", STANDARD.encode(pk.to_bytes()));
}
