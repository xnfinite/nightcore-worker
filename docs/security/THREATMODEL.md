# 🛡️ Night Core Security Threat Model
**Version:** 1.0 (B106 Edition)  
**Applies To:** Night Core Runtime + AUFS + Night Mesh v1.0  
**Author:** B106 Labs · 2025  
**Last Updated:** October 2025

---

## 1. Purpose
This document defines Night Core’s **security threat model**, covering its architecture, assets, and defenses.  
It follows the **STRIDE** methodology and references **MITRE ATT&CK**, **ISO 27005**, and **NIST SP 800-53** controls.

---

## 2. Assets to Protect

| Asset | Description | Sensitivity |
|--------|--------------|-------------|
| **Runtime Binary** | The compiled Night Core engine. | Critical |
| **Tenant Modules (.wasm)** | Executable sandboxed workloads. | High |
| **Ed25519 Keys** | Signing and verification keys for modules. | Critical |
| **Upgrade Manifests** | AUFS metadata controlling self-updates. | Critical |
| **Audit Logs / Proof Bundles** | Evidence of integrity and compliance. | High |
| **SBOM / Attestations** | Build provenance and dependency trust. | High |
| **Dashboard HTML** | User-facing log and proof interface. | Medium |

---

## 3. Threat Actors

| Actor | Description | Capability |
|--------|--------------|-------------|
| **Malicious Tenant** | A tenant attempting to escape sandbox or exfiltrate data. | Moderate |
| **Compromised Maintainer Key** | Insider or stolen signing key. | High |
| **Supply Chain Attacker** | Attempts to modify source or CI build pipeline. | High |
| **Unauthorized Fork / Impersonator** | Clone or rebrand without proof or trust validation. | Moderate |
| **External Nation-State / APT** | Targeted persistent threat seeking to backdoor AUFS or Wasmtime. | Critical |

---

## 4. STRIDE Breakdown

| Category | Example Threat | Mitigation in Night Core |
|-----------|----------------|---------------------------|
| **S – Spoofing** | Fake tenant module signed by rogue key. | Ed25519 verification + trusted key registry + revocation list. |
| **T – Tampering** | Modified `.wasm` or manifest file. | SHA-256 integrity check + threshold-signed manifests. |
| **R – Repudiation** | Deletion of logs or denial of changes. | Hash-chained audit log + append-only transparency log. |
| **I – Information Disclosure** | Tenant reads another tenant’s data. | Per-tenant sandbox (FS preopen) + optional `--strongbox` isolation. |
| **D – Denial of Service** | Infinite loop or memory exhaustion in module. | Fuel/time/memory limits enforced by Wasmtime config. |
| **E – Elevation of Privilege** | Tenant breaks out of WASI sandbox. | Minimal WASI capabilities + Wasmtime process isolation + seccomp. |

---

## 5. Mitigation Summary

| Layer | Controls |
|--------|-----------|
| **Cryptographic** | Ed25519 + SHA-256 verification, FIPS toggle, threshold-signed upgrades. |
| **Runtime** | Fuel/time/memory limits, deterministic execution, `--strongbox` sandbox mode. |
| **Policy** | Strict manifest enforcement for FS/NET/ENV capabilities. |
| **Upgrade Chain** | TUF-style root/targets/snapshot/timestamp validation, rollback protection. |
| **Auditability** | Tamper-evident hash chain + proof bundles with SBOM + attestations. |
| **Incident Response** | Revocation lists, cordon commands, forensic bundle export. |

---

## 6. Supply Chain Security

| Risk | Mitigation |
|-------|-------------|
| **Malicious crate dependency** | `cargo vet` + `cargo deny` enforced in CI. |
| **CI compromise** | Reproducible builds verified by second runner + checksum comparison. |
| **Dependency poisoning** | Lockfile review and SBOM validation. |
| **Binary substitution** | Post-build Ed25519 signing + SHA-256 checksum bundle. |

---

## 7. AUFS-Specific Risks

| Risk | Description | Mitigation |
|-------|--------------|------------|
| **Threshold Key Theft** | Attacker steals one AUFS signing key. | Requires 2-of-3 keys for upgrades; key rotation supported. |
| **Rollback Attack** | Reverting to prior vulnerable version. | AUFS refuses downgrades unless dual-signed override. |
| **Fake Upgrade Source** | Attacker hosts spoofed upgrade metadata. | Root.json + snapshot.json signatures validated offline. |
| **Upgrade Tampering** | Modified engine adapter or schema translator. | SHA-256 validation + audit hash chain. |

---

## 8. Cloud & AWS Threats

| Risk | AWS Context | Mitigation |
|-------|--------------|------------|
| **Key Leakage** | KMS or CloudHSM misconfiguration. | Strict IAM roles, audit KMS events, optional external HSM storage. |
| **Tenant Escape** | Fargate/Nitro container breakout. | Enclave isolation + Night Core fuel/mem/time enforcement. |
| **Unauthorized Lambda Trigger** | AUFS Lambda pipeline hijack. | IAM least-privilege roles + signed AUFS manifests only. |
| **Data Corruption in S3** | Object overwrite or versioning disabled. | Enable S3 Versioning + Object Lock + server-side encryption. |

---

## 9. Logging & Audit Strategy

- **Hash-Chained JSONL Logs** stored locally and mirrored to S3/DynamoDB.  
- **Merkle Transparency Root** optionally published per release.  
- **Proof Bundle** contains:
  - Dashboard HTML hash
  - SBOM + checksums
  - audit.tail (final hash)
  - in-toto attestations

These artifacts provide cryptographic evidence of operational integrity.

---

## 10. Residual Risk Assessment

| Risk | Severity | Residual Level | Notes |
|-------|-----------|----------------|-------|
| Tenant misconfiguration | Medium | Low | Controlled by strict manifest schema. |
| Build pipeline downtime | Low | Low | Mitigated via redundant runners. |
| Key rotation delay | High | Medium | Manual review process enforced. |
| Future ABI regression | High | Low | Managed by adapter registry. |

---

## 11. Recommendations

- Enforce **quarterly key rotation** for AUFS maintainers.  
- Automate **revocation propagation** via Night Mesh broadcasts.  
- Mirror audit logs to at least **two storage regions** (e.g., AWS + on-prem).  
- Maintain **cold backups** of all root.json and proof bundles.  
- Enable **FIPS mode** when deployed in regulated or AWS GovCloud environments.

---

## 12. Conclusion

Night Core’s threat model demonstrates **defense-in-depth** across cryptography, runtime isolation, supply chain, and cloud infrastructure.

> **Goal:** No single failure — cryptographic, procedural, or human — should compromise module integrity or upgrade trust.

Night Core AUFS ensures that even if the ecosystem evolves, **trust remains mathematically provable**.

---

**© 2025 B106 Labs · All Rights Reserved**  
Night Core Open-Core edition licensed under MIT; B106 branding and security documentation proprietary to B106 Labs.
