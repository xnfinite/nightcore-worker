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
```

---

## 📚 Documentation
- **ROADMAP.md** — Long-term development path  
- **AUFS Overview** — Upgrade & proof system  
- **Threat Model** — Security design  
- **Governance** — Contributor & maintainer policy  
- **Cold Storage** — Secure key and manifest handling  

---

## 🧩 Repository Layout
```
src/        → Rust sources (CLI, orchestration, verify, policy)
modules/    → Tenant modules (e.g., tenantA-hello, tenantB-math)
configs/    → crypto.toml, policy.default.toml
upgrades/   → AUFS manifests & adapters
docs/       → Project documentation
ops/        → Operational procedures
logs/       → Runtime & audit outputs
```

---

## 🛡️ Security Baseline
- Fuel / timeout / memory limits per tenant  
- FS / NET sandbox policy per manifest  
- Threshold-signed upgrades (2-of-3)  
- Hash-chained audit logs & reproducible builds  
- Optional **FIPS mode** (`--crypto-mode fips`)

---

## ☁️ AWS Integration Strategy (Preview)
Night Core is designed for native deployment on **AWS Cloud** infrastructure:
- **Nitro Enclaves / Fargate** for tenant isolation  
- **KMS / CloudHSM** for key management  
- **S3 / DynamoDB** for manifest and proof storage  
- **Lambda / EventBridge** for AUFS automation  
- **CloudWatch / Security Hub** for compliance visibility  

---

## 🧭 Development Roadmap
| Phase | Focus | Status |
|:--|:--|:--|
| 1️⃣ Night Core Baseline | Wasmtime 37 + WASI P1 + Ed25519 verified execution | ✅ Complete |
| 2️⃣ AUFS | Autonomous Upgrade & Fork System | 🚧 In Progress |
| 3️⃣ Guardian | AI Containment Kernel (fuel + byte drift + rollback) | 🧠 Pioneer Layer |
| 4️⃣ Night Mesh | Distributed proof sync and audit sharing | 🔄 Planned |
| 5️⃣ Vesper | Self-documenting AI assistant | 💬 Final Layer |

---

## 📈 Profit & Launch Timeline
| Phase | Period | Goal | Revenue Range |
|:--|:--|:--|:--|
| Phase 0 — 2025 Q4 | Brand setup & GitHub launch | Foundation + community | — |
| Phase 1 — 2026 H1 | AUFS Enterprise launch (AWS Marketplace) | Trust monetization | $50K–$150K |
| Phase 2 — 2026 H2 | Guardian prototype + OEM licensing | Pioneer layer | $250K–$1M |
| Phase 3 — 2027 H1–H2 | Night Mesh + B106 Pro CLI SaaS | Recurring revenue | $100K–$250K |
| Phase 4 — 2027–2028 | Vesper AI Assistant (SaaS API) | Compliance automation | $300K–$700K |
| Phase 5 — 2028+ | AWS / Robotics integration & licensing | Expansion / acquisition | $1M–$6M+ |

**Profit Model Summary**
- Open-core MIT → Developer adoption + community growth  
- AUFS Enterprise → Paid trust & compliance  
- Guardian OEM → Robotics / AI safety licensing  
- B106 Pro CLI → SaaS subscription model  
- Vesper API → Enterprise compliance & intelligence  

---

## 🤝 Contributing
Contributions are welcome!   
See **Governance** and **AUFS Overview** for workflow and signing policies.

---

## 📜 License
**Night Core Open-Core Edition** is licensed under the **MIT License**.  
The **Night Core™**, **B106 Edition**, and **Guardian** names, logos, and dashboard visuals are proprietary trademarks of **Gabriel Ginn / B106 Labs**.

---

## 🌟 Vision
> *“Night Core becomes a self-healing, provable, autonomous compute standard —  
> secure enough for enterprises, open enough for everyone.”*

---

<p align="center"><i>© 2025 Gabriel Ginn — Building the future of verifiable autonomous compute.</i></p>