Night Core™ v38 — README (Verified)
---------------------------------------------------------------

Night Core v38 Verified Badge
---------------------------------------------------------------
docs/assets/nightcore_logo_tm.png  (logo file)
https://github.com/xnfinite/nightcore-worker/actions
Badge: AUFS Verified v38-success

Night Core™ — Secure • Autonomous • Verified
---------------------------------------------------------------

Night Core™ v38 — Verified Build Summary
------------------------------------------------
Commit ID: 26c43b3
Timestamp: 2025-10-31 21:00 UTC
Audit Hash: 6dfaebee909b96f077e0d668b5c401d68ee44bbe3937e647f8aafe12dbf06cb5
Maintainers: core-ops • system-check
Status: ✅ Verified

---------------------------------------------------------------

Overview
------------------------------------------------
Night Core™ is an open-source, enterprise-grade WebAssembly orchestration framework written in Rust.
It verifies, isolates, and executes signed .wasm modules in secure sandboxes — supporting multi-tenant workloads, self-healing upgrades, and cryptographic proof of integrity.

---------------------------------------------------------------

Core Features
------------------------------------------------
✅ Wasmtime 37 + WASI P1 sandbox runtime
🔐 Ed25519 + SHA-256 signature & integrity verification
🧩 Multi-Tenant Orchestration (--all) with per-tenant policies
📊 HTML Proof Dashboard + JSONL audit logs
🔁 AUFS — Autonomous Upgrade & Fork System (threshold-signed, self-healing)
🪶 MIT Open Core — “B106 Edition” visuals reserved

---------------------------------------------------------------

Quick Start (Verified CLI)
------------------------------------------------
1️⃣ Clone & Build
git clone https://github.com/xnfinite/nightcore-worker.git
cd nightcore-worker
cargo +nightly build

2️⃣ Verify Environment
cargo +nightly run -- verify-env

3️⃣ Generate Keys
cargo +nightly run -- generate-keys --out-dir keys/

4️⃣ Sign Tenant Module
cargo +nightly run -- sign --dir modules/tenantA-hello --key keys/maintainers/admin1.key

5️⃣ Run All Tenants (Verified & Sandboxed)
cargo +nightly run -- run --all

6️⃣ Export Public Key Hashes (for AUFS / Audit)
cargo +nightly run -- export-pubkey-hashes

7️⃣ Verify & Apply Upgrade Manifest (Threshold-Signed)
cargo +nightly run -- upgrade --manifest upgrades/manifests/upgrade_manifest.json

8️⃣ Optional — Sign Upgrade Manifest as Maintainer
cargo +nightly run -- sign-upgrade --manifest upgrades/manifests/upgrade_manifest.json

💡 Show Command Help
cargo run -- --help

---------------------------------------------------------------

Documentation
------------------------------------------------
docs/aufs-overview.md — AUFS overview & upgrade system
docs/security/THREAT_MODEL.md — Security design and risk model
docs/security/GOVERNANCE.md — Maintainer & contributor governance
docs/internal/RELEASE_POLICY.md — Foundation release policy
docs/release_notes_v38.md — Verified build notes for v38
docs/guardian.md — Guardian AI Containment Kernel
docs/vesper.md — Vesper Dev Agent Design
docs/nightmesh.md — Night Mesh distributed proof sync plan
docs/legacy/Night_Core_Chronicle.md — Founder’s Chronicle (Proof Log)
docs/provenance/nightcore_v38_provenance.html — Cryptographic provenance record
docs/releases/v38.html — Release Dashboard
docs/assets/nightcore_logo_tm.png — Verified Logo Asset

---------------------------------------------------------------

Security Baseline
------------------------------------------------
- Fuel / timeout / memory caps per tenant
- FS / NET sandbox policy per manifest
- Threshold-signed upgrades (2-of-3)
- Hash-chained audit logs & reproducible builds
- Optional FIPS mode (--crypto-mode fips)

---------------------------------------------------------------

AWS Integration Strategy (Soon)
------------------------------------------------
Night Core is designed for native deployment on AWS Cloud infrastructure:
- Nitro Enclaves / Fargate for tenant isolation
- KMS / CloudHSM for key management
- S3 / DynamoDB for manifest & proof storage
- Lambda / EventBridge for AUFS automation
- CloudWatch / Security Hub for compliance visibility

---------------------------------------------------------------

Development Roadmap
------------------------------------------------
Phase 1️⃣ Night Core Baseline — Wasmtime 37 + WASI P1 + Ed25519 verified execution → ✅ Complete
Phase 2️⃣ AUFS — Autonomous Upgrade & Fork System → 🚧 In Progress
Phase 3️⃣ Guardian — AI Containment Kernel (fuel + rollback + drift) → 🧠 Pioneer Layer
Phase 4️⃣ Night Mesh — Distributed proof sync → 🔄 Planned
Phase 5️⃣ Vesper — Self-Documenting AI Agent → 💬 Final Layer

---------------------------------------------------------------

Contributing
------------------------------------------------
Contributions welcome!
See docs/security/GOVERNANCE.md and docs/aufs-overview.md for policies & workflow.

---------------------------------------------------------------

License
------------------------------------------------
Night Core Open-Core Edition is licensed under the MIT License.
The Night Core™, B106 Edition™, and Guardian™ names, logos, and dashboard visuals are proprietary trademarks of xnfinite / B106 Labs.

---------------------------------------------------------------

Vision
------------------------------------------------
“Night Core becomes a self-healing, provable, autonomous compute standard —
secure enough for enterprises, open enough for everyone.”

---------------------------------------------------------------
© 2025 xnfinite — Building the future of verifiable autonomous compute.
---------------------------------------------------------------
