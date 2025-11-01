<!-- Night Core™ v38 — How-To Guide -->

<p align="center">

&nbsp; <img src="../assets/nightcore\_logo\_tm.png" alt="Night Core Logo™" width="260"/>

</p>



<h1 align="center">Night Core™ v38 — Full How-To Guide</h1>



<p align="center">

&nbsp; <strong>Secure • Autonomous • Verified</strong>

</p>



---



\## 🧩 1. Overview

Night Core™ is a verifiable compute framework written in Rust using Wasmtime 37 + WASI P1.  

This guide walks through every step — from initial build to tenant verification, signing, and AUFS upgrades.



---



\## ⚙️ 2. Environment Setup



\### Prerequisites

\- \*\*Rust (nightly)\*\* — install via https://rustup.rs

\- \*\*Git + GPG\*\* — for signing commits and keys

\- \*\*PowerShell 7+\*\* (recommended)

\- \*\*Wasmtime 37\*\* — installed automatically by Cargo



\### Directory Layout

```

C:\\Users\\gabeg\\source\\repos\\worker

│   Cargo.toml

│   README.md

│

├── src\\

│   ├── main.rs

│   └── sign\_tenant.rs

│

├── modules\\

│   ├── tenantA-hello\\

│   └── tenantB-math\\

│

├── keys\\

│   └── maintainers\\

│

└── logs\\

```



---



\## 🔐 3. Key Generation \& Signing



\### Generate Maintainer Keys

```bash

cargo +nightly run -- generate-keys --out-dir keys/maintainers

```



\### Sign a Tenant Module

```bash

cargo +nightly run -- sign --dir modules/tenantA-hello --key keys/maintainers/admin1.key

```

Outputs:

\- module.sig

\- pubkey.b64

\- module.sha256



---



\## 🧩 4. Running Tenants



\### Verify Environment

```bash

cargo +nightly run -- verify-env

```



\### Run All Tenants

```bash

cargo +nightly run -- run --all

```



Night Core:

\- Verifies each module’s Ed25519 signature  

\- Checks SHA-256 integrity  

\- Runs each .wasm in a sandbox  

\- Records audit data in logs/orchestration\_report.json  

\- Updates logs/nightcore\_dashboard.html



---



\## 🔁 5. AUFS — Autonomous Upgrade \& Fork System



\### Submit \& Verify an Upgrade

```bash

cargo +nightly run -- upgrade --manifest upgrades/manifests/upgrade\_manifest.json

```



\### Sign an Upgrade Manifest (2-of-3)

```bash

cargo +nightly run -- sign-upgrade --manifest upgrades/manifests/upgrade\_manifest.json

```



\### Safe Push Workflow

Each commit triggers the Night Core Foundation Lock hook:

```

🔒 Night Core Foundation Lock active...

✅ Hook active — baseline integrity verified.

```

Only verified baselines are accepted into main.



---



\## 🧮 6. Proof Mode

Proof Mode verifies signatures and hashes without executing .wasm:

```bash

cargo +nightly run -- run --all --proof

```

Outputs:

\- logs/nightcore\_proof.html

\- logs/audit.log  

\- Chronicle sync via scripts/nightcore\_manual\_proof\_push.ps1



---



\## 🧠 7. Troubleshooting

| Issue | Cause | Fix |

|-------|--------|-----|

| ❌ “input bytes aren’t valid utf-8” | Binary .wasm printed raw data | Use text or filter output |

| ❌ “signature verification failed” | Out-of-date signature | Re-sign the module |

| ❌ “gpg: no agent running” | Windows GPG lock corruption | Run scripts/fix\_gpg\_agent.ps1 |

| ❌ “manifest missing” | No manifest.json in tenant dir | Copy from template in /modules/example |



---



\## 📜 8. Logs \& Proofs

Generated automatically:

\- logs/audit.log — hash-chained integrity record  

\- logs/nightcore\_dashboard.html — visual proof summary  

\- logs/orchestration\_report.json — machine-readable run report  



---



\## ☁️ 9. AWS Integration (Preview)

Night Core supports deployment via:

\- AWS Nitro Enclaves for isolation  

\- AWS KMS/CloudHSM for key management  

\- S3/DynamoDB for audit and manifest storage  

\- Lambda/EventBridge for automated AUFS triggers  



---



\## 🪶 10. Version Tags

| Tag | Description |

|------|-------------|

| v38-stable-aufs-verified | Verified AUFS baseline |

| v38-proof-mode | Proof-only run completed |

| v38-dashboard | HTML proof dashboard published |



---



\## ⚡ 11. CLI Quick Reference



| Command | Description |

|:--------|:-------------|

| `cargo +nightly run -- verify-env` | Verify Wasmtime + environment setup |

| `cargo +nightly run -- generate-keys --out-dir keys/` | Generate Ed25519 key pairs |

| `cargo +nightly run -- sign --dir <tenant> --key <key>` | Sign a tenant module |

| `cargo +nightly run -- run --all` | Run all verified tenants |

| `cargo +nightly run -- run --all --proof` | Proof-only verification mode |

| `cargo +nightly run -- export-pubkey-hashes` | Export tenant public key hashes |

| `cargo +nightly run -- upgrade --manifest <file>` | Run AUFS upgrade verification |

| `cargo +nightly run -- sign-upgrade --manifest <file>` | Sign an AUFS upgrade manifest |

| `cargo +nightly run -- help` | Show command list |



---



<p align="center"><i>Part of the Night Core™ Secure Compute Stack — B106 Edition.</i></p>



