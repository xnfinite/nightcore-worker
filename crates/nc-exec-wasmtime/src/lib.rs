use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use nc_exec::{ExecConfig, ExecProof, SandboxBackend};
use sha2::{Digest, Sha256};
use std::{convert::TryInto, fs, path::Path};

use wasmtime::{Engine, Linker, Module, Store, Val};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder};        // ✅ root builder + perms
use wasmtime_wasi::p1::{add_to_linker_sync, WasiP1Ctx};           // ✅ P1 linker + context
use base64::{engine::general_purpose::STANDARD, Engine as Base64Engine};

pub struct WasmtimeBackend;

impl SandboxBackend for WasmtimeBackend {
    fn name(&self) -> &'static str { "wasmtime" }

    fn verify(&self, module_path: &Path) -> Result<()> {
        let dir = module_path.parent().ok_or_else(|| anyhow!("No parent dir"))?;
        let wasm = fs::read(module_path).context("Read module")?;
        let digest = Sha256::digest(&wasm);
        println!("✅ SHA256 digest: {:x}", digest);

        // --- Ed25519 signature verification
        let pk_b64 = fs::read_to_string(dir.join("pubkey.b64")).context("pubkey.b64")?;
        let sig_b64 = fs::read_to_string(dir.join("module.sig")).context("module.sig")?;
        let sig_bytes = STANDARD.decode(sig_b64.trim()).context("Signature not valid base64")?;

        let pk_vec = STANDARD.decode(pk_b64.trim()).context("Invalid base64 pubkey")?;
        let pk: [u8; 32] = pk_vec.try_into().map_err(|_| anyhow!("Pubkey len != 32"))?;
        let sig_arr: [u8; 64] = sig_bytes.try_into().map_err(|_| anyhow!("Signature len != 64"))?;

        let vk = VerifyingKey::from_bytes(&pk).context("Bad pubkey")?;
        let sig = Signature::from_bytes(&sig_arr);
        vk.verify(&wasm, &sig).context("Signature verify failed")?;
        println!("✅ Ed25519 signature verified");
        Ok(())
    }

    fn execute(&self, cfg: &ExecConfig) -> Result<ExecProof> {
        let started = Utc::now().to_rfc3339();

        // --- Engine + module
        let engine = Engine::default();
        let module = Module::from_file(&engine, &cfg.module_path)?;

        // --- Build WASI P1 context using the public v37 API
        let mut builder = WasiCtxBuilder::new();
        for dir in &cfg.preopen_dirs {
            // ✅ ensure directory exists before preopening
            fs::create_dir_all(dir)
                .with_context(|| format!("creating preopen dir: {}", dir.display()))?;

            builder
                .preopened_dir(
                    dir,                                   // host path (Path)
                    dir.to_string_lossy(),                 // guest path (str)
                    DirPerms::all(),                       // directory permissions
                    FilePerms::all(),                      // file permissions
                )
                .context("Preopen directory failed")?;
        }

        // ✅ Build WASI P1 context
        let wasi_p1: WasiP1Ctx = builder.build_p1();

        // --- Store + linker + run (P1 expects &mut WasiP1Ctx)
        let mut store = Store::new(&engine, wasi_p1);
        let mut linker = Linker::new(&engine);
        add_to_linker_sync(&mut linker, |cx: &mut WasiP1Ctx| cx)?; // ✅ matches trait bound

        let instance = linker.instantiate(&mut store, &module)?;
        if let Some(start) = instance.get_func(&mut store, "_start") {
            let mut results: Vec<Val> = vec![];
            start.call(&mut store, &[], &mut results)?; // ✅ Wasmtime v37 call signature
        }

        let finished = Utc::now().to_rfc3339();
        let hash = format!("{:x}", Sha256::digest(fs::read(&cfg.module_path)?));

        // --- Proof output
        let proof = ExecProof {
            tenant: cfg.tenant.clone(),
            module_sha256: hash.clone(),
            signer_key_b64: "<verified>".into(),
            started_at: started.clone(),
            finished_at: finished.clone(),
            status: "ok".into(),
            backend: self.name().into(),
        };

        let dir = format!("logs/{}/", cfg.tenant);
        fs::create_dir_all(&dir)?;
        fs::write(
            format!("{}proof_report.jsonl", dir),
            serde_json::to_string(&proof)? + "\n",
        )?;
        fs::write(
            format!("{}proof_dashboard.html", dir),
            format!(
                r#"
<!doctype html><html><meta charset="utf-8"><body>
<h3>Tenant {}</h3>
<p>SHA256 {} ✅<br/>Backend Wasmtime</p>
<p>Started {}<br/>Finished {}</p>
</body></html>"#,
                cfg.tenant, hash, started, finished
            ),
        )?;

        Ok(proof)
    }
}
