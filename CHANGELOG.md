# 🧩 Night Core™ Worker — Changelog

This document tracks verified build milestones and framework evolution under the Night Core™ ecosystem.

---

## v39.2 — AUFS + Chronicle Auto-Sync (2025-11-03)

- ✅ First fully autonomous AUFS + Chronicle verification pipeline.  
- 🔐 Ed25519 + SHA-256 integrity checks validated across `Cargo.toml` and `src/main.rs`.  
- 🧾 Immutable proof archives generated → `logs/aufs_audit.html` and `logs/archive/`.  
- 🪶 Chronicle ledger appended and clearsigned with maintainer key (`DDB2CE648EEC90D40C368AFB11980F4DCB664279`).  
- 📦 Release tag `v39.2-pro-verified` published.

---

## v38 — Verified Baseline (2025-10-28)

- 🧱 Established multi-tenant orchestration (Tenant A & Tenant B).  
- 🔑 Ed25519 signature + SHA-256 integrity verification.  
- ⚙️ WASI P1 sandbox runtime (WASMTIME 37).  
- 📊 HTML dashboard and JSONL orchestration logs introduced.  
- 🪪 Baseline MIT open-core license confirmed.  

---

**Maintainer:** `xnfinite — Night Core Maintainer`  
**License:** MIT (Open Core) • B106 Edition branding reserved
