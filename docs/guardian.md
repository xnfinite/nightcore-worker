$guardian = @"

<p align="center">

&nbsp; <img src="https://raw.githubusercontent.com/xinfinite/worker/main/assets/nightcore\_logo\_tm.png" alt="Night Core™ Logo" width="280"/>

</p>



<h1 align="center">Night Core™ Guardian — AI Containment Kernel</h1>



<p align="center">

&nbsp; <strong>Fuel Limits • Byte Drift Detection • Rollback Safety</strong>

</p>



---



\## 🔒 Overview

Guardian is the Night Core security layer that continuously monitors running WASM tenants for behavioral drift, fuel overconsumption, and state tampering.  

When abnormal patterns appear, Guardian automatically enforces sandbox rollback or termination, maintaining system integrity across autonomous environments.



---



\## ⚙️ Core Responsibilities

\- \*\*Fuel Enforcement Engine\*\* — monitors instruction and memory usage per tenant.  

\- \*\*Byte Drift Detector\*\* — compares current module hash against baseline SHA-256 fingerprint.  

\- \*\*Rollback Protocol\*\* — reverts sandbox state to last trusted checkpoint.  

\- \*\*Containment Hooks\*\* — intercepts anomalous syscalls via WASI boundary.  

\- \*\*Incident Reporting\*\* — appends tamper events to `logs/audit.log` and updates the dashboard feed.



---



\## 🧩 Architecture

Guardian is implemented as a WASI P1 extension module within Night Core’s multi-tenant runtime.  

It runs in parallel with AUFS to ensure that both upgrades and executions remain cryptographically verifiable.



```text

┌─────────────────────────────┐

│  Tenant Module              │

│  (e.g., tenantA-hello)      │

├─────────────────────────────┤

│  Guardian Runtime Monitor   │

│  (fuel + drift sensors)     │

├─────────────────────────────┤

│  AUFS Upgrade Layer         │

├─────────────────────────────┤

│  Night Core Orchestrator    │

└─────────────────────────────┘



