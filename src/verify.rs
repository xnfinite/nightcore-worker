use anyhow::{Context, Result, anyhow};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::{Signature, SigningKey, Signer, VerifyingKey, Verifier};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
use wasmtime::{Engine, Module, Store, Instance, Func};

/// ===========================================================
/// Night Core v38 — Verify & Run (No-WASI baseline)
/// ===========================================================

/// 🔍 Engine self-test
pub fn verify_environment() -> Result<()> {
    println!("🔍 Night Core — Engine verification (no WASI)");
    let engine = Engine::default();
    let wat = r#"(module (func (export "main")))"#;
    let module = Module::new(&engine, wat)?;
    let mut store = Store::new(&engine, ());
    let _ = Instance::new(&mut store, &module, &[])?;
    println!("Night Core OK ✅ Engine initialized (no WASI needed)");
    Ok(())
}

/// 🔄 Auto-sync tenant pubkey
pub fn ensure_pubkey_sync(tenant_dir: &str, tenant_name: &str) -> Result<()> {
    let priv_path = Path::new(tenant_dir).join(format!("{}.key", tenant_name));
    let pub_path = Path::new(tenant_dir).join("pubkey.b64");
    if !priv_path.exists() { return Ok(()); }

    let priv_b64 = fs::read_to_string(&priv_path)?;
    let priv_bytes = STANDARD.decode(priv_b64.trim())?;
    if priv_bytes.len() != 32 {
        return Err(anyhow!("invalid private key length: {}", priv_bytes.len()));
    }

    let signing_key = SigningKey::from_bytes(&priv_bytes.try_into().unwrap());
    let derived_pub_b64 = STANDARD.encode(signing_key.verifying_key().to_bytes());
    let current_pub_b64 = fs::read_to_string(&pub_path).unwrap_or_default();

    if current_pub_b64.trim() != derived_pub_b64 {
        fs::write(&pub_path, &derived_pub_b64)?;
        println!("🔄 Auto-synced Ed25519 pubkey for tenant {}", tenant_name);
    }
    Ok(())
}

/// ✍️ Sign a module with Ed25519
pub fn sign_module(dir: &Path, key_path: &Path) -> Result<()> {
    let module_path = dir.join("module.wasm");
    let module_bytes = fs::read(&module_path)?;
    let key_b64 = fs::read_to_string(key_path)?;
    let sk_bytes = STANDARD.decode(key_b64.trim())?;
    let signing_key = SigningKey::from_bytes(&sk_bytes.try_into().unwrap());
    let sig = signing_key.sign(&module_bytes);
    fs::write(dir.join("module.sig"), STANDARD.encode(sig.to_bytes()))?;
    println!("✅ Signed {}", dir.display());
    Ok(())
}

/// 📄 Inspect manifest
pub fn inspect_manifest(dir: &Path) -> Result<()> {
    let manifest_path = dir.join("manifest.json");
    let contents = fs::read_to_string(&manifest_path)?;
    println!("{}\n{}", manifest_path.display(), contents);
    Ok(())
}

/// ✅ Verify Ed25519 signature and execute tenant module
pub fn verify_and_run(dir: &Path) -> Result<String> {
    let module_path = dir.join("module.wasm");
    let sig_path = dir.join("module.sig");
    let pub_path = dir.join("pubkey.b64");

    let module_bytes = fs::read(&module_path)
        .with_context(|| format!("reading {:?}", module_path))?;

    // --- verify signature ---
    let sig_bytes_vec = STANDARD.decode(fs::read_to_string(&sig_path)?.trim())?;
    let sig_bytes_clone = sig_bytes_vec.clone();
    let sig_bytes: [u8; 64] = sig_bytes_clone
        .try_into()
        .map_err(|_| anyhow!("invalid signature length: {}", sig_bytes_vec.len()))?;
    let sig = Signature::from_bytes(&sig_bytes);

    let pub_bytes_vec = STANDARD.decode(fs::read_to_string(&pub_path)?.trim())?;
    let pub_bytes_clone = pub_bytes_vec.clone();
    let pub_bytes: [u8; 32] = pub_bytes_clone
        .try_into()
        .map_err(|_| anyhow!("invalid pubkey length: {}", pub_bytes_vec.len()))?;
    let vk = VerifyingKey::from_bytes(&pub_bytes)?;
    vk.verify(&module_bytes, &sig).context("signature verification failed")?;

    // --- SHA-256 audit hash ---
    let sha_hex = format!("{:X}", Sha256::digest(&module_bytes));

    // --- Instantiate & run ---
    let engine = Engine::default();
    let module = Module::new(&engine, &module_bytes)?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])
        .with_context(|| "instantiating module (if this fails with missing imports, the module needs WASI)")?;

    // Find entry
    let entry = find_entry(&instance, &mut store)
        .ok_or_else(|| anyhow!("no callable entrypoint found (tried: _start, main, run)"))?;

    // Call entry
        if let Err(e) = entry.call(&mut store, &[], &mut []) {
        let msg = e.to_string();
        let safe = String::from_utf8_lossy(msg.as_bytes());
        eprintln!("⚠️ Tenant {} runtime error: {}", dir.display(), safe);
    } else {
        // 🧠 After module executes, capture possible stdout files (optional future use)
        println!("🏁 Tenant execution complete: {}", dir.display());

        // 🔒 Safe UTF-8 handling for any extra sandbox output
        if let Ok(extra) = fs::read(dir.join("sandbox/msg.txt")) {
            let safe_output = String::from_utf8_lossy(&extra).to_string();
            if !safe_output.trim().is_empty() {
                println!("🧠 Tenant output:\n{}", safe_output);
            }
        }
    }

    Ok(sha_hex)

}

/// Helper: find `_start`, `main`, or `run`
fn find_entry(instance: &Instance, store: &mut Store<()>) -> Option<Func> {
    if let Some(f) = instance.get_func(&mut *store, "_start") { return Some(f); }
    if let Some(f) = instance.get_func(&mut *store, "main") { return Some(f); }
    if let Some(f) = instance.get_func(&mut *store, "run") { return Some(f); }
    None
}
