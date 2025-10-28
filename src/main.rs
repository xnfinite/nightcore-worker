#![allow(static_mut_refs)]
#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf, time::Instant};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::{Signature, SigningKey, Signer, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use wasmtime::{
    Config, Engine as WasmEngine, Linker, Module, Store, StoreLimits, StoreLimitsBuilder,
};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder};
use wasmtime_wasi::p1::{wasi_snapshot_preview1, WasiP1Ctx};
use chrono::Local;

mod verify;
mod aufs;
mod audit;
mod generate_keys;
use generate_keys::generate_keys;
mod sign;
mod sign_upgrade;
use sign_upgrade::sign_upgrade;

const ALLOWED_PERMS: &[&str] = &["stdout", "fs:read"];

#[allow(dead_code)]
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
    name = "nightcore",
    version,
    about = "Night Core v38 — Secure Multi-Tenant WASM Runner (xnfinite Stable)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run {
        #[arg(long)]
        dir: Option<PathBuf>,
        #[arg(long)]
        all: bool,
    },
    Verify,
    Inspect {
        #[arg(long)]
        dir: PathBuf,
    },
    Sign {
        #[arg(long)]
        dir: PathBuf,
        #[arg(long)]
        key: PathBuf,
    },
    Dashboard,
    Upgrade {
        #[arg(long)]
        manifest: PathBuf,
    },
    GenerateKeys {
        #[arg(long, default_value = "keys")]
        out_dir: String,
    },
    /// ✍️  Automatically sign an upgrade manifest with all maintainer keys
    SignUpgrade {
        #[arg(long)]
        manifest: PathBuf,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct RunReport {
    tenant: String,
    sha256: String,
    verified: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Run { dir, all } => {
            println!("🌑 Running all tenants...");
            let start = Instant::now();
            let mut reports = Vec::new();

            if all {
                let modules_dir = PathBuf::from("modules");
                for entry in fs::read_dir(&modules_dir)? {
                    let entry = entry?;
                    if entry.path().is_dir() {
                        let tenant_dir = entry.path();
                        let tenant_name =
                            tenant_dir.file_name().unwrap().to_string_lossy().to_string();

                        println!("Discovered tenant: {}", tenant_name);

                        match verify::verify_and_run(&tenant_dir).await {
                            Ok(sha) => {
                                println!("Signature verified (Ed25519 + SHA256)");
                                println!("Executing module...");
                                println!("✅ {} completed successfully.", tenant_name);
                                reports.push(RunReport {
                                    tenant: tenant_name,
                                    sha256: sha,
                                    verified: true,
                                });
                            }
                            Err(e) => {
                                eprintln!("❌ {} failed: {}", tenant_name, e);
                                reports.push(RunReport {
                                    tenant: tenant_name,
                                    sha256: "error".to_string(),
                                    verified: false,
                                });
                            }
                        }
                    }
                }
            } else if let Some(dir) = dir {
                verify::verify_and_run(&dir).await?;
            }

            let duration = start.elapsed().as_secs_f32();
            println!("✨ Multi-tenant orchestration finished in {:.2}s", duration);

            let dashboard_path = PathBuf::from("logs/nightcore_dashboard.html");
            fs::create_dir_all("logs")?;
            let html = generate_dashboard_html(&reports);
            fs::write(&dashboard_path, html)?;
            println!("📊 Dashboard written to {}", dashboard_path.display());
        }

        Commands::Verify => {
            println!("Verifying Wasmtime + WASI environment...");
            verify::verify_environment().await?;
        }

        Commands::Inspect { dir } => {
            verify::inspect_manifest(&dir)?;
        }

        Commands::Sign { dir, key } => {
            verify::sign_module(&dir, &key)?;
        }

        Commands::Dashboard => {
            verify::generate_dashboard()?;
        }

        Commands::Upgrade { manifest } => {
            println!("🔄 Running AUFS verification...");
            match aufs::verify_upgrade(&manifest) {
                Ok(_) => {
                    println!("✅ AUFS verification passed — hash chain updated");
                    let _ = audit::append(
                        "aufs_verification_passed",
                        serde_json::json!({
                            "manifest": manifest.display().to_string(),
                            "status": "success",
                        }),
                    );
                }
                Err(e) => {
                    eprintln!("❌ AUFS verification failed: {}", e);
                    let _ = audit::append(
                        "aufs_verification_failed",
                        serde_json::json!({
                            "manifest": manifest.display().to_string(),
                            "status": "failed",
                            "error": e.to_string(),
                        }),
                    );
                }
            }
        }

        Commands::GenerateKeys { out_dir } => {
            generate_keys(&out_dir)?;
        }

        // ✅ Simplified call
        Commands::SignUpgrade { manifest } => {
            sign_upgrade(&manifest)?;
        }
    }

    Ok(())
}

// =========================================================
// 📊 DASHBOARD GENERATOR (unchanged)
// =========================================================
fn generate_dashboard_html(reports: &[RunReport]) -> String {
    use chrono::Local;

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut html = String::from(
        r#"<!DOCTYPE html><html><head><meta charset="UTF-8">
<title>Night Core Dashboard</title>
<style>
body { font-family: Arial, sans-serif; background:#0a0a0a; color:#ddd; margin:40px; }
h1 { color:#6cf; text-align:center; }
table { width:100%; border-collapse: collapse; margin-top:20px; }
td, th { padding:8px; border-bottom:1px solid #333; text-align:left; }
.success { color:#0f0; }
.fail { color:#f33; }
.footer { margin-top:40px; text-align:center; font-size:14px; color:#666; }
</style></head><body>
<h1>Night Core — Multi-Tenant Report</h1>
<p style='text-align:center;'>Generated at: TIMESTAMP</p>
<table><tr><th>Tenant</th><th>SHA256</th><th>Status</th><th>Time</th></tr>
"#,
    );

    for r in reports {
        let status_class = if r.verified { "success" } else { "fail" };
        let status_text = if r.verified { "✅ Verified" } else { "❌ Failed" };
        let time = Local::now().format("%H:%M:%S").to_string();
        html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td class='{}'>{}</td><td>{}</td></tr>\n",
            r.tenant, r.sha256, status_class, status_text, time
        ));
    }

    html.push_str(&format!(
        "</table><div class='footer'><hr><p><i>Night Core™ — Secure. Autonomous. Verified.</i><br>
        <strong>v38 xnfinite Stable • Generated at {}</strong></p></div></body></html>",
        now
    ));

    html.replace("TIMESTAMP", &now)
}
