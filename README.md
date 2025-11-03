<!-- Night Core™ Worker — Verified Open-Core Foundation -->
<p align="center">
  <img src="assets/nightcore_logo_tm.png" width="220" alt="Night Core™ Logo"/>
  <br/>
  <a href="https://github.com/xnfinite/night-core-worker/actions">
    <img src="https://img.shields.io/badge/AUFS%20Verified-v39.2-success?style=for-the-badge&color=0B3D91" alt="AUFS Verified"/>
  </a>
  <br/>
  <sub>Night Core™ Worker — Secure • Autonomous • Verified</sub>
</p>

---

## 🧩 Night Core™ Worker — Verified Runtime for Trusted Compute

**Night Core Worker** is an open-source, Rust-based runtime that delivers trusted compute for WebAssembly (WASM).  
It verifies, isolates, and executes signed modules with cryptographic assurance, giving developers a secure foundation for distributed and autonomous workloads.

### 🔐 Key Features
- **Trusted Execution:** Runs WebAssembly modules inside Wasmtime 37 + WASI P1 sandboxes.  
- **Digital Signature Verification:** Uses Ed25519 to confirm every module’s authenticity.  
- **Integrity Protection:** SHA-256 hashing detects tampering or modification.  
- **Multi-Tenant Orchestration:** Safely runs multiple verified workloads in isolation.  
- **Audit & Proof Logging:** The Chronicle proof ledger records every verification for full transparency.

---

### 🧠 Why It Matters
Night Core Worker enables verifiable, secure compute at the edge or in the cloud—ensuring that only trusted, signed code can run.  
It’s the foundation of the Night Core Framework, powering advanced layers such as **AUFS (Autonomous Upgrade & Fork System)**, **Guardian**, and **Vesper**, the upcoming AI documentation agent.

---

## 📜 Proof & Verification
See [`docs/proof/README.md`](docs/proof/README.md) for detailed cryptographic evidence, including audit logs, Chronicle signatures, and visual proof reports.

Example verification snippet:

<p align="center">
  <img src="../../assets/nightcore_v38_proof_report.png" width="720" alt="Night Core™ Verified Proof Report"/>
  <br/>
  <sub>Night Core™ v38 → v39.2 Proof Lineage — Ed25519 + SHA-256 Verified</sub>
</p>

---

## ⚙️ Quick Start

```bash
git clone https://github.com/xnfinite/night-core-worker.git
cd night-core-worker
cargo +nightly build
cargo +nightly run -- run --all
🚀 Night Core™ Pro — Advanced Edition
Night Core Pro extends this open-core framework with:

🧩 AUFS (Autonomous Upgrade & Fork System) — self-healing verified builds

🪶 Chronicle Sync — auto-signed audit ledger

🔒 Guardian — security enforcement and FIPS isolation

☁️ AWS/Nitro Integration — enterprise deployment

🧠 Vesper AI Agent — self-documenting orchestration

<p align="center"> <a href="https://github.com/xnfinite/nightcore-pro"> <img src="https://img.shields.io/badge/Night%20Core™%20Pro-v39.2%20AUFS-blue?style=for-the-badge&color=0B3D91" alt="Night Core™ Pro"/> </a> </p>
Access: Night Core Pro is currently in a closed release.
Verified maintainers and enterprise partners can request access to the private repository.

🧩 Roadmap
Phase	Goal	Status
v39.x	AUFS + Chronicle Automation Chain	✅ Completed
v40.x	Guardian Security Layer	🔄 In Design
v41.x	Night Mesh Distributed Proof Network	🧩 Planned
v42.x	Vesper AI Agent Integration	🧠 Planned

🪪 License & Maintainer
License: MIT (Open Core) • B106 Edition branding reserved
Maintainer: xnfinite — Night Core Maintainer
Repository: https://github.com/xnfinite/night-core-worker