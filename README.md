<p align="center">
  <img src="assets/nightcore_logo_tm.png" alt="Night Core™ Logo" width="280"/>
</p>

<h1 align="center">Night Core™ — Secure. Autonomous. Verified.</h1>

<p align="center">
  <strong>B106 Edition · Rust + Wasmtime v37 + WASI P1 · Ed25519 · MIT Open Core</strong><br/>
  Verifiable Compute Framework for Secure Multi-Tenant Execution
</p>

---

## 🌑 Overview
**Night Core™** is an open-source, enterprise-grade WebAssembly orchestration framework written in Rust.  
It verifies, isolates, and executes signed `.wasm` modules in secure sandboxes — supporting multi-tenant workloads, self-healing upgrades, and cryptographic proof of integrity.

---

## 🧱 Core Features
- ✅ **Wasmtime 37 + WASI P1** sandbox runtime  
- 🔐 **Ed25519 + SHA-256** signature & integrity verification  
- 🧩 **Multi-Tenant Orchestration (`--all`)** with per-tenant policies  
- 📊 **HTML Dashboard** with JSONL logs & audit hash chain  
- 🔁 **AUFS** — Autonomous Upgrade & Fork System (threshold-signed, self-healing)  
- ☁️ **AWS-Ready** (Nitro Enclaves / Fargate / KMS / Lambda integration path)  
- 🪶 **MIT Open Core** — “B106 Edition” branding & dashboard visuals reserved  

---

## 🚀 Quick Start
```bash
git clone https://github.com/xnfinite/nightcore.git
cd nightcore
cargo build --release

# Generate signing keys
nightcore generate-keys --out-dir keys/

# Verify a tenant module
nightcore verify --tenant tenantA-hello --explain

# Run all tenants in parallel
nightcore run --all --parallel 2

# Launch dashboard
nightcore dashboard --open
