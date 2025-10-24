# ❄️ Night Core Cold Storage and Archival Plan
**Version:** 1.0 (B106 Edition)  
**Author:** B106 Labs  
**Last Updated:** October 2025  

---

## 1. Purpose
The **Cold Storage and Archival Plan** defines how B106 Labs and verified partners securely back up, preserve, and verify Night Core’s most critical cryptographic and operational assets.

Cold storage ensures that even in catastrophic events, **keys, proofs, and audit trails** can be restored without loss of trust or historical continuity.

---

## 2. Assets Protected

| Asset | Description | Storage Class |
|--------|--------------|----------------|
| **Root Keys (offline)** | AUFS root.json + master Ed25519/FIPS keys. | Tier 1 (Offline Vault) |
| **Upgrade Manifests** | Threshold-signed AUFS version manifests. | Tier 2 (Encrypted Drive) |
| **Proof Bundles** | SBOM, checksums, audit.tail, dashboard hashes. | Tier 2 |
| **Audit Chain Logs** | `logs/audit.jsonl` + `audit.tail` file. | Tier 2 |
| **Revocation Lists** | `keys/revoked_keys.list` for compromised keys. | Tier 2 |
| **Cold Signing Certificates** | Revocation and rotation ceremony records. | Tier 1 |
| **Critical Documentation** | Governance, Threat Model, AUFS design docs. | Tier 3 (Immutable Archive) |

---

## 3. Storage Tiers

| Tier | Location | Access Policy | Refresh Interval |
|------|-----------|----------------|------------------|
| **Tier 1 — Offline Vault** | Air-gapped encrypted drives stored in two physical locations. | Chief Maintainer + 1 Security Maintainer (dual custody). | Every 90 days. |
| **Tier 2 — Encrypted Cloud Backup** | AES-256 encrypted volumes in AWS S3 Glacier Deep Archive (or equivalent). | Read-only by Night Core AUFS automation; writes signed. | Every 30 days. |
| **Tier 3 — Immutable Archive** | Public proof bundle mirrors (GitHub + IPFS + Archive.org). | Public read-only. | Every release. |

---

## 4. Cryptographic Protections

- **Encryption:** AES-256-GCM for all cold archives.  
- **Signatures:** Detached Ed25519 or ECDSA signatures over encrypted blobs.  
- **Hash Chain:** SHA-256 integrity proof stored separately from encrypted data.  
- **Redundancy:** Minimum two copies per geographic region (US + EU).  
- **FIPS Mode (optional):** FIPS 140-3 compliant crypto modules when deployed to AWS GovCloud or regulated regions.

---

## 5. Backup Workflow

1. AUFS performs automated **proof bundle** export:
   - `proof/` → SBOM, attestation
