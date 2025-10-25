<p align="center">
  <img src="../../assets/nightcore_logo_tm.png" alt="Night Core™ Logo" width="180"/>
</p>

<h1 align="center">Night Core™ Governance Framework</h1>

---

## 🧭 Overview
The **Night Core™ Governance Framework** defines the decision-making structure, contributor roles, and compliance requirements that protect the Night Core ecosystem.  
It ensures every change, commit, and upgrade follows verified, auditable procedures consistent with the Foundation Lock and AUFS standards.

---

## 🎯 Purpose
- Maintain verifiable integrity of the Night Core open-core codebase.  
- Enforce cryptographic signature and audit-chain compliance.  
- Define transparent contributor responsibilities and escalation paths.  
- Preserve the Night Core brand, naming, and licensing boundaries.

---

## 👥 Roles & Responsibilities
| Role | Description |
|------|--------------|
| **Maintainer** | Approves PRs, validates audit logs, enforces signing and AUFS thresholds. |
| **Contributor** | Submits code or documentation under MIT terms; must pass lint and signature checks. |
| **Auditor** | Reviews cryptographic logs (`logs/audit.log`), reports anomalies, verifies post-commit hashes. |
| **Foundation Custodian** | Oversees baseline.json, Foundation Lock integrity, and long-term version governance. |

---

## 🧩 Commit Verification Policy
All commits are subject to Night Core’s **Foundation Lock** and **audit hash chain** verification.  
By default:
- Every commit triggers pre-commit and post-commit hooks.  
- Bypass commits (`--no-verify`) must include a justification message recorded in `logs/audit.log`.  
- Threshold signatures (2-of-3) are required for official AUFS releases.

---

## 🪶 Licensing & Branding
- The **Night Core™** name, **B106 Edition**, and dashboard visuals are proprietary trademarks of **Gabriel Ginn / B106 Labs**.  
- All source code under `/src`, `/modules`, and `/ops` is MIT-licensed.  
- Commercial forks must maintain visible attribution and audit transparency.

---

<p align="center"><i>© 2025 Gabriel Ginn — Governance built for verifiable autonomy.</i></p>
