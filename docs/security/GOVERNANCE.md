# 🧭 Night Core Governance & Maintainer Policy
**Edition:** B106 (Open-Core MIT)  
**Applies To:** Core Runtime, AUFS, and Night Mesh Infrastructure  
**Version:** 1.0 · October 2025  
**Author:** B106 Labs

---

## 1. Purpose
This document defines the **governance, approval, and cryptographic control structure** for Night Core and its Autonomous Upgrade & Fork System (AUFS).  
It ensures upgrades, forks, and key management are executed in a **secure, transparent, and auditable** manner consistent with enterprise expectations (ISO 27001, SOC 2, and AWS compliance frameworks).

---

## 2. Governance Philosophy

Night Core governance follows a **hybrid open-core model**:
- The **MIT-licensed core** remains free and community-maintained.  
- The **B106 Edition** (branding + dashboard design) and **AUFS control layer** remain **proprietary** to B106 Labs.  
- Security, upgrades, and major policy changes are performed under a **threshold-maintainer model** to prevent unilateral actions.

---

## 3. Organizational Roles

| Role | Responsibility | Authority Level |
|------|-----------------|-----------------|
| **Chief Maintainer (B106 Lead)** | Final authority on releases, cryptographic policy, and brand use. | Level 3 |
| **Security Maintainers (2+)** | Approve or reject AUFS upgrades and key rotations (threshold signatures). | Level 2 |
| **Core Developers** | Contribute code under MIT; must pass code review and signature verification. | Level 1 |
| **Community Contributors** | Submit PRs, issues, and documentation updates; no signing authority. | Level 0 |
| **Enterprise Partners** | May operate forks or extensions under open-core terms. | Delegated |
| **AWS / Enterprise Auditors** | Read-only access to audit bundles, proofs, and attestations. | External |

---

## 4. Decision-Making Hierarchy

| Decision Type | Required Approvals | Logging Requirement |
|----------------|--------------------|---------------------|
| **Runtime / API Change** | Chief Maintainer + 1 Security Maintainer | Audit Chain Entry |
| **AUFS Upgrade Manifest** | 2-of-3 Maintainers (threshold-signed) | Audit + Proof Bundle |
| **Key Generation / Rotation** | 2-of-3 Maintainers | Audit + Cold Storage Record |
| **Policy Relaxation (e.g. enabling network)** | Chief Maintainer + Dual-Sign | Audit Chain |
| **Emergency Revocation / Cordon** | 1 Maintainer + Post-Approval | Audit Entry |
| **Branding / License Changes** | Chief Maintainer only | Legal Record |

---

## 5. Key Management Policy

| Policy Area | Description |
|--------------|-------------|
| **Root Keys** | Stored offline; used only for major version bootstrap. |
| **Upgrade Keys (AUFS)** | 3 active maintainers; any 2 must sign new upgrade manifests. |
| **Module Signing Keys** | Issued per tenant; trusted via manifest and revocation registry. |
| **Revocation Procedure** | Compromised keys appended to `keys/revoked_keys.list` with timestamp and reason. |
| **Rotation Schedule** | Every 180 days or upon security incident. |
| **Cold Storage** | Root and AUFS keys stored on air-gapped media in two physical locations. |

---

## 6. Approval Workflow (AUFS)

1. **Draft Upgrade Proposal**
   - Core developers submit proposed upgrade manifest.
   - Includes SHA-256 of target binaries and schema diffs.
2. **Peer Review**
   - Security maintainers review code, verify reproducibility.
3. **Threshold Signing**
   - Two maintainers apply Ed25519 (or FIPS ECDSA) signatures to manifest.
4. **Self-Test & Verification**
   - AUFS executes sandbox test run before commit.
5. **Audit & Publication**
   - Event recorded in `logs/audit.jsonl`.
   - Proof bundle (`proof/upgrade-proof.jsonl`) published to GitHub and/or AWS S3.

---

## 7. Conflict Resolution Policy

- Disputes about roadmap or governance are resolved by **consensus among maintainers**.
- In absence of consensus:
  1. Chief Maintainer’s decision prevails for **security or brand issues**.
  2. Public RFC process applies for **feature disputes**.
  3. Forks are permitted under MIT, provided they respect branding boundaries.

---

## 8. Release & Version Control

| Artifact | Managed By | Integrity Proof |
|-----------|-------------|----------------|
| **Releases (Tags)** | Chief Maintainer | Signed Git Tag + Proof Bundle |
| **Proof Bundles** | Auto-Proof Workflow | SHA-256 checksums + SBOM |
| **Audit Logs** | AUFS Subsystem | Hash-Chained Entries |
| **Binary Builds** | CI/CD Reproducible Builds | in-toto attestations |
| **SBOM Files** | Auto-Proof + CI | CycloneDX Format |
| **Documentation** | Community & Core | Markdown Review PRs |

---

## 9. AWS Enterprise Governance Alignment

To support AWS enterprise and potential acquisition readiness:

| Control | AWS Equivalent | Implementation |
|----------|----------------|----------------|
| **IAM Segregation** | IAM Roles & SCPs | Maintain separate AWS accounts for build, proof, and key operations. |
| **KMS / CloudHSM** | Key Custody | Store AUFS signing keys in HSM-backed keyrings. |
| **Change Management** | AWS Change Manager / CloudTrail | Mirror audit logs into CloudTrail-compatible format. |
| **Access Logging** | CloudWatch Logs | Forward AUFS audit events. |
| **Compliance Frameworks** | AWS Audit Manager | Map AUFS evidence bundles to ISO 27001 / SOC 2 controls. |

---

## 10. Brand and License Enforcement

| Element | Status | Enforcement |
|----------|---------|-------------|
| **Night Core Name** | Open-core under MIT | Attribution required |
| **B106 Edition Name** | Proprietary trademark | Exclusive use by B106 Labs |
| **Dashboard Visuals** | Proprietary | Reuse requires license or re-skin |
| **Documentation** | Open for educational use | Must retain MIT and B106 attribution |
| **Commercial Distribution** | Allowed under MIT | Branding license required for resale |

---

## 11. Audit & Compliance Artifacts

Each release of Night Core must include:

1. **Proof Bundle**  
   - SBOM, checksums, audit.tail, attestations  
2. **Audit Log Extract**  
   - SHA-256 hash chain verification proof  
3. **Cold Storage Record**  
   - Secure key manifest signed offline  
4. **Change Record**  
   - YAML manifest of changes and signatures  
5. **Public Digest**  
   - Posted summary (Substack / GitHub Releases)

---

## 12. Termination & Succession Plan

If B106 Labs ceases active maintenance:

1. The **root.json** and keychain are released to the **Night Core Foundation** under MIT.  
2. Threshold keys are rotated under supervision of successor maintainers.  
3. Branding rights remain with B106 Labs (no transfer).  
4. AUFS self-verification and schema adapters continue functioning autonomously.

---

## 13. Governance Summary

| Pillar | Principle |
|---------|------------|
| **Transparency** | All security decisions and upgrades are logged and signed. |
| **Redundancy** | 2-of-3 approval model prevents unilateral actions. |
| **Accountability** | Audit logs and proof bundles serve as legal evidence of intent. |
| **Continuity** | AUFS ensures continuity even if maintainers change. |
| **Openness** | MIT license ensures freedom to fork and evolve responsibly. |

---

**© 2025 B106 Labs**  
Night Core is an open-core MIT project; B106 Labs reserves trademarks and visual branding for the “B106 Edition.”  
This document is proprietary to B106 Labs and may be shared with enterprise partners or auditors under NDA or standard review agreements.
