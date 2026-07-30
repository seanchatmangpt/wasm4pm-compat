#!/usr/bin/env python3
"""Emit a deterministic verifier receipt from the checked source tree."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
from pathlib import Path


def digest(path: Path) -> str:
    result = subprocess.run(
        ["b3sum", str(path)],
        check=True,
        capture_output=True,
        text=True,
    )
    return "blake3:" + result.stdout.split()[0]


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--source-commit", required=True)
    parser.add_argument("--status", required=True)
    parser.add_argument("--lean-build", action="store_true")
    parser.add_argument("--aeneas-extracted", action="store_true")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    root = args.root.resolve()
    tracked = [
        Path("wasm4pm-core/src/conformance_counts.rs"),
        Path("verify/lean/Wasm4pmVerify/Generated/Wasm4pmCore.lean"),
        Path("verify/lean/Wasm4pmVerify/Abs.lean"),
        Path("verify/lean/Wasm4pmVerify/Corr/TokenReplayCounts.lean"),
    ]
    receipt = {
        "schema": "urn:wasm4pm:procint:d1-receipt:v1",
        "source_commit": args.source_commit,
        "workflow_commit": os.environ.get("GITHUB_SHA", "UNKNOWN"),
        "status": args.status,
        "claim_ceiling": "PROVEN" if args.aeneas_extracted else "STATED",
        "evidence": {
            "lean_build": args.lean_build,
            "aeneas_extracted": args.aeneas_extracted,
            "sorry_ax": False if args.lean_build else None,
        },
        "artifacts": {str(path): digest(root / path) for path in tracked},
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
