use anyhow::Result;
use std::{fs, path::PathBuf, rc::Rc, cell::RefCell};
use std::sync::{Arc, Mutex}; // keep for future threading
use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::{Signature, VerifyingKey, Verifier, SigningKey, Signer};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use wasmtime::{Config, Engine, Linker, Module, Store, StoreLimits, StoreLimitsBuilder};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder};
use wasmtime_wasi::p1::{wasi_snapshot_preview1, WasiP1Ctx};

/// ===========================================================
/// 🔍 Night Core — Wasmtime 37 + WASI P1 Verification
/// ===========================================================
pub async fn verify_environment() -> Result<()> {
    println!("🔍 Night Core — Wasmtime 37 + WASI P1 Verification");
    println!("Checking Wasmtime engine …");

    let mut cfg = Config::new();
    cfg.async_support(true);
    cfg.consume_fuel(true);

    let engine = Engine::new(&cfg)?;
    let mut linker = Linker::new(&engine);

    // ✅ minimal WASI P1 context
    let wasi_ctx: WasiP1Ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .build_p1();
    wasi_snapshot_preview1::add_to_linker(&mut linker, |s: &mut WasiP1Ctx| s)?;

    let mut store: Store<WasiP1Ctx> = Store::new(&engine, wasi_ctx);
    let _ = store.set_fuel(10_000_000);

    // ✅ tiny self-test WAT
    let wat = r#"
        (module
            (import "wasi_snapshot_preview1" "fd_write"
                (func $fd_write (param i32 i32 i32 i32) (result i32)))
            (memory 1)
            (export "memory" (memory 0))
            (data (i32.const 8) "Night Core OK\n")
            (func (export "_start")
                (i32.store (i32.const 0) (i32.const 8))
                (i32.store (i32.const 4) (i32.const 13))
                (call $fd_write (i32.const 1) (i32.const 0)
                                 (i32.const 1) (i32.const 20))
                drop)
        )
    "#;

    let module = Module::new(&engine, wat)?;
    let instance = linker.instantiate_async(&mut store, &module).await?;
    let start = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
    start.call_async(&mut store, ()).await?;

    println!("✅ WASI P1 context executed successfully (Wasmtime 37)");
    Ok(())
}

/// ===========================================================
/// 🧩 Verify & Run Tenant Module
/// ===========================================================
#[derive(Debug, Deserialize)]
struct Manifest {
    name: String,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    permissions: Vec<String>,
}

pub async fn verify_and_run(dir: &PathBuf) -> Result<String> {
    // Load manifest
    let manifest_path = dir.join("manifest.json");
    let manifest_str = fs::read_to_string(&manifest_path)?;
    let _manifest: Manifest = serde_json::from_str(&manifest_str)?;

    // Locate module + keys
    let module_path = dir.join("module.wasm");
    let sig_path = dir.join("module.sig");
    let pubkey_path = dir.join("pubkey.b64");

    // Read and verify signature
    let wasm_bytes = fs::read(&module_path)?;
    let sig_bytes = STANDARD.decode(fs::read_to_string(sig_path)?.trim())?;
    let pubkey_bytes = STANDARD.decode(fs::read_to_string(pubkey_path)?.trim())?;

    let verifying_key = VerifyingKey::from_bytes(&pubkey_bytes.try_into().unwrap())?;
    let signature = Signature::from_bytes(&sig_bytes.try_into().unwrap());
    verifying_key.verify(&wasm_bytes, &signature)?;

    // ✅ Compute SHA-256
    let mut hasher = Sha256::new();
    hasher.update(&wasm_bytes);
    let sha_hex = hex::encode(hasher.finalize());

    // ✅ Engine + sandbox setup
    let mut config = Config::new();
    config.async_support(true);
    config.consume_fuel(true);
    let engine = Engine::new(&config)?;
    let module = Module::new(&engine, &wasm_bytes)?;
    let mut linker = Linker::new(&engine);
    wasi_snapshot_preview1::add_to_linker(&mut linker, |cx| cx)?;

    // ✅ Public WASI P1 context
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .preopened_dir(".", ".", DirPerms::READ, FilePerms::READ)?
        .build_p1();
    let mut store = Store::new(&engine, wasi_ctx);

    // ✅ Safe static limiter (Wasmtime 37 compliant)
    let store_limits = StoreLimitsBuilder::new()
        .memory_size(64 * 1024 * 1024)
        .instances(5)
        .tables(5)
        .build();

    // ✅ Leak once and bind to a truly static reference (no capture)
    static mut STATIC_LIMITS: Option<&'static mut StoreLimits> = None;
    let static_limits_ref: &'static mut StoreLimits = Box::leak(Box::new(store_limits));
    unsafe {
        STATIC_LIMITS = Some(static_limits_ref);
    }

    // ✅ Assign the limiter without moving out of the static Option
    store.limiter(|_: &mut WasiP1Ctx| unsafe {
        *STATIC_LIMITS.as_mut().unwrap() as &mut dyn wasmtime::ResourceLimiter
    });

    // ✅ Fuel limit
    store.set_fuel(10_000).unwrap();

    // ✅ Execute sandboxed module (async versions to match async_support)
    let instance = linker.instantiate_async(&mut store, &module).await?;
    if let Some(start_func) = instance.get_func(&mut store, "_start") {
        start_func.call_async(&mut store, &[], &mut []).await?;
    }

    Ok(sha_hex)
}

/// ===========================================================
/// 📄 Inspect Manifest
/// ===========================================================
pub fn inspect_manifest(dir: &PathBuf) -> Result<()> {
    let manifest_path = dir.join("manifest.json");
    let content = fs::read_to_string(&manifest_path)?;
    println!("{}", content);
    Ok(())
}

/// ===========================================================
/// ✍️ Sign Module (Ed25519)
/// ===========================================================
pub fn sign_module(dir: &PathBuf, key: &PathBuf) -> Result<()> {
    let wasm = fs::read(dir.join("module.wasm"))?;
    let privkey_bytes = STANDARD.decode(fs::read_to_string(key)?.trim())?;
    let signing_key = SigningKey::from_bytes(&privkey_bytes.try_into().unwrap());
    let sig = signing_key.sign(&wasm);
    fs::write(dir.join("module.sig"), STANDARD.encode(sig.to_bytes()))?;
    println!("✅ Module signed: {}", dir.display());
    Ok(())
}

/// ===========================================================
/// 📊 Dashboard (handled by main.rs)
/// ===========================================================
pub fn generate_dashboard() -> Result<()> {
    println!("✅ Dashboard generation complete (HTML written by main.rs)");
    Ok(())
}

