#![allow(static_mut_refs)]

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::{
    collections::HashSet,
    fs,
    path::PathBuf,
    time::Instant,
};
use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use ed25519_dalek::{Signature, SigningKey, Verifier, VerifyingKey, Signer};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::fs as tokio_fs;
use wasmtime::{Config, Engine as WasmEngine, Linker, Module, Store, StoreLimits, StoreLimitsBuilder};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder};
use wasmtime_wasi::p1::{wasi_snapshot_preview1, WasiP1Ctx};
use chrono::Local;
use open;

/// Allowed permissions for sandbox
const ALLOWED_PERMS: &[&str] = &["stdout", "fs:read"];

#[derive(Debug, Deserialize)]
struct Manifest {
    name: String,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    permissions: Vec<String>,
    #[serde(default)]
    fuel_limit: Option<u64>,
    #[serde(default)]
    timeout_ms: Option<u64>,
    #[serde(default)]
    max_memory_kb: Option<u64>,
}

#[derive(Parser, Debug)]
#[command(name="worker", version, about="Night Core v37 B106 — Secure WASI P1 Runner")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run {
        #[arg(long, value_name="DIR", required=false)]
        dir: Option<PathBuf>,
        #[arg(long)]
        all: bool,
    },
    Verify {
        #[arg(long, value_name="DIR")]
        dir: PathBuf,
    },
    Inspect {
        #[arg(long, value_name="DIR")]
        dir: PathBuf,
    },
    Sign {
        #[arg(long, value_name="DIR")]
        dir: PathBuf,
        #[arg(long, value_name="PRIVKEY.b64")]
        key: PathBuf,
    },
    Dashboard,
}

#[derive(Debug, Serialize, Deserialize)]
struct RunReport {
    tenant: String,
    sha256: String,
    verified: bool,
    success: bool,
    duration_ms: u128,
    status: String,
}

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Run { dir, all } => {
            if all {
                orchestrate_all().await
            } else if let Some(dir) = dir {
                run_module_dir(&dir).await
            } else {
                anyhow::bail!("please provide --dir or use --all")
            }
        }
        Commands::Verify { dir } => verify_dir(&dir),
        Commands::Inspect { dir } => inspect_manifest(&dir),
        Commands::Sign { dir, key } => sign_module(&dir, &key),
        Commands::Dashboard => generate_dashboard(),
    }
}

// ──────────────────────────────
// Utility
// ──────────────────────────────
fn read_manifest(dir: &PathBuf) -> Result<Manifest> {
    let p = dir.join("manifest.json");
    let text = fs::read_to_string(&p).with_context(|| format!("reading {}", p.display()))?;
    let m: Manifest = serde_json::from_str(&text).context("parsing manifest.json")?;

    let requested: HashSet<String> = m.permissions.iter().cloned().collect();
    let allowed: HashSet<&str> = ALLOWED_PERMS.iter().copied().collect();
    let unknown: Vec<&str> = requested.iter().map(String::as_str).filter(|p| !allowed.contains(p)).collect();
    if !unknown.is_empty() {
        anyhow::bail!("manifest requests unsupported permissions: {:?}", unknown);
    }
    Ok(m)
}

fn load_artifacts(dir: &PathBuf) -> Result<(Vec<u8>, Signature, VerifyingKey)> {
    let wasm_path = dir.join("module.wasm");
    let sig_path = dir.join("module.sig");
    let pk_path = dir.join("pubkey.b64");

    let wasm = fs::read(&wasm_path)?;
    let sig_b64 = fs::read_to_string(&sig_path)?;
    let pk_b64 = fs::read_to_string(&pk_path)?;

    let sig_vec = STANDARD.decode(sig_b64.trim())?;
    let pk_vec = STANDARD.decode(pk_b64.trim())?;

    let sig_bytes: [u8; 64] = sig_vec.try_into().map_err(|_| anyhow::anyhow!("signature must be 64 bytes"))?;
    let pk_bytes: [u8; 32] = pk_vec.try_into().map_err(|_| anyhow::anyhow!("pubkey must be 32 bytes"))?;

    let signature = Signature::from_bytes(&sig_bytes);
    let vkey = VerifyingKey::from_bytes(&pk_bytes)?;
    Ok((wasm, signature, vkey))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}

