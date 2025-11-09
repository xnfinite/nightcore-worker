🔍 What Is Night Core Worker?
---------------------------------------------

<!-- === Proof Badge Row (GitHub-safe) === -->
<p align="center">
  <a href="logs/nightcore_proof.html">
    <img src="https://img.shields.io/badge/Firecracker%20Verified-v38-success?style=for-the-badge&color=0B3D91" alt="Firecracker Verified"/>
  </a>
  <a href="https://github.com/xnfinite/nightcore-worker/actions">
    <img src="https://img.shields.io/badge/AUFS%20Verified-v38-success?style=for-the-badge&color=2E8B57" alt="AUFS Verified"/>
  </a>
  <a href="docs/legacy/Night_Core_Chronicle.md">
    <img src="https://img.shields.io/badge/Chronicle-Proof%20Synced-blue?style=for-the-badge&color=4682B4" alt="Chronicle Synced"/>
  </a>
</p>

Night Core™ Worker is an open-core Rust framework for securely running WebAssembly (WASM) modules in isolated sandboxes.  
It automatically discovers, verifies, and executes all trusted tenant modules under /modules, ensuring every execution is cryptographically proven.

🖥️ Proof & Dashboard System (v39)
<p align="center"> <img src="assets/nc-proof.png" alt="Night Core Proof Dashboard" width="800"/><br/> <sub><b>Night Core™ v39 — Proof Dashboard:</b> Live verification view showing Ed25519 + SHA-256 validated tenants.</sub> </p> <p align="center"> <img src="assets/nc-proof1.png" alt="Night Core Orchestration Dashboard" width="800"/><br/> <sub><b>Night Core™ v39 — Orchestration Summary:</b> Parallel tenant execution metrics and integrity status.</sub> </p> <p align="center"> <img src="assets/nc-hproof.png" alt="Night Core Historical Proof Dashboard" width="800"/><br/> <sub><b>Night Core™ v39 — Historical Proof Ledger:</b> Aggregated multi-tenant state history from <code>export-dashboard --diff</code>.</sub> </p>

Night Core Worker uses:  
 • 🦀 Rust for reliability & performance  
 • 🔒 Ed25519 digital signatures for authenticity  
 • 🧱 SHA-256 integrity hashes for tamper detection  
 • 🧩 Wasmtime 37 + WASI Preview 1 for secure sandboxing  
 • 📄 HTML + JSONL audit logs for transparency  

⸻

⚙️ Quick Start

1️⃣ Clone & Build  
```bash
git clone https://github.com/xnfinite/nightcore-worker.git
cd nightcore-worker
cargo +nightly build
```

2️⃣ Generate Keys  
```bash
cargo +nightly run -- generate-keys
```
Creates Ed25519 public/private key pairs under /keys/maintainers/.

3️⃣ Sign Your Modules  
```bash
cargo +nightly run -- sign --dir modules/tenantA-hello --key keys/maintainers/admin1.key
cargo +nightly run -- sign --dir modules/tenantB-math  --key keys/maintainers/admin1.key
```
Generates:  
- module.sig — Ed25519 signature  
- pubkey.b64 — public key in base64  
- module.sha256 — integrity hash  

4️⃣ Run All Tenants (Wasmtime Default)  
```bash
cargo +nightly run -- run --all
```

💡 To run using **Firecracker microVM backend**, use:  
```bash
cargo +nightly run -- run --all --backend firecracker --vm-timeout 10
```
Automatically starts a verified Firecracker VM, executes Tenant A (hello) and Tenant B (math) securely, then shuts down cleanly after timeout.  
Proof output:  
- logs/firecracker_boot.log  
- logs/nightcore_proof.html 
---

🧾 Security Model
---------------------------------------------

Night Core™ Worker enforces a trustless execution model, ensuring every tenant module runs inside a verifiable, cryptographically protected sandbox.

🔐 Core Guarantees

| Layer | Mechanism | Purpose |
|-------|------------|----------|
| Authenticity | Ed25519 digital signatures | Confirms each .wasm module originates from a trusted maintainer. |
| Integrity | SHA-256 hash verification | Detects any unauthorized modification before execution. |
| Isolation | Wasmtime 37 + WASI P1 | Provides memory, syscalls, and execution limits for each tenant. |
| Accountability | JSONL + HTML audit logs | Every event, signature, and result is recorded in tamper-evident format. |
| Resilience | Multi-tenant orchestration | Faults in one tenant do not affect others. |

