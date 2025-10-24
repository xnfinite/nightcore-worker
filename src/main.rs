#![allow(static_mut_refs)]

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::{collections::HashSet, fs, path::PathBuf, time::Instant};

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use ed25519_dalek::{Signature, SigningKey, Signer, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::fs as tokio_fs;
use wasmtime::{
    Config, Engine as WasmEngine, Linker, Module, Store, StoreLimits, StoreLimitsBuilder,
};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder};
use wasmtime_wasi::p1::{wasi_snapshot_preview1, WasiP1Ctx};
use chrono::Local;
use open;

mod verify; // ✅ Async verification module

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
#[command(
    name = "worker",
    version,
    about = "Night Core v37 — Secure Multi-Tenant WASM Runner (B106 Edition)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a single tenant or all tenants
    Run {
        #[arg(long)]
        dir: Option<PathBuf>,
        #[arg(long)]
        all: bool,
    },

    /// Verify Wasmtime 37 + WASI P1 environment
    Verify,

    /// Inspect manifest for a given tenant
    Inspect {
        #[arg(long)]
        dir: PathBuf,
    },

    /// Sign a tenant module with Ed25519 private key
    Sign {
        #[arg(long)]
        dir: PathBuf,
        #[arg(long)]
        key: PathBuf,
    },

    /// Generate local HTML dashboard for orchestration logs
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

#[tokio::main(flavor = "current_thread")]
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

        // ✅ Async-safe verification
        Commands::Verify => verify::verify_environment().await,

        Commands::Inspect { dir } => inspect_manifest(&dir),
        Commands::Sign { dir, key } => sign_module(&dir, &key),
        Commands::Dashboard => generate_dashboard(),
    }
}

//
// ─── MANIFEST HELPERS ───────────────────────────────────────────────────────────
//
fn read_manifest(dir: &PathBuf) -> Result<Manifest> {
    let p = dir.join("manifest.json");
    let text = fs::read_to_string(&p).with_context(|| format!("reading {}", p.display()))?;
    let m: Manifest = serde_json::from_str(&text).context("parsing manifest.json")?;

    let requested: HashSet<String> = m.permissions.iter().cloned().collect();
    let allowed: HashSet<&str> = ALLOWED_PERMS.iter().copied().collect();
    let unknown: Vec<&str> = requested
        .iter()
        .map(String::as_str)
        .filter(|p| !allowed.contains(p))
        .collect();
    if !unknown.is_empty() {
        anyhow::bail!("manifest requests unsupported permissions: {:?}", unknown);
    }
    Ok(m)
}

