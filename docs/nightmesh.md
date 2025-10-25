<p align='center'><img src='../assets/nightcore_logo_tm.png' alt='Night Core Logo' width='280'/></p>

<h1 align='center'>Night Core Night Mesh  Distributed Proof & Audit Synchronization</h1>

<p align='center'><strong>Cross-Node Verification  Tamper-Evident Sync  Encrypted Mesh Network</strong></p>

---

##  Overview
Night Mesh is the distributed verification and audit synchronization layer of Night Core. It allows multiple nodes to share proof bundles, verify logs, and maintain global integrity without central authority.

---

##  Core Functions
- Proof Synchronization  exchanges signed proof bundles between nodes.  
- Audit Gossip Protocol  uses encrypted mesh channels for peer-to-peer log updates.  
- Tamper Detection  verifies hash chains to ensure no node diverges from baseline.  
- Resilience  maintains operational continuity even if a subset of peers go offline.

---

##  Security & Encryption
- End-to-end AES-256-GCM encryption for all proof transfers.  
- Peer authentication via Ed25519 public keys.  
- Hash-based tamper detection before synchronization.  

---

##  Configuration Reference
Defined in configs/policy.default.toml.

---

<p align='center'><i>Part of the Night Core Secure Compute Stack  Ensuring Integrity Beyond a Single Node.</i></p>