⚙️ Execution Flow
1. Discovery → All tenants under /modules are enumerated.
2. Verification → Each module’s .sig and .sha256 are checked using Ed25519.
3. Isolation → The module runs inside a secure Wasmtime/WASI sandbox.
4. Proof Logging → Results are written to logs/ for full reproducibility.

---

📦 Project Structure
---------------------------------------------

nightcore-worker/
│
├── Cargo.toml
│
├── src/
│   ├── main.rs
│   ├── generate_keys.rs
│   ├── sign_tenant.rs
│   ├── verify.rs
│   └── run.rs
│
├── modules/
│   ├── tenantA-hello/
│   │   ├── module.wasm
│   │   ├── module.sig
│   │   ├── module.sha256
│   │   ├── pubkey.b64
│   │   └── manifest.json
│   └── tenantB-math/
│       └── ...
│
├── logs/
│   ├── nightcore_dashboard.html
│   └── orchestration_report.json
│
└── keys/
    └── maintainers/
        ├── admin1.key
        └── admin1.pub

---

🧠 Architecture Overview
---------------------------------------------

Night Core™ Worker is designed as a modular, auditable orchestration engine built around three tightly integrated layers:

1️⃣ Verification Layer
- Handles trust and proof before execution.
- Validates Ed25519 signatures and SHA-256 hashes.
- Rejects any module that fails verification with full audit context.

2️⃣ Execution Layer
- Provides secure sandboxed execution using Wasmtime 37 + WASI P1.
- Enforces per-tenant fuel, memory, and time limits.
- Supports sequential and parallel execution modes.

3️⃣ Audit Layer
- Writes HTML and JSONL logs for transparency.
- Includes timestamps, SHA-256 digests, and status codes.
- Supports proof-only verification mode (--proof).

🏗️ Layer Interaction
Verification → Execution → Audit (Proof Chain)

---

💡 Extending Night Core
---------------------------------------------

Night Core™ Worker is built to scale — adding new tenants or workloads is fast, secure, and verifiable.

1️⃣ Create a New Tenant Directory
mkdir modules/tenantC-ai

2️⃣ Add Your WebAssembly Module
modules/tenantC-ai/module.wasm

3️⃣ Sign the Module
cargo +nightly run -- sign --dir modules/tenantC-ai --key keys/maintainers/admin1.key

4️⃣ Define Tenant Metadata
manifest.json:
{
  "tenant": "tenantC-ai",
  "description": "AI inference module running under WASI sandbox",
  "version": "1.0.0",
  "maintainer": "core-ops",
  "permissions": { "network": false, "filesystem": false }
}

5️⃣ Verify & Run
cargo +nightly run -- run --all

✅ Discover → Verify → Execute → Log

---

🧱 Technology Stack
---------------------------------------------

| Layer | Technology | Purpose |
|--------|-------------|----------|
| Runtime | Rust + Cargo (nightly) | Safety, concurrency, performance |
| Sandbox | Wasmtime 37 + WASI P1 | Deterministic, secure WASM runtime |
| Crypto | Ed25519 (ed25519-dalek) | Authenticity verification |
| Integrity | SHA-256 (sha2 crate) | Tamper detection |
| Serialization | Serde + JSONL | Audit transparency |
| Logging | HTML + JSON reports | Readable dashboards and proofs |

---
---

🧩 Night Core™ Worker v39 — Update Summary
---------------------------------------------

Night Core™ Worker v39 introduces persistent proof tracking, full multi-tenant dashboards, and modular backend architecture under `/crates`.

This marks a major evolution of the open-core Worker into a **stateful, verifiable orchestration engine**, capable of securely recording, inspecting, and exporting long-term proof histories for each tenant.

---

## 🚀 New in v39

### 1️⃣ Persistent Proof State (`nc_state`)
Each tenant now maintains its own lightweight database powered by **sled**, stored under:

```
/state/<tenant>/sled
```

Automatically records:
- `last_proof` → most recent verification metadata  
- `proof_history` → append-only list of past verifications  

New commands:
```bash
nightcore inspect-state --tenant tenantA-hello
nightcore inspect-state --tenant tenantA-hello --summary
```

Outputs:
- `logs/<tenant>_proof_history.json`
- Aggregated proof statistics and verification percentage.

---

### 2️⃣ Historical Proof Dashboard (`export-dashboard`)
All tenant histories are now combined into a **global HTML ledger** with visual statistics:

```bash
nightcore export-dashboard
nightcore export-dashboard --diff
```

Creates:
- `logs/nightcore_history_dashboard.html`  
- Auto-opens in browser  
- Dark theme, compact layout, and diff mode for visual change tracking.

---

### 3️⃣ Proof-Oriented Orchestration (`--proof`)
The run engine now supports deterministic proof-only mode with capped parallelism for verifiability:

