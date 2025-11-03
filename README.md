<!-- Night Core™ Worker — Verified Open-Core Foundation -->
<p align="center">
  <img src="assets/nightcore_logo_tm.png" width="220" alt="Night Core™ Logo"/>
  <br/>
  <a href="https://github.com/xnfinite/nightcore-pro"><b>Night Core™ Pro</b></a>
  <br/>
  <a href="https://github.com/xnfinite/nightcore-worker/actions">
    <img src="https://img.shields.io/badge/AUFS%20Verified-v39.2-success?style=for-the-badge&color=0B3D91" alt="AUFS Verified"/>
  </a>
  <br/>
  <sub>Night Core™ Worker — Secure • Autonomous • Verified</sub>
</p>

---

## 🧩 Overview
**Night Core™ Worker** is the verified open-core layer of the Night Core framework.  
It provides a secure Rust runtime for WebAssembly (WASM) modules using:

- **Wasmtime 37 + WASI P1**
- **Ed25519 Signature Verification**
- **SHA-256 Integrity Checking**
- **Multi-Tenant Execution**
- **Chronicle Proof Ledger**

All higher-tier **Night Core™ Pro** features (AUFS, Guardian, Vesper, AWS Integration) build upon this foundation.

---

## ⚙️ Quick Start

```bash
git clone https://github.com/xnfinite/nightcore-worker.git
cd nightcore-worker
cargo +nightly build
cargo +nightly run -- run --all
🔏 Verified Proof Lineage
Night Core Worker maintains a public ledger of proofs under
docs/legacy/Night_Core_Chronicle.md
recording every verified release and audit entry.

Example verification snippet:

mathematica
Copy code
PGP Signature Verified ✔  
Ed25519 Signatures Valid ✔  
SHA-256 Integrity Passed ✔
📜 Full Proof & Verification Page:
See docs/proof/README.md for detailed cryptographic evidence,
including audit logs, Chronicle signatures, and visual proof reports.

🧱 Architecture Summary
Layer	Purpose
Worker Core	Executes verified WASM modules inside isolated tenants
Verifier	Performs Ed25519 + SHA-256 checks
Chronicle	Writes immutable proof records
AUFS Interface	Automates upgrade and fork integrity
Dashboard (HTML)	Displays orchestration status and proof summaries

🧭 Roadmap
Phase	Goal
v39.x	AUFS + Chronicle Automation Chain
v40.x	Guardian Security Layer
v41.x	Night Mesh Distributed Proof Network
v42.x	Vesper AI Agent Integration

🪪 License & Maintainer
License: MIT (Open Core) • B106 Edition branding reserved
Maintainer: xnfinite — Night Core Maintainer
Repository: https://github.com/xnfinite/nightcore-worker


