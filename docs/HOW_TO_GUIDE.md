<!-- Night Core™ v38 — How-To Guide (Verified Edition) -->
<p align="center">
  <img src="../assets/nightcore_logo_tm.png" alt="Night Core Logo™" width="260"/>
</p>

<h1 align="center">Night Core™ v38 — Full How-To Guide</h1>

<p align="center">
  <strong>Secure • Autonomous • Verified</strong>
</p>

---

## 🧩 Overview
Night Core™ is an open-source, verifiable compute framework written in Rust using Wasmtime 37 + WASI P1.  
This guide provides a complete walkthrough — from build to tenant signing, AUFS upgrades, and proof verification.

---

## ⚙️ Environment Setup
### Prerequisites
- **Rust (nightly)** — <https://rustup.rs>  
- **Git + GPG** — for commit and maintainer signing  
- **PowerShell 7+** (recommended)  
- **Wasmtime 37** — installed automatically by Cargo  

### Project Layout
```
C:\Users\gabeg\source\repos\worker
│   Cargo.toml
│   README.md
│
├── src\
│   ├── main.rs
│   └── sign_tenant.rs
│
├── modules\
│   ├── tenantA-hello\
│   └── tenantB-math\
│
├── keys\
│   └── maintainers\
│
└── logs\
```

---

## 🔐 Key Generation & Signing
### Generate Maintainer Keys
```bash
cargo +nightly run -- generate-keys --out-dir keys/maintainers
```

### Sign a Tenant Module
```bash
cargo +nightly run -- sign --dir modules/tenantA-hello --key keys/maintainers/admin1.key
```
Outputs → `module.sig`  `pubkey.b64`  `module.sha256`

---

## 🧩 Running Tenants
### Verify Environment
```bash
cargo +nightly run -- verify-env
```

### Run All Tenants
```bash
cargo +nightly run -- run --all
```

Each tenant’s `.wasm` is:
- Verified via Ed25519 signature  
- Checked for SHA-256 integrity  
- Executed inside its own Wasmtime sandbox  
- Logged to `logs/orchestration_report.json` and `logs/nightcore_dashboard.html`

---

## 🔁 AUFS — Autonomous Upgrade & Fork System
### Verify an Upgrade Manifest
```bash
cargo +nightly run -- upgrade --manifest upgrades/manifests/upgrade_manifest.json
```

### Sign an Upgrade (2-of-3 threshold)
```bash
cargo +nightly run -- sign-upgrade --manifest upgrades/manifests/upgrade_manifest.json
```

### Safe Push Workflow
Every commit runs the Foundation Lock hook:
```
🔒 Night Core Foundation Lock active…
✅ Hook active — baseline integrity verified.
```
Only verified baselines are accepted into `main`.

---

## 🧮 Proof Mode
Verify signatures and hashes without executing WASM:
```bash
cargo +nightly run -- run --all --proof
```
Generates:
- `logs/nightcore_proof.html`  
- `logs/audit.log`  
- `scripts/nightcore_manual_proof_push.ps1` for Chronicle sync  

---

## 🧠 Troubleshooting

| Issue | Cause | Fix |
|:------|:------|:----|
| ❌ “input bytes aren’t valid utf-8” | WASM printed raw binary data | Use text output or filter stdout |
| ❌ “signature verification failed” | Out-of-date signature | Re-sign the module |
| ❌ “gpg: no agent running” | Windows GPG lock corruption | Run `scripts/fix_gpg_agent.ps1` |
| ❌ “manifest missing” | Missing `manifest.json` | Copy template from `/modules/example` |

---

## 📜 Logs & Proofs
Night Core automatically creates:
- `logs/audit.log` — Hash-chained integrity record  
- `logs/nightcore_dashboard.html` — Visual proof summary  
- `logs/orchestration_report.json` — Machine-readable execution report  

---

## ☁️ AWS Integration (Preview)
Night Core is designed for AWS Cloud deployment using:
- **Nitro Enclaves / Fargate** for tenant isolation  
- **KMS / CloudHSM** for key management  
- **S3 / DynamoDB** for manifest and proof storage  
- **Lambda / EventBridge** for automated AUFS upgrades  

---

## 🪶 Version Tags
| Tag | Description |
|:----|:-------------|
| `v38-stable-aufs-verified` | Verified AUFS baseline |
| `v38-proof-mode` | Proof-only verification |
| `v38-dashboard` | HTML proof dashboard published |

---

## ⚡ CLI Quick Reference

| Command | Description |
|:---------|:-------------|
| `cargo +nightly run -- verify-env` | Verify Wasmtime environment |
| `cargo +nightly run -- generate-keys --out-dir keys/` | Generate Ed25519 key pairs |
| `cargo +nightly run -- sign --dir <tenant> --key <key>` | Sign a tenant module |
| `cargo +nightly run -- run --all` | Run all verified tenants |
| `cargo +nightly run -- run --all --proof` | Proof-only verification mode |
| `cargo +nightly run -- export-pubkey-hashes` | Export tenant public key hashes |
| `cargo +nightly run -- upgrade --manifest <file>` | Run AUFS upgrade verification |
| `cargo +nightly run -- sign-upgrade --manifest <file>` | Sign an AUFS upgrade manifest |
| `cargo +nightly run -- help` | Show command list |

---

<p align="center"><i>Part of the Night Core™ Secure Compute Stack — B106 Edition.</i></p>
