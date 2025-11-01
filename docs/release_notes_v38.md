<!-- Night Core™ v38 — Release Notes (Stable Verified Build) -->
<p align="center">
  <img src="../assets/nightcore_logo_tm.png" alt="Night Core Logo™" width="260"/>
</p>

<h1 align="center">Night Core™ v38 — Stable Verified Release Notes</h1>

<p align="center">
  <strong>Secure • Autonomous • Verified</strong>
</p>

---

## 🧩 Build Overview
**Version:** v38 Stable  
**Release Date:** 2025-10-31  
**Commit ID:** `26c43b3`  
**Maintainers:** core-ops • system-check  
**Audit Hash (SHA-256):**  
`6dfaebee909b96f077e0d668b5c401d68ee44bbe3937e647f8aafe12dbf06cb5`  
**Status:** ✅ Verified

---

## 🌑 Summary
Night Core™ v38 introduces a verified and reproducible baseline for secure multi-tenant orchestration,  
featuring AUFS threshold upgrades, proof mode verification, and Ed25519-signed module validation.

This marks the **first fully verified AUFS chain build**, ensuring integrity across all tenant modules, manifests, and logs.

---

## ⚙️ Core Improvements
- ✅ **AUFS Integration:** Threshold-signed (2-of-3) upgrade manifests  
- ✅ **Proof Mode:** Dedicated CLI flag `--proof` for verification-only runs  
- ✅ **Multi-Tenant Runner:** Orchestrates all tenants via `--all`  
- ✅ **Ed25519 + SHA-256:** Full chain signature + integrity checks  
- ✅ **Dashboard Logs:** Automatic HTML & JSONL outputs  
- ✅ **FIPS Mode:** Optional via `--crypto-mode fips`  
- ✅ **Safe Push:** Foundation lock hook validates baseline before commits  

---

## 🔐 Security Enhancements
- Threshold-signed AUFS manifests (2-of-3 model)  
- Enforced reproducible builds for audit integrity  
- Hash-chained logs and proof entries  
- Signature verification enforced for every `.wasm` module  
- Sandboxed runtime limits (fuel, timeout, memory)  
- Verified release policy fingerprint check (`RELEASE_POLICY.md`)  

---

## 🧠 Developer Notes
- `main.rs` defines the immutable v38 baseline (AUFS + Proof Mode integrated)  
- `verify.rs` supports proof-only verification workflows  
- `sign_tenant.rs` handles Ed25519 signing for all modules  
- All `.wasm` tenants validated before execution (tenantA-hello, tenantB-math)  
- Manual Chronicle sync supported via PowerShell script (`scripts/nightcore_manual_proof_push.ps1`)  

---

## 📊 Verification Results
| Check | Status | Description |
|:------|:--------|:-------------|
| AUFS Threshold | ✅ Passed | 2-of-3 maintainer signatures valid |
| Tenant Integrity | ✅ Verified | tenantA + tenantB SHA-256 matched |
| Audit Log Chain | ✅ Linked | `logs/audit.log` extended cleanly |
| Proof Dashboard | ✅ Updated | `logs/nightcore_proof.html` generated |
| Foundation Lock | ✅ Active | GPG-signed commit chain verified |

---

## 🪶 Tag
**`v38-stable-aufs-verified`**

---

<p align="center"><i>Part of the Night Core™ Secure Compute Stack — B106 Edition.</i></p>
