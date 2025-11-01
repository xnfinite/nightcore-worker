<!-- Night Core v38 Verified Badge -->
<p align="center">
  <img src="docs/assets/nightcore_logo_tm.png" alt="Night Core Logo™" width="220"/>
  <br/>
  <a href="https://github.com/xnfinite/nightcore-worker/actions">
    <img src="https://img.shields.io/badge/AUFS%20Verified-v38-success?style=for-the-badge&color=0B3D91" alt="AUFS Verified"/>
  </a>
  <br/>
  <sub>Night Core™ — Secure • Autonomous • Verified</sub>
</p>

<!-- Night Core v38 Verified Summary -->
### 🧩 Night Core™ v38 — Verified Build Summary

| Field | Value |
|-------|-------|
| **Commit ID** | 26c43b3 |
| **Timestamp** | 2025-10-31 21 |
| **Audit Hash** | 6dfaebee909b96f077e0d668b5c401d68ee44bbe3937e647f8aafe12dbf06cb5 |
| **Maintainers** | core-ops • system-check |
| **Status** | ✅ Verified |

Night Core™ — Secure • Autonomous • Verified Build Summary |

Night Core™ — Secure • Autonomous • Verified

### 🧩 Night Core™ v38 — Verified Build Summary

| Field | Value |
|-------|-------|
| **Commit ID** | 26c43b3 |
| **Timestamp** | 2025-10-31 21 |
| **Audit Hash** | 6dfaebee909b96f077e0d668b5c401d68ee44bbe3937e647f8aafe12dbf06cb5 |
| **Maintainers** | core-ops • system-check |
| **Status** | ✅ Verified |

Night Core™ — Secure • Autonomous • Verified Build Summary |

Night Core™ — Secure • Autonomous • Verified
<p align="center">
  <img src="assets/nightcore_logo_tm.png" alt="Night Core™ Logo" width="280"/>
</p>

<!-- Night Core v38 Verified Summary -->
### 🧩 Night Core™ v38 — Verified Build Summary

| Field | Value |
|-------|-------|
| **Commit ID** | 26c43b3 |
| **Timestamp** | 2025-10-31 21 |
| **Audit Hash** | 6dfaebee909b96f077e0d668b5c401d68ee44bbe3937e647f8aafe12dbf06cb5 |
| **Maintainers** | core-ops • system-check |
| **Status** | ✅ Verified |

Night Core™ — Secure • Autonomous • Verified Build Summary |

Night Core™ — Secure • Autonomous • Verified

### 🧩 Night Core™ v38 — Verified Build Summary

| Field | Value |
|-------|-------|
| **Commit ID** | 26c43b3 |
| **Timestamp** | 2025-10-31 21 |
| **Audit Hash** | 6dfaebee909b96f077e0d668b5c401d68ee44bbe3937e647f8aafe12dbf06cb5 |
| **Maintainers** | core-ops • system-check |
| **Status** | ✅ Verified |

Night Core™ — Secure • Autonomous • Verified Build Summary |

Night Core™ — Secure • Autonomous • Verified

<h1 align="center">Night Core™ — Secure. Autonomous. Verified.</h1>

<p align="center">
  <strong>B106 Edition · Rust + Wasmtime v37 + WASI P1 · Ed25519 · MIT Open Core</strong><br/>
  Verifiable Compute Framework for Secure Multi-Tenant Execution
</p>

<!-- Night Core v38 Verified Summary -->
### 🧩 Night Core™ v38 — Verified Build Summary

| Field | Value |
|-------|-------|
| **Commit ID** | 26c43b3 |
| **Timestamp** | 2025-10-31 21 |
| **Audit Hash** | 6dfaebee909b96f077e0d668b5c401d68ee44bbe3937e647f8aafe12dbf06cb5 |
| **Maintainers** | core-ops • system-check |
| **Status** | ✅ Verified |

Night Core™ — Secure • Autonomous • Verified Build Summary |

Night Core™ — Secure • Autonomous • Verified

### 🧩 Night Core™ v38 — Verified Build Summary

| Field | Value |
|-------|-------|
| **Commit ID** | 26c43b3 |
| **Timestamp** | 2025-10-31 21 |
| **Audit Hash** | 6dfaebee909b96f077e0d668b5c401d68ee44bbe3937e647f8aafe12dbf06cb5 |
| **Maintainers** | core-ops • system-check |
| **Status** | ✅ Verified |

