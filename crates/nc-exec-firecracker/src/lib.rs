use anyhow::{anyhow, Context, Result};
use nc_exec::{ExecConfig, ExecProof, SandboxBackend};
use std::{fs, path::Path, process::Stdio};
use tempfile::tempdir;
use tokio::process::Command;

/// Firecracker adapter (MVP):
/// - Writes minimal VM config
/// - Boots `firecracker` with a vsock/serial console
/// - Runs `nc-exec-cli` inside the guest (future: initrd w/ static binary)
pub struct FirecrackerBackend;

impl SandboxBackend for FirecrackerBackend {
    fn name(&self) -> &'static str { "firecracker" }

    fn verify(&self, module_path: &Path) -> Result<()> {
        // Reuse host-side verify (same checks).
        // You could enforce "require-signed-guest-rootfs" here later.
        nc_exec::default_verify(module_path)
    }

    fn execute(&self, cfg: &ExecConfig) -> Result<ExecProof> {
        // Preconditions
        let fc = which::which("firecracker")
            .map_err(|_| anyhow!("firecracker binary not found in PATH"))?;
        let kernel = std::env::var("NC_FC_KERNEL")
            .context("Set NC_FC_KERNEL to a vmlinux path")?;
        let rootfs = std::env::var("NC_FC_ROOTFS")
            .context("Set NC_FC_ROOTFS to a rootfs image (ext4)")?;

        // Temp workspace for FC socket + cfg
        let tmp = tempdir()?;
        let api_sock = tmp.path().join("fc.sock");
        let cfg_path = tmp.path().join("vmcfg.json");

        // Minimal machine config (vCPU/mem tunables can be dynamic)
        let vcpus = std::env::var("NC_FC_VCPUS").ok().and_then(|s| s.parse().ok()).unwrap_or(1);
        let mem_mib = std::env::var("NC_FC_MEM").ok().and_then(|s| s.parse().ok()).unwrap_or(512);

        let cfg = serde_json::json!({
          "boot-source": { "kernel_image_path": kernel, "boot_args": "console=ttyS0 reboot=k panic=1 pci=off" },
          "drives": [{
            "drive_id": "rootfs",
            "path_on_host": rootfs,
            "is_root_device": true,
            "is_read_only": false
          }],
          "machine-config": { "vcpu_count": vcpus, "mem_size_mib": mem_mib, "ht_enabled": false }
        });
        fs::write(&cfg_path, serde_json::to_vec_pretty(&cfg)?)?;

        // Launch Firecracker with API socket
        let mut child = Command::new(fc)
            .args(["--api-sock", api_sock.to_str().unwrap()])
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .context("spawn firecracker")?;

        // POST config via `curl` (MVP: rely on system curl)
        // You can replace with direct Unix-domain-socket HTTP in Rust later.
        let apply_cfg = || -> Result<()> {
            let body = fs::read(&cfg_path)?;
            let status = Command::new("curl")
                .args([
                    "-sS", "-X", "PUT",
                    "--unix-socket", api_sock.to_str().unwrap(),
                    "http://localhost/boot-source",
                    "-d", &serde_json::to_string(&cfg["boot-source"])?,
                ])
                .status()?;
            if !status.success() { return Err(anyhow!("apply boot-source failed")); }

            let status = Command::new("curl")
                .args([
                    "-sS", "-X", "PUT",
                    "--unix-socket", api_sock.to_str().unwrap(),
                    "http://localhost/drives/rootfs",
                    "-d", &serde_json::to_string(&cfg["drives"][0])?,
                ])
                .status()?;
            if !status.success() { return Err(anyhow!("apply rootfs failed")); }

            let status = Command::new("curl")
                .args([
                    "-sS", "-X", "PUT",
                    "--unix-socket", api_sock.to_str().unwrap(),
                    "http://localhost/machine-config",
                    "-d", &serde_json::to_string(&cfg["machine-config"])?,
                ])
                .status()?;
            if !status.success() { return Err(anyhow!("apply machine-config failed")); }

            let status = Command::new("curl")
                .args([
                    "-sS", "-X", "PUT",
                    "--unix-socket", api_sock.to_str().unwrap(),
                    "http://localhost/actions",
                    "-d", r#"{"action_type":"InstanceStart"}"#,
                ])
                .status()?;
            if !status.success() { return Err(anyhow!("start vm failed")); }
            Ok(())
        };
        apply_cfg()?;

        // TODO: exec ‘nc-exec-cli’ inside guest (via init or vsock agent).
        // For MVP, we’ll return a stub proof to validate end-to-end wiring.
        // Next patch: add a guest agent that receives cfg over vsock and runs wasmtime.
        let proof = ExecProof {
            tenant: cfg.tenant.clone(),
            module_sha256: "<vm-run-stub>".into(),
            signer_key_b64: "<verified>".into(),
            started_at: chrono::Utc::now().to_rfc3339(),
            finished_at: chrono::Utc::now().to_rfc3339(),
            status: "ok".into(),
            backend: self.name().into(),
        };

        // Ensure Firecracker exits (MVP expects kernel to exit quickly)
        let _ = child.kill();
        Ok(proof)
    }
}
