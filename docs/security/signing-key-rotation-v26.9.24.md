# Signing-key rotation (v26.9.24)

Recorded 2026-09-24 (fleet key scan after the single-repo migration). Base `c4fadd8e7d56` of `wasm4pm-compat`.
Every private key listed here was committed to this repository and is therefore compromised: every receipt or
attestation signed with it carries no signing authority (standing REFUSED, broken_term R_missing_authority).
The keys leave the tree (history is not rewritten; no force-push), and each key directory's `.gitignore` now
covers both halves. Every checkout keeps its own pair: ggen generates one on first use, and a tracked public
half without its private half would make that first `ggen sync` refuse [FM-KEY-010/011]. The canonical
checkout's new public key is published below for anyone verifying its future receipts.

| key dir | removed private key sha256 | removed public key sha256 | new public key (canonical checkout) |
|---|---|---|---|
| `ggen/.ggen/keys` | `8917155e2ab6c565cc5afe01a7fdade25a10e9f47e353985fae1773fada3c424` | `0e02dd099abe40dd8db424ff049e2668eb33b70bb8f8f4ea44a754f4a2708804` | `be22c172f02144970c98830f0bb02a3c5649f6ab4b235db8e8a2d21ac97fee8d` |
| `ggen/templates-breeds/.ggen/keys` | `1dd4b84639a6b724ebf1d1602a311e6caafec0744d55c79f4b4717b137c09243` | `d4e91685c08f3c3d707c9e42584d4a12b6c6cf944be034d4d425f274a0318438` | `5321d5cffcec3f9bc64f785ac7e53e1c3691cd5a2a70fd468a589e7ed8955af7` |