Night Core™ — Secure • Autonomous • Verified Build Summary |

Night Core™ — Secure • Autonomous • Verified

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
- 🪶 **MIT Open Core** — “B106 Edition” branding & dashboard visuals reserved  

---

## 🚀 Quick Start
```bash
git clone https://github.com/xnfinite/nightcore-worker.git
cd nightcore-worker
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
- [**ROADMAP.md**](https://github.com/xnfinite/nightcore-worker/blob/main/ROADMAP.md) — Long-term development path  
- [**AUFS Overview**](https://github.com/xnfinite/nightcore-worker/blob/main/docs/aufs-overview.md) — Upgrade & proof system  
- [**Threat Model**](https://github.com/xnfinite/nightcore-worker/blob/main/docs/security/THREAT_MODEL.md) — Security design  
- [**Governance**](https://github.com/xnfinite/nightcore-worker/blob/main/docs/security/GOVERNANCE.md) — Contributor & maintainer policy  
- [**Cold Storage**](https://github.com/xnfinite/nightcore-worker/blob/main/ops/cold-storage.md) — Secure key and manifest handling  

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

## ☁️ AWS Integration Strategy (SOON)
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

## 🤝 Contributing
Contributions are welcome!   
See [**Governance**](https://github.com/xnfinite/nightcore-worker/blob/main/docs/security/GOVERNANCE.md) and [**AUFS Overview**](https://github.com/xnfinite/nightcore-worker/blob/main/docs/aufs-overview.md) for workflow and signing policies.

---

## 📜 License
**Night Core Open-Core Edition** is licensed under the **MIT License**.  
The **Night Core™**, **B106 Edition**, and **Guardian** names, logos, and dashboard visuals are proprietary trademarks of **xnfinite / B106 Labs**.

---

## 🌟 Vision
> *“Night Core becomes a self-healing, provable, autonomous compute standard —  
> secure enough for enterprises, open enough for everyone.”*

---
---

## 🛡️ Proof of Authorship & Legal Notice
>Copyright (c) 2025 xnfinite  
All Rights Reserved.

Night Core™ is an open-source project authored and maintained by the xnfinite organization.  
Original source code was first published on GitHub under the account “xnfinite” in 2025.  
All commits, signatures, and hashes in this repository serve as cryptographic proof of authorship and date of creation.

This project is released under the MIT License.  
Any use, modification, or redistribution of the source code must retain the above copyright notice  
and include a copy of the MIT License. Removal of author attribution is a violation of this license.

The names “Night Core™”, “Night Mesh™”, and “B106 Edition™”, along with associated branding,  
logos, and dashboard visuals, are trademarks of the xnfinite organization and may not be used  
in derivative works or commercial products without written permission.

Unauthorized rebranding, impersonation, or misrepresentation of authorship  
constitutes a violation of international copyright and trademark law.  
All derivative works must clearly disclose that they are forks of the original  
Night Core framework and may not claim official affiliation.

<p align="center"><i>© 2025 xnfinite — Building the future of verifiable autonomous compute.</i></p>

<!-- Night Core v38 Verified Summary -->
### 🧩 Night Core™ v38 — Verified Build Summary

| Field | Value |
|-------|-------|
| **Commit ID** | 26c43b3 |
| **Timestamp** | 2025-10-31 21 |
| **Audit Hash** | 6dfaebee909b96f077e0d668b5c401d68ee44bbe3937e647f8aafe12dbf06cb5 |
| **Maintainers** | core-ops • system-check |
| **Status** | ✅ Verified |

Night Core™ — Secure • Autonomous • Verified Build Summary |

Night Core™ — Secure • Autonomous • Verified

### 🧩 Night Core™ v38 — Verified Build Summary

| Field | Value |
|-------|-------|
| **Commit ID** | 26c43b3 |
| **Timestamp** | 2025-10-31 21 |
| **Audit Hash** | 6dfaebee909b96f077e0d668b5c401d68ee44bbe3937e647f8aafe12dbf06cb5 |
| **Maintainers** | core-ops • system-check |
| **Status** | ✅ Verified |

Night Core™ — Secure • Autonomous • Verified Build Summary |

Night Core™ — Secure • Autonomous • Verified








