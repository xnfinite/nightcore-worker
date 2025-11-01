<p align="center">
  <img src="../assets/nightcore_logo_tm.png" alt="Night Core Logo" width="280"/>
</p>

<h1 align="center">Night Core� AUFS � Autonomous Upgrade & Fork System</h1>

<p align="center">
  <strong>Threshold-Signed  Self-Healing  Tamper-Evident</strong>
</p>

---

## ?? Overview
AUFS (Autonomous Upgrade & Fork System) enables Night Core to self-upgrade and fork securely.  
Each upgrade proposal is cryptographically signed by maintainers (2-of-3 threshold) and verified before execution.

##  Architecture
- Threshold Ed25519 signatures
- Manifest-based schema adapters for WASI upgrades
- Reproducible builds for deterministic integrity
- Hash-chained audit logs and transparency proofs

##  Workflow
1. **Proposal:** Maintainers submit signed upgrade manifest (upgrade.json)
2. **Verification:** Night Core validates all Ed25519 signatures and SHA-256 hashes
3. **Execution:** Approved upgrade runs in isolated AUFS sandbox
4. **Audit:** New baseline recorded in oundation/baseline.json and logged to logs/audit.log

##  Security Design
- Threshold-signed approvals (2-of-3)
- SHA-256 manifest integrity
- Immutable audit chain linking old and new baselines
- Reversible rollback system for failed or tampered updates

##  Purpose
AUFS ensures the Night Core framework can:
- Evolve without central authority
- Maintain enterprise-grade trust and auditability
- Enable reproducible forks for verified downstream ecosystems

<p align="center"><i>Part of the Night Core Secure Compute Stack.</i></p>


