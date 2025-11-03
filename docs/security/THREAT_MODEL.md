<p align="center">
  <img src="../../assets/nightcore_logo_tm.png" alt="Night Core™ Logo" width="180"/>
</p>

<h1 align="center">Night Core™ Threat Model</h1>

---

## ⚙️ Overview
The **Night Core™ Threat Model** identifies and mitigates risks associated with executing multi-tenant WebAssembly workloads in a secure, auditable, and autonomous environment.  
It is designed to align with modern zero-trust, AUFS, and AWS Nitro Enclave security paradigms.

---

## 🧠 Core Assumptions
- All WASM modules are **untrusted by default**.  
- Tenants operate in strict sandbox isolation (fuel/time/memory caps).  
- Ed25519 + SHA-256 verification ensures modules cannot be modified post-signature.  
- The AUFS chain and audit log provide **tamper-evident version tracking**.

---

## 🔍 Threat Landscape
| Category | Description | Mitigation |
|-----------|--------------|-------------|
| **Code Injection** | Unauthorized WASM modification or rogue tenant upload. | Ed25519 signature verification and manifest hashing. |
| **Resource Exhaustion** | Infinite loops or fuel misuse to consume host resources. | Fuel and timeout enforcement per tenant policy. |
| **Privilege Escalation** | Attempted access to restricted host FS or network. | WASI P1 sandboxing + FS/NET policy enforcement. |
| **Key Compromise** | Exposure of signing keys or verification keys. | Offline cold-storage for signing keys, AUFS integrity alerts. |
| **Audit Tampering** | Post-event manipulation of logs. | Hash-chained audit logs and reproducible build validation. |

---

## 🛡️ Mitigation Layers
1. **Cryptographic Verification** — Every WASM and manifest must pass Ed25519 + SHA-256 checks.  
2. **Sandbox Isolation** — Per-tenant fuel, memory, and timeout enforcement.  
3. **AUFS Audit Chain** — Immutable hash-chain ensures all system events are verifiable.  
4. **Cold Storage Keys** — Sensitive signing materials stored in `/ops/cold-storage.md` environments.  
5. **External Integrity Proofs** — Planned integration with AWS KMS and Nitro Enclaves for attestation.

---

## 📊 Residual Risk & Future Work
- Continuous improvement of AUFS validation and anomaly detection.  
- Optional integration with **Guardian** kernel for byte-drift detection and rollback control.  
- Expansion of automated proof-of-execution reports via **Night Mesh** nodes.  

---

<p align="center"><i>© 2025 xnfinite — Threat modeling built for verifiable compute.</i></p>