fn load_artifacts(dir: &PathBuf) -> Result<(Vec<u8>, Signature, VerifyingKey)> {
    let wasm = fs::read(dir.join("module.wasm"))?;
    let sig_b64 = fs::read_to_string(dir.join("module.sig"))?;
    let pk_b64 = fs::read_to_string(dir.join("pubkey.b64"))?;

    let sig_vec = STANDARD.decode(sig_b64.trim())?;
    let pk_vec = STANDARD.decode(pk_b64.trim())?;

    let sig_bytes: [u8; 64] = sig_vec.try_into().unwrap();
    let pk_bytes: [u8; 32] = pk_vec.try_into().unwrap();

    Ok((
        wasm,
        Signature::from_bytes(&sig_bytes),
        VerifyingKey::from_bytes(&pk_bytes)?,
    ))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

//
// ─── CORE COMMANDS ──────────────────────────────────────────────────────────────
//
fn inspect_manifest(dir: &PathBuf) -> Result<()> {
    let m = read_manifest(dir)?;
    println!(
        "📄 Manifest\n  name: {}\n  version: {}\n  description: {}\n  perms: {:?}\n  fuel_limit: {:?}\n  timeout_ms: {:?}",
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

    let sk_bytes: [u8; 32] = key_vec.try_into().unwrap();
    let signing_key = SigningKey::from_bytes(&sk_bytes);
    let sig = signing_key.sign(&wasm);
    fs::write(dir.join("module.sig"), STANDARD.encode(sig.to_bytes()))?;
    println!("✍️  Wrote signature => {}", dir.join("module.sig").display());
    Ok(())
}

fn verify_dir(dir: &PathBuf) -> Result<()> {
    let manifest = read_manifest(dir)?;
    let (wasm, signature, vkey) = load_artifacts(dir)?;
    vkey.verify(&wasm, &signature)?;

    println!(
        "🧾 Manifest OK: {}{} | perms={:?} | fuel={:?} | timeout={:?}ms",
        manifest.name,
        manifest.version.as_deref().unwrap_or(""),
        manifest.permissions,
        manifest.fuel_limit,
        manifest.timeout_ms
    );
    println!("✅ Signature verified");
    println!("🔐 Module SHA256: {}", sha256_hex(&wasm));
    Ok(())
}

//
// ─── EXECUTION ─────────────────────────────────────────────────────────────────
//
async fn run_module_dir(dir: &PathBuf) -> Result<()> {
    let manifest = read_manifest(dir)?;
    let (wasm, signature, vkey) = load_artifacts(dir)?;
    vkey.verify(&wasm, &signature)?;

    println!(
        "🧾 Manifest OK: {}{} | perms={:?} | fuel={:?} | timeout={:?}ms",
        manifest.name,
        manifest.version.as_deref().unwrap_or(""),
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
    let requested: HashSet<String> = manifest.permissions.iter().cloned().collect();
    if requested.contains("stdout") {
        wasi_builder.inherit_stdio();
    }
    if requested.contains("fs:read") {
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
    unsafe { LIMITS_PTR = Some(limits_ref) }
    store.limiter(|_| unsafe { LIMITS_PTR.as_deref_mut().unwrap() });

    let instance = linker.instantiate_async(&mut store, &module).await?;
    if let Ok(start) = instance.get_typed_func::<(), ()>(&mut store, "_start") {
        let started = Instant::now();
        let timeout = manifest.timeout_ms.unwrap_or(3_000);
        match tokio::time::timeout(
            std::time::Duration::from_millis(timeout),
            start.call_async(&mut store, ()),
        )
        .await
        {
            Ok(Ok(_)) => println!(
                "🚀 _start executed successfully | ⏱️ {} ms",
                started.elapsed().as_millis()
            ),
            Ok(Err(e)) => anyhow::bail!("module trapped: {e:?}"),
            Err(_) => anyhow::bail!("⏱️ execution timed out after {}ms", timeout),
        }
    }
    Ok(())
}

//
// ─── ORCHESTRATION ──────────────────────────────────────────────────────────────
//
async fn orchestrate_all() -> Result<()> {
    let base = PathBuf::from("modules");
    if !base.exists() {
        anyhow::bail!("modules/ directory not found");
    }

    let mut reports: Vec<RunReport> = Vec::new();
    for entry in fs::read_dir(&base)? {
        let path = entry?.path();
        if !path.is_dir() {
            continue;
        }

        let tenant_name = path.file_name().unwrap().to_string_lossy().to_string();
        let manifest_path = path.join("manifest.json");
        if !manifest_path.exists() {
            continue;
        }

        println!("🌐 Launching tenant: {}", tenant_name);
        let start = Instant::now();
        let result = run_module_dir(&path).await;

        let (verified, success, sha256, status) = match result {
            Ok(_) => {
                let (wasm, _, _) = load_artifacts(&path)?;
                (true, true, sha256_hex(&wasm), "success".into())
            }
            Err(e) => {
                eprintln!("⚠️  Tenant {} failed: {}", tenant_name, e);
                let sha = fs::read(path.join("module.wasm"))
                    .map(|d| sha256_hex(&d))
                    .unwrap_or_else(|_| "<missing>".into());
                (false, false, sha, "failed".into())
            }
        };

        reports.push(RunReport {
            tenant: tenant_name,
            sha256,
            verified,
            success,
            duration_ms: start.elapsed().as_millis(),
            status,
        });
    }

    fs::create_dir_all("logs").ok();
    let json = serde_json::to_string_pretty(&reports)?;
    tokio_fs::write("logs/orchestration_report.json", json).await?;
    println!("📜 Orchestration complete → logs/orchestration_report.json");
    Ok(())
}

//
// ─── DASHBOARD ─────────────────────────────────────────────────────────────────
//
fn generate_dashboard() -> Result<()> {
    let now = Local::now();
    let report_path = PathBuf::from("logs/orchestration_report.json");
    let data = fs::read_to_string(&report_path).context("reading orchestration report")?;
    let entries: Vec<RunReport> = serde_json::from_str(&data).context("parsing report")?;

    let rows: String = entries
        .iter()
        .map(|r| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{} ms</td></tr>",
                r.tenant, r.sha256, r.verified, r.success, r.status, r.duration_ms
            )
        })
        .collect();

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en"><head><meta charset="UTF-8">
<title>Night Core Dashboard</title>
<style>
body {{ background:#0e0e0e;color:#eee;font-family:Consolas,monospace; }}
table {{ width:100%;border-collapse:collapse; }}
th,td {{ border:1px solid #444;padding:8px;text-align:left; }}
th {{ background:#222; }}
.success {{ color:#00ff88; }}
.failed {{ color:#ff4444; }}
</style></head>
<body>
<h2>🌙 Night Core Dashboard (v37 B106 Stable)</h2>
<p>Generated: {}</p>
<table>
<tr><th>Tenant</th><th>SHA256</th><th>Verified</th><th>Success</th><th>Status</th><th>Duration</th></tr>
{}
</table>
</body></html>
"#,
        now.format("%Y-%m-%d %H:%M:%S"),
        rows
    );

    let out = "logs/nightcore_dashboard.html";
    fs::write(out, html)?;
    println!("🌐 Dashboard written to '{}'", out);
    let _ = open::that(out);
    Ok(())
}