// ──────────────────────────────
// Core Commands
// ──────────────────────────────
fn verify_dir(dir: &PathBuf) -> Result<()> {
    let manifest = read_manifest(dir)?;
    let (wasm, sig, vkey) = load_artifacts(dir)?;
    vkey.verify(&wasm, &sig)?;
    println!(
        "🧾 Manifest OK: {}{} | perms={:?} | fuel={:?} | timeout={:?} ms",
        manifest.name,
        manifest.version.as_deref().map(|v| format!(" v{}", v)).unwrap_or_default(),
        manifest.permissions,
        manifest.fuel_limit,
        manifest.timeout_ms
    );
    println!("✅ Signature verified");
    println!("🔐 Module SHA256: {}", sha256_hex(&wasm));
    Ok(())
}

fn inspect_manifest(dir: &PathBuf) -> Result<()> {
    let m = read_manifest(dir)?;
    println!(
        "📄 Manifest\n name: {}\n version: {}\n description: {}\n perms: {:?}\n fuel_limit: {:?}\n timeout_ms: {:?}",
        m.name,
        m.version.as_deref().unwrap_or("<none>"),
        m.description.as_deref().unwrap_or("<none>"),
        m.permissions,
        m.fuel_limit,
        m.timeout_ms
    );
    Ok(())
}

fn sign_module(dir: &PathBuf, key_path: &PathBuf) -> Result<()> {
    let wasm = fs::read(dir.join("module.wasm"))?;
    let key_b64 = fs::read_to_string(key_path)?;
    let key_vec = STANDARD.decode(key_b64.trim())?;

    let sk_bytes: [u8; 32] = key_vec.try_into().map_err(|_| anyhow::anyhow!("privkey must be 32 bytes"))?;
    let signing_key = SigningKey::from_bytes(&sk_bytes);
    let sig = signing_key.sign(&wasm);
    let sig_b64 = STANDARD.encode(sig.to_bytes());
    fs::write(dir.join("module.sig"), sig_b64)?;
    println!("✍️ Wrote signature => {}/module.sig", dir.display());
    Ok(())
}

// ──────────────────────────────
// Execution
// ──────────────────────────────
async fn run_module_dir(dir: &PathBuf) -> Result<()> {
    let manifest = read_manifest(dir)?;
    let (wasm, sig, vkey) = load_artifacts(dir)?;
    vkey.verify(&wasm, &sig)?;

    println!(
        "🧾 Manifest OK: {}{} | perms={:?} | fuel={:?} | timeout={:?} ms",
        manifest.name,
        manifest.version.as_deref().map(|v| format!(" v{}", v)).unwrap_or_default(),
        manifest.permissions,
        manifest.fuel_limit,
        manifest.timeout_ms
    );
    println!("✅ Signature verified");
    println!("🔐 Module SHA256: {}", sha256_hex(&wasm));

    let mut cfg = Config::default();
    cfg.async_support(true);
    cfg.consume_fuel(true);
    let engine = WasmEngine::new(&cfg)?;
    let mut linker = Linker::new(&engine);

    let mut wasi_builder = WasiCtxBuilder::new();
    let req: HashSet<String> = manifest.permissions.iter().cloned().collect();

    if req.contains("stdout") {
        wasi_builder.inherit_stdio();
    }
    if req.contains("fs:read") {
        let host = dir.join("sandbox");
        if host.exists() {
            wasi_builder.preopened_dir(&host, "/sandbox", DirPerms::READ, FilePerms::READ)?;
        }
    }

    let wasi: WasiP1Ctx = wasi_builder.build_p1();
    wasi_snapshot_preview1::add_to_linker(&mut linker, |s: &mut WasiP1Ctx| s)?;
    let module = Module::new(&engine, &wasm)?;
    let mut store: Store<WasiP1Ctx> = Store::new(&engine, wasi);

    let _ = store.set_fuel(manifest.fuel_limit.unwrap_or(50_000));

    static mut LIMITS_PTR: Option<&'static mut StoreLimits> = None;
    let limits = StoreLimitsBuilder::new().memories(1).tables(2).instances(1).build();
    let limits_ref: &'static mut StoreLimits = Box::leak(Box::new(limits));
    unsafe { LIMITS_PTR = Some(limits_ref); }
    store.limiter(|_| unsafe { LIMITS_PTR.as_deref_mut().unwrap() });

    let instance = linker.instantiate_async(&mut store, &module).await?;
    if let Ok(start) = instance.get_typed_func::<(), ()>(&mut store, "_start") {
        let timeout_ms = manifest.timeout_ms.unwrap_or(3_000);
        let started = Instant::now();
        let call = tokio::time::timeout(std::time::Duration::from_millis(timeout_ms), start.call_async(&mut store, ()));
        match call.await {
            Ok(Ok(_)) => println!("🚀 _start executed successfully | ⏱️ {} ms", started.elapsed().as_millis()),
            Ok(Err(e)) => anyhow::bail!("module trapped: {e:?}"),
            Err(_) => anyhow::bail!("⏱️ execution timed out after {} ms", timeout_ms),
        }
    }
    Ok(())
}

