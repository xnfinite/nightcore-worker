#![allow(static_mut_refs)]

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::{
    fs, path::PathBuf, thread, sync::mpsc,
    time::{Duration, Instant},
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use chrono::{Utc, DateTime};
use open; // ✅ for automatic browser open

mod verify;
mod aufs;
mod sign_tenant;
mod unlock;
mod firecracker_adapter;


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

        #[arg(long)]
        proof: bool,

        #[arg(long, default_value_t = 0)]
        parallel: usize,

        #[arg(long, default_value = "wasmtime")]
        backend: String, // 🔥 backend selector (wasmtime | firecracker)

        /// Optional path to a single tenant module
        path: Option<PathBuf>,

        /// 🕒 Optional Firecracker VM timeout (seconds)
        #[arg(long, default_value_t = 5)]
        vm_timeout: u64, // ✅ Added safely
    },

    VerifyEnv,

    Sign {
        #[arg(long)]
        dir: PathBuf,
        #[arg(long)]
        key: PathBuf,
    },

    Inspect {
        #[arg(long)]
        dir: PathBuf,
    },

    ExportPubkeyHashes,

    /// Build a historical HTML ledger from /state (add --diff for per-tenant deltas)
    ExportDashboard {
        /// Show last vs previous proof deltas per tenant
        #[arg(long)]
        diff: bool,
    },

    /// Inspect persisted state for a tenant (optionally summarize)
    InspectState {
        #[arg(long)]
        tenant: Option<String>,
        #[arg(long)]
        all_tenants: bool,
        #[arg(long)]
        summary: bool,
    },

    Upgrade {
        #[arg(short, long, default_value = "upgrades/manifests/upgrade_manifest.json")]
        manifest: String,
    },

    SignUpgrade {
        #[arg(short, long, default_value = "upgrades/manifests/upgrade_manifest.json")]
        manifest: String,
        #[arg(short, long, default_value = "keys/maintainers/admin1.key")]
        key: String,
    },

    Unlock,
}



/// ─────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct OrchestrationReport {
    timestamp: String,
    tenants_executed: usize,
    total_time_s: f64,
    avg_time_s: f64,
    parallel_threads: usize,
    tenants: Vec<TenantTiming>,
}

#[derive(Serialize, Clone)]
struct TenantTiming {
    name: String,
    sha: String,
    duration_s: f64,
    status: String,
}

/// Compact row used in dashboards
#[derive(Serialize, Deserialize, Clone)]
struct ProofRow {
    sha256: String,
    size: u64,
    verified: bool,
    timestamp: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if unlock::check_unlock() {
        println!("🪪 Pro mode active — AUFS + Proof extensions enabled.\n");
    } else {
        println!("🔒 Running in open-core mode (MIT Edition).\n");
    }

