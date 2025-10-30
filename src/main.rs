#![allow(static_mut_refs)]

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use sha2::{Sha256, Digest};

mod verify;
mod aufs; // ✅ AUFS integration

/// ===========================================================
/// 🧭 Night Core CLI — Secure. Autonomous. Verified.
/// ===========================================================
#[derive(Parser)]
#[command(name = "nightcore")]
#[command(about = "Night Core — Secure. Autonomous. Verified.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 🧩 Run all tenant modules or a single one
    Run {
        #[arg(long)]
        all: bool,
        path: Option<PathBuf>,
    },

    /// 🔍 Run Wasmtime environment self-check
    VerifyEnv,

    /// ✍️ Sign a module using a private key
    Sign {
        #[arg(long)]
        dir: PathBuf,
        #[arg(long)]
        key: PathBuf,
    },

    /// 📄 Inspect a tenant manifest.json
    Inspect {
        #[arg(long)]
        dir: PathBuf,
    },

    /// 🧾 Export all tenant pubkey hashes (for AUFS / audit)
    ExportPubkeyHashes,

    /// 🚀 Run AUFS upgrade verification
    Upgrade {
        /// Path to upgrade manifest (default: upgrades/manifests/upgrade_manifest.json)
        #[arg(short, long, default_value = "upgrades/manifests/upgrade_manifest.json")]
        manifest: String,
    },

    /// 🔏 Sign an AUFS upgrade manifest with a maintainer key
    SignUpgrade {
        /// Path to the upgrade manifest JSON
        #[arg(short, long, default_value = "upgrades/manifests/upgrade_manifest.json")]
        manifest: String,

        /// Path to the private key (Base64)
        #[arg(short, long, default_value = "keys/maintainers/admin1.key")]
        key: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        // ===================================================
        Commands::VerifyEnv => {
            verify::verify_environment()?;
        }

        // ===================================================
        Commands::Run { all, path } => {
            if all {
                let modules_dir = PathBuf::from("modules");
                for entry in fs::read_dir(&modules_dir)
                    .with_context(|| format!("reading modules directory: {}", modules_dir.display()))?
                {
                    let entry = entry?;
                    if !entry.path().is_dir() {
                        continue;
                    }
                    let tenant_dir = entry.path();
                    let tenant_name = entry.file_name().to_string_lossy().into_owned();

                    if let Err(e) =
                        verify::ensure_pubkey_sync(tenant_dir.to_str().unwrap_or_default(), &tenant_name)
                    {
                        eprintln!("⚠️ Pubkey sync failed for {}: {}", tenant_name, e);
                    }

                    match verify::verify_and_run(&tenant_dir) {
                        Ok(sha) => println!("✅ Tenant OK: {} (sha {})", tenant_name, sha),
                        Err(e) => println!("❌ Tenant {} failed: {}", tenant_name, e),
                    }
                }
            } else if let Some(p) = path {
                let tenant_name = p
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "unknown".to_string());

                if let Err(e) =
                    verify::ensure_pubkey_sync(p.to_str().unwrap_or_default(), &tenant_name)
                {
                    eprintln!("⚠️ Pubkey sync failed: {}", e);
                }

                let sha = verify::verify_and_run(&p)?;
                println!("✅ {} executed successfully (sha {})", tenant_name, sha);
            } else {
                println!("⚙️ Usage: nightcore run --all OR --path <tenant_dir>");
            }
        }

        // ===================================================
        Commands::Sign { dir, key } => {
            verify::sign_module(&dir, &key)?;
        }

        // ===================================================
        Commands::Inspect { dir } => {
            verify::inspect_manifest(&dir)?;
        }

        // ===================================================
        Commands::ExportPubkeyHashes => {
            println!("🔍 Exporting pubkey hashes for upgrade manifest:");
            let modules_dir = PathBuf::from("modules");
            for entry in fs::read_dir(&modules_dir)
                .with_context(|| format!("reading modules directory: {}", modules_dir.display()))?
            {
                let entry = entry?;
                if !entry.path().is_dir() {
                    continue;
                }
                let tenant_dir = entry.path();
                let tenant_name = entry.file_name().to_string_lossy().into_owned();
                let pubkey_path = tenant_dir.join("pubkey.b64");

                if pubkey_path.exists() {
                    let pubkey_b64 = fs::read_to_string(&pubkey_path)?.trim().to_string();
                    let pubkey_bytes = STANDARD.decode(&pubkey_b64)?;
                    let hash = Sha256::digest(&pubkey_bytes);
                    println!(
                        "{{ \"name\": \"{}\", \"pubkey_hash\": \"SHA256:{}\" }}",
                        tenant_name,
                        hex::encode(hash)
                    );
                } else {
                    println!(
                        "{{ \"name\": \"{}\", \"pubkey_hash\": \"missing pubkey.b64\" }}",
                        tenant_name
                    );
                }
            }
            println!("✅ Export complete.");
        }

        // ===================================================
        Commands::Upgrade { manifest } => {
            aufs::verify_upgrade(PathBuf::from(&manifest).as_path())?;
        }

        // ===================================================
        Commands::SignUpgrade { manifest, key } => {
            aufs::sign_upgrade_manifest(PathBuf::from(&manifest), PathBuf::from(&key))?;
        }
    }

    println!("✨ Night Core execution complete.\n");
    Ok(())
}

