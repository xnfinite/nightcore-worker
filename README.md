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

🧩 Night Core™ Worker — Verified Runtime for Trusted Compute

Night Core Worker is an open-source, Rust-based runtime that delivers trusted compute for WebAssembly (WASM).
It verifies, isolates, and executes signed modules with cryptographic assurance, giving developers a secure foundation for distributed and autonomous workloads.

🔐 Key Features

Trusted Execution: Runs WebAssembly modules inside Wasmtime 37 + WASI P1 sandboxes.

Digital Signature Verification: Uses Ed25519 to confirm every module’s authenticity.

Integrity Protection: SHA-256 hashing detects tampering or modification.

Multi-Tenant Orchestration: Safely runs multiple verified workloads in isolation.

Audit & Proof Logging: The Chronicle proof ledger records every verification for full transparency.

🧠 Why It Matters

Night Core Worker enables verifiable, secure compute at the edge or in the cloud, ensuring that only trusted, signed code can run.
It’s the foundation of the Night Core Framework, powering future layers like AUFS (Autonomous Upgrade & Fork System) and Vesper—the upcoming AI documentation agent.
---

📜 **Full Proof & Verification Page:**  
See [`docs/proof/README.md`](docs/proof/README.md) for detailed cryptographic evidence,
including audit logs, Chronicle signatures, and visual proof reports.

## ⚙️ Quick Start

```bash
git clone https://github.com/xnfinite/night-core-worker.git
cd night-core-worker
cargo +nightly build
cargo +nightly run -- run --all
🔏 Verified Proof Lineage
Night Core Worker maintains a public ledger of proofs under docs/legacy/Night_Core_Chronicle.md which records every verified release and audit entry.

Example verification snippet:

mathematica
Copy code
PGP Signature Verified ✔  
Ed25519 Signatures Valid ✔  
SHA-256 Integrity Passed ✔
🧩 Roadmap
Phase	Goal
v39.x	AUFS + Chronicle Automation Chain
v40.x	Guardian Security Layer
v41.x	Night Mesh Distributed Proof Network
v42.x	Vesper AI Agent Integration

🪪 License & Maintainer
License: MIT (Open Core) • B106 Edition branding reserved
Maintainer: xnfinite — Night Core Maintainer
Repository: https://github.com/xnfinite/night-core-worker