    match cli.command {
        Commands::VerifyEnv => verify::verify_environment()?,

        Commands::Run { all, proof, parallel, backend, path, vm_timeout } => {

    println!("🧭 Backend selected: {}", backend);

    if backend == "firecracker" {
    println!("🧭 Backend selected: firecracker");
    firecracker_adapter::launch_microvm_with_timeout(

        "firecracker_assets/vmlinux",
        "firecracker_assets/rootfs.ext4",
        vm_timeout,
    )?;
}



    if all {
        let start_total = Instant::now();
        let timestamp = Utc::now().to_rfc3339();
        


                let modules_dir = PathBuf::from("modules");
                let entries: Vec<_> = fs::read_dir(&modules_dir)?
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                    .collect();

                let mut parallel = if parallel == 0 {
                    std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4)
                } else { parallel };

                if proof && parallel > 2 {
                    println!("⚠️ Proof mode active — limiting concurrency to 2 threads.");
                    parallel = 2;
                }

                println!("🧩 Running {} tenants with parallelism = {}", entries.len(), parallel);

                let (tx, rx) = mpsc::channel();
                let mut timing: Vec<TenantTiming> = vec![];

                for chunk in entries.chunks(parallel) {
                    let mut handles = vec![];

                    for entry in chunk {
                        let tx = tx.clone();
                        let tenant_dir = entry.path();
                        let tenant_name = entry.file_name().to_string_lossy().into_owned();
                        let proof_mode = proof;

                        handles.push(thread::spawn(move || {
                            let t0 = Instant::now();
                            let result = verify::verify_and_run(&tenant_dir, proof_mode);
                            let elapsed = t0.elapsed().as_secs_f64();

                            match result {
                                Ok(sha) => { let _ = tx.send(TenantTiming { name: tenant_name, sha, duration_s: elapsed, status: "ok".into() }); }
                                Err(e)   => { let _ = tx.send(TenantTiming { name: tenant_name, sha: "<none>".into(), duration_s: elapsed, status: format!("error: {}", e) }); }
                            };
                        }));
                    }

                    for handle in handles { let _ = handle.join(); }
                    thread::sleep(Duration::from_millis(250));
                }

                drop(tx);
                for tenant in rx {
                    println!("{:<20} {:<40} ⏱️  {:.2}s", tenant.name, tenant.status, tenant.duration_s);
                    timing.push(tenant);
                }

                let total_time = start_total.elapsed().as_secs_f64();
                let avg_time = if !timing.is_empty() {
                    timing.iter().map(|t| t.duration_s).sum::<f64>() / timing.len() as f64
                } else { 0.0 };

                println!("\n📊 Performance Summary");
                println!("────────────────────────────────────────────");
                println!("Tenants Executed : {}", timing.len());
                println!("Total Time       : {:.2}s", total_time);
                println!("Average per Tenant: {:.2}s", avg_time);
                println!("Parallel Threads : {}", parallel);
                println!("────────────────────────────────────────────");
                println!("✨ Night Core parallel orchestration complete.\n");

                fs::create_dir_all("logs")?;
                let report = OrchestrationReport {
                    timestamp: timestamp.clone(),
                    tenants_executed: timing.len(),
                    total_time_s: total_time,
                    avg_time_s: avg_time,
                    parallel_threads: parallel,
                    tenants: timing.clone(),
                };

                fs::write("logs/orchestration_report.json", serde_json::to_string_pretty(&report)?)?;
                println!("📁 Saved orchestration report → logs/orchestration_report.json");

                // lightweight live dashboard (kept from before)
                let mut html = String::from(
                    "<!doctype html><html><head><meta charset='utf-8'>\
                     <meta http-equiv='refresh' content='10'>\
                     <title>Night Core Dashboard</title>\
                     <style>body{font-family:Arial;background:#0b0e13;color:#e0e0e0;}\
                     h2{color:#74c0fc;}table{width:100%;border-collapse:collapse;}\
                     th,td{padding:6px;border-bottom:1px solid #333;}\
                     tr:hover{background:#1a1f2a;} .ok{color:#74ff7e;} .err{color:#ff6b6b;}\
                     </style></head><body>"
                );
                html.push_str(&format!(
                    "<h2>🧩 Night Core™ Proof Dashboard</h2>\
                     <p><b>Timestamp:</b> {}<br>\
                     <b>Tenants:</b> {} &nbsp; <b>Parallel:</b> {} &nbsp; \
                     <b>Total:</b> {:.2}s &nbsp; <b>Avg:</b> {:.2}s</p>\
                     <table><tr><th>Tenant</th><th>SHA-256</th><th>Status</th><th>Duration (s)</th></tr>",
                    timestamp, report.tenants_executed, parallel, report.total_time_s, report.avg_time_s
                ));
                for t in &report.tenants {
                    let cls = if t.status == "ok" { "ok" } else { "err" };
                    html.push_str(&format!(
                        "<tr><td>{}</td><td>{}</td><td class='{}'>{}</td><td>{:.2}</td></tr>",
                        t.name, t.sha, cls, t.status, t.duration_s
                    ));
                }
                html.push_str("</table><br><p>🔒 Verified by Night Core AUFS Proof System</p></body></html>");
                fs::write("logs/nightcore_dashboard.html", html)?;
                println!("🖥️  Updated proof dashboard → logs/nightcore_dashboard.html");

                if let Err(e) = open::that("logs/nightcore_dashboard.html") {
                    eprintln!("⚠️ Failed to auto-open dashboard: {}", e);
                } else {
                    println!("🌐 Dashboard opened in default browser.\n");
                }
            } else if let Some(p) = path {
                let tenant_name = p.file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "unknown".to_string());

                if let Err(e) = verify::ensure_pubkey_sync(p.to_str().unwrap_or_default(), &tenant_name) {
                    eprintln!("⚠️ Pubkey sync failed: {}", e);
                }

                let t0 = Instant::now();
                let sha = verify::verify_and_run(&p, proof)?;
                let elapsed = t0.elapsed().as_secs_f64();
                println!("✅ {} executed successfully (sha {}, {:.2}s)", tenant_name, sha, elapsed);
            } else {
                println!("⚙️ Usage: nightcore run --all [--proof] [--parallel N] OR --path <tenant_dir> [--proof]");
            }
        }

        Commands::Sign { dir, key } => sign_tenant::sign_tenant(&dir, &key)?,
        Commands::Inspect { dir } => verify::inspect_manifest(&dir)?,
        Commands::ExportPubkeyHashes => export_pubkeys()?,

        Commands::ExportDashboard { diff } => {
            build_global_history_dashboard(diff)?;
            println!("✅ Exported → logs/nightcore_history_dashboard.html");
            if let Err(e) = open::that("logs/nightcore_history_dashboard.html") {
                eprintln!("⚠️ Failed to auto-open dashboard: {}", e);
            } else {
                println!("🌐 Dashboard opened in default browser.");
            }
        }

        Commands::InspectState { tenant, all_tenants, summary } => {
            use nc_state::TenantState;

            if all_tenants {
                println!("\n🌐 Inspecting all tenants under /state\n");
                let tenants = list_state_tenants()?;
                for t in tenants {
                    let state = TenantState::open(".", &t)?;
                    let hist = state.get_json::<Vec<Value>>("proof_history")?.unwrap_or_default();
                    let ok = hist.iter().filter(|v| v.get("verified").and_then(|b| b.as_bool()).unwrap_or(false)).count();
                    println!("🧠 Tenant: {} ({} proofs)", t, hist.len());
                    println!("✅ Verified {}/{} ({:.1}%)", ok, hist.len(), if hist.is_empty() { 0.0 } else { (ok as f64)*100.0/(hist.len() as f64) });
                    println!("───────────────────────────────────────\n");
                }
            } else if let Some(tenant) = tenant {
                use chrono::Utc;
                println!("\n🧠 Inspecting persistent state for tenant: {}\n", tenant);
                let state = TenantState::open(".", &tenant)?;

                if let Some(last) = state.get_json::<Value>("last_proof")? {
                    println!("🧾 Last Proof Record");
                    println!("{}", serde_json::to_string_pretty(&last)?);
                    println!();
                } else {
                    println!("(no last_proof found)\n");
                }

                let hist = state.get_json::<Vec<Value>>("proof_history")?.unwrap_or_default();
                println!("📜 Full Proof History ({} entries):", hist.len());
                for (i, v) in hist.iter().enumerate() {
                    println!("  #{} → {}", i + 1, serde_json::to_string_pretty(v)?);
                }

                fs::create_dir_all("logs")?;
                fs::write(
                    format!("logs/{}_proof_history.json", tenant),
                    serde_json::to_string_pretty(&hist)?,
                )?;
                println!("\n💾 Exported proof history → logs/{}_proof_history.json", tenant);

                if summary {
                    // compute quick stats
                    let total = hist.len();
                    let mut ok_count = 0usize;
                    let mut size_sum = 0u64;
                    let mut first_ts: Option<DateTime<Utc>> = None;
                    let mut last_ts: Option<DateTime<Utc>> = None;

                    for v in &hist {
                        if v.get("verified").and_then(|b| b.as_bool()).unwrap_or(false) {
                            ok_count += 1;
                        }
                        if let Some(sz) = v.get("size").and_then(|n| n.as_u64()) {
                            size_sum += sz;
                        }
                        if let Some(ts) = v.get("timestamp").and_then(|s| s.as_str()) {
                            if let Ok(dt) = ts.parse::<DateTime<Utc>>() {
                                first_ts = Some(first_ts.map_or(dt, |cur| cur.min(dt)));
                                last_ts  = Some(last_ts .map_or(dt, |cur| cur.max(dt)));
                            }
                        }
                    }

                    let avg_size = if total == 0 { 0 } else { size_sum / total as u64 };

                    println!("\n📊 Tenant Summary");
                    println!("───────────────────────────────────────");
                    println!("Tenant           : {}", tenant);
                    println!("Total Proofs     : {}", total);
                    println!("Verified Success : {} ({:.1}%)", ok_count, if total == 0 { 0.0 } else { (ok_count as f64)*100.0/(total as f64) });
                    println!("Average Size     : {} bytes", avg_size);
                    println!("First Proof      : {}", first_ts.map(|d| d.to_rfc3339()).unwrap_or_else(|| "n/a".into()));
                    println!("Last Proof       : {}",  last_ts.map(|d| d.to_rfc3339()).unwrap_or_else(|| "n/a".into()));
                    println!("───────────────────────────────────────");
                }
            } else {
                println!("⚙️ Usage: nightcore inspect-state --tenant <NAME> [--summary]  OR  --all-tenants");
            }
        }

        Commands::Upgrade { manifest } => aufs::verify_upgrade(PathBuf::from(&manifest).as_path())?,
        Commands::SignUpgrade { manifest, key } => aufs::sign_upgrade_manifest(PathBuf::from(&manifest), PathBuf::from(&key))?,
        Commands::Unlock => {
            if unlock::check_unlock() {
                println!("✅ Unlock succeeded — Pro features enabled");
            } else {
                println!("❌ Unlock failed — remaining in open-core mode");
            }
        }
    }

    println!("✨ Night Core execution complete.\n");
    Ok(())
}

