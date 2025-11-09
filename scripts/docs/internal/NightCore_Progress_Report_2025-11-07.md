# 🧭 Night Core™ Worker — Progress Report (v39 Verified Open Core)
_Stable Development Branch: v39 Worker Edition (B106 Foundation)_

---

## ⚙️ Core Engine
- ✅ Verified compute foundation: **Wasmtime 37 + WASI P1**
- ✅ Ed25519 + SHA-256 signature and integrity verification
- ✅ Multi-tenant orchestration with parallel execution
- ✅ Proof Mode with performance metrics and audit summaries
- ✅ HTML + JSON proof logs generated automatically

---

## 🧮 Persistent State Layer — `nc_state`
- ✅ Embedded sled database per tenant (`/state/<tenant>/sled`)
- ✅ API methods: `put_json`, `get_json`, `append_json`
- ✅ Persistent proof records: `last_proof` + `proof_history`
- ✅ CLI commands:
  cargo +nightly run -- inspect-state --tenant tenantA-hello --summary
  cargo +nightly run -- inspect-state --all-tenants
This enables verifiable continuity between module executions — every run appends a proof record with timestamp, SHA-256, and verification result.

🧱 Execution Backends
Located under /crates:



crates/
├── nc-exec              # Core runtime API
├── nc-exec-cli          # CLI adapter for standalone runs
├── nc-exec-firecracker  # MicroVM backend (isolation prototype)
├── nc-exec-wasmtime     # Default Wasmtime runtime
└── nc_state             # Persistent KV store (sled)
✅ Active Backend
Wasmtime (nc-exec-wasmtime)

Default execution environment using WASI P1.

Used in all verified tenant module runs.

🧱 In-Progress Backend
Firecracker (nc-exec-firecracker)

Lightweight microVM sandbox for isolated workloads.

Planned integration with AUFS governance for attested runs.

📊 Dashboard & Audit Layer
✅ Live dashboard → logs/nightcore_dashboard.html

✅ Historical dark ledger →

cargo +nightly run -- export-dashboard --diff
Tracks per-tenant delta in proof history (SHA, time, size).

✅ JSON report: logs/orchestration_report.json

✅ Proof persistence powered by nc_state

🪪 Governance & Security
✅ AUFS (Autonomous Upgrade & Fork System) integrated

✅ nightcore unlock flag enables AUFS/Proof extensions

✅ Signed upgrade manifests and chain verification

✅ MIT Open Core + B106™ brand layer reserved

✅ Audit log SHA-256 chain verified and consistent

🧠 Intelligence & Automation Roadmap
🟩 Next: Vesper AI Assistant (local LLM + TenantState queries)

🟩 Next: Guardian containment sandbox

🟩 Planned: Night Mesh — distributed proof sync over LAN

🟩 Planned: AWS Integration (Nitro Enclaves / Fargate / KMS)

📊 Repository Snapshot
Path: C:\Users\gabeg\source\repos\nightcore-worker
Branch: main
Crates: nc_state, nc_exec, nc_exec_cli, nc_exec_wasmtime, nc_exec_firecracker
License: MIT (Open Core) + B106™ brand layer
Status: 🧩 Verified Stable Build — ready for Vesper integration

🧩 Next Steps
Implement backend switching via --backend (Firecracker support)

Add Vesper AI as crates/nc-vesper

Integrate AUFS upgrade audit flow into Worker layer

Extend export-dashboard to include snapshot archives

Prototype Night Mesh LAN proof replication



---

## 📘 **README Update Section**
Add beneath your existing “## Proof Dashboard” heading:


---

## ⚙️ November 2025 — Night Core™ Worker v39 Update
Night Core™ Worker now introduces **persistent state**, a **dark audit dashboard**, and a **modular execution backend** for verified multi-tenant compute.

### New Capabilities
- 🧮 Tenant state tracking via sled (`nc_state`)
- 📜 Persistent proof history and summaries
- 🧱 Modular runtime support (Wasmtime + Firecracker)
- ⚡ Parallel orchestration with timing analytics
- 🌑 Dark ledger dashboard (`--export-dashboard --diff`)
- 🧠 Inspect-state command with per-tenant summaries

### Structure Overview
crates/
├── nc-exec
├── nc-exec-cli
├── nc-exec-firecracker
├── nc-exec-wasmtime
└── nc_state


Night Core™ Worker v39 represents a **verified open-core foundation** for secure, auditable compute — forming the backbone for future layers like **Vesper**, **Guardian**, and **Night Mesh**.

# 🧩 Night Core™ Worker v39 — CLI Command Reference
All commands are invoked through Cargo or the built binary:

```
cargo +nightly run -- <command> [options]
# or
target\debug\nightcore.exe <command> [options]
```

---

### 🔹 1. Run Modules
Run a single tenant or all tenants in parallel, with optional proof mode.

```
nightcore run --all [--proof] [--parallel N]
nightcore run --path modules/tenantA-hello [--proof]
```

**Flags**
- `--all` — Executes all tenants under `/modules`
- `--path` — Runs a specific tenant folder
- `--proof` — Enables proof-only verification mode (writes proof logs, no WASM execution)
- `--parallel` — Controls thread count (default auto-detect; capped at 2 in proof mode)

Outputs:
- `logs/nightcore_dashboard.html`
- `logs/orchestration_report.json`

---

### 🔹 2. Verify Environment
Basic runtime check for Wasmtime + signature libraries.

```
nightcore verify-env
```

---

### 🔹 3. Sign a Tenant Module
Create or refresh digital signatures for tenant modules using an Ed25519 private key.

```
nightcore sign --dir modules/tenantA-hello --key keys/maintainers/admin1.key
```

Generates:
- `module.sig`
- `pubkey.b64`
- `module.sha256`

---

### 🔹 4. Inspect Tenant Manifest
```
nightcore inspect --dir modules/tenantA-hello
```

---

### 🔹 5. Export Pubkey Hashes
```
nightcore export-pubkey-hashes
```

---

### 🔹 6. Verify or Sign AUFS Upgrades
```
nightcore upgrade --manifest upgrades/manifests/upgrade_manifest.json
nightcore sign-upgrade --manifest upgrades/manifests/upgrade_manifest.json --key keys/maintainers/admin1.key
```

---

### 🔹 7. Unlock (Feature Flag)
```
nightcore unlock
```

---

### 🔹 8. Inspect Persistent Tenant State
```
nightcore inspect-state --tenant tenantA-hello [--summary]
nightcore inspect-state --all-tenants [--summary]
```

Outputs proof history + summary metrics.

---

### 🔹 9. Export Historical Dashboard
```
nightcore export-dashboard [--diff]
```

Outputs `logs/nightcore_history_dashboard.html`

---

### 🔹 10. Verify Environment Self-Test
```
nightcore verify-env
```

### 🔹 11. Help
```
nightcore --help


## 🧩 Example Workflow
```
cargo +nightly run -- sign --dir modules/tenantA-hello --key keys/maintainers/admin1.key
cargo +nightly run -- sign --dir modules/tenantB-math --key keys/maintainers/admin1.key
cargo +nightly run -- run --all --proof
cargo +nightly run -- inspect-state --all-tenants --summary
cargo +nightly run -- export-dashboard --diff
```
