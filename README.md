# Night Core Pro — Licensed Unlocker (v38.1)

- Unlocks: AUFS, Cloud (WIP), Guardian (WIP — not trademarked)
- Verifies Pro license and AUFS manifests (Ed25519 + SHA-256)
- Works *with* the open-core repo: **nightcore-worker**

## Quick Start
`powershell
# Build Pro
cargo +nightly build --release

# Verify license & AUFS
.\target\release\nightcore-pro.exe checklicense
.\target\release\nightcore-pro.exe aufs
Notes
Night Core™ is a trademark of xnfinite.

Guardian is not trademarked.
