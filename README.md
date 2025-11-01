Night Core™ v38 — README (Clean Final Version)
---------------------------------------------

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

-------------------------------------------------------

🧩 Night Core™ v38 — Verified Build Summary
-------------------------------------------
Commit ID: 26c43b3  
Timestamp: 2025-10-31 21:00 UTC  
Audit Hash: 6dfaebee909b96f077e0d668b5c401d68ee44bbe3937e647f8aafe12dbf06cb5  
Maintainers: core-ops • system-check  
Status: ✅ Verified

-------------------------------------------------------

Night Core™ — Secure. Autonomous. Verified.
-------------------------------------------
B106 Edition · Rust + Wasmtime v37 + WASI P1 · Ed25519 · MIT Open Core  
Verifiable Compute Framework for Secure Multi-Tenant Execution

-------------------------------------------------------

🌑 Overview
-----------
Night Core™ is an open-source, enterprise-grade WebAssembly orchestration framework written in Rust.  
It verifies, isolates, and executes signed `.wasm` modules in secure sandboxes — supporting multi-tenant workloads, self-healing upgrades, and cryptographic proof of integrity.

-------------------------------------------------------

🧱 Core Features
----------------
- ✅ Wasmtime 37 + WASI P1 sandbox runtime
- 🔐 Ed25519 + SHA-256 signature & integrity verification
- 🧩 Multi-Tenant Orchestration (--all) with per-tenant policies
- 📊 HTML Dashboard with JSONL logs & audit hash chain
- 🔁 AUFS — Autonomous Upgrade & Fork System (threshold-signed, self-healing)
- 🪶 MIT Open Core — “B106 Edition” branding & dashboard visuals reserved

-------------------------------------------------------

🚀 Quick Start
---------------
git clone https://github.com/xnfinite/nightcore-worker.git
cd nightcore-worker
cargo +nightly build

# Generate signing keys
cargo +nightly run -- generate-keys --out-dir keys/

# Verify a tenant module
cargo +nightly run -- verify --tenant tenantA-hello --explain

# Run all tenants in parallel
cargo +nightly run -- run --all --parallel 2

# Launch dashboard
cargo +nightly run -- dashboard --open

-------------------------------------------------------

📚 Documentation
----------------
- docs/operations/RUN_AND_PUSH_GUIDE.md — Build, proof, and audit workflow  
- docs/ROADMAP.md — Long-term development path  
- docs/aufs-overview.md — Upgrade & proof system  
- docs/security/THREAT_MODEL.md — Security design  
- docs/security/GOVERNANCE.md — Contributor & maintainer policy  
- ops/cold-storage.md — Secure key and manifest handling

-------------------------------------------------------

🛡️ Security Baseline
---------------------
- Fuel / timeout / memory limits per tenant  
- FS / NET sandbox policy per manifest  
- Threshold-signed upgrades (2-of-3)  
- Hash-chained audit logs & reproducible builds  
- Optional FIPS mode (--crypto-mode fips)

-------------------------------------------------------

☁️ AWS Integration Strategy (SOON)
----------------------------------
Night Core is designed for native deployment on AWS Cloud infrastructure:
- Nitro Enclaves / Fargate for tenant isolation  
- KMS / CloudHSM for key management  
- S3 / DynamoDB for manifest and proof storage  
- Lambda / EventBridge for AUFS automation  
- CloudWatch / Security Hub for compliance visibility

-------------------------------------------------------

🧭 Development Roadmap
-----------------------
Phase 1️⃣ Night Core Baseline — Wasmtime 37 + WASI P1 + Ed25519 verified execution → ✅ Complete  
Phase 2️⃣ AUFS — Autonomous Upgrade & Fork System → 🚧 In Progress  
Phase 3️⃣ Guardian — AI Containment Kernel (fuel + byte drift + rollback) → 🧠 Pioneer Layer  
Phase 4️⃣ Night Mesh — Distributed proof sync and audit sharing → 🔄 Planned  
Phase 5️⃣ Vesper — Self-documenting AI assistant → 💬 Final Layer

-------------------------------------------------------

🤝 Contributing
----------------
Contributions are welcome!  
See Governance and AUFS Overview for workflow and signing policies.

-------------------------------------------------------

📜 License
-----------
Night Core Open-Core Edition is licensed under the MIT License.  
The Night Core™, B106 Edition™, and Guardian™ names, logos, and dashboard visuals are proprietary trademarks of xnfinite / B106 Labs.

-------------------------------------------------------

🌟 Vision
----------
“Night Core becomes a self-healing, provable, autonomous compute standard —  
secure enough for enterprises, open enough for everyone.”

-------------------------------------------------------

© 2025 xnfinite — Building the future of verifiable autonomous compute.
-------------------------------------------------------



