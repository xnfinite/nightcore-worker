# 🧩 Night Core™ Worker — Firecracker Backend Guide (v39 Verified)

## Overview
The **Firecracker backend** allows Night Core™ Worker to execute verified WebAssembly (WASM) modules inside **microVMs** with hardware-grade isolation.  
It integrates with the existing Night Core Worker runtime that already provides:

- ✅ Ed25519 signature verification  
- ✅ SHA-256 integrity checking  
- ✅ Multi-tenant sandbox execution  
- ✅ Chronicle / Proof dashboard reporting  

Firecracker extends this by adding a **microVM layer** around each tenant’s module, providing stronger isolation than standard WASI sandboxes.

---

## ⚙️ Architecture Flow
```
Night Core CLI (main.rs)
        ↓
Firecracker Adapter (src/firecracker_adapter.rs)
        ↓
Firecracker MicroVM (guest WASI environment)
        ↓
Tenant WASM module → Verified & Executed
```

**main.rs** — Handles CLI commands (`run`, `verify`, `sign`, etc.) and routes to the selected backend (`wasmtime` or `firecracker`).  
**firecracker_adapter.rs** — Implements the Firecracker backend by:
1. Preparing a temporary VM rootfs with the tenant’s verified WASM.  
2. Launching `firecracker` with a JSON config (`firecracker_config.json`).  
3. Passing WASI arguments and collecting stdout/stderr for proof logging.  
4. Destroying the microVM after execution to maintain stateless isolation.

---

## 🧰 Requirements
| Component | Description |
|------------|-------------|
| **Rust 1.80+ (nightly)** | for building Night Core Worker |
| **Firecracker v1.9.0 +** | microVM binary (downloaded separately) |
| **Wasmtime 37 + WASI P1** | for non-microVM sandbox mode |
| **Linux or WSL2 kernel 5.10 +** | Firecracker host requirement |

---

## 🔧 Setup (Developers)
1. **Clone the repository**
   ```bash
   git clone https://github.com/xnfinite/nightcore-worker.git
   cd nightcore-worker
   ```

2. **Ensure dependencies**
   ```bash
   cargo +nightly build
   ```

3. **Download Firecracker**
   ```bash
   mkdir firecracker_assets
   cd firecracker_assets
   curl -LO https://github.com/firecracker-microvm/firecracker/releases/download/v1.9.0/firecracker-v1.9.0-x86_64.tgz
   tar -xzf firecracker-v1.9.0-x86_64.tgz
   cd ..
   ```

4. **Create Firecracker config**
   ```jsonc
   {
     "boot-source": { "kernel_image_path": "vmlinux.bin", "boot_args": "console=ttyS0 reboot=k panic=1 pci=off" },
     "drives": [{ "drive_id": "rootfs", "path_on_host": "rootfs.ext4", "is_root_device": true, "is_read_only": false }],
     "machine-config": { "vcpu_count": 1, "mem_size_mib": 128 }
   }
   ```

   Save as `firecracker_config.json` in the repo root.

5. **Run Night Core Worker**
   ```bash
   cargo +nightly run -- run --all --backend firecracker --vm-timeout 15
   ```

   Each tenant module (e.g. `tenantA-hello`, `tenantB-math`) will:
   - Verify Ed25519 signature + SHA-256 hash  
   - Mount into its own microVM  
   - Execute with resource limits  
   - Log proof data to `logs/nightcore_proof.html`

---

## 🔐 Security Model
| Layer | Protection |
|--------|-------------|
| **Ed25519 Verification** | Validates module signature against tenant pubkey |
| **SHA-256 Integrity** | Confirms module content hash matches manifest |
| **WASI Sandbox** | Restricts syscalls and I/O within VM |
| **Firecracker MicroVM** | Hardware-virtualized boundary around each tenant |
| **Foundation Lock Hooks** | Prevents baseline and key modification in Git |

---

## 🧾 Proof and Audit Logs
Execution results appear in:
- `logs/nightcore_proof.html`
- `logs/orchestration_report.json`
- `docs/legacy/Night_Core_Chronicle.md`

Each entry records:
```
Tenant: tenantA-hello
Backend: Firecracker v1.9.0
Signature: ✔ Verified (Ed25519)
Integrity: ✔ SHA-256 match
Execution: Success (Exit code 0)
```

---

## 🧪 Example Manual Test
```bash
cargo +nightly run -- run --path modules/tenantA-hello --backend firecracker
```

Output:
```
🔍 Verifying module signature and hash...
✅ Verification passed.
🔥 Launching Firecracker microVM...
💾 Output: Hello from Tenant A!
🔒 Shutting down microVM...
```

---

## 🧩 WSL 2 + Ubuntu Installation
If you're on Windows 10/11, run Night Core Worker inside WSL 2 Ubuntu.

### 1. Enable WSL 2
```powershell
wsl --install -d Ubuntu
wsl -l -v
```
Ensure VERSION=2.

### 2. Inside Ubuntu
```bash
sudo apt update && sudo apt install -y build-essential git curl wget qemu-kvm libvirt-daemon-system virt-manager
lsmod | grep kvm
egrep -c '(vmx|svm)' /proc/cpuinfo
```

### 3. Verify /dev/kvm
```bash
ls -l /dev/kvm
sudo usermod -aG kvm $USER
newgrp kvm
```

### 4. Run Night Core Worker
```bash
cargo +nightly run -- run --all --backend firecracker --vm-timeout 15
```

Check Firecracker microVMs:
```bash
ps aux | grep firecracker
ls /tmp/firecracker-*.socket
```

Query active VM:
```bash
curl --unix-socket /tmp/firecracker-tenantA.socket -i http://localhost/machine-config
```

### 5. Monitor Lifecycle
```bash
ps aux | grep firecracker
tail -n 10 logs/nightcore_proof.html
```

---

## ⚡ Quick VM Debug Commands
| Action | Command |
|---------|----------|
| List microVM processes | `ps -ef | grep firecracker` |
| View VM stdout | `journalctl -u firecracker` |
| Query API socket | `curl --unix-socket /tmp/firecracker-tenantX.socket http://localhost/` |
| Clean orphan sockets | `sudo rm /tmp/firecracker-*.socket` |

---

## ✅ Summary
- 🔒 Verified with Ed25519 + SHA-256  
- 🧩 Multi-tenant Firecracker microVM execution  
- 🧾 Proof logs for every tenant run  
- 🛡️ Immutable Foundation baseline protected by Git hooks  

Night Core Worker + Firecracker = **verifiable, micro-VM-isolated WASM execution.**

© 2025 B106 Labs / Night Core™ — Secure • Autonomous • Verified
