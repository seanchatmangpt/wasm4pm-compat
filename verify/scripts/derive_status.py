#!/usr/bin/env python3
"""Derive D1 standing from observed evidence and refuse illegal promotion."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any


class StatusRefusal(RuntimeError):
    pass


def derive_status(evidence: dict[str, Any]) -> str:
    if not evidence.get("declared", False):
        return "UNKNOWN"
    if not evidence.get("lean_build", False):
        return "DECLARED"
    if evidence.get("sorry_ax", True):
        return "BUILD_BROKEN"
    if not evidence.get("aeneas_extracted", False):
        return "STATED"
    if not evidence.get("source_hash_matches", False):
        return "EXTRACTED"
    return "PROVEN"


def validate_claim(evidence: dict[str, Any]) -> str:
    derived = derive_status(evidence)
    claimed = evidence.get("claimed_status")
    if claimed is not None and claimed != derived:
        if claimed == "PROVEN" and derived != "PROVEN":
            raise StatusRefusal("STATED_PROMOTED_TO_PROVEN")
        raise StatusRefusal(f"STATUS_MISMATCH:{claimed}:{derived}")
    return derived


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("evidence", type=Path)
    args = parser.parse_args()

    evidence = json.loads(args.evidence.read_text(encoding="utf-8"))
    try:
        status = validate_claim(evidence)
    except StatusRefusal as refusal:
        print(str(refusal), file=sys.stderr)
        return 64
    print(status)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