/// 🔍 Pubkey export helper
fn export_pubkeys() -> Result<()> {
    println!("🔍 Exporting pubkey hashes for upgrade manifest:");
    let modules_dir = PathBuf::from("modules");
    for entry in fs::read_dir(&modules_dir)
        .with_context(|| format!("reading modules directory: {}", modules_dir.display()))? {
        let entry = entry?;
        if !entry.path().is_dir() { continue; }
        let tenant_dir = entry.path();
        let tenant_name = entry.file_name().to_string_lossy().into_owned();
        let pubkey_path = tenant_dir.join("pubkey.b64");

        if pubkey_path.exists() {
            let pubkey_b64 = fs::read_to_string(&pubkey_path)?.trim().to_string();
            let pubkey_bytes = STANDARD.decode(&pubkey_b64)?;
            let hash = Sha256::digest(&pubkey_bytes);
            println!(
                "{{ \"name\": \"{}\", \"pubkey_hash\": \"SHA256:{}\" }}",
                tenant_name, hex::encode(hash)
            );
        } else {
            println!(
                "{{ \"name\": \"{}\", \"pubkey_hash\": \"missing pubkey.b64\" }}",
                tenant_name
            );
        }
    }
    println!("✅ Export complete.");
    Ok(())
}

/// ─────────────────────────────────────────────────────────────
/// Global HISTORY dashboard with optional per-tenant diff
/// ─────────────────────────────────────────────────────────────
fn build_global_history_dashboard(show_diff: bool) -> Result<()> {
    use nc_state::TenantState;

    fs::create_dir_all("logs")?;
    let tenants = list_state_tenants()?;

    let mut sections = String::new();

    for t in tenants {
        let state = TenantState::open(".", &t)?;
        let hist_val = state.get_json::<Vec<Value>>("proof_history")?.unwrap_or_default();

        // Coerce into typed rows and sort by timestamp ASC
        let mut rows: Vec<ProofRow> = hist_val.into_iter().filter_map(|v| {
            let sha = v.get("sha256")?.as_str()?.to_string();
            let size = v.get("size")?.as_u64()?;
            let verified = v.get("verified")?.as_bool().unwrap_or(false);
            let ts = v.get("timestamp")?.as_str()?.to_string();
            Some(ProofRow { sha256: sha, size, verified, timestamp: ts })
        }).collect();

        rows.sort_by_key(|r| r.timestamp.clone());

        let (last, prev) = (rows.last().cloned(), if rows.len() >= 2 { rows.get(rows.len()-2).cloned() } else { None });

        let mut header = format!(
            "<div class='tenant'><h3>🧩 {name}</h3><p>Total proofs: <b>{n}</b></p>",
            name = t, n = rows.len()
        );

        if show_diff {
            let diff_html = render_diff_block(&last, &prev);
            header.push_str(&diff_html);
        }

        // Table of all proofs
        header.push_str("<table><tr><th>#</th><th>Timestamp</th><th>SHA-256</th><th>Size</th><th>Verified</th></tr>");
        for (i, r) in rows.iter().enumerate() {
            let vcls = if r.verified { "ok" } else { "err" };
            header.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td class='mono'>{}</td><td>{}</td><td class='{}'>{}</td></tr>",
                i+1, r.timestamp, r.sha256, r.size, vcls, r.verified
            ));
        }
        header.push_str("</table></div>");
        sections.push_str(&header);
    }

    let html = format!(r#"<!doctype html>
<html>
<head>
  <meta charset="utf-8"/>
  <title>Night Core — Historical Ledger</title>
  <style>
    body {{ font-family: Inter, Segoe UI, Arial, sans-serif; background:#0b0e13; color:#dfe6ee; margin:0; padding:24px; }}
    h1 {{ color:#74c0fc; }}
    h3 {{ color:#a5d8ff; margin-bottom:8px; }}
    .tenant {{ background:#121826; border:1px solid #223047; border-radius:14px; padding:16px; margin:16px 0; box-shadow:0 1px 10px rgba(0,0,0,.25); }}
    table {{ width:100%; border-collapse:collapse; margin-top:8px; }}
    th,td {{ padding:8px; border-bottom:1px solid #223047; font-size:14px; }}
    tr:hover {{ background:#182133; }}
    .mono {{ font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace; font-size:13px; }}
    .ok {{ color:#74ff7e; }}
    .err {{ color:#ff6b6b; }}
    .tag {{ display:inline-block; padding:2px 8px; border-radius:999px; border:1px solid #2d3c57; background:#101625; font-size:12px; margin-right:8px; }}
    .tag.ok {{ border-color:#1e5c30; background:#0f2415; color:#74ff7e; }}
    .tag.warn {{ border-color:#665b21; background:#231f0e; color:#ffd43b; }}
    .tag.err {{ border-color:#6b1f1f; background:#241010; color:#ff6b6b; }}
    .delta {{ margin:6px 0 10px; }}
  </style>
</head>
<body>
  <h1>Night Core™ — Historical Proof Ledger</h1>
  <p>Generated: {ts} &nbsp; • &nbsp; Mode: {mode}</p>
  {sections}
  <br/>
  <p style="opacity:.7">🔒 All records minted from per-tenant sled state. SHA changes highlight module mutations between last two runs.</p>
</body>
</html>"#,
        ts = Utc::now().to_rfc3339(),
        mode = if show_diff { "diff" } else { "basic" },
        sections = sections
    );

    fs::write("logs/nightcore_history_dashboard.html", html)?;
    Ok(())
}

fn render_diff_block(last: &Option<ProofRow>, prev: &Option<ProofRow>) -> String {
    match (last, prev) {
        (Some(l), Some(p)) => {
            let last_ts = l.timestamp.parse::<DateTime<Utc>>().ok();
            let prev_ts = p.timestamp.parse::<DateTime<Utc>>().ok();
            let (delta_str, delta_tag) = if let (Some(a), Some(b)) = (last_ts, prev_ts) {
                let secs = (a - b).num_seconds().max(0) as u64;
                (format!("Δ Time: {}s", secs), "tag")
            } else {
                ("Δ Time: n/a".to_string(), "tag")
            };

            let sha_changed = if l.sha256 != p.sha256 { "Yes" } else { "No" };
            let sha_class = if sha_changed == "Yes" { "warn" } else { "ok" };

            let ver_changed = if l.verified != p.verified { "Yes" } else { "No" };
            let ver_class = if ver_changed == "Yes" { "warn" } else { "ok" };

            format!(
                "<div class='delta'>
                   <span class='tag {delta_cls}'>{delta}</span>
                   <span class='tag {sha_cls}'>SHA-256 Changed: {sha_changed}</span>
                   <span class='tag {ver_cls}'>Verification Changed: {ver_changed}</span>
                 </div>
                 <div class='mono' style='font-size:12px;opacity:.85'>
                   <div>Last:     {lts}  |  {lsha}  |  ok={lok}</div>
                   <div>Previous: {pts}  |  {psha}  |  ok={pok}</div>
                 </div>",
                delta_cls = delta_tag,
                delta = delta_str,
                sha_cls = sha_class,
                sha_changed = sha_changed,
                ver_cls = ver_class,
                ver_changed = ver_changed,
                lts = l.timestamp,
                lsha = l.sha256,
                lok = l.verified,
                pts = p.timestamp,
                psha = p.sha256,
                pok = p.verified
            )
        }
        (Some(_), None) => "<div class='delta'><span class='tag'>First proof recorded</span></div>".into(),
        _ => "<div class='delta'><span class='tag'>No proofs found</span></div>".into(),
    }
}

/// scan ./state for tenant folders
fn list_state_tenants() -> Result<Vec<String>> {
    let mut names = vec![];
    let root = PathBuf::from("state");
    if !root.exists() { return Ok(names); }
    for e in fs::read_dir(&root)? {
        let e = e?;
        if e.path().is_dir() {
            names.push(e.file_name().to_string_lossy().into_owned());
        }
    }
    names.sort();
    Ok(names)
}
