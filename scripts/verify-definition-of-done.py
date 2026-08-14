#!/usr/bin/env python3
"""Verify the repository Definition of Done contract and emit an exact-tree receipt."""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CONTRACT_PATH = ROOT / "definition-of-done.toml"
CARGO_PATH = ROOT / "Cargo.toml"

EXPECTED_SCHEMA = "wasm4pm-compat/definition-of-done/v1"
EXPECTED_CAPABILITY_EVIDENCE = {
    "bounded-scope",
    "named-refusal",
    "positive-verification",
    "negative-verification",
    "format",
    "clippy",
    "unit-integration",
}
EXPECTED_LANES = {
    "admission",
    "inspection",
    "capabilities",
    "gall_checkpoints",
}
EXPECTED_WORKFLOWS = {
    "CI Control Plane",
    "Build Matrix",
    "Security Release",
    "Doctor Multi-Runner",
    "Repair Rustfmt",
}


def refuse(message: str) -> None:
    raise SystemExit(f"DefinitionOfDoneRefused: {message}")


def git_tree() -> str:
    result = subprocess.run(
        ["git", "rev-parse", "HEAD^{tree}"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return result.stdout.strip()


def load_toml(path: Path) -> dict:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def require_exact_set(observed: list[str], expected: set[str], label: str) -> None:
    observed_set = set(observed)
    if observed_set != expected or len(observed) != len(expected):
        refuse(
            f"{label} drift: observed={sorted(observed_set)!r} "
            f"expected={sorted(expected)!r}"
        )


def main() -> int:
    contract = load_toml(CONTRACT_PATH)
    cargo = load_toml(CARGO_PATH)

    if contract.get("schema") != EXPECTED_SCHEMA:
        refuse(f"schema drift: {contract.get('schema')!r}")
    if contract.get("verifier") != "scripts/verify-definition-of-done.py":
        refuse("verifier path drift")

    capability = contract["capability"]
    pull_request = contract["pull_request"]
    release = contract["release"]
    dfcm = contract["dfcm"]
    feature_contract = contract["features"]

    require_exact_set(
        capability["required_evidence"],
        EXPECTED_CAPABILITY_EVIDENCE,
        "capability evidence",
    )
    if capability["standing"] != "PARTIAL_ALIVE":
        refuse("capability DoD attempted crown promotion")

    require_exact_set(pull_request["required_lanes"], EXPECTED_LANES, "PR lanes")
    if pull_request["standing"] != "PARTIAL_ALIVE":
        refuse("PR DoD attempted crown promotion")
    for key in (
        "requires_exact_subject",
        "requires_definition_receipt",
        "requires_external_standing",
    ):
        if pull_request[key] is not True:
            refuse(f"PR DoD weakened {key}")

    require_exact_set(release["required_workflows"], EXPECTED_WORKFLOWS, "release workflows")
    if release["required_standing"] != "ALIVE":
        refuse("release DoD no longer requires ALIVE")
    if release["requires_exact_subject"] is not True:
        refuse("release DoD no longer requires exact subject")
    if release["requires_owner_merge_authorization"] is not True:
        refuse("release DoD no longer requires owner merge authorization")

    formats = int(dfcm["formats"])
    directions = int(dfcm["directions"])
    transports = int(dfcm["transports"])
    feature_axes = int(dfcm["feature_axes"])
    if int(dfcm["construction_cells"]) != formats * directions * transports:
        refuse("DFCM connector construction cell count is not closed")
    if int(dfcm["admission_cells"]) != formats * formats:
        refuse("DFCM admission cell count is not closed")
    if int(dfcm["feature_cells"]) != 2**feature_axes:
        refuse("DFCM feature powerset is not closed")

    axes = feature_contract["axes"]
    require_exact_set(axes, {"formats", "strict", "wasm4pm"}, "feature axes")
    if len(axes) != feature_axes:
        refuse("feature axis cardinality drift")

    cargo_features = cargo.get("features", {})
    public_feature_axes = set(cargo_features) - {"default"}
    if public_feature_axes != set(axes):
        refuse(
            f"Cargo feature drift: observed={sorted(public_feature_axes)!r} "
            f"expected={sorted(axes)!r}"
        )
    if cargo_features.get("default") != feature_contract["default"]:
        refuse("Cargo default feature contract drift")

    lane_results = {
        "admission": os.environ.get("DOD_ADMISSION_RESULT", "missing"),
        "inspection": os.environ.get("DOD_INSPECTION_RESULT", "missing"),
        "capabilities": os.environ.get("DOD_CAPABILITIES_RESULT", "missing"),
        "gall_checkpoints": os.environ.get("DOD_GALL_RESULT", "missing"),
    }
    failed = {name: result for name, result in lane_results.items() if result != "success"}
    if failed:
        refuse(f"dependency closure failed: {failed!r}")

    source_commit = os.environ.get("DOD_SOURCE_COMMIT") or os.environ.get("GITHUB_SHA")
    if not source_commit:
        refuse("exact source commit unavailable")
    source_tree = git_tree()
    if len(source_commit) != 40 or len(source_tree) != 40:
        refuse("exact subject identity is not canonical SHA-1 length")

    receipt = {
        "schema": "wasm4pm-compat/definition-of-done-receipt/v1",
        "source_commit": source_commit,
        "source_tree": source_tree,
        "standing": "PARTIAL_ALIVE",
        "definitions": {
            "capability": {
                "id": capability["id"],
                "passed": True,
                "required_evidence": capability["required_evidence"],
            },
            "pull_request": {
                "id": pull_request["id"],
                "passed": True,
                "lanes": lane_results,
            },
            "release": {
                "id": release["id"],
                "required_standing": release["required_standing"],
                "required_workflows": release["required_workflows"],
                "requires_owner_merge_authorization": True,
            },
        },
        "dfcm": {
            "construction_cells": formats * directions * transports,
            "admission_cells": formats * formats,
            "feature_cells": 2**feature_axes,
        },
    }

    output = Path(os.environ.get("DOD_RECEIPT_PATH", "target/definition-of-done/receipt.json"))
    if not output.is_absolute():
        output = ROOT / output
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
    print(json.dumps(receipt, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
