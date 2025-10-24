# 🌑 Night Core — Secure Multi-Tenant WebAssembly Orchestration Framework
### **B106 Edition · Wasmtime 37 + WASI P1 · Ed25519 · MIT Open Core**

Night Core is an open-source, enterprise-grade **WASM orchestration framework** written in Rust.  
It verifies, isolates, and executes signed .wasm modules in secure sandboxes — supporting multi-tenant workloads, self-healing upgrades, and cryptographic proof of integrity.

## 🧱 Core Features
- ✅ **Wasmtime 37 + WASI P1** sandbox runtime  
- 🔐 **Ed25519 + SHA-256** signature & integrity verification  
- 🧩 **Multi-Tenant Orchestration** (--all) with per-tenant policies  
- 📊 **HTML Dashboard** with JSONL logs & audit hash chain  
- 🔁 **AUFS** — Autonomous Upgrade & Fork System (threshold-signed, self-healing)  
- ☁️ **AWS-Ready** (Nitro Enclaves / Fargate / KMS / Lambda integration path)  
- 🪶 MIT Open Core — B106 branding & dashboard visuals reserved  

## 🚀 Quick Start

\\\ash
git clone https://github.com/<your-user>/nightcore.git
cd nightcore
cargo build --release
nightcore generate-keys --out-dir keys/
nightcore verify --tenant tenantA-hello --explain
nightcore run --all --parallel 2
nightcore dashboard --open
\\\

## 📚 Documentation
- [**ROADMAP.md**](./ROADMAP.md)
- [**AUFS Overview**](./docs/aufs-overview.md)
- [**Threat Model**](./docs/security/THREATMODEL.md)
- [**Governance**](./docs/security/GOVERNANCE.md)
- [**Cold Storage**](./ops/cold-storage.md)

## 🧩 Repository Layout
\\\
src/           → Rust sources (CLI, orchestration, verify, policy)
modules/       → Tenant modules (e.g., tenantA-hello, tenantB-math)
configs/       → crypto.toml, policy.default.toml
upgrades/      → AUFS manifests & adapters
docs/          → project documentation
ops/           → operational procedures
logs/          → runtime & audit outputs
\\\

## 🛡️ Security Baseline
- Fuel / timeout / memory limits per tenant  
- FS/NET sandbox policy per manifest  
- Threshold-signed upgrades (2-of-3)  
- Hash-chained audit logs & reproducible builds  
- Optional FIPS mode (\--crypto-mode fips\)

## ☁️ AWS Integration Strategy (Preview)
- Nitro Enclaves / Fargate, KMS/CloudHSM, S3/DynamoDB, Lambda/EventBridge, CloudWatch/SecHub

## 🤝 Contributing
See [Governance](./docs/security/GOVERNANCE.md) and [AUFS Overview](./docs/aufs-overview.md).

## 📜 License
Night Core Open-Core Edition is licensed under the [MIT License](./LICENSE).  
“B106 Edition” name, logo, and dashboard visuals are trademarks of **B106 Labs**.

## 🌟 Vision
> *“Night Core becomes a self-healing, provable, autonomous compute standard — secure enough for enterprises, open enough for everyone.”*
