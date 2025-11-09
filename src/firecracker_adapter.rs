use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::{env, fs, thread, time::Duration};

/// 🔥 Night Core — Firecracker microVM adapter (dual-mode + schema-aware)
/// Accepts vm_timeout directly from main.rs
pub fn launch_microvm_with_timeout(kernel: &str, rootfs: &str, timeout_secs: u64) -> Result<()> {
    println!("⚙️  Starting Firecracker microVM...");

    // ─────────────────────────────
    // Detect Firecracker schema
    // ─────────────────────────────
    let mut supports_smt = true;
    let version_output = Command::new(if cfg!(target_os = "windows") { "wsl" } else { "firecracker" })
        .arg("--version")
        .output()
        .ok();

    if let Some(out) = version_output {
        if let Ok(txt) = String::from_utf8(out.stdout) {
            println!("🔍 Firecracker version detected: {}", txt.trim());
            if txt.contains("v0.") || txt.contains("v1.0") || txt.contains("v1.8") {
                supports_smt = false;
            }
        }
    }

    // ─────────────────────────────
    // Prepare logger and config paths
    // ─────────────────────────────
    let logs_dir = std::path::PathBuf::from("logs");
    fs::create_dir_all(&logs_dir).ok();
    let boot_log = logs_dir.join("firecracker_boot.log");

    // ─────────────────────────────
    // Resolve kernel/rootfs paths (absolute WSL-safe)
    // ─────────────────────────────
    let (kernel_abs, rootfs_abs) = if cfg!(target_os = "windows") {
        (
            "/var/lib/firecracker/vmlinux".to_string(),
            "/var/lib/firecracker/rootfs.ext4".to_string(),
        )
    } else {
        (kernel.to_string(), rootfs.to_string())
    };

    let escape = |s: &str| s.replace('\\', "\\\\");
    let machine_field = if supports_smt { "\"smt\": false" } else { "\"ht_enabled\": false" };

    // ─────────────────────────────
    // Generate config (auto-exit init)
    // ─────────────────────────────
    let config = format!(
        r#"{{
  "boot-source": {{
    "kernel_image_path": "{kernel}",
    "boot_args": "console=ttyS0 reboot=k panic=1 pci=off root=/dev/vda rw init=/bin/true"
  }},
  "drives": [
    {{
      "drive_id": "rootfs",
      "path_on_host": "{rootfs}",
      "is_root_device": true,
      "is_read_only": false
    }}
  ],
  "machine-config": {{
    "vcpu_count": 1,
    "mem_size_mib": 128,
    {machine_field}
  }},
  "logger": {{
    "log_path": "/mnt/c/Users/gabeg/source/repos/nightcore-worker/logs/firecracker_boot.log",
    "level": "Info",
    "show_level": true,
    "show_log_origin": false
  }}
}}"#,
        kernel = escape(&kernel_abs),
        rootfs = escape(&rootfs_abs),
        machine_field = machine_field
    );

    fs::write("firecracker_config.json", config)
        .context("writing firecracker_config.json")?;
    println!("📄 Firecracker config saved → firecracker_config.json");
    println!("🧾 Boot log → {}", boot_log.display());

    // ─────────────────────────────
    // Detect OS and prepare command
    // ─────────────────────────────
    let os = env::consts::OS;
    println!("🧠 Detected host OS: {}", os);

    let mut cmd = if os == "windows" {
        println!("💡 Launching Firecracker inside WSL...\n");
        let mut c = Command::new("wsl");
        c.args([
            "firecracker",
            "--no-api",
            "--config-file",
            "firecracker_config.json",
        ]);
        c.stdout(Stdio::null()).stderr(Stdio::null());
        c
    } else {
        let mut c = Command::new("firecracker");
        c.args(["--no-api", "--config-file", "firecracker_config.json"]);
        c.stdout(Stdio::null()).stderr(Stdio::null());
        c
    };

    // ─────────────────────────────
    // Run with provided timeout
    // ─────────────────────────────
    println!("🕒 VM timeout set to {} seconds", timeout_secs);
    let mut child = cmd.spawn().context("launching firecracker process")?;
    println!("🚀 Firecracker process started (PID = {:?})", child.id());

    thread::sleep(Duration::from_secs(timeout_secs));

    match child.try_wait()? {
        Some(status) => println!("✅ Firecracker exited cleanly ({})", status),
        None => {
            println!("⚠️  Firecracker still running — killing after {}s timeout.", timeout_secs);
            let _ = child.kill();
        }
    }

    println!("📁 Boot log written to {}", boot_log.display());
    Ok(())
}