```bash
nightcore run --all --proof
```

Writes:
- `logs/nightcore_dashboard.html` — visual proof dashboard  
- `logs/orchestration_report.json` — detailed timing & integrity data  

---

### 4️⃣ Expanded CLI
New commands extend Night Core Worker’s operational scope:

| Command | Description |
|----------|--------------|
| `inspect-state` | View or summarize per-tenant proof history |
| `export-dashboard` | Build a unified global proof ledger |
| `unlock` | Verify Pro license (AUFS / proof extensions) |
| `sign-upgrade` | Sign AUFS manifests (for Night Core Pro) |

---

## 🧱 Backend Architecture (v39 Modular Crates)

Night Core Worker is now composed of modular backend crates designed for isolation, performance, and future backend expansion.

```
crates/
│
├── nc-exec/
│   ├── Cargo.toml
│   └── src/lib.rs
│   → Core execution interface for verified WASM modules
│
├── nc-exec-cli/
│   ├── Cargo.toml
│   └── src/main.rs
│   → CLI frontend for direct module execution
│
├── nc-exec-firecracker/
│   ├── Cargo.toml
│   └── src/lib.rs
│   → (Experimental) Firecracker microVM backend for high-security sandboxing
│
├── nc-exec-wasmtime/
│   ├── Cargo.toml
│   └── src/lib.rs
│   → Primary runtime backend — Wasmtime 37 + WASI Preview 1
│
└── nc_state/
    ├── Cargo.toml
    └── src/lib.rs
    → Persistent proof state and audit tracking (sled engine)
```

---

## 🔐 Security Reinforcement

| Layer | Mechanism | Purpose |
|--------|------------|----------|
| Authenticity | Ed25519 digital signatures | Validates module authorship |
| Integrity | SHA-256 hashing | Detects tampering before execution |
| Persistence | sled key-value store | Maintains verifiable proof history |
| Transparency | HTML + JSONL dashboards | Exportable, immutable audit trails |

---

## 📊 Logs & Reports Overview

| File | Description |
|-------|--------------|
| `logs/nightcore_dashboard.html` | Live per-run proof dashboard |
| `logs/nightcore_history_dashboard.html` | Global proof history ledger |
| `logs/orchestration_report.json` | Structured performance summary |
| `logs/<tenant>_proof_history.json` | Persistent per-tenant proof record |

---

## 🧠 Version Metadata

| Property | Value |
|-----------|--------|
| **Version** | v39 Stable |
| **Runtime** | Wasmtime 37 + WASI P1 |
| **Crypto** | Ed25519 (ed25519-dalek) + SHA-256 |
| **Persistence** | sled embedded KV |
| **License** | MIT (open-core) |
| **Trademark** | “Night Core™” and “B106 Edition” — proprietary marks of B106 Labs |
| **Repository** | [github.com/xnfinite/nightcore-worker](https://github.com/xnfinite/nightcore-worker) |

---

✨ **Night Core™ Worker — Secure • Autonomous • Verified**

⚡ Night Core™ Pro (Coming Soon)
---------------------------------------------

Night Core™ Pro extends the open-core Worker framework into a fully autonomous orchestration platform, integrating advanced systems for verified upgrades and enterprise governance.

🧩 Included & Verified
- ✅ Full AUFS (Autonomous Upgrade & Fork System)

🚧 In Progress
- 🛡️ Guardian Layer — advanced tenant containment & policy control
- ☁️ AWS Integration — Nitro Enclaves, KMS, Fargate support

🪪 Availability
Night Core™ Pro will be available soon as a verified binary distribution for professional and enterprise environments.
Follow updates in docs/legacy/Night_Core_Chronicle.md or the official GitHub repository.

---

📜 License & Credits
---------------------------------------------

- License: MIT (open-core)
- Trademark: “Night Core™” and “B106 Edition” are proprietary marks of B106 Labs
- Core Repository: https://github.com/xnfinite/nightcore-worker

---

🏛️ Open-Core Policy
---------------------------------------------

Night Core™ Worker is released under the **MIT License** as a fully open-core framework.  
It is intended for developers, researchers, and security engineers who wish to explore or build on verified WebAssembly orchestration.  
All advanced systems — including **AUFS (Autonomous Upgrade & Fork System)**, **Guardian Layer**, **Vesper AI**, and **AWS Integration** — are part of **Night Core™ Pro**, a closed-source commercial edition.  

The open-core edition will remain free and maintained for transparency, education, and collaboration, while Night Core™ Pro continues advancing the enterprise feature set.

---
