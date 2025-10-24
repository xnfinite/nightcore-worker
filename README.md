# 🌑 Night Core — Secure Multi-Tenant WebAssembly Orchestration Framework
### **B106 Edition · Wasmtime 37 + WASI P1 · Ed25519 · MIT Open Core**

Night Core is an open-source, enterprise-grade **WebAssembly orchestration framework** written in **Rust**.  
It verifies, isolates, and executes signed `.wasm` modules in secure sandboxes — supporting multi-tenant workloads, self-healing upgrades, and cryptographic proof of integrity.

---

## 🧱 Core Features

✅ **Wasmtime 37 + WASI P1 sandbox runtime**  
🔐 **Ed25519 + SHA-256 signature & integrity verification**  
🧩 **Multi-Tenant Orchestration (`--all`) with per-tenant policies**  
📊 **HTML Dashboard with JSONL logs & audit hash chain**  
🔁 **AUFS — Autonomous Upgrade & Fork System (threshold-signed, self-healing)**  
☁️ **AWS-Ready** (Nitro Enclaves / Fargate / KMS / Lambda integration path)  
🧠 **Night Mesh Communication Stack (v1.0)** — Proof automation & decentralized communication  
🪶 **MIT Open Core** — B106 branding & dashboard visuals reserved  

---

## 🚀 Quick Start

```bash
git clone https://github.com/b106labs/nightcore.git
cd nightcore
cargo build --release

# Generate Ed25519 keys
nightcore generate-keys --out-dir keys/

# Verify and explain module signature
nightcore verify --tenant tenantA-hello --explain

# Run all verified tenants in parallel
nightcore run --all --parallel 2

# Open dashboard telemetry
nightcore dashboard --open