// ──────────────────────────────
// Orchestration + Dashboard
// ──────────────────────────────
async fn orchestrate_all() -> Result<()> {
    let base = PathBuf::from("modules");
    if !base.exists() { anyhow::bail!("modules/ directory not found"); }

    let mut reports: Vec<RunReport> = Vec::new();
    for entry in fs::read_dir(&base)? {
        let path = entry?.path();
        if !path.is_dir() { continue; }
        if !path.join("manifest.json").exists() { continue; }

        let tenant = path.file_name().unwrap().to_string_lossy().to_string();
        let started = Instant::now();
        println!("🌐 Launching tenant: {}", tenant);

        let result = run_module_dir(&path).await;
        let (verified, success, sha256, status) = match result {
            Ok(_) => {
                let (wasm, _, _) = load_artifacts(&path)?;
                (true, true, sha256_hex(&wasm), "success".to_string())
            }
            Err(e) => {
                eprintln!("⚠️ Tenant {} failed: {}", tenant, e);
                let sha = fs::read(path.join("module.wasm"))
                    .map(|d| sha256_hex(&d))
                    .unwrap_or_else(|_| "<missing>".into());
                (false, false, sha, "failed".to_string())
            }
        };

        reports.push(RunReport {
            tenant,
            sha256,
            verified,
            success,
            duration_ms: started.elapsed().as_millis(),
            status,
        });
    }

    fs::create_dir_all("logs").ok();
    let json = serde_json::to_string_pretty(&reports)?;
    tokio_fs::write("logs/orchestration_report.json", json).await?;
    println!("📜 Orchestration complete → logs/orchestration_report.json");
    Ok(())
}

fn generate_dashboard() -> Result<()> {
    let data = fs::read_to_string("logs/orchestration_report.json")
        .context("reading orchestration report")?;
    let reports: Vec<RunReport> = serde_json::from_str(&data).context("parsing JSON report")?;
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html><head><meta charset='utf-8'>");
    html.push_str("<title>Night Core Dashboard</title>");
    html.push_str("<link rel=\"stylesheet\" href=\"theme.css\">");
    html.push_str("</head><body>");
    html.push_str("<h2>Night Core Multi-Tenant Dashboard — B106 Edition</h2>");
    html.push_str(&format!("<p>Generated at <b>{}</b></p>", timestamp));
    html.push_str("<table><tr><th>Tenant</th><th>SHA256</th><th>Verified</th><th>Success</th><th>Duration (ms)</th><th>Status</th></tr>");
    for r in &reports {
        let verified = if r.verified { "✅" } else { "❌" };
        let success = if r.success { "<span class='success'>OK</span>" } else { "<span class='failed'>Fail</span>" };
        html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            r.tenant, r.sha256, verified, success, r.duration_ms, r.status
        ));
    }
    html.push_str("</table>");
    html.push_str("<div class='footer'>Generated by Night Core v37 B106 • <span id='ts'></span></div>");
    html.push_str("<script>document.getElementById('ts').textContent=new Date().toLocaleString();</script>");
    html.push_str("</body></html>");

    fs::create_dir_all("logs").ok();
    fs::write("logs/nightcore_dashboard.html", html)?;
    println!("🌐 Dashboard written to 'logs/nightcore_dashboard.html'");
    let _ = open::that("logs/nightcore_dashboard.html");
    Ok(())
}